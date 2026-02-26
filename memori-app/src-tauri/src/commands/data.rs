use crate::oauth::cloudflare;
use crate::state::{AppState, TCPConnection};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{Clock, MemoriWidget, Name, Twitch, UpdateFrequency, Weather, WidgetId, WidgetKind},
    MemoriState,
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tauri::http::response;
use tauri::utils::resources;
use tauri::State;
use transport::HostTransport as _;

#[tauri::command]
#[specta::specta]
pub async fn hello(name: String) -> Result<String, String> {
    Ok(format!("hi there, {}", name))
}

#[tauri::command]
#[specta::specta]
pub async fn send_github(_state: State<'_, AppState>, token: String) -> Result<String, String> {
    // let mut state_guard = state.tcp_conn.lock().await;
    #[derive(Deserialize)]
    struct User {
        id: u32,
        login: String,
    }
    println!("{}", token);
    let url = "https://api.github.com/user";
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|e| e.to_string())?,
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github+json"),
    );
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        println!("Response Body: {}", body);
        let user: User = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        println!("User ID: {}, Login: {}", user.id, user.login);
    } else {
        println!("Error: {}", response.status());
        return Err(format!("Error: {}", response.status()));
    }
    Ok("ok".to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn send_twitch(state: State<'_, AppState>, _token: String) -> Result<(), String> {
    let mut state_guard = state.tcp_conn.lock().await;
    #[derive(Debug, Deserialize)]
    struct Broadcaster {
        broadcaster_type: String,
        created_at: String,
        description: String,
        display_name: String,
        email: String,
        id: String,
        login: String,
        view_count: u64,
    }
    #[derive(Debug, Deserialize)]
    struct TwitchResponse {
        data: Vec<Broadcaster>,
    }
    let args = json!({
        "provider": "twitch",
        "url": "https://api.twitch.tv/helix/users",
        "headers": json!({}),
    });
    let twitch_response = match cloudflare("call_api", args).await {
        Ok(data) => data,
        Err(_) => return Err("cloudflare error".to_string()),
    };
    let api_response: TwitchResponse =
        serde_json::from_value(twitch_response).map_err(|e| e.to_string())?;
    let broadcaster = match api_response.data.get(0) {
        Some(first_element) => first_element,
        None => return Err("err".to_string()),
    };
    println!("{:?}", broadcaster.id);
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
pub async fn get_widget_kinds() -> Result<[MemoriWidget; 2], String> {
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
    ])
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
pub async fn send_temp(state: State<'_, AppState>, lat: f64, lon: f64) -> Result<String, String> {
    // let mut state_guard = state.tcp_conn.lock().await;

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
        "url": "https://api.openweathermap.org/data/2.5/weather?appid={}&lat={lat}&lon={lon}&units=metric",
        "lat": lat.to_string(),//lat.to_string().as_str(),
        "lon": lon.to_string(),// lon.to_string().as_str(),
    });
    let response_data = match cloudflare("call_api", request_body).await {
        Ok(data) => data,
        Err(_) => return Err("err".to_string()),
    };
    let response: WeatherResponse =
        serde_json::from_value(response_data).map_err(|e| e.to_string())?;
    Ok(format!("{:?}", response.main.temp))
    /*
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
    */
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

    let request_body = json!({
        "provider": "bustime",
        "url": "https://rt.scmetro.org/bustime/api/v3/getroutes?key={}&format=json",
    });
    let response_data = match cloudflare("call_api", request_body).await {
        Ok(data) => data,
        Err(_) => return Err("err".to_string()),
    };
    let response: BustimeResponse<Routes> =
        serde_json::from_value(response_data).map_err(|e| e.to_string())?;

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
        let response_data = match cloudflare("call_api", args).await {
            Ok(data) => data,
            Err(_) => return Err("err".to_string()),
        };
        let response: BustimeResponse<Directions> =
            serde_json::from_value(response_data).map_err(|e| e.to_string())?;
        for direction in response.bustime_response.directions {
            let stops_url = format!(
                "https://rt.scmetro.org/bustime/api/v3/getstops?key={{}}&rt={}&dir={}&format=json",
                route.rt, direction.id
            );
            let args2 = json!({
                "provider": "bustime",
                "url": stops_url,
            });
            let response_data = match cloudflare("call_api", args2).await {
                Ok(data) => data,
                Err(_) => return Err("err".to_string()),
            };
            let response: BustimeResponse<Stops> =
                serde_json::from_value(response_data).map_err(|e| e.to_string())?;
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
