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
use esp_backtrace as _;

use ble_device::DeviceBLETransport;
use esp_hal::spi;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, clock::CpuClock};
use esp_radio::ble::controller::BleConnector;
use log::{info, trace};
use memori::{Memori, MemoriState};
use memori_esp32c3::ble::ble_task;
use memori_esp32c3::{MemTermInitPins, setup_term};
use static_cell::StaticCell;
use transport::{DeviceTransport, WidgetId};
use weact_studio_epd::graphics::Display290BlackWhite;

extern crate alloc;

static RADIO: StaticCell<esp_radio::Controller<'static>> = StaticCell::new();

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

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 170 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let radio_init: esp_radio::Controller<'_> =
        esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");

    let radio = RADIO.init(radio_init);

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
        rst_pin: peripherals.GPIO4,
        busy_pin: peripherals.GPIO5,
    };

    // spawner
    //     .spawn(hello_task())
    //     .expect("Failed to begin hello_task");

    // spawner
    //     .spawn(ui_task(spi_bus, term_init_pins))
    //     .expect("Failed to begin ui_task");

    spawner
        .spawn(ble_task(radio, peripherals.BT))
        .expect("failed to begin ble task");

    spawner
        .spawn(logic_task())
        .expect("failed to begin logic task");
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
    let term = setup_term(spi, &mut display, term_init_pins);
    let mut memori = Memori::new(term);
    let mut mem_state = MemoriState::default();

    loop {
        let instant = Instant::now();
        memori
            .update(&mem_state)
            .expect("should have been successfull");

        let frame_time = instant.elapsed();

        trace!("frame time: {:?}ms", frame_time.as_millis());

        match mem_state {
            MemoriState::Example(ref mut cont) => cont.i += 1,
        }
    }
}

#[embassy_executor::task]
async fn logic_task() {
    let mut transport = DeviceBLETransport::new();

    loop {
        Timer::after(Duration::from_secs(1)).await;

        match transport.ping().await {
            Ok(_) => info!("[logic] ping success"),
            Err(e) => info!("[logic] ping failed: {:?}", e),
        }

        Timer::after(Duration::from_secs(1)).await;

        match transport.refresh_data(WidgetId(12)).await {
            Ok(data) => match core::str::from_utf8(&data) {
                Ok(s) => info!("[logic] refresh success, data: {}", s),
                Err(_) => info!("[logic] refresh success (invalid str): {:?}", data),
            },
            Err(e) => info!("[logic] refresh failed: {:?}", e),
        }
    }
}
