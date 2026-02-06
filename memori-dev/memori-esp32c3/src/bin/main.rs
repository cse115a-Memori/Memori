#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use alloc::vec;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;

use esp_hal::spi;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, clock::CpuClock};
use log::{debug, info};
use memori_esp32c3::ble::ble_task;
use memori_esp32c3::{MemTermInitPins, setup_term};
use memori_ui::layout::MemoriLayout;
use memori_ui::widgets::{MemoriWidget, Name, UpdateFrequency, WidgetId, WidgetKind};
use memori_ui::{Memori, MemoriState};
use static_cell::StaticCell;
use weact_studio_epd::graphics::Display290BlackWhite;

extern crate alloc;

static RADIO: StaticCell<esp_radio::Controller<'static>> = StaticCell::new();

static MEMORI_STATE: StaticCell<Mutex<CriticalSectionRawMutex, MemoriState>> = StaticCell::new();

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> () {
    // Generator version: 1.1.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    //NOTE: Need to check exactly how much memory we should use / if this
    // will suffice
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    // COEX needs more RAM - so we've added some more
    // esp_alloc::heap_allocator!(size: 64 * 1024);
    // shit ton of memory allocation
    // esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 66320);
    esp_alloc::heap_allocator!(size: 170* 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let radio = RADIO.init(esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller"));

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

    let mem_state = MEMORI_STATE.init_with(|| {
        let mem_state = MemoriState::new(
            0,
            vec![MemoriWidget::new(
                WidgetId(0),
                WidgetKind::Name(Name::new("Surendra")),
                UpdateFrequency::Never,
            )],
            vec![MemoriLayout::Full(WidgetId(0))],
            5,
        );

        Mutex::new(mem_state)
    });

    spawner
        .spawn(hello_task())
        .expect("Failed to begin hello_task");

    spawner
        .spawn(ui_task(spi_bus, term_init_pins, mem_state))
        .expect("Failed to begin ui_task");

    spawner
        .spawn(ble_task(radio, peripherals.BT))
        .expect("Failed to start ble_task");
}

// This is an example of how to create a task.
#[embassy_executor::task]
pub async fn hello_task() {
    loop {
        info!("Hello everyone!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
#[allow(
    clippy::large_stack_frames,
    reason = "The display needs a large frame buffer."
)]
/// The UI task for our application.
pub async fn ui_task(
    spi: Spi<'static, Blocking>,
    term_init_pins: MemTermInitPins,
    state: &'static Mutex<CriticalSectionRawMutex, MemoriState>,
) {
    info!("UI Task Begun!");

    let mut display = Display290BlackWhite::new();
    let term = setup_term(spi, &mut display, term_init_pins);

    debug!("initialized terminal");
    let mut memori = Memori::new(term);

    loop {
        let state = &*state.lock().await;
        memori
            .update(state)
            .expect("memori should not panic on render");

        // TODO: in reality this should wait for a signal for
        // "hey! State changed you should re-render!"
        Timer::after(Duration::from_secs(1)).await;
    }
}
