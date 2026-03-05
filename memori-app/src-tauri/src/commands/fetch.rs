use crate::oauth::cloudflare;
use memori_ui::widgets::Github;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;
use std::time::Duration;

const GITHUB_TIMEOUT_SECS: u64 = 3;
const DEFAULT_GITHUB_REPO: &str = "Memori";

#[derive(Debug, Deserialize)]
struct TwitchBroadcaster {
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct TwitchResponse {
    data: Vec<TwitchBroadcaster>,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
}

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GithubUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct BustimeResponse<T> {
    #[serde(rename = "bustime-response")]
    bustime_response: T,
}

#[derive(Debug, Deserialize)]
struct RoutesPayload {
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    rt: String,
    rtnm: String,
}

#[derive(Debug, Deserialize)]
struct DirectionsPayload {
    directions: Vec<Direction>,
}

#[derive(Debug, Deserialize)]
struct Direction {
    id: String,
}

#[derive(Debug, Deserialize)]
struct StopsPayload {
    stops: Vec<Stop>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Stop {
    pub(crate) stpid: String,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

async fn call_api_json<T>(req_body: serde_json::Value) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let api_res = cloudflare("call_api", req_body)
        .await
        .map_err(|err| format!("cloudflare error: {err}"))?;
    serde_json::from_value(api_res).map_err(|err| err.to_string())
}

pub(crate) async fn fetch_weather_temp(lat: f64, lon: f64) -> Result<f32, String> {
    let req_body = json!({
        "provider": "weather",
        "url": format!(
            "https://api.openweathermap.org/data/2.5/weather?appid={{}}&lat={lat}&lon={lon}&units=metric"
        ),
        "lat": lat.to_string(),
        "lon": lon.to_string(),
    });
    let weather_res: WeatherResponse = call_api_json(req_body).await?;
    Ok(weather_res.main.temp)
}

pub(crate) async fn fetch_twitch_display_name(token: &str) -> Result<String, String> {
    let client_id = twitch_client_id()?;
    let mut headers = serde_json::Map::new();
    headers.insert(
        "Authorization".to_string(),
        serde_json::Value::String(format!("Bearer {token}")),
    );
    headers.insert(
        "Client-ID".to_string(),
        serde_json::Value::String(client_id),
    );

    let req_body = json!({
        "provider": "twitch",
        "url": "https://api.twitch.tv/helix/users",
        "headers": serde_json::Value::Object(headers),
    });
    let twitch_res: TwitchResponse = call_api_json(req_body).await?;
    let broadcaster = twitch_res
        .data
        .first()
        .ok_or("Twitch response contained no user".to_string())?;
    Ok(broadcaster.display_name.clone())
}

pub(crate) async fn fetch_github_widget(token: &str) -> Result<Github, String> {
    let url = "https://api.github.com/user";
    let client = Client::builder()
        .timeout(Duration::from_secs(GITHUB_TIMEOUT_SECS))
        .build()
        .map_err(|err| err.to_string())?;
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "tauri-app")
        .send()
        .await
        .map_err(|err| err.to_string())?
        .error_for_status()
        .map_err(|err| err.to_string())?;
    let user = response
        .json::<GithubUser>()
        .await
        .map_err(|err| err.to_string())?;

    Ok(Github::new(user.login, DEFAULT_GITHUB_REPO.to_string()))
}

fn twitch_client_id() -> Result<String, String> {
    std::env::var("TWITCH_CLIENT_ID")
        .ok()
        .or_else(|| option_env!("TWITCH_CLIENT_ID").map(ToString::to_string))
        .ok_or("TWITCH_CLIENT_ID is not configured".to_string())
}

async fn fetch_ucsc_routes() -> Result<Vec<Route>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": "https://rt.scmetro.org/bustime/api/v3/getroutes?key={}&format=json",
    });
    let routes_res: BustimeResponse<RoutesPayload> = call_api_json(req_body).await?;

    Ok(routes_res
        .bustime_response
        .routes
        .into_iter()
        .filter(|route| route.rtnm.contains("UCSC"))
        .collect())
}

async fn fetch_route_directions(route_id: &str) -> Result<Vec<Direction>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!(
            "https://rt.scmetro.org/bustime/api/v3/getdirections?key={{}}&rt={route_id}&format=json"
        ),
    });
    let directions_res: BustimeResponse<DirectionsPayload> = call_api_json(req_body).await?;
    Ok(directions_res.bustime_response.directions)
}

async fn fetch_route_stops(route_id: &str, direction_id: &str) -> Result<Vec<Stop>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!(
            "https://rt.scmetro.org/bustime/api/v3/getstops?key={{}}&rt={route_id}&dir={direction_id}&format=json"
        ),
    });
    let stops_res: BustimeResponse<StopsPayload> = call_api_json(req_body).await?;
    Ok(stops_res.bustime_response.stops)
}

async fn fetch_all_route_stops(ucsc_routes: &[Route]) -> Result<Vec<Stop>, String> {
    let mut all_stops = Vec::new();

    for route in ucsc_routes {
        let directions = fetch_route_directions(&route.rt).await?;
        for dir in directions {
            let route_stops = fetch_route_stops(&route.rt, &dir.id).await?;
            all_stops.extend(route_stops);
        }
    }

    Ok(all_stops)
}

pub(crate) async fn fetch_all_ucsc_stops() -> Result<Vec<Stop>, String> {
    let ucsc_routes = fetch_ucsc_routes().await?;
    fetch_all_route_stops(&ucsc_routes).await
}
