#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use alloc::format;
use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;

use esp_hal::spi;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, clock::CpuClock};
use esp_radio::ble::controller::BleConnector;
use log::{debug, info, trace};
use memori_dev::{MemTermInitPins, setup_term};
use trouble_host::prelude::*;
use weact_studio_epd::graphics::Display290BlackWhite;

extern crate alloc;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 1;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

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

    //NOTE: Need to check exactly how much memory we should use / if this
    // will suffice
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    // COEX needs more RAM - so we've added some more
    // esp_alloc::heap_allocator!(size: 64 * 1024);
    // shit ton of memory alloc
    // esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 66320);
    esp_alloc::heap_allocator!(size: 170* 1024);

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
    let _stack = trouble_host::new(ble_controller, &mut resources);

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

    spawner
        .spawn(hello_task())
        .expect("Failed to begin hello_task");

    spawner
        .spawn(ui_task(spi_bus, term_init_pins))
        .expect("Failed to begin ui_task");

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

/// The UI task for our application.
#[embassy_executor::task]
#[allow(
    clippy::large_stack_frames,
    reason = "The display needs a large frame buffer."
)]
pub async fn ui_task(spi: Spi<'static, Blocking>, term_init_pins: MemTermInitPins) {
    info!("UI Task Begun!");
    let mut display = Display290BlackWhite::new();
    let mut term = setup_term(spi, &mut display, term_init_pins);
    debug!("initialized terminal");
    let mut i = 0;

    loop {
        let string = format!("Hello world! {i}");
        trace!("render frame");
        term.draw(|f| {
            f.render_widget(string, f.area());
        })
        .unwrap();
        i += 1;
        // how often the display updates
        // Timer::after_secs(1).await;
    }
}
