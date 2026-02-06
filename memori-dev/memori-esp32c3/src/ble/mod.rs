use ble_device::{BLE_CONNECTED, BLE_HOST_RESPONSE};
use core::usize;
use embassy_futures::{join::join, select::select};
use esp_hal::peripherals;
use esp_radio::ble::controller::BleConnector;
use log::{info, warn};
use postcard::{from_bytes, to_slice};
use transport::ble_types::*;
use transport::{TransError, TransResult};
use trouble_host::prelude::*;

use crate::ble::host_handler::handle_host_cmd;
use crate::ble::sender::sender_task;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 1;
const MAX_INFLIGHT: usize = 4;

const PERIPHERAL_NAME: &str = "memori";

/// Functionality to send messages to host.
mod sender;

/// Functionality regarding host communication.
mod host_handler;

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
                    let b = sender_task(&server, &conn);
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
    let rx_handle = server.nus_service.rx.handle;
    let battery_handle = server.battery_service.level.handle;

    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                match &event {
                    GattEvent::Write(event) => {
                        if event.handle() == rx_handle {
                            handle_receive_data(event.data(), server, conn).await;
                        }
                    }
                    GattEvent::Read(event) => {
                        if event.handle() == battery_handle {
                            let value = server.get(&server.battery_service.level);
                            info!(
                                "[gatt] Read event to battery level characteristic: {:?}",
                                value
                            );
                        }
                    }
                    _ => {}
                };
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

async fn handle_receive_data<P: PacketPool>(
    data: &[u8],
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) {
    info!("[gatt] received {} bytes", data.len());
    let packet = match from_bytes::<BLEPacket>(data) {
        Ok(packet) => packet,
        Err(e) => {
            warn!("[gatt] failed to decode BLEPacket: {:?}", e);
            return;
        }
    };

    let payload = match packet.payload {
        BLEPacketPayload::HostPacket(payload) => payload,
        BLEPacketPayload::DevicePacket { .. } => {
            warn!("[transport] received devicepacket...");
            return;
        }
    };

    match payload {
        HostBLEPacket::Command(cmd) => {
            handle_host_cmd(cmd, packet.id, server, conn).await;
        }
        HostBLEPacket::Response(resp) => {
            // we have a response!
            let index = packet.id as usize; // As long as architecture is >= 32b we chill
            BLE_HOST_RESPONSE[index % MAX_INFLIGHT].signal(resp);
        }
    }
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

async fn send_packet<P: PacketPool>(
    packet: DeviceBLEPacket,
    msg_id: MessageID,
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> TransResult<()> {
    let tx = server.nus_service.tx;
    let mut buffer = [0u8; BLE_CHAR_SIZE];

    let packet = BLEPacket {
        payload: BLEPacketPayload::DevicePacket(packet),
        id: msg_id,
    };

    let _encoded = to_slice(&packet, &mut buffer).map_err(|_| TransError::InternalError)?;

    tx.notify(conn, &buffer)
        .await
        .map_err(|_| TransError::InternalError)?;

    Ok(())
}
