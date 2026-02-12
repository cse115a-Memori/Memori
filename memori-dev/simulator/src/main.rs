use color_eyre::eyre::Result;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use memori_tcp::{DeviceResponse, DeviceTcpTransport, HostRequest, Sequenced};
use memori_ui::layout::MemoriLayout;
use memori_ui::widgets::{MemoriWidget, Name, UpdateFrequency, WidgetId, WidgetKind};
use memori_ui::{Memori, MemoriState};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};
use transport::DeviceTransport;

use ratatui::Terminal;
use tracing::{Level, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();

    // Install global subscriber configured based on RUST_LOG envvar.

    tracing_subscriber::fmt()
        // Filter spans/events with level TRACE or higher.
        .with_max_level(Level::DEBUG)
        // Build but do not install the subscriber.
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
        font_regular: memori_ui::FONT_REGULAR,
        font_bold: memori_ui::FONT_BOLD,
        font_italic: memori_ui::FONT_ITALIC,
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

    let mem_state = {
        let state = MemoriState::new(
            1,
            vec![MemoriWidget::new(
                WidgetId(0),
                WidgetKind::Name(Name::new("surendra")),
                UpdateFrequency::Never,
                UpdateFrequency::Never,
            )],
            vec![
                MemoriLayout::Fourths {
                    top_left: WidgetId(0),
                    top_right: WidgetId(0),
                    bottom_left: WidgetId(0),
                    bottom_right: WidgetId(0),
                },
                MemoriLayout::VSplit {
                    left: WidgetId(0),
                    right: WidgetId(0),
                },
            ],
            5,
        );
        Arc::new(Mutex::new(state))
    };

    tokio::spawn(state_handler(mem_state.clone()));

    // This loop contains the logic for running the UI
    loop {
        memori
            .update(&*mem_state.lock().await)
            .expect("should have been successfull");

        // Thread sleep so it doesn't busy loop
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

async fn state_handler(state: Arc<Mutex<MemoriState>>) -> Result<()> {
    let transport = DeviceTcpTransport::default();

    let (mut conn, (mut host_req_rx, dev_resp_tx)) = transport.connect().await?;
    loop {
        conn.ping().await?;
        info!("Connected!");

        if let Ok(req) = host_req_rx.try_recv() {
            info!("received device request! {req:?}");
            let resp = match req.msg_kind {
                HostRequest::Ping => DeviceResponse::Pong,
                HostRequest::GetBatteryLevel => DeviceResponse::BatteryLevel(69),
                HostRequest::SetDeviceConfig(_config) => {
                    todo!()
                }
                HostRequest::SetState(new_state) => {
                    let state = &mut *state.lock().await;
                    *state = *new_state;
                    DeviceResponse::Success
                }
                HostRequest::GetWidget(_id) => todo!(),
            };

            info!("sending response: {resp:#?}");
            dev_resp_tx
                .send(Sequenced::new(req.seq_num, resp))
                .inspect_err(|e| error!("failed to send: {e}"))
                .unwrap();
        }

        sleep(Duration::from_secs(1)).await;
    }
}
