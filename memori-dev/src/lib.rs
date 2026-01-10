#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    Blocking,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    peripherals::{GPIO0, GPIO1, GPIO2, GPIO3},
    spi::master::Spi,
};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig, fonts};
use weact_studio_epd::{
    WeActStudio290BlackWhiteDriver,
    graphics::{Display, DisplayRotation},
};

use ratatui::Terminal;
/// Helper type for the display we are working with.
pub type MemDisplay = Display<128, 296, 4736, weact_studio_epd::Color>;

/// Helper type for the pseudo-terminal.
pub type MemTerm<'a> = Terminal<
    EmbeddedBackend<'a, Display<128, 296, 4736, weact_studio_epd::Color>, weact_studio_epd::Color>,
>;

pub struct MemTermInitPints {
    pub cs_pin: GPIO3<'static>,
    pub dc_pin: GPIO2<'static>,
    pub rst_pin: GPIO1<'static>,
    pub busy_pin: GPIO0<'static>,
}

pub fn setup_term<'a>(
    spi: Spi<'static, Blocking>,
    display: &'a mut MemDisplay,
    pins: MemTermInitPints,
) -> MemTerm<'a> {
    let cs = Output::new(pins.cs_pin, Level::High, OutputConfig::default());
    let busy = Input::new(pins.busy_pin, InputConfig::default().with_pull(Pull::Up));
    let dc = Output::new(pins.dc_pin, Level::Low, OutputConfig::default());
    let rst = Output::new(pins.rst_pin, Level::High, OutputConfig::default());
    let delay = Delay::new();

    let spi_device = ExclusiveDevice::new(spi, cs, delay.clone()).unwrap();

    let spi_interface = SPIInterface::new(spi_device, dc);

    let mut driver = WeActStudio290BlackWhiteDriver::new(spi_interface, busy, rst, delay.clone());
    display.set_rotation(DisplayRotation::Rotate90);
    driver.init().unwrap();

    let config = EmbeddedBackendConfig {
        font_regular: fonts::MONO_10X20,
        flush_callback: Box::new(move |d| {
            driver.fast_update(d).unwrap();
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(display, config);

    Terminal::new(backend).unwrap()
}
