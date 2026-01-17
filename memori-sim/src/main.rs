use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_6X9, FONT_10X20},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use profont::{PROFONT_12_POINT, PROFONT_18_POINT};

fn main() -> Result<(), std::convert::Infallible> {
    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 4,
            ..Default::default()
        },
    );

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(296, 128));
    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, BinaryColor::On);
    let mut i = 0;

    simulator_window.set_max_fps(1);

    loop {
        display.clear(BinaryColor::Off)?;
        Text::new(&format!("Hello World {i}"), Point::new(0, 18), text_style).draw(&mut display)?;
        i += 1;

        simulator_window.update(&display);

        for event in simulator_window.events() {
            if event == embedded_graphics_simulator::SimulatorEvent::Quit {
                return Ok(());
            }
        }
    }
}
