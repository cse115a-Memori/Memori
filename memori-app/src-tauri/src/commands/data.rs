use super::fetch::{
    fetch_all_ucsc_stops, fetch_github_widget, fetch_twitch_display_name, fetch_weather_temp, Stop,
};
use crate::state::{AppState, DeviceConnection};
use crate::widget_data::github_data::*;
use memori_ui::{
    layout::MemoriLayout,
    widgets::{
        Bus, Clock, Github, MemoriWidget, Name, Twitch, UpdateFrequency, Weather, WidgetId,
        WidgetKind,
    },
    MemoriState,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::time::Duration;
use tauri::{AppHandle, State};
use tauri_plugin_svelte::ManagerExt;
use tokio::time::timeout;
use transport::HostTransport as _;

const DEFAULT_WEATHER_TEXT: &str = "20.0";
const DEFAULT_GITHUB_USERNAME: &str = "CaiNann";
const DEFAULT_GITHUB_REPO: &str = "Memori";
const DEFAULT_TWITCH_USER: &str = "twitch_user";
const DEFAULT_BUS_PREDICTION: &str = "9 min";
const DEFAULT_BUS_ROUTE: &str = "Route 19";
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

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
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

async fn resolve_weather_text(prefs: &PrefsState) -> String {
    let (lat, lon) = coords_from_prefs(prefs);

    fetch_weather_temp(lat, lon)
        .await
        .map(|temp_celsius| format!("{temp_celsius:.1}"))
        .unwrap_or_else(|_| DEFAULT_WEATHER_TEXT.to_string())
}

fn fallback_github_username(auth: &AuthState) -> String {
    auth.users_by_provider
        .github
        .as_ref()
        .and_then(|user| non_empty(&user.name))
        .unwrap_or_else(|| DEFAULT_GITHUB_USERNAME.to_string())
}

fn fallback_twitch_user(auth: &AuthState) -> String {
    auth.users_by_provider
        .twitch
        .as_ref()
        .and_then(|user| non_empty(&user.name))
        .unwrap_or_else(|| DEFAULT_TWITCH_USER.to_string())
}

fn default_bus_data() -> (String, String) {
    (
        DEFAULT_BUS_PREDICTION.to_string(),
        DEFAULT_BUS_ROUTE.to_string(),
    )
}

async fn resolve_bus_data(prefs: &PrefsState) -> (String, String) {
    let location = match prefs.last_known_location.as_ref() {
        Some(location) => location,
        None => return default_bus_data(),
    };

    let lat = location.coords.latitude;
    let lon = location.coords.longitude;

    let all_stops = match timeout(
        Duration::from_secs(BUS_FETCH_TIMEOUT_SECS),
        fetch_all_ucsc_stops(),
    )
    .await
    {
        Ok(Ok(stops)) => stops,
        Err(_) => return default_bus_data(),
        Ok(Err(_)) => return default_bus_data(),
    };

    match find_closest_stop(&all_stops, lat, lon) {
        Some(stop) => {
            let distance_km = haversine_km(lat, lon, stop.lat, stop.lon);
            (
                format!("{distance_km:.1} km"),
                format!("Stop {}", stop.stpid),
            )
        }
        None => default_bus_data(),
    }
}

fn never_widget(id: u32, kind: WidgetKind) -> MemoriWidget {
    MemoriWidget::new(
        WidgetId(id),
        kind,
        UpdateFrequency::Never,
        UpdateFrequency::Never,
    )
}

fn build_twitch_state(display_name: String) -> MemoriState {
    MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Twitch(Twitch::new(display_name)),
            UpdateFrequency::Seconds(1),
            UpdateFrequency::Seconds(1),
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    )
}

fn build_weather_state(temp_celsius: f32) -> MemoriState {
    MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Weather(Weather::new(temp_celsius.to_string())),
            UpdateFrequency::Seconds(60),
            UpdateFrequency::Seconds(60),
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    )
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius_km = 6371.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    earth_radius_km * c
}

fn find_closest_stop(stops: &[Stop], lat: f64, lon: f64) -> Option<&Stop> {
    stops.iter().min_by(|stop_a, stop_b| {
        let dist_a = haversine_km(lat, lon, stop_a.lat, stop_a.lon);
        let dist_b = haversine_km(lat, lon, stop_b.lat, stop_b.lon);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
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
pub async fn send_github(_state: State<'_, AppState>, token: String) -> Result<String, String> {
    let github_widget = fetch_github_widget(&token).await?;
    Ok(github_widget.username)
}

#[tauri::command]
#[specta::specta]
pub async fn send_twitch(state: State<'_, AppState>, token: String) -> Result<(), String> {
    let display_name = fetch_twitch_display_name(&token).await?;
    set_memori_state(&state, build_twitch_state(display_name)).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_widget_kinds(app: AppHandle) -> Result<[MemoriWidget; 6], String> {
    let prefs: PrefsState = read_store_state(&app, "prefs");
    let auth: AuthState = read_store_state(&app, "auth");
    let temp_text = resolve_weather_text(&prefs).await;
    let (bus_prediction, bus_route) = resolve_bus_data(&prefs).await;
    let github = refresh_github_widget(&app).await.unwrap_or_default();
    println!("github widget: {:?}", github);
    let twitch_user = fallback_twitch_user(&auth);
    let name = prefs.name;

    Ok([
        never_widget(0, WidgetKind::Name(Name::new(name))),
        never_widget(1, WidgetKind::Clock(Clock::new(1, 0, 0))),
        never_widget(2, WidgetKind::Bus(Bus::new(bus_prediction, bus_route))),
        never_widget(3, WidgetKind::Weather(Weather::new(temp_text))),
        never_widget(4, WidgetKind::Github(github)),
        never_widget(5, WidgetKind::Twitch(Twitch::new(twitch_user))),
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

#[tauri::command]
#[specta::specta]
pub async fn send_temp(state: State<'_, AppState>, lat: f64, lon: f64) -> Result<String, String> {
    let temp_celsius = fetch_weather_temp(lat, lon).await?;
    set_memori_state(&state, build_weather_state(temp_celsius)).await?;
    Ok(format!("{temp_celsius:?}"))
}

#[tauri::command]
#[specta::specta]
pub async fn test_github(app: AppHandle) -> Result<(), String> {
    let widget = refresh_github_widget(&app).await;
    println!("{:?}", widget);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn send_bustime(
    _state: State<'_, AppState>,
    lat: f64,
    lon: f64,
) -> Result<String, String> {
    let all_stops = fetch_all_ucsc_stops().await?;
    let nearest_stop = find_closest_stop(&all_stops, lat, lon)
        .ok_or("No nearby bus stop was found".to_string())?;

    Ok(format!("closest stop: {}", nearest_stop.stpid))
}
