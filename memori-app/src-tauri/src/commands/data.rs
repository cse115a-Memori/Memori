use crate::oauth::cloudflare;
use crate::state::{AppState, DeviceConnection};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{
        Bus, Clock, MemoriWidget, Name, Twitch, UpdateFrequency, Weather, WidgetId, WidgetKind,
    },
    MemoriState,
};
use crate::widget_data::github_data::*;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;
use tauri::{State, AppHandle};
use transport::HostTransport as _;
use tauri_plugin_store::StoreExt;

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

async fn set_memori_state(
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

async fn call_api_json<T>(args: serde_json::Value) -> Result<T, String>
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
pub async fn send_github(_state: State<'_, AppState>, token: String) -> Result<String, String> {
    let url = "https://api.github.com/user";
    let client = Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "tauri-app")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let response = response.error_for_status().map_err(|e| e.to_string())?;
    let user_info: serde_json::Value = response.json().await.map_err(|err| err.to_string())?;
    let _ = user_info;
    Ok("ok".to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn send_twitch(state: State<'_, AppState>, token: String) -> Result<(), String> {
    #[derive(Debug, Deserialize)]
    struct Broadcaster {
        broadcaster_type: String,
        created_at: String,
        description: String,
        display_name: String,
        email: Option<String>,
        id: String,
        login: String,
        view_count: u64,
    }
    #[derive(Debug, Deserialize)]
    struct TwitchResponse {
        data: Vec<Broadcaster>,
    }
    let mut headers = serde_json::Map::new();
    let client_id = std::env::var("TWITCH_CLIENT_ID")
        .ok()
        .or_else(|| option_env!("TWITCH_CLIENT_ID").map(ToString::to_string))
        .ok_or("TWITCH_CLIENT_ID is not configured".to_string())?;
    headers.insert(
        "Authorization".to_string(),
        serde_json::Value::String(format!("Bearer {}", token)),
    );
    headers.insert(
        "Client-ID".to_string(),
        serde_json::Value::String(client_id),
    );
    let args = json!({
        "provider": "twitch",
        "url": "https://api.twitch.tv/helix/users",
       "headers": serde_json::Value::Object(headers),
    });

    let api_response: TwitchResponse = call_api_json(args).await?;
    let broadcaster = match api_response.data.get(0) {
        Some(first_element) => first_element,
        None => return Err("Twitch response contained no user".to_string()),
    };
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Twitch(Twitch::new(broadcaster.display_name.clone())),
            UpdateFrequency::Seconds(1),
            UpdateFrequency::Seconds(1),
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await
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
            UpdateFrequency::Never,
            UpdateFrequency::Never,
        ),
        MemoriWidget::new(
            WidgetId(3),
            WidgetKind::Weather(Weather::new("20.0")),
            UpdateFrequency::Never,
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

#[tauri::command]
#[specta::specta]
pub async fn send_temp(state: State<'_, AppState>, lat: f64, lon: f64) -> Result<String, String> {
    #[derive(Deserialize, Debug)]
    struct WeatherResponse {
        main: Main,
    }

    #[derive(Deserialize, Debug)]
    struct Main {
        temp: f32,
    }

    let request_body = json!({
        "provider": "weather",
        "url": format!(
            "https://api.openweathermap.org/data/2.5/weather?appid={{}}&lat={lat}&lon={lon}&units=metric"
        ),
        "lat": lat.to_string(),//lat.to_string().as_str(),
        "lon": lon.to_string(),// lon.to_string().as_str(),
    });
    let response: WeatherResponse = call_api_json(request_body).await?;
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Weather(Weather::new(response.main.temp.to_string())),
            UpdateFrequency::Seconds(60),
            UpdateFrequency::Seconds(60),
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await?;
    Ok(format!("{:?}", response.main.temp))
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
    #[derive(Debug, Deserialize)]
    struct BustimeResponse<T> {
        #[serde(rename = "bustime-response")]
        bustime_response: T,
    }

    #[derive(Debug, Deserialize)]
    struct Routes {
        routes: Vec<Route>,
    }

    #[derive(Debug, Deserialize)]
    struct Route {
        rt: String,
        rtnm: String,
    }

    #[derive(Debug, Deserialize)]
    struct Directions {
        directions: Vec<Direction>,
    }

    #[derive(Debug, Deserialize)]
    struct Direction {
        id: String,
    }

    #[derive(Debug, Deserialize)]
    struct Stops {
        stops: Vec<Stop>,
    }

    #[derive(Debug, Deserialize)]
    struct Stop {
        stpid: String,
        lat: f64,
        lon: f64,
    }

    let request_body = json!({
        "provider": "bustime",
        "url": "https://rt.scmetro.org/bustime/api/v3/getroutes?key={}&format=json",
    });
    let response: BustimeResponse<Routes> = call_api_json(request_body).await?;

    let routes: Vec<&Route> = response
        .bustime_response
        .routes
        .iter()
        .filter(|route| route.rtnm.contains("UCSC"))
        .collect();

    let mut stops = Vec::new();
    for route in routes {
        let directions_url = format!(
            "https://rt.scmetro.org/bustime/api/v3/getdirections?key={{}}&rt={}&format=json",
            route.rt
        );
        let args = json!({
            "provider": "bustime",
            "url": directions_url,
        });
        let response: BustimeResponse<Directions> = call_api_json(args).await?;
        for direction in response.bustime_response.directions {
            let stops_url = format!(
                "https://rt.scmetro.org/bustime/api/v3/getstops?key={{}}&rt={}&dir={}&format=json",
                route.rt, direction.id
            );
            let args2 = json!({
                "provider": "bustime",
                "url": stops_url,
            });
            let response: BustimeResponse<Stops> = call_api_json(args2).await?;
            stops.extend(response.bustime_response.stops);
        }
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

    let closest_stop = stops.iter().min_by(|a, b| {
        let a_dist = haversine_km(lat, lon, a.lat, a.lon);
        let b_dist = haversine_km(lat, lon, b.lat, b.lon);
        a_dist
            .partial_cmp(&b_dist)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if let Some(stop) = closest_stop {
        Ok(format!("closest stop: {}", stop.stpid))
    } else {
        Err("No nearby bus stop was found".into())
    }
}
