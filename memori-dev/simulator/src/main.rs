use color_eyre::eyre::Result;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use memori::{Memori, MemoriState};
use memori_tcp::{DeviceResponse, DeviceTcpTransport, HostRequest};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 4,
            ..Default::default()
        },
    );

    let transport = DeviceTcpTransport::new(request_handler).await?;

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

async fn request_handler(req: HostRequest) -> DeviceResponse {
    match req {
        HostRequest::GetBatteryLevel => todo!(),
        HostRequest::Ping => todo!(),
        HostRequest::SetDeviceConfig(device_config) => todo!(),
        HostRequest::SetWidgets(widget) => todo!(),
        HostRequest::GetWidget(widget_id) => todo!(),
    }
}
