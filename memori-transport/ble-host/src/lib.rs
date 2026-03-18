use btleplug::api::Characteristic;
use btleplug::api::{
    Central, Manager as _, Peripheral as _, ScanFilter, ValueNotification, WriteType,
    bleuuid::uuid_from_u16, CentralEvent
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use memori_ui::MemoriState;
use memori_ui::widgets::{MemoriWidget, WidgetId};
use postcard::from_bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use transport::ble_types::*;
use transport::ble_types::{
    BATTERY_LEVEL_CHAR_UUID as BATTERY_CHAR_STR, NUS_RX_CHAR_UUID as NUS_RX_STR,
    NUS_TX_CHAR_UUID as NUS_TX_STR,
};

use transport::*;
use uuid::Uuid;

const NUS_RX_CHAR_UUID: Uuid = Uuid::from_u128(NUS_RX_STR);
const NUS_TX_CHAR_UUID: Uuid = Uuid::from_u128(NUS_TX_STR);
const BATTERY_LEVEL_CHAR_UUID: Uuid = uuid_from_u16(BATTERY_CHAR_STR);

type ResponseMap = Arc<Mutex<HashMap<MessageID, oneshot::Sender<DeviceBLEResponse>>>>;

struct OutboundPacket {
    packet: HostBLEPacket,
    id: Option<MessageID>, // none = auto-assign, some = use this ID
    response_tx: Option<oneshot::Sender<DeviceBLEResponse>>,
}

async fn find_memori(central: &Adapter, code: &str) -> Option<Peripheral> {
    let looking_for = format!("memori-{}", code.trim());
    let mut events = central.events().await.ok()?;
    central.start_scan(ScanFilter::default()).await.ok()?;
   
    tokio::time::timeout(Duration::from_secs(30), async {
    loop {
        if let Some(CentralEvent::DeviceDiscovered(id)) = events.next().await {
                let Ok(peripheral) = central.peripheral(&id).await else { continue };
                let Ok(Some(props)) = peripheral.properties().await else { continue };
                //eprintln!("[ble-host] discovered: {:?}", props.local_name);
                if props.local_name.iter().any(|n| n.contains(&looking_for)) {
                    return Some(peripheral);
                }
            }
        }
    })
    .await
    .ok()
    .flatten()
}

async fn send_packet(
    packet: BLEPacket,
    peripheral: &Peripheral,
    char: &btleplug::api::Characteristic,
) -> TransResult<()> {
    let mut buf = [0u8; BLE_CHAR_SIZE];
    let encoded = postcard::to_slice(&packet, &mut buf).map_err(|_| TransError::InvalidMessage)?;

    peripheral
        .write(char, encoded, WriteType::WithoutResponse)
        .await
        .map_err(|e| {
            eprintln!("[ble-host] send_packet error: {:?}", e);
            TransError::ProtocolIssue
        })?;

    Ok(())
}

pub struct HostBLETransport {
    outbound: mpsc::Sender<OutboundPacket>,
    battery_char: Characteristic,
    pub peripheral: Peripheral,
    read_handle: JoinHandle<()>,
    write_handle: JoinHandle<()>,
    command_handle: JoinHandle<()>,
}

async fn find_or_reconnect(central: &Adapter, code: &str, known_id: Option<&str>) -> Option<Peripheral> {
    eprintln!("[ble-host] trying direct connect to {}", known_id.unwrap_or(code));
    
    // do a brief scan to populate cache first
    central.start_scan(ScanFilter::default()).await.ok()?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    central.stop_scan().await.ok();
    
    let peripherals = central.peripherals().await.ok()?;
    eprintln!("[ble-host] found {} peripherals in cache", peripherals.len());

    for p in &peripherals {
        // Try ID match first (fast path on reconnect)
        if let Some(id) = known_id {
            if p.id().to_string() == id || p.address().to_string() == id {
                eprintln!("[ble-host] found device by id in cache: {:?}", p.id());
                return Some(p.clone());
            }
        }

        // Fall back to name matching
        if let Some(props) = p.properties().await.ok().flatten() {
            let name = props.local_name.as_deref().unwrap_or("");
            if name.contains(code) {
                eprintln!("[ble-host] found device by name in cache: {:?}", p.id());
                return Some(p.clone());
            }
        }
    }

    eprintln!("[ble-host] no device found in cache, falling back to scan");
    find_memori(central, code).await
}

impl HostBLETransport {
    pub async fn connect(
        code: &str,
        known_address: Option<&str>,
    ) -> anyhow::Result<(
        Self,
        String,
        (
            mpsc::UnboundedReceiver<DeviceBLECommand>,
            mpsc::UnboundedSender<HostBLEResponse>,
        ),
    )> {
        let manager = Manager::new().await?;
        let central = manager
            .adapters()
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No BLE adapters found"))?;

        let peripheral = find_or_reconnect(&central, code, known_address)
            .await
            .ok_or_else(|| anyhow::anyhow!("Memori device not found"))?;
        eprintln!("[ble-host] found peripheral, attempting connect...");
         
        // only stop scan if we had to scan
        if known_address.is_none() {
            eprintln!("[ble-host] stopping scan because we had to scan");
            central.stop_scan().await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        
        peripheral.connect().await?;
        eprintln!("[ble-host] connected, discovering services...");
        
        eprintln!("[ble-host] peripheral id: {:?}", peripheral.id());
        eprintln!("[ble-host] peripheral address: {:?}", peripheral.address());
        
        
        peripheral.discover_services().await?;
        eprintln!("[ble-host] services discovered");

        let chars = peripheral.characteristics();
        eprintln!("[ble-host] characteristics: {:?}", chars);
        
        let rx_char = chars
            .iter()
            .find(|c| c.uuid == NUS_RX_CHAR_UUID)
            .ok_or_else(|| anyhow::anyhow!("NUS RX characteristic not found"))?
            .clone();
        let tx_char = chars
            .iter()
            .find(|c| c.uuid == NUS_TX_CHAR_UUID)
            .ok_or_else(|| anyhow::anyhow!("NUS TX characteristic not found"))?
            .clone();
        let battery_char = chars
            .iter()
            .find(|c| c.uuid == BATTERY_LEVEL_CHAR_UUID)
            .ok_or_else(|| anyhow::anyhow!("Battery level characteristic not found"))?
            .clone();

        peripheral.subscribe(&tx_char).await?;

        let (out_tx, out_rx) = mpsc::channel::<OutboundPacket>(16);
        let (cmd_tx, cmd_rx) = mpsc::channel::<(DeviceBLECommand, u32)>(16);

        let (device_command_tx, device_command_rx) = mpsc::unbounded_channel::<DeviceBLECommand>();
        let (host_response_tx, host_response_rx) = mpsc::unbounded_channel::<HostBLEResponse>();

        let pending_responses: ResponseMap = Arc::new(Mutex::new(HashMap::new()));
        let notif_stream = peripheral.notifications().await?;

        let read_handle = tokio::spawn(Self::notification_reader(
            notif_stream,
            cmd_tx,
            pending_responses.clone(),
        ));

        let write_handle = tokio::spawn(Self::ble_writer(
            out_rx,
            peripheral.clone(),
            rx_char.clone(),
            pending_responses.clone(),
        ));

        let command_handle = tokio::spawn(Self::server_command_handler(
            cmd_rx,
            out_tx.clone(),
            device_command_tx,
            host_response_rx,
        ));
        
        let address = {
            let addr = peripheral.address().to_string();
            if addr == "00:00:00:00:00:00" {
                peripheral.id().to_string()
            } else {
                addr
            }
        };
        Ok((
            Self {
                outbound: out_tx,
                battery_char,
                peripheral,
                read_handle,
                write_handle,
                command_handle,
            },
            address,
            (device_command_rx, host_response_tx),
        ))
    }

    // Send a command and wait for response
    async fn send_command(&self, command: HostBLECommand) -> TransResult<DeviceBLEResponse> {
        let (tx, rx) = oneshot::channel();

        let packet = OutboundPacket {
            packet: HostBLEPacket::Command(command),
            id: None, // Let blewriter assign the id
            response_tx: Some(tx),
        };

        self.outbound
            .send(packet)
            .await
            .map_err(|_| TransError::ProtocolIssue)?;

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err(TransError::ProtocolIssue),
            Err(_) => Err(TransError::Timeout),
        }
    }

    async fn notification_reader(
        mut notif_stream: impl futures::Stream<Item = ValueNotification> + Unpin,
        cmd_tx: mpsc::Sender<(DeviceBLECommand, MessageID)>,
        pending_responses: ResponseMap,
    ) {
        while let Some(notification) = notif_stream.next().await {
            if notification.uuid != NUS_TX_CHAR_UUID {
                continue;
            }

            let Ok(packet) = from_bytes::<BLEPacket>(&notification.value) else {
                eprintln!("[ble-host] notif-reader: failed to parse BLEPacket");
                continue;
            };

            let BLEPacketPayload::DevicePacket(device_packet) = packet.payload else {
                eprintln!("[ble-host] notif-reader: received unexpected HostPacket from device");
                continue;
            };

            match device_packet {
                DeviceBLEPacket::Command(cmd) => {
                    if let Err(e) = cmd_tx.send((cmd, packet.id)).await {
                        eprintln!("[ble-host] notif-reader: Failed to send command: {:?}", e);
                    }
                }
                DeviceBLEPacket::Response(resp) => {
                    println!(
                        "[ble-host] notif-reader: Received response: {:?} (id: {})",
                        resp, packet.id
                    );
                    let mut map = pending_responses.lock().await;
                    if let Some(tx) = map.remove(&packet.id) {
                        if tx.send(resp).is_err() {
                            eprintln!(
                                "[ble-host] notif-reader: Failed to send response to waiting task (id: {})",
                                packet.id
                            );
                        }
                    } else {
                        eprintln!(
                            "[ble-host] notif-reader: No pending request for id: {}",
                            packet.id
                        );
                    }
                }
            }
        }
    }

    async fn ble_writer(
        mut outbound_rx: mpsc::Receiver<OutboundPacket>,
        peripheral: Peripheral,
        rx_char: Characteristic,
        pending_responses: ResponseMap,
    ) {
        let mut next_msg_id: MessageID = 0;

        while let Some(outbound) = outbound_rx.recv().await {
            let id = outbound.id.unwrap_or_else(|| {
                let current_id = next_msg_id;
                next_msg_id = next_msg_id.wrapping_add(1);
                current_id
            });

            if let Some(response_tx) = outbound.response_tx {
                pending_responses.lock().await.insert(id, response_tx);
            }

            let packet = BLEPacket {
                payload: BLEPacketPayload::HostPacket(outbound.packet),
                id,
            };

            if let Err(e) = send_packet(packet, &peripheral, &rx_char).await {
                eprintln!("[ble-host] BLE write failed: {:?}", e);
                // error handling
                pending_responses.lock().await.remove(&id);
            }
        }
    }

    async fn server_command_handler(
        mut cmd_rx: mpsc::Receiver<(DeviceBLECommand, MessageID)>,
        outbound_tx: mpsc::Sender<OutboundPacket>,
        device_command_tx: mpsc::UnboundedSender<DeviceBLECommand>,
        mut host_response_rx: mpsc::UnboundedReceiver<HostBLEResponse>,
    ) {
        while let Some((cmd, id)) = cmd_rx.recv().await {
            if let Err(e) = device_command_tx.send(cmd) {
                eprintln!(
                    "[ble-host] command: Failed to forward device command: {:?}",
                    e
                );
                continue;
            }

            // wait for synchronous response from external handler (in tauri app)
            let Some(response) = host_response_rx.recv().await else {
                eprintln!("[ble-host] command: Response channel closed");
                break;
            };

            let packet = OutboundPacket {
                packet: HostBLEPacket::Response(response),
                id: Some(id),
                response_tx: None,
            };

            if let Err(e) = outbound_tx.send(packet).await {
                eprintln!("[ble-host] command: Failed to send response: {:?}", e);
            }
        }
    }

    pub async fn disconnect(self) {
        tokio::spawn(async move {
            if let Err(e) = self.peripheral.disconnect().await {
                eprintln!("[ble-host] disconnect: failed to disconnect: {}", e);
            }
        });

        self.read_handle.abort();
        self.write_handle.abort();
        self.command_handle.abort();
    }
}

impl HostTransport for HostBLETransport {
    async fn set_state(&mut self, state: MemoriState) -> TransResult<()> {
        let command = HostBLECommand::SetState { state };
        let response = self.send_command(command).await?;

        match response {
            DeviceBLEResponse::SetState { result } => result,
            _ => {
                eprintln!("[host_transport] Unexpected response type");
                Err(TransError::ProtocolIssue)
            }
        }
    }

    async fn get_widget(&mut self, id: WidgetId) -> TransResult<MemoriWidget> {
        let command = HostBLECommand::GetWidget { widget_id: id };
        let response = self.send_command(command).await?;

        match response {
            DeviceBLEResponse::WidgetGet { result } => result,
            _ => {
                eprintln!("[host_transport] Unexpected response type");
                Err(TransError::ProtocolIssue)
            }
        }
    }

    async fn get_battery_level(&mut self) -> TransResult<u8> {
        let data = self
            .peripheral
            .read(&self.battery_char)
            .await
            .map_err(|_| {
                eprintln!("[ble-host] Failed to read battery characteristic");
                TransError::ProtocolIssue
            })?;

        match data.as_slice() {
            [level] => Ok(*level),
            _ => Err(TransError::InvalidMessage),
        }
    }

    async fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()> {
        let command = HostBLECommand::SetConfig { config };
        let response = self.send_command(command).await?;

        match response {
            DeviceBLEResponse::DeviceConfigSet { result } => result,
            _ => Err(TransError::ProtocolIssue),
        }
    }
}
