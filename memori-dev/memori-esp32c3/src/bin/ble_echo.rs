#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use embassy_futures::{join::join, select::select};
use esp_backtrace as _;

use esp_hal::spi;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, clock::CpuClock};
use esp_radio::ble::controller::BleConnector;
use log::{info, trace, warn};
use memori::{Memori, MemoriState};
use memori_esp32c3::{MemTermInitPins, setup_term};
use trouble_host::prelude::*;
use weact_studio_epd::graphics::Display290BlackWhite;

extern crate alloc;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 1;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const NUS_SERVICE_UUID: [u8; 16] = [
    0x9e, 0xca, 0xdc, 0x24, 0x0e, 0xe5, 0xa9, 0xe0,
    0x93, 0xf3, 0xa3, 0xb5, 0x01, 0x00, 0x40, 0x6e,
];

// GATT Server definition
#[gatt_server]
struct Server {
    nus_service: NordicUartService,
}

#[gatt_service(uuid = "6e400001-b5a3-f393-e0a9-e50e24dcca9e")]
struct NordicUartService {
    #[characteristic(uuid = "6e400002-b5a3-f393-e0a9-e50e24dcca9e", write, write_without_response)]
    #[descriptor(uuid = "2901", value = "RX Characteristic")]
    rx: [u8; 32],  // max BLE packet size?
    
    #[characteristic(uuid = "6e400003-b5a3-f393-e0a9-e50e24dcca9e", read, notify)]
    #[descriptor(uuid = "2901", value = "TX Characteristic")]
    tx: [u8; 32],
}
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> () {
    // generator version: 1.1.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 170 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");
    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let transport = BleConnector::new(&radio_init, peripherals.BT, Default::default()).unwrap();
    let ble_controller = ExternalController::<_, 1>::new(transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    let mosi_pin = peripherals.GPIO10;
    let sclk_pin = peripherals.GPIO8;

    let spi_bus = Spi::new(
        peripherals.SPI2,
        spi::master::Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(spi::Mode::_0),
    )
    .expect("Failed to create SPI bus")
    .with_sck(sclk_pin)
    .with_mosi(mosi_pin);

    let term_init_pins = MemTermInitPins {
        cs_pin: peripherals.GPIO3,
        dc_pin: peripherals.GPIO2,
        rst_pin: peripherals.GPIO1,
        busy_pin: peripherals.GPIO0,
    };

    let address: Address = Address::random([0xfc, 0x8f, 0x1a, 0x05, 0xe1, 0x0f]);
    info!("Our address = {:?}", address);

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "Memori",
        appearance: &appearance::computer::GENERIC_COMPUTER,
    }))
    .unwrap();

    let _ = join(ble_task(runner), async {
        loop {
            match advertise("memori", &mut peripheral, &server).await {
                Ok(conn) => {
                    // set up tasks when the connection is established to a central, so they don't
                    // run when no one is connected.
                    let a = gatt_events_task(&server, &conn);
                    let b = custom_task(&server, &conn, &stack);
                    // run until any task ends (usually because the connection has been closed),
                    // then return to advertising state.
                    select(a, b).await;
                }
                Err(e) => {
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;

    // spawner
    //     .spawn(hello_task())
    //     .expect("Failed to begin hello_task");

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v~1.0/examples
}

// This is an example of how to create a task.
#[embassy_executor::task]
pub async fn hello_task() {
    loop {
        info!("Hello everyone!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

// The UI task for our application.
// #[embassy_executor::task]
// #[allow(
//     clippy::large_stack_frames,
//     reason = "The display needs a large frame buffer."
// )]
// pub async fn ui_task(spi: Spi<'static, Blocking>, term_init_pins: MemTermInitPins) {
//     info!("UI Task Begun!");
//     let mut display = Display290BlackWhite::new();
//     let term = setup_term(spi, &mut display, term_init_pins);
//     let mut memori = Memori::new(term);
//     let mut mem_state = MemoriState::default();
//
//     loop {
//         let instant = Instant::now();
//         memori
//             .update(&mem_state)
//             .expect("should have been successfull");
//
//         let frame_time = instant.elapsed();
//
//         trace!("frame time: {:?}ms", frame_time.as_millis());
//
//         match mem_state {
//             MemoriState::Example(ref mut cont) => cont.i += 1,
//         }
//     }
// }
//

/// This is a background task that is required to run forever alongside any other BLE tasks.
///
/// ## Alternative
///
/// If you didn't require this to be generic for your application, you could statically spawn this
/// with i.e.
///
/// ```rust,ignore
/// 
/// #[embassy_executor::task]
/// async fn ble_task(mut runner: Runner<'static, SoftdeviceController<'static>>) {
///     runner.run().await;
/// }
///
/// spawner.must_spawn(ble_task(runner));
/// ```
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            panic!("[ble_task] error: {:?}", e);
        }
    }
}

/// Stream Events until the connection closes.
///
/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task<P: PacketPool>(
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
                            let mut tx_buffer = [0u8; 32];
                            let len = data.len().min(32);
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
            AdStructure::ServiceUuids128(&[NUS_SERVICE_UUID]),
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

/// Example task to use the BLE notifier interface.
/// This task will notify the connected central of a counter value every 2 seconds.
/// It will also read the RSSI value every 2 seconds.
/// and will stop when the connection is closed by the central or an error occurs.
async fn custom_task<C: Controller, P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) {
    let mut tick: u8 = 0;
    // let level = server.config_service.level;
    loop {
        // tick = tick.wrapping_add(1);
        // info!("[custom_task] notifying connection of tick {}", tick);
        // // if level.notify(conn, &tick).await.is_err() {
        // //     info!("[custom_task] error notifying connection");
        // //     break;
        // // };
        // // read RSSI (Received Signal Strength Indicator) of the connection.
        // if let Ok(rssi) = conn.raw().rssi(stack).await {
        //     info!("[custom_task] RSSI: {:?}", rssi);
        // } else {
        //     info!("[custom_task] error getting RSSI");
        //     break;
        // };
        Timer::after_secs(2).await;
    }
}
