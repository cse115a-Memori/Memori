use crate::commands::translation_structs::*;
use crate::state::{AppState, DeviceConnection};
use crate::widget_data::bus_data::refresh_bus_widget;
use crate::widget_data::clock_data::{refresh_clock_widget, clock_to_memori_widget};
use crate::widget_data::github_data::refresh_github_widget;
use crate::widget_data::twitch_data::refresh_twitch_widget;
use crate::widget_data::weather_data::refresh_weather_widget;
use memori_ui::widgets::{WidgetKind};
use memori_ui::widgets::Name;
use memori_ui::{widgets::MemoriWidget, MemoriState};
use serde::de::DeserializeOwned;
use tauri::{AppHandle, State};
use tauri_plugin_svelte::ManagerExt;
use transport::HostTransport as _;

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

async fn set_memori_state(
    state: &State<'_, AppState>,
    memori_state: MemoriState,
) -> Result<(), String> {
    println!("set_memori_state payload: {:?}", &memori_state);

    let mut conn_guard = state.conn.lock().await;

    let result = match &mut *conn_guard {
        DeviceConnection::RealDevice(transport) => transport
            .set_state(memori_state.clone())
            .await
            .map_err(|e| format!("Failed to set state: {e}")),
        DeviceConnection::Simulator(transport) => transport
            .set_state(memori_state.clone())
            .await
            .map_err(|e| format!("Failed to set state: {e}")),
        DeviceConnection::Disconnected => {
            return Err("Device is not connected".to_string());
        }
    };

    if result.is_ok() {
        let mut memori_guard = state.memori.write().await;
        *memori_guard = Some(memori_state);
    }

    Ok(())
}

pub fn read_store_state<T>(app: &AppHandle, store_id: &str) -> T
where
    T: DeserializeOwned + Default,
{
    app.svelte().state_or_default(store_id).unwrap_or_default()
}

/// Flashes the `MemoriStateInput` to the device.
///
/// # Errors
/// Could error if the device isnt connected.
#[tauri::command]
#[specta::specta]
pub async fn flash_memori_state(
    state: State<'_, AppState>,
    memori_state: MemoriStateInput,
) -> Result<(), String> {
    set_memori_state(&state, memori_state.into_memori_state()?).await
}

/// Sends back a copy of the different widget types that are used during drag and drop.
#[tauri::command]
#[specta::specta]
pub async fn get_widget_kinds(app: AppHandle) -> Result<[MemoriWidget; 6], String> {
    let prefs: PrefsState = read_store_state(&app, "prefs");
    let clock = refresh_clock_widget().await.unwrap_or_default();
    let weather = refresh_weather_widget(36.97145812967173, -122.03535749883835).await.unwrap_or_default();
    let bus = refresh_bus_widget().await.unwrap_or_default();
    let github = refresh_github_widget(&app).await.unwrap_or_default();
    let twitch = refresh_twitch_widget(&app).await.unwrap_or_default();
    let name = memori_ui::widgets::Name { name: prefs.name.clone() };

    Ok([
        clock_to_memori_widget(1, clock).await.unwrap(),
        MemoriWidget::with_minute_update_frequency(2, WidgetKind::Weather(weather), 30),
        MemoriWidget::with_minute_update_frequency(3, WidgetKind::Bus(bus), 1),
        MemoriWidget::with_minute_update_frequency(4, WidgetKind::Github(github), 1),
        MemoriWidget::with_second_update_frequency(5, WidgetKind::Twitch(twitch), 5),
        MemoriWidget::with_never_update_frequency(6, WidgetKind::Name(name))
    ])
}
