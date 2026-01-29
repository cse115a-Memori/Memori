use btleplug::api::Characteristic;
use btleplug::api::{
    Central, Manager as _, Peripheral as _, ScanFilter, ValueNotification, WriteType,
    bleuuid::uuid_from_u16,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::str::from_utf8;
use std::time::Duration;
use tokio::time::{self, sleep};
use transport::{ByteArray, WidgetId};
use postcard::{from_bytes, to_slice};
use tokio::io::{self, AsyncBufReadExt};
use transport::*;
use transport::ble_types::*;
use transport::ble_types::{
    NUS_RX_CHAR_UUID as NUS_RX_STR, 
    NUS_TX_CHAR_UUID as NUS_TX_STR, 
    NUS_SERVICE_UUID as NUS_SERVICE_STR,
    BATTERY_LEVEL_CHAR_UUID as BATTERY_CHAR_STR
};
use tokio::sync::mpsc;
use log::{info, trace, error};

use uuid::Uuid;

const NUS_RX_CHAR_UUID: Uuid = Uuid::from_u128(NUS_RX_STR);
const NUS_TX_CHAR_UUID: Uuid = Uuid::from_u128(NUS_TX_STR);
const BATTERY_LEVEL_CHAR_UUID: Uuid = uuid_from_u16(BATTERY_CHAR_STR);

// eventually HashMap<id, oneshot::Sender<_>> with packet_id % buckets

async fn find_memori(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("memori"))
        {
            return Some(p);
        }
    }
    None
}

async fn send_packet(packet: HostBLEPacket, peripheral: &Peripheral, char: &btleplug::api::Characteristic) -> TransResult<()> {
    let mut buf = [0u8; BLE_CHAR_SIZE];
    let encoded = postcard::to_slice(&packet, &mut buf)
        .map_err(|_| TransError::InvalidMessage)?; // TODO this should be changed

    peripheral
        .write(char, encoded, WriteType::WithoutResponse)
        .await
        .map_err(|_| TransError::ProtocolIssue)?;

    Ok(())
}

pub struct HostBLETransport {
    outbound: mpsc::Sender<HostBLEPacket>,
    responses: mpsc::Receiver<DeviceBLEResponse>,
    battery_char: Characteristic,
    peripheral: Peripheral,
}

impl HostBLETransport {
    pub async fn connect() -> anyhow::Result<Self> {
        println!("[connect] Creating BLE manager...");
        let manager = Manager::new().await?;
        let central = manager.adapters().await?
            .into_iter().next().expect("no adapters");
        println!("[connect] Found adapter, starting scan...");
        central.start_scan(ScanFilter::default()).await?;
        sleep(Duration::from_secs(3)).await;

        println!("[connect] Looking for Memori device...");
        let peripheral = find_memori(&central)
            .await.expect("no memori found");
        println!("[connect] Connecting to device...");
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        println!("[connect] Services discovered");

        let chars = peripheral.characteristics();
        let rx_char = chars.iter()
            .find(|c| c.uuid == NUS_RX_CHAR_UUID).unwrap().clone();
        let tx_char = chars.iter()
            .find(|c| c.uuid == NUS_TX_CHAR_UUID).unwrap().clone();
        let battery_char = chars.iter()
            .find(|c| c.uuid == BATTERY_LEVEL_CHAR_UUID).unwrap().clone();
        println!("[connect] Found RX, TX, and battery characteristics");

        peripheral.subscribe(&tx_char).await?;
        println!("[connect] Subscribed to TX characteristic");

        let (out_tx, out_rx) = mpsc::channel::<HostBLEPacket>(16);
        let (cmd_tx, cmd_rx) = mpsc::channel::<DeviceBLECommand>(16);
        let (resp_tx, resp_rx) = mpsc::channel::<DeviceBLEResponse>(16);

        let notif_stream = peripheral.notifications().await?;
        tokio::spawn(Self::notification_reader(
            notif_stream,
            cmd_tx,
            resp_tx,
        ));
        println!("[connect] Notification reader spawned");

        tokio::spawn(Self::ble_writer(
            out_rx,
            peripheral.clone(),
            rx_char.clone(),
        ));
        println!("[connect] BLE writer spawned");

        tokio::spawn(Self::server_command_handler(
            cmd_rx,
            out_tx.clone(),
        ));
        println!("[connect] Server command handler spawned");

        Ok(Self {
            outbound: out_tx,
            responses: resp_rx,
            battery_char,
            peripheral,
        })
    }

    // handles ble notifications
    async fn notification_reader(
        mut notif_stream: impl futures::Stream<Item = ValueNotification> + Unpin,
        cmd_tx: mpsc::Sender<DeviceBLECommand>,
        resp_tx: mpsc::Sender<DeviceBLEResponse>,
    ) {
        println!("[notif_reader] Started");
        while let Some(notification) = notif_stream.next().await {
            println!("[notif_reader] Received notification: {:?}", notification.uuid);
            if notification.uuid != NUS_TX_CHAR_UUID {
                continue;
            }

            let Ok(pkt) = from_bytes::<DeviceBLEPacket>(&notification.value) else {
                eprintln!("[notif_reader] Failed to parse packet");
                continue;
            };

            match pkt {
                DeviceBLEPacket::Command(cmd) => {
                    println!("[notif_reader] Forwarding command: {:?}", cmd);
                    if let Err(e) = cmd_tx.send(cmd).await {
                        eprintln!("[notif_reader] Failed to send command: {:?}", e);
                    }
                }
                DeviceBLEPacket::Response(resp) => {
                    println!("[notif_reader] Forwarding response: {:?}", resp);
                    if let Err(e) = resp_tx.send(resp).await {
                        eprintln!("[notif_reader] Failed to send response: {:?}", e);
                    }
                }
            }
        }
    }

    // handles all outgoing messages
    async fn ble_writer(
        mut outbound_rx: mpsc::Receiver<HostBLEPacket>,
        peripheral: Peripheral,
        rx_char: Characteristic,
    ) {
        println!("[ble_writer] Started");
        while let Some(msg) = outbound_rx.recv().await {
            println!("[ble_writer] Sending packet: {:?}", msg);
            if let Err(e) = send_packet(msg, &peripheral, &rx_char).await {
                eprintln!("[ble_writer] BLE write failed: {:?}", e);
            }
        }
    }

    // fulfills commands sent to us by the device
    async fn server_command_handler(
        mut cmd_rx: mpsc::Receiver<DeviceBLECommand>,
        outbound_tx: mpsc::Sender<HostBLEPacket>,
    ) {
        println!("[cmd_handler] Started");
        while let Some(cmd) = cmd_rx.recv().await {
            println!("[cmd_handler] Handling command: {:?}", cmd);
            match cmd {
                DeviceBLECommand::Ping => {
                    let pkt = HostBLEPacket::Response(
                        HostBLEResponse::Ping { result: Ok(()) }
                    );
                    if let Err(e) = outbound_tx.send(pkt).await {
                        eprintln!("[cmd_handler] Failed to send ping: {:?}", e);
                    } else {
                        println!("[cmd_handler] Ping response sent");
                    }
                }

                DeviceBLECommand::RefreshData { widget_id } => {
                    let mut bytes: ByteArray = Default::default();
                    bytes.extend_from_slice(b"default data").unwrap();
                    let pkt = HostBLEPacket::Response(
                        HostBLEResponse::RefreshData { result: Ok(bytes) }
                    );
                    if let Err(e) = outbound_tx.send(pkt).await {
                        eprintln!("[cmd_handler] Failed to send refresh: {:?}", e);
                    } else {
                        println!("[cmd_handler] Refresh response sent for widget {:?}", widget_id);
                    }
                }
            }
        }
    }
}

impl HostTransport for HostBLETransport {
    async fn set_widgets(&mut self, widget: Widget) -> TransResult<()> {
        println!("[host_transport] set_widgets called");
        todo!()
    }

    async fn get_widget(&mut self, id: WidgetId) -> TransResult<Widget> {
        println!("[host_transport] Sending GetWidget command for {:?}", id);
        let _ = self.outbound.send(
            HostBLEPacket::Command(
                HostBLECommand::GetWidget { widget_id: id.clone() }
            )
        ).await;

        match self.responses.recv().await {
            Some(DeviceBLEResponse::WidgetGet { result: Ok(widget) }) => {
                println!("[host_transport] Widget received: {:?}", widget);
                Ok(widget)
            }
            _ => {
                eprintln!("[host_transport] Widget not found for {:?}", id);
                Err(TransError::WidgetNotFound)
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
        todo!()
    }
}
