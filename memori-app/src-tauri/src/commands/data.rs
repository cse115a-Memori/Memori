use crate::state::{AppState, DeviceConnection, TCPConnection};
use memori_ui::{
    MemoriState, layout::MemoriLayout, widgets::{Bus, Clock, MemoriWidget, Name, UpdateFrequency, Weather, WidgetId, WidgetKind}
};
use reqwest::Client;
use serde::Deserialize;
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
pub async fn hello(name: String) -> Result<String, String> {
    Ok(format!("hi there, {}", name))
}

#[tauri::command]
#[specta::specta]
pub async fn send_twitch(_state: State<'_, AppState>, token: String) -> Result<String, String> {
    println!("token: {}", token);
    Ok(format!("access token: {}", token))
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
pub async fn set_memori_state(
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

    // Ok("flash success".to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn send_name(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let mut state_guard = state.tcp_conn.lock().await;

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

    if let TCPConnection::Connected(conn) = &mut *state_guard {
        return conn
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}"));
    }

    Err("Device is not connected".to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn send_temp(state: State<'_, AppState>, city: String) -> Result<(), String> {
    let mut state_guard = state.tcp_conn.lock().await;

    #[derive(Deserialize, Debug)]
    struct WeatherResponse {
        main: Main,
    }

    #[derive(Deserialize, Debug)]
    struct Main {
        temp: f32,
    }

    let api_key = match std::env::var("API_KEY_W")
        .ok()
        .or_else(|| option_env!("API_KEY_W").map(ToString::to_string))
    {
        Some(value) => value,
        None => return Ok(()),
    };

    println!("city: {}", city);
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let client = Client::new();
    let response: WeatherResponse = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("request err: {e}"))?
        .json()
        .await
        .map_err(|e| format!("deserialize err: {e}"))?;

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

    if let TCPConnection::Connected(conn) = &mut *state_guard {
        return conn
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}"));
    }

    Err("Device is not connected".to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn send_bustime(
    state: State<'_, AppState>,
    lat: f64,
    lon: f64,
) -> Result<String, String> {
    let _state_guard = state.tcp_conn.lock().await;

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

    let api_key = match std::env::var("API_KEY")
        .ok()
        .or_else(|| option_env!("API_KEY").map(ToString::to_string))
    {
        Some(value) => value,
        None => return Ok("Bus API key not configured".to_string()),
    };

    let client = Client::new();
    let routes_url = format!(
        "https://rt.scmetro.org/bustime/api/v3/getroutes?key={}&format=json",
        api_key
    );

    let response: BustimeResponse<Routes> = client
        .get(&routes_url)
        .send()
        .await
        .map_err(|e| format!("request err: {e}"))?
        .json()
        .await
        .map_err(|e| format!("deserialize err: {e}"))?;

    let routes: Vec<&Route> = response
        .bustime_response
        .routes
        .iter()
        .filter(|route| route.rtnm.contains("UCSC"))
        .collect();

    let mut stops = Vec::new();
    for route in routes {
        let directions_url = format!(
            "https://rt.scmetro.org/bustime/api/v3/getdirections?key={}&rt={}&format=json",
            api_key, route.rt
        );

        let response: BustimeResponse<Directions> = client
            .get(&directions_url)
            .send()
            .await
            .map_err(|e| format!("request err: {e}"))?
            .json()
            .await
            .map_err(|e| format!("deserialize err: {e}"))?;

        for direction in response.bustime_response.directions {
            let stops_url = format!(
                "https://rt.scmetro.org/bustime/api/v3/getstops?key={}&rt={}&dir={}&format=json",
                api_key, route.rt, direction.id
            );

            let response: BustimeResponse<Stops> = client
                .get(&stops_url)
                .send()
                .await
                .map_err(|e| format!("request err: {e}"))?
                .json()
                .await
                .map_err(|e| format!("deserialize err: {e}"))?;

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
        Err("1111".into())
    }
}
