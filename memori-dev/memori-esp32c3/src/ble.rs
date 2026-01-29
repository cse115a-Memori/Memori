use ble_device::{BLE_CMD_CHANNEL, BLE_RESP_CHANNEL, BLE_CONNECTED};
use embassy_executor::Spawner;
use embassy_futures::{join::join, select::select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_time::{Duration, with_timeout};
use esp_hal::peripherals;
use esp_radio::ble::controller::BleConnector;
use log::{info, trace, warn};
use postcard::{from_bytes, to_slice};
use transport::ble_types::*;
use transport::{ByteArray, TransError, TransResult, Widget, WidgetId};
use trouble_host::prelude::*;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 1;

const PERIPHERAL_NAME: &str = "memori";

// this will be used to pass response messages to the async functions handling them
static BLE_HOST_RESPONSE: Channel<CriticalSectionRawMutex, HostBLEResponse, 1> = Channel::new();

// GATT Server definition
#[gatt_server]
struct Server {
    nus_service: NordicUartService,
    battery_service: BatteryService,
}

#[gatt_service(uuid = NUS_SERVICE_UUID)]
struct NordicUartService {
    #[characteristic(
        uuid = NUS_RX_CHAR_UUID,
        write,
        write_without_response,
        value = [0u8; BLE_CHAR_SIZE]
    )]
    #[descriptor(uuid = "2901", value = "RX Characteristic")]
    rx: [u8; BLE_CHAR_SIZE], // max BLE packet size?

    #[characteristic(uuid = NUS_TX_CHAR_UUID, read, notify, value = [0u8; BLE_CHAR_SIZE])]
    #[descriptor(uuid = "2901", value = "TX Characteristic")]
    tx: [u8; BLE_CHAR_SIZE],
}

#[gatt_service(uuid = service::BATTERY)]
struct BatteryService {
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[characteristic(uuid = BATTERY_LEVEL_CHAR_UUID, read, notify, value = 42)]
    level: u8,
    #[characteristic(uuid = BATTERY_NOTIFY_CHAR_UUID, write, read, notify)]
    status: bool,
}

#[embassy_executor::task]
pub async fn ble_task(
    radio: &'static esp_radio::Controller<'static>,
    bt: peripherals::BT<'static>,
) {
    info!("ble start");
    let transport = BleConnector::new(radio, bt, Default::default()).unwrap();
    let ble_controller: ExternalController<BleConnector<'_>, 20> =
        ExternalController::<_, 20>::new(transport);

    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: PERIPHERAL_NAME,
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    let _ = join(ble_bg_task(runner), async {
        loop {
            match advertise(PERIPHERAL_NAME, &mut peripheral, &server).await {
                Ok(conn) => {
                    BLE_CONNECTED.store(true, core::sync::atomic::Ordering::SeqCst);

                    let a = gatt_events_task(&server, &conn);
                    let b = channel_task(&server, &conn);
                    select(a, b).await;

                    BLE_CONNECTED.store(false, core::sync::atomic::Ordering::SeqCst);
                }
                Err(e) => {
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;
}

async fn ble_bg_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            panic!("[ble_task] error: {:?}", e);
        }
    }
}

async fn gatt_events_task<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                match &event {
                    GattEvent::Write(event) => {
                        if event.handle() == server.nus_service.rx.handle {
                            let data = event.data();
                            info!("[gatt] received {} bytes", data.len());

                            // bit wide
                            match from_bytes::<HostBLEPacket>(data) {
                                Ok(packet) => {
                                    match packet {
                                        HostBLEPacket::Command(cmd) => {
                                            info!("[transport] received cmd {:?}", cmd);
                                            match cmd {
                                                HostBLECommand::GetWidget { widget_id } => {
                                                    // dubious naming. should be (get_widget)_response
                                                    get_widget_response(widget_id, server, conn).await;
                                                },
                                                HostBLECommand::SetWidget { widget } => {
                                                    todo!()
                                                },
                                                HostBLECommand::SetConfig { config } => {
                                                    todo!()
                                                },
                                            }
                                            // handle command
                                        }
                                        HostBLEPacket::Response(resp) => {
                                            info!("[transport] received response {:?}", resp);
                                            // push into resp_rx channel
                                            BLE_HOST_RESPONSE.send(resp).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("[gatt] failed to decode HostBLEPacket: {:?}", e);
                                }
                            }
                        }
                    }
                    GattEvent::Read(event) => {
                        if event.handle() == server.nus_service.tx.handle {
                           info!("[gatt] read from TX characteristic");
                        }
                        if event.handle() == server.battery_service.level.handle {
                            let value = server.get(&server.battery_service.level);
                            info!("[gatt] Read Event to Level Characteristic: {:?}", value);}
                    }
                    _ => {}
                };
                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("[gatt] error sending response: {:?}", e),
                };
            }
            _ => {} // ignore other Gatt Connection Events
        }
    };
    info!("[gatt] disconnected: {:?}", reason);
    Ok(())
}

async fn channel_task<P: PacketPool>(server: &Server<'_>, conn: &GattConnection<'_, '_, P>) {
    let cmd_rx = BLE_CMD_CHANNEL.receiver();
    let resp_tx = BLE_RESP_CHANNEL.sender();

    loop {
        let cmd = cmd_rx.receive().await;
        match cmd {
            DeviceBLECommand::Ping => {
                info!("[transport-channel] running send ping");
                let result = send_ping(server, conn).await;
                // send result of ble operation back to transport
                resp_tx.send(HostBLEResponse::Ping { result }).await;
            }
            DeviceBLECommand::RefreshData { widget_id } => {
                let result = request_refresh(server, conn, widget_id).await;
                resp_tx.send(HostBLEResponse::RefreshData { result }).await;
            }
        };
    }
}

// device commands

//TODO abstract sending
async fn send_ping<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> TransResult<()> {
    let tx = server.nus_service.tx;
    let mut buffer = [0u8; BLE_CHAR_SIZE];

    let pkt = DeviceBLEPacket::Command(DeviceBLECommand::Ping);
    let _encoded = to_slice(&pkt, &mut buffer).unwrap();

    // are trailing 0s ok here
    tx.notify(conn, &buffer)
        .await
        .map_err(|_| TransError::ProtocolIssue)?;


    // because we're sending commands over channels we
    // need to await this with a timeout in addition to
    // hte main task or else BLE_HOST_RESPONSE writers
    // start piling up
    // TODO switch to signal arr to handle multiple in flight
    let response: HostBLEResponse = with_timeout(
        Duration::from_secs(5),
        BLE_HOST_RESPONSE.receive(),
    ).await.map_err(|_| TransError::Timeout)?; 
    match response {
        HostBLEResponse::Ping { result } => {
            return result
        }
        _ => {
            return Err(TransError::ProtocolIssue);
        }
    }
}

async fn request_refresh<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    widget_id: WidgetId,
) -> TransResult<ByteArray> {
    let tx = server.nus_service.tx;
    let mut buffer = [0u8; BLE_CHAR_SIZE];

    let pkt = DeviceBLEPacket::Command(DeviceBLECommand::RefreshData { widget_id });
    let _encoded = to_slice(&pkt, &mut buffer).unwrap();

    // are trailing 0s ok here
    tx.notify(conn, &buffer)
        .await
        .map_err(|_| TransError::ProtocolIssue)?;

    let response: HostBLEResponse = with_timeout(
        Duration::from_secs(5),
        BLE_HOST_RESPONSE.receive(),
    ).await.map_err(|_| TransError::Timeout)?; 
    match response {
        HostBLEResponse::RefreshData { result } => {
            return result
        }
        _ => {
            return Err(TransError::ProtocolIssue);
        }
    }
}

// host responses
//
async fn get_widget_response<P: PacketPool>(
    widget_id: WidgetId,
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) {
    let tx = server.nus_service.tx;
    let mut buffer = [0u8; BLE_CHAR_SIZE];

    let mut bytes: ByteArray = Default::default();
    bytes.extend_from_slice(b"this is the data of a widget").unwrap();

    let widget = Widget::new(widget_id, bytes);

    let pkt = DeviceBLEPacket::Response(DeviceBLEResponse::WidgetGet { result: Ok(widget) });
    let _encoded = to_slice(&pkt, &mut buffer).unwrap();

    // are trailing 0s ok here
    tx.notify(conn, &buffer)
        .await;
        // .map_err(|_| TransError::ProtocolIssue)?
}


#[embassy_executor::task]
pub async fn ble_echo_task(
    radio: &'static esp_radio::Controller<'static>,
    bt: peripherals::BT<'static>,
) {
    info!("ble start");
    let transport = BleConnector::new(radio, bt, Default::default()).unwrap();
    let ble_controller: ExternalController<BleConnector<'_>, 20> =
        ExternalController::<_, 20>::new(transport);

    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: PERIPHERAL_NAME,
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    let _ = join(ble_bg_task(runner), async {
        loop {
            match advertise(PERIPHERAL_NAME, &mut peripheral, &server).await {
                Ok(conn) => {
                    gatt_events_echo_task(&server, &conn).await;
                }
                Err(e) => {
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;
}

/// Stream Events until the connection closes.
///
/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_echo_task<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    let rx = server.nus_service.rx;
    let tx = server.nus_service.tx;
    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                match &event {
                    GattEvent::Read(event) => {
                        if event.handle() == tx.handle {
                            let value = server.get(&tx);
                            info!("[gatt] read from TX characteristic");
                        }
                    }
                    GattEvent::Write(event) => {
                        if event.handle() == rx.handle {
                            let data = event.data();
                            info!("[gatt] received {} bytes on RX characteristic", data.len());

                            if let Ok(text) = core::str::from_utf8(data) {
                                info!("[gatt] received text: {}", text);
                            } else {
                                info!("[gatt] received raw data: {:?}", data);
                            }

                            // echo the data back through tx
                            let mut tx_buffer = [0u8; BLE_CHAR_SIZE];
                            let len = data.len().min(BLE_CHAR_SIZE);
                            tx_buffer[..len].copy_from_slice(&data[..len]);
                            server.set(&tx, &tx_buffer);

                            // Notify on the TX characteristic
                            if let Err(e) = tx.notify(conn, &tx_buffer).await {
                                warn!("[gatt] failed to send notification: {:?}", e);
                            } else {
                                info!("[gatt] echoed {} bytes back", len);
                            }
                        }
                    }
                    _ => {}
                };
                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("[gatt] error sending response: {:?}", e),
                };
            }
            _ => {} // ignore other Gatt Connection Events
        }
    };
    info!("[gatt] disconnected: {:?}", reason);
    Ok(())
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'values, 'server, C: Controller>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids128(&[NUS_SERVICE_UUID.to_le_bytes()]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}

// async fn custom_task<C: Controller, P: PacketPool>(
//     server: &Server<'_>,
//     conn: &GattConnection<'_, '_, P>,
//     stack: &Stack<'_, C, P>,
// ) {
//     let mut tick: u8 = 0;
//     let level = server.battery_service.level;
//     loop {
//         tick = tick.wrapping_add(1);
//
//         // read RSSI (Received Signal Strength Indicator) of the connection.
//         if let Ok(rssi) = conn.raw().rssi(stack).await {
//             info!("[custom_task] RSSI: {:?}", rssi);
//         } else {
//             info!("[custom_task] error getting RSSI");
//             break;
//         };
//         Timer::after_secs(2).await;
//     }
// }
