use std::{sync::Arc, time::Duration};

use color_eyre::eyre::Result;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use memori::{Memori, MemoriState};
use memori_tcp::{DeviceResponse, DeviceTcpTransport, HostRequest};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use tokio::{sync::Mutex, time::sleep};
use transport::DeviceTransport;

use ratatui::Terminal;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();

    // install global subscriber configured based on RUST_LOG envvar.

    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::DEBUG)
        // build but do not install the subscriber.
        .init();

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
    let mem_state = Arc::new(Mutex::new(MemoriState::default()));

    tokio::spawn(state_handler(mem_state.clone()));

    loop {
        memori
            .update(&*mem_state.lock().await)
            .expect("should have been successfull");

        // thread sleep so it doesnt busy loop
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

async fn state_handler(state: Arc<Mutex<MemoriState>>) -> Result<()> {
    let transport = DeviceTcpTransport::new(request_handler);
    let mut transport = transport.connect().await?;

    loop {
        transport.ping().await?;
        info!("Connected!");

        let state = &mut *state.lock().await;
        match state {
            MemoriState::Example(counter) => counter.i += 1,
        }

        sleep(Duration::from_secs(1)).await;
    }
}

async fn request_handler(req: HostRequest) -> DeviceResponse {
    match req {
        HostRequest::GetBatteryLevel => DeviceResponse::BatteryLevel(10),
        HostRequest::Ping => todo!(),
        HostRequest::SetDeviceConfig(device_config) => todo!(),
        HostRequest::SetWidgets(widget) => todo!(),
        HostRequest::GetWidget(widget_id) => todo!(),
    }
}
