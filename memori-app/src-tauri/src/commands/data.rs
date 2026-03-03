use crate::oauth::cloudflare;
use crate::state::{AppState, DeviceConnection};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{Bus, Clock, MemoriWidget, Name, UpdateFrequency, Weather, WidgetId, WidgetKind},
    MemoriState,
};
use serde::{de::DeserializeOwned, Deserialize};
use tauri::State;
use transport::HostTransport as _;

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct MemoriStateInput {
    active_frame_idx: u32,
    widgets: Vec<MemoriWidget>,
    frames: Vec<MemoriLayout>,
    frame_time: u32,
}

impl MemoriStateInput {
    fn into_memori_state(self) -> Result<MemoriState, String> {
        let active_frame_idx = usize::try_from(self.active_frame_idx)
            .map_err(|_| "activeFrameIdx is out of range for this platform".to_string())?;

        if self.frames.is_empty() {
            return Err("frames cannot be empty".to_string());
        }

        if active_frame_idx >= self.frames.len() {
            return Err(format!(
                "activeFrameIdx {} is out of bounds for {} frame(s)",
                self.active_frame_idx,
                self.frames.len()
            ));
        }

        Ok(MemoriState::new(
            active_frame_idx,
            self.widgets,
            self.frames,
            self.frame_time,
        ))
    }
}

#[tauri::command]
#[specta::specta]
pub async fn flash_memori_state(
    state: State<'_, AppState>,
    memori_state: MemoriStateInput,
) -> Result<(), String> {
    let memori_state = memori_state.into_memori_state()?;
    let mut guard = state.conn.lock().await;

    match &mut *guard {
        DeviceConnection::RealDevice(transport) => transport
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}")),
        DeviceConnection::Simulator(transport) => transport
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}")),
        DeviceConnection::Disconnected => Err("Device is not connected".to_string()),
    }
}

pub async fn set_memori_state(
    state: &State<'_, AppState>,
    memori_state: MemoriState,
) -> Result<(), String> {
    let mut state_guard = state.conn.lock().await;

    match &mut *state_guard {
        DeviceConnection::RealDevice(host_bletransport) => host_bletransport
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}")),
        DeviceConnection::Simulator(host_tcp_transport) => host_tcp_transport
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}")),
        DeviceConnection::Disconnected => Err("Device is not connected".to_string()),
    }
}

pub async fn call_api_json<T>(args: serde_json::Value) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let response = cloudflare("call_api", args)
        .await
        .map_err(|e| format!("cloudflare error: {e}"))?;
    serde_json::from_value(response).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_widget_kinds() -> Result<[MemoriWidget; 4], String> {
    Ok([
        MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Name(Name::new("John Doe")),
            UpdateFrequency::Never,
            UpdateFrequency::Never,
        ),
        MemoriWidget::new(
            WidgetId(1),
            WidgetKind::Clock(Clock::new(1, 0, 0)),
            UpdateFrequency::Never,
            UpdateFrequency::Never,
        ),
        MemoriWidget::new(
            WidgetId(2),
            WidgetKind::Bus(Bus::new("9 min", "Route 19")),
            UpdateFrequency::Seconds(5),
            UpdateFrequency::Never,
        ),
        MemoriWidget::new(
            WidgetId(3),
            WidgetKind::Weather(Weather::new("20.0")),
            UpdateFrequency::Seconds(1),
            UpdateFrequency::Never,
        ),
    ])
}

#[tauri::command]
#[specta::specta]
pub async fn send_name(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Name(Name::new(name)),
            UpdateFrequency::Seconds(1),
            UpdateFrequency::Seconds(1),
        )],
        vec![MemoriLayout::Fourths {
            top_right: WidgetId(0),
            bottom_left: WidgetId(0),
            bottom_right: WidgetId(0),
            top_left: WidgetId(0),
        }],
        5,
    );
    set_memori_state(&state, memori_state).await
}
