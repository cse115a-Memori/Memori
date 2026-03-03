use color_eyre::eyre::Result;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use memori_tcp::{DeviceResponse, DeviceTcpTransport, HostRequest, Sequenced};
use memori_ui::layout::MemoriLayout;
use memori_ui::widgets::{MemoriWidget, Name, UpdateFrequency, WidgetId, WidgetKind};
use memori_ui::{Memori, MemoriState};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use std::{collections::HashMap, sync::Arc, time::Duration, time::Instant};
use tokio::{sync::Mutex, time::sleep};
use transport::DeviceTransport;

use ratatui::{Terminal, widgets};
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
            4,
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
                MemoriLayout::Full(WidgetId(0)),
                MemoriLayout::VSplit {
                    left: WidgetId(0),
                    right: WidgetId(0),
                },
                MemoriLayout::HSplit {
                    top: WidgetId(0),
                    bottom: WidgetId(0),
                },
                MemoriLayout::VSplitWithLeftHSplit {
                    left_top: WidgetId(0),
                    left_bottom: WidgetId(0),
                    right: WidgetId(0),
                },
                MemoriLayout::VSplitWithRightHSplit {
                    right_top: WidgetId(0),
                    right_bottom: WidgetId(0),
                    left: WidgetId(0),
                },
                MemoriLayout::HSplitWithTopVSplit {
                    top_left: WidgetId(0),
                    top_right: WidgetId(0),
                    bottom: WidgetId(0),
                },
                MemoriLayout::HSplitWithBottomVSplit {
                    top: WidgetId(0),
                    bottom_left: WidgetId(0),
                    bottom_right: WidgetId(0),
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
    let mut last_refresh: HashMap<WidgetId, Instant> = HashMap::new();
    loop {
        conn.ping().await?;
        info!("Connected!");

        // refresh logic
        let widgets_to_refresh: Vec<MemoriWidget> = {
            let current_state = state.lock().await;
            current_state
                .widgets
                .iter()
                .filter(|(_, widget)| {
                    widget.get_remote_update_frequency() != UpdateFrequency::Never
                })
                .map(|(_, widget)| widget.clone())
                .collect()
        };
        let now = Instant::now();
        println!("refresh map: ");
        for (widget_id, last_time) in &last_refresh {
            let elapsed = now.duration_since(*last_time).as_secs();
            println!("  Widget {:?}: {} seconds ago", widget_id, elapsed);
        }
        if last_refresh.is_empty() {
            println!("  (no widgets refreshed yet)");
        }
        if let Some(widget) = widgets_to_refresh.first() {
            let should_refresh = match last_refresh.get(&widget.id) {
                None => true,
                Some(last_time) => {
                    if let Some(freq_seconds) = widget.get_remote_update_frequency().to_seconds() {
                        now.duration_since(*last_time).as_secs() >= freq_seconds as u64
                    } else {
                        false
                    }
                }
            };
            if should_refresh {
                match conn.refresh_data(WidgetId(0)).await {
                    Ok(updated_widget) => {
                        let mut state = state.lock().await;
                        state.widgets.insert(widget.id, updated_widget);
                        last_refresh.insert(widget.id, now);
                    }
                    Err(_) => {
                        // Continue without updating on error
                    }
                }
            }
        }

        // request logic
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
