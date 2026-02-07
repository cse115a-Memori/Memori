use btleplug::api::Characteristic;
use btleplug::api::{
    Central, Manager as _, Peripheral as _, ScanFilter, ValueNotification, WriteType,
    bleuuid::uuid_from_u16,
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
use tokio::time::sleep;
use transport::ByteArray;
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

// function to find the first currently advertising memori device
//
// eventually we want this to be a pairing process with some sort of bonded id
// instead of something that just finds the nearest device
async fn find_memori(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.ok()?.into_iter() {
        // p.properties returns a future<result<option<deviceproperties>>>
        //  therefore we await the future, .ok to convert result to option
        //  then flatten the external option giving us opt<properties> which
        //  we can run map on to reach inside and run the closure on
        let has_memori = p
            .properties()
            .await
            .ok()
            .flatten()
            .map(|props| props.local_name.iter().any(|name| name.contains("memori")))
            .unwrap_or(false);

        if has_memori {
            return Some(p);
        }
    }
    None
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
        .map_err(|_| TransError::ProtocolIssue)?;

    Ok(())
}

pub struct HostBLETransport {
    outbound: mpsc::Sender<OutboundPacket>,
    battery_char: Characteristic,
    peripheral: Peripheral,
}

impl HostBLETransport {
    pub async fn connect() -> anyhow::Result<Self> {
        println!("[connect] Creating BLE manager...");
        let manager = Manager::new().await?;
        let central = manager
            .adapters()
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No BLE adapters found"))?;

        println!("[connect] Found adapter, starting scan...");
        central.start_scan(ScanFilter::default()).await?;
        sleep(Duration::from_secs(5)).await;

        println!("[connect] Looking for Memori device...");
        let peripheral = find_memori(&central)
            .await
            .ok_or_else(|| anyhow::anyhow!("Memori device not found"))?;

        println!("[connect] Connecting to device...");
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        println!("[connect] Services discovered");

        let chars = peripheral.characteristics();
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

        println!("[connect] Found RX, TX, and battery characteristics");

        peripheral.subscribe(&tx_char).await?;
        println!("[connect] Subscribed to TX characteristic");

        let (out_tx, out_rx) = mpsc::channel::<OutboundPacket>(16);
        let (cmd_tx, cmd_rx) = mpsc::channel::<(DeviceBLECommand, u32)>(16);

        let pending_responses: ResponseMap = Arc::new(Mutex::new(HashMap::new()));
        let notif_stream = peripheral.notifications().await?;

        tokio::spawn(Self::notification_reader(
            notif_stream,
            cmd_tx,
            pending_responses.clone(),
        ));
        println!("[connect] Notification reader spawned");

        tokio::spawn(Self::ble_writer(
            out_rx,
            peripheral.clone(),
            rx_char.clone(),
            pending_responses.clone(),
        ));
        println!("[connect] BLE writer spawned");

        tokio::spawn(Self::server_command_handler(cmd_rx, out_tx.clone()));
        println!("[connect] Server command handler spawned");

        Ok(Self {
            outbound: out_tx,
            battery_char,
            peripheral,
        })
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
        println!("[notif_reader] Started");
        while let Some(notification) = notif_stream.next().await {
            println!(
                "[notif_reader] received notification: {:?}",
                notification.uuid
            );

            if notification.uuid != NUS_TX_CHAR_UUID {
                continue;
            }

            let Ok(packet) = from_bytes::<BLEPacket>(&notification.value) else {
                eprintln!("[notif_reader] failed to parse BLEPacket");
                continue;
            };

            let BLEPacketPayload::DevicePacket(device_packet) = packet.payload else {
                eprintln!("[notif_reader] received unexpected HostPacket from device");
                continue;
            };

            match device_packet {
                DeviceBLEPacket::Command(cmd) => {
                    println!(
                        "[notif_reader] Forwarding command: {:?} (id: {})",
                        cmd, packet.id
                    );
                    if let Err(e) = cmd_tx.send((cmd, packet.id)).await {
                        eprintln!("[notif_reader] Failed to send command: {:?}", e);
                    }
                }
                DeviceBLEPacket::Response(resp) => {
                    println!(
                        "[notif_reader] Received response: {:?} (id: {})",
                        resp, packet.id
                    );
                    let mut map = pending_responses.lock().await;
                    if let Some(tx) = map.remove(&packet.id) {
                        if tx.send(resp).is_err() {
                            eprintln!(
                                "[notif_reader] Failed to send response to waiting task (id: {})",
                                packet.id
                            );
                        }
                    } else {
                        eprintln!("[notif_reader] No pending request for id: {}", packet.id);
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
                eprintln!("[ble_writer] BLE write failed: {:?}", e);
                // error handling
                pending_responses.lock().await.remove(&id);
            }
        }
    }

    async fn server_command_handler(
        mut cmd_rx: mpsc::Receiver<(DeviceBLECommand, MessageID)>,
        outbound_tx: mpsc::Sender<OutboundPacket>,
    ) {
        println!("[cmd_handler] Started");

        while let Some((cmd, id)) = cmd_rx.recv().await {
            println!("[cmd_handler] Handling command: {:?} (id: {})", cmd, id);

            let response = match cmd {
                DeviceBLECommand::Ping => {
                    println!("[cmd_handler] Sending ping response");
                    HostBLEResponse::Ping { result: Ok(()) }
                }
                DeviceBLECommand::RefreshData { widget_id } => {
                    let mut bytes: ByteArray = Default::default();
                    bytes
                        .extend_from_slice(b"widget data for widget: ")
                        .unwrap();
                    bytes
                        .extend_from_slice(widget_id.0.to_string().as_bytes())
                        .unwrap();
                    println!(
                        "[cmd_handler] Sending refresh response for widget {:?}",
                        widget_id
                    );

                    let Ok(widget) = from_bytes::<MemoriWidget>(&bytes) else {
                        eprintln!("Failed to deserialize widget");
                        continue;
                    };

                    HostBLEResponse::RefreshData { result: Ok(widget) }
                }
            };

            let packet = OutboundPacket {
                packet: HostBLEPacket::Response(response),
                id: Some(id),
                response_tx: None,
            };

            if let Err(e) = outbound_tx.send(packet).await {
                eprintln!("[cmd_handler] Failed to send response: {:?}", e);
            }
        }
    }
}

impl HostTransport for HostBLETransport {
    async fn set_state(&mut self, state: MemoriState) -> TransResult<()> {
        println!("[host_transport] set_widgets called");
        // let command = HostBLECommand::SetWidgets { state };
        // let response = self.send_command(command).await?;

        // match response {
        //     DeviceBLEResponse::WidgetSet { result } => result,
        //     _ => Err(TransError::ProtocolIssue),
        // }
        todo!()
    }

    async fn get_widget(&mut self, id: WidgetId) -> TransResult<MemoriWidget> {
        println!("[host_transport] Sending GetWidget command for {:?}", id);
        let command = HostBLECommand::GetWidget { widget_id: id };
        let response = self.send_command(command).await?;

        match response {
            DeviceBLEResponse::WidgetGet { result } => {
                println!("[host_transport] Widget received");
                result
            }
            _ => {
                eprintln!("[host_transport] Unexpected response type");
                Err(TransError::ProtocolIssue)
            }
        }
    }

    async fn get_battery_level(&mut self) -> TransResult<u8> {
        println!("[host_transport] Reading battery level...");
        let data = self
            .peripheral
            .read(&self.battery_char)
            .await
            .map_err(|_| {
                eprintln!("[host_transport] Failed to read battery characteristic");
                TransError::ProtocolIssue
            })?;

        match data.as_slice() {
            [level] => {
                println!("[host_transport] Battery level: {}", level);
                Ok(*level)
            }
            _ => {
                eprintln!("[host_transport] Invalid battery response");
                Err(TransError::InvalidMessage)
            }
        }
    }

    async fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()> {
        println!("[host_transport] set_device_config called");
        let command = HostBLECommand::SetConfig { config };
        let response = self.send_command(command).await?;

        match response {
            DeviceBLEResponse::DeviceConfigSet { result } => result,
            _ => Err(TransError::ProtocolIssue),
        }
    }
}
