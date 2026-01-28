use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use memori::{Memori, MemoriState};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;
use std::time::Instant;
use std::time::Duration;

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

    let mut last_tick = Instant::now();

    loop {

        let instant = Instant::now();
        let elapsed = instant - last_tick;

        memori
            .update(&mem_state)
            .expect("should have been successfull");

        match mem_state {
            MemoriState::Example(ref mut cont) => cont.i += 1,

            //We will sync actual time later.
            MemoriState::Time(ref mut clock) => {
                if elapsed >= Duration::from_secs(1) {
                    clock.seconds += 1;

                    if clock.seconds >= 60 {
                        clock.seconds = 0;
                        clock.minutes += 1;
                    }

                    if clock.minutes >= 60 {
                        clock.minutes = 0;
                        clock.hours += 1;
                    }

                    if clock.hours >= 12 {
                        clock.hours = 1;
                    }

                    last_tick = instant;
                }
            }
        }

        // thread sleep so it doesnt busy loop
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
