#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;

use ble_device::DeviceBLETransport;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use log::info;
use memori_esp32c3::ble::ble_task;
use memori_ui::MemoriState;
use static_cell::StaticCell;
use transport::DeviceTransport;

extern crate alloc;

static RADIO: StaticCell<esp_radio::Controller<'static>> = StaticCell::new();

static MEMORI_STATE: StaticCell<Mutex<CriticalSectionRawMutex, MemoriState>> = StaticCell::new();

static BLE_TRANSPORT: StaticCell<Mutex<CriticalSectionRawMutex, DeviceBLETransport>> =
    StaticCell::new();

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

    let mem_state = MEMORI_STATE.init_with(|| {
        let mem_state = MemoriState::default();

        Mutex::new(mem_state)
    });
    let transport = BLE_TRANSPORT.init(Mutex::<CriticalSectionRawMutex, DeviceBLETransport>::new(
        DeviceBLETransport::new(),
    ));

    spawner
        .spawn(ble_task(
            radio,
            peripherals.BT,
            transport,
            mem_state,
            spawner,
        ))
        .expect("failed to begin ble task");

    spawner
        .spawn(logic_task())
        .expect("failed to begin logic task");

    // spawner
    //     .spawn(logic_task2())
    //     .expect("failed to begin logic2 task");
    // spawner
    //     .spawn(logic_task3())
    //     .expect("failed to begin logic2 task");
    // spawner
    //     .spawn(logic_task4())
    //     .expect("failed to begin logic2 task");
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
    }
}

// #[embassy_executor::task]
// async fn logic_task2() {
//     let mut transport = DeviceBLETransport::new();

//     loop {
//         Timer::after(Duration::from_secs(1)).await;

//         match transport.refresh_data(WidgetId(1)).await {
//             Ok(data) => info!("[logic] ref1 success: {:?}", data),
//             Err(e) => info!("[logic] ref1 failed: {:?}", e),
//         }
//     }
// }
// #[embassy_executor::task]
// async fn logic_task3() {
//     let mut transport = DeviceBLETransport::new();

//     loop {
//         Timer::after(Duration::from_secs(1)).await;

//         match transport.refresh_data(WidgetId(2)).await {
//             Ok(data) => info!("[logic] ref2 success: {:?}", data),
//             Err(e) => info!("[logic] ref2 failed: {:?}", e),
//         }
//     }
// }
// #[embassy_executor::task]
// async fn logic_task4() {
//     let mut transport = DeviceBLETransport::new();

//     loop {
//         Timer::after(Duration::from_secs(1)).await;

//         match transport.refresh_data(WidgetId(3)).await {
//             Ok(data) => info!("[logic] ref3 success: {:?}", data),
//             Err(e) => info!("[logic] ref3 failed: {:?}", e),
//         }
//     }
// }
