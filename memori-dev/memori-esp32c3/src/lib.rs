#![no_std]
extern crate alloc;

pub mod ble;
pub mod local_widget_update;

use alloc::boxed::Box;
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    Blocking,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    peripherals::{GPIO2, GPIO3, GPIO4, GPIO5},
    spi::master::Spi,
};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;
use weact_studio_epd::{
    WeActStudio290BlackWhiteDriver,
    graphics::{Display, DisplayRotation},
};

/// Helper type for the WeActStudio display.
pub type MemDisplay = Display<128, 296, 4736, weact_studio_epd::Color>;

/// Helper type for the Terminal.
pub type MemTerm<'a> = Terminal<
    EmbeddedBackend<'a, Display<128, 296, 4736, weact_studio_epd::Color>, weact_studio_epd::Color>,
>;

/// Pins needed to initialize the terminal.
pub struct MemTermInitPins {
    pub cs_pin: GPIO3<'static>,
    pub dc_pin: GPIO2<'static>,
    pub rst_pin: GPIO4<'static>,
    pub busy_pin: GPIO5<'static>,
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

    let mut frame_count = 0;

    let config = EmbeddedBackendConfig {
        font_regular: memori_ui::FONT_REGULAR,
        font_bold: memori_ui::FONT_BOLD,
        font_italic: memori_ui::FONT_ITALIC,
        flush_callback: Box::new(move |d| {
            // do full update every 5 frames to maintain display health.
            frame_count += 1;
            if frame_count == 5 {
                driver.full_update(d).unwrap();
                frame_count = 0;
            } else {
                driver.fast_update(d).unwrap();
            }
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(display, config);

    Terminal::new(backend).unwrap()
}
