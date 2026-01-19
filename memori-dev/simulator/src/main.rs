use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use memori::{Memori, MemoriState};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;

fn main() -> Result<(), std::convert::Infallible> {
    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 4,
            ..Default::default()
        },
    );

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(296, 128));

    simulator_window.set_max_fps(1);

    let backend_config = EmbeddedBackendConfig {
        font_regular: memori::FONT_REGULAR,
        font_bold: memori::FONT_BOLD,
        font_italic: memori::FONT_ITALIC,
        // Define how to display newly rendered widgets to the simulator window
        flush_callback: Box::new(move |display| {
            simulator_window.update(display);
            if simulator_window.events().any(|e| e == SimulatorEvent::Quit) {
                panic!("simulator window closed");
            }
        }),
        ..Default::default()
    };
    let backend: EmbeddedBackend<SimulatorDisplay<_>, _> =
        EmbeddedBackend::new(&mut display, backend_config);

    // Start ratatui with our simulator backend
    let term = Terminal::new(backend).expect("something went wrong");

    let mut memori = Memori::new(term);
    let mut mem_state = MemoriState::default();

    loop {
        memori
            .update(&mem_state)
            .expect("should have been successfull");

        match mem_state {
            MemoriState::Example(ref mut cont) => cont.i += 1,
        }

        // thread sleep so it doesnt busy loop
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
