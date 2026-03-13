#![no_std]
extern crate alloc;

pub mod ble;
pub mod local_widget_update;

use alloc::boxed::Box;
use display_interface_spi::SPIInterface;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Receiver, Sender},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    Blocking,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    peripherals::{GPIO2, GPIO3, GPIO4, GPIO5, GPIO6},
    spi::master::Spi,
};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;
use weact_studio_epd::{
    WeActStudio290BlackWhiteDriver,
    graphics::{Display, DisplayRotation},
};

pub const DEVICE_ID: &str = env!("DEVICE_ID");

/// Helper type for the WeActStudio display.
pub type MemDisplay = Display<128, 296, 4736, weact_studio_epd::Color>;

/// ZST to send a render message to the ui task.
pub struct Render {}

/// Render Receiver type to make things easier.
pub type RenderRx = Receiver<'static, CriticalSectionRawMutex, Render, 10>;

/// Render Sender type to make things easier.
pub type RenderTx = Sender<'static, CriticalSectionRawMutex, Render, 10>;

/// Helper type for the Terminal.
pub type MemTerm<'a> = Terminal<
    EmbeddedBackend<'a, Display<128, 296, 4736, weact_studio_epd::Color>, weact_studio_epd::Color>,
>;

/// Pins needed to initialize the terminal.
pub struct MemTermInitPins {
    pub cs_pin: GPIO4<'static>,
    pub dc_pin: GPIO3<'static>,
    pub rst_pin: GPIO5<'static>,
    pub busy_pin: GPIO6<'static>,
}

/// Set up the terminal with the given SPI device and display.
pub fn setup_term<'a>(
    spi: Spi<'static, Blocking>,
    display: &'a mut MemDisplay,
    pins: MemTermInitPins,
) -> MemTerm<'a> {
    let cs = Output::new(pins.cs_pin, Level::High, OutputConfig::default());
    let busy = Input::new(pins.busy_pin, InputConfig::default().with_pull(Pull::Up));
    let dc = Output::new(pins.dc_pin, Level::Low, OutputConfig::default());
    let rst = Output::new(pins.rst_pin, Level::High, OutputConfig::default());
    let delay = Delay::new();

    let spi_device = ExclusiveDevice::new(spi, cs, delay).unwrap();
    let spi_interface = SPIInterface::new(spi_device, dc);

    let mut driver = WeActStudio290BlackWhiteDriver::new(spi_interface, busy, rst, delay);
    display.set_rotation(DisplayRotation::Rotate90);
    driver.init().unwrap();

    let config = EmbeddedBackendConfig {
        font_regular: memori_ui::FONT_REGULAR,
        font_bold: memori_ui::FONT_BOLD,
        font_italic: memori_ui::FONT_ITALIC,
        flush_callback: Box::new(move |d| {
            driver.full_update(d).unwrap();
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(display, config);

    Terminal::new(backend).unwrap()
}
