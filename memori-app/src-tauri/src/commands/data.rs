use super::fetch::fetch_github_widget;
use crate::state::{AppState, DeviceConnection};
use crate::widget_data::{bus_data::*, github_data::*, twitch_data::*, weather_data::*};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{Bus, Clock, MemoriWidget, Name, UpdateFrequency, Weather, WidgetId, WidgetKind},
    MemoriState,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::time::Duration;
use tauri::{AppHandle, State};
use tauri_plugin_svelte::ManagerExt;
use tokio::time::timeout;
use transport::HostTransport as _;

const DEFAULT_TWITCH_USER: &str = "jujamont";
const DEFAULT_GITHUB_USERNAME: &str = "CaiNann";
const DEFAULT_GITHUB_REPO: &str = "Memori";
const DEFAULT_BUS_PREDICTION: (&str, &str, u16) = ("19", "Donwtown to Watsonville", 7);
const DEFAULT_BUS_STOP: (&str, &str) = ("High and Front", "1230");
const BUS_FETCH_TIMEOUT_SECS: u64 = 3;

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct MemoriStateInput {
    active_frame_idx: u32,
    widgets: Vec<MemoriWidget>,
    frames: Vec<MemoriLayout>,
    frame_time: u32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PrefsState {
    // location_status: String,
    pub last_known_location: Option<Position>,
    // onboarded: bool,
    // last_known_device_id: Option<String>,
    // system_options: serde_json::Value,
    name: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthState {
    pub users_by_provider: ProviderUsers,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUsers {
    pub github: Option<AuthUserInfo>,
    pub twitch: Option<AuthUserInfo>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthUserInfo {
    pub id: String,
    pub name: String,
    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    // pub timestamp: f64,
    pub coords: Coordinates,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    // pub accuracy: f64,
    // pub altitude: Option<f64>,
    // pub altitude_accuracy: Option<f64>,
    // pub heading: Option<f64>,
    // pub speed: Option<f64>,
}

fn coords_from_prefs(prefs: &PrefsState) -> (f64, f64) {
    const DEF_LAT: f64 = 36.97412;
    const DEF_LON: f64 = -122.0308;

    match prefs.last_known_location.as_ref() {
        Some(location) => (location.coords.latitude, location.coords.longitude),
        None => (DEF_LAT, DEF_LON),
    }
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

async fn set_memori_state(
    state: &State<'_, AppState>,
    memori_state: MemoriState,
) -> Result<(), String> {
    println!("set_memori_state payload: {:?}", &memori_state);

    let mut conn_guard = state.conn.lock().await;

    match &mut *conn_guard {
        DeviceConnection::RealDevice(ble_transport) => ble_transport
            .set_state(memori_state)
            .await
            .map_err(|err| format!("Failed to set state: {err}")),
        DeviceConnection::Simulator(sim_transport) => sim_transport
            .set_state(memori_state)
            .await
            .map_err(|err| format!("Failed to set state: {err}")),
        DeviceConnection::Disconnected => Err("Device is not connected".to_string()),
    }
}

pub fn read_store_state<T>(app: &AppHandle, store_id: &str) -> T
where
    T: DeserializeOwned + Default,
{
    app.svelte().state_or_default(store_id).unwrap_or_default()
}

async fn resolve_weather_text(
    prefs: &PrefsState,
) -> (String, String, String, String, String, String, String) {
    let (lat, lon) = coords_from_prefs(prefs);

    fetch_weather_temp(lat, lon).await.unwrap()
    /*
    .map(|temp_celsius| format!("{temp_celsius:.1}"))
    .unwrap_or_else(|_| DEFAULT_WEATHER_TEXT.to_string())
    */
}

fn never_widget(id: u32, kind: WidgetKind) -> MemoriWidget {
    MemoriWidget::new(
        WidgetId(id),
        kind,
        UpdateFrequency::Never,
        UpdateFrequency::Never,
    )
}

fn second_widget(id: u32, kind: WidgetKind, seconds: u32) -> MemoriWidget {
    MemoriWidget::new(
        WidgetId(id),
        kind,
        UpdateFrequency::Seconds(seconds),
        UpdateFrequency::Never,
    )
}

#[tauri::command]
#[specta::specta]
pub async fn flash_memori_state(
    state: State<'_, AppState>,
    memori_state: MemoriStateInput,
) -> Result<(), String> {
    set_memori_state(&state, memori_state.into_memori_state()?).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_widget_kinds(app: AppHandle) -> Result<[MemoriWidget; 6], String> {
    let prefs: PrefsState = read_store_state(&app, "prefs");
    // let auth: AuthState = read_store_state(&app, "auth");
    let weather = resolve_weather_text(&prefs).await;
    let (bus_stop, stop_predictions) = resolve_bus_data(&prefs).await;
    let github = refresh_github_widget(&app).await.unwrap_or_default();
    println!("github widget: {:?}\n\n\n\n\n\n\n", github);
    // let twitch_user = fallback_twitch_user(&auth);
    let live = refresh_twitch_widget(&app).await.unwrap();
    println!("twitch widget: {:?}\n\n\n\n\n\n\n", live);
    let name = prefs.name;

    Ok([
        never_widget(0, WidgetKind::Name(Name::new(name))),
        never_widget(1, WidgetKind::Clock(Clock::new(1, 0, 0))),
        never_widget(2, WidgetKind::Bus(Bus::new(bus_stop, stop_predictions))),
        never_widget(
            3,
            WidgetKind::Weather(Weather::new(
                weather.0, weather.1, weather.2, weather.3, weather.4, weather.5, weather.6,
            )),
        ),
        never_widget(4, WidgetKind::Github(github)),
        second_widget(5, WidgetKind::Twitch(live), 5),
    ])
}
