use memori_ui::widgets::Bus;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::env;

const DEFAULT_BUS_PREDICTION: (&str, &str, u16) = ("19", "Donwtown to Watsonville", 7);
const DEFAULT_BUS_STOP: (&str, &str) = ("High and Front", "1230");
const BUS_FETCH_TIMEOUT_SECS: u64 = 3;

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
pub(crate) struct Stop {
    pub(crate) stpid: String,
    pub(crate) stpnm: String,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

#[derive(Debug, Deserialize)]
struct Predictions {
    prd: Vec<Prediction>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Prediction {
    pub(crate) rt: String,
    pub(crate) rtdir: String,
    pub(crate) prdctdn: String,
}

async fn bus_get(url: &str) -> Result<Value, String> {
    let response = Client::new()
        .get(url)
        .header("User-Agent", "tauri-app")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string());
    match response {
        Ok(data) => Ok(data),
        Err(error) => Err(format!("failed to call sc metro bustime api: {error}")),
    }
}

async fn fetch_ucsc_routes(key: &str) -> Result<Vec<Route>, String> {
    let url = format!("https://rt.scmetro.org/bustime/api/v3/getroutes?key={key}&format=json");
    let routes_res: BustimeResponse<Routes> = match bus_get(&url).await {
        Ok(data) => match serde_json::from_value(data) {
            Ok(response) => response,
            Err(error) => return Err(format!("failed to deserialize bus routes: {error}")),
        },
        Err(error) => return Err(error),
    };
    Ok(routes_res
        .bustime_response
        .routes
        .into_iter()
        .filter(|route| route.rtnm.contains("UCSC"))
        .collect())
}

async fn fetch_route_directions(key: &str, route_id: &str) -> Result<Vec<Direction>, String> {
    let url = format!(
        "https://rt.scmetro.org/bustime/api/v3/getdirections?key={key}&rt={route_id}&format=json"
    );
    let directions_res: BustimeResponse<Directions> = match bus_get(&url).await {
        Ok(data) => match serde_json::from_value(data) {
            Ok(response) => response,
            Err(error) => return Err(format!("failed to deserialize route directions: {error}")),
        },
        Err(error) => return Err(error),
    };
    Ok(directions_res.bustime_response.directions)
}

async fn fetch_route_stops(
    key: &str,
    route_id: &str,
    direction_id: &str,
) -> Result<Vec<Stop>, String> {
    let url = format!(
            "https://rt.scmetro.org/bustime/api/v3/getstops?key={key}&rt={route_id}&dir={direction_id}&format=json"
    );
    let stops_res: BustimeResponse<Stops> = match bus_get(&url).await {
        Ok(data) => match serde_json::from_value(data) {
            Ok(response) => response,
            Err(error) => return Err(format!("failed to deserialize route stops: {error}")),
        },
        Err(error) => return Err(error),
    };
    Ok(stops_res.bustime_response.stops)
}

async fn fetch_all_route_stops(key: &str, ucsc_routes: &[Route]) -> Result<Vec<Stop>, String> {
    let mut all_stops = Vec::new();

    for route in ucsc_routes {
        let directions = fetch_route_directions(key, &route.rt).await?;
        for dir in directions {
            let route_stops = fetch_route_stops(key, &route.rt, &dir.id).await?;
            all_stops.extend(route_stops);
        }
    }

    Ok(all_stops)
}

pub(crate) async fn fetch_all_ucsc_stops(key: &str) -> Result<Vec<Stop>, String> {
    let ucsc_routes = fetch_ucsc_routes(key).await?;
    fetch_all_route_stops(key, &ucsc_routes).await
}

pub(crate) async fn fetch_predictions(key: &str, stop: &Stop) -> Result<Vec<Prediction>, String> {
    let url = format!(
        "https://rt.scmetro.org/bustime/api/v3/getpredictions?key={key}&stpid={}&format=json",
        stop.stpid
    );
    let predictions: BustimeResponse<Predictions> = match bus_get(&url).await {
        Ok(data) => match serde_json::from_value(data) {
            Ok(response) => response,
            Err(error) => return Err(format!("failed to deserialize predictions: {error}")),
        },
        Err(error) => return Err(error),
    };
    Ok(predictions.bustime_response.prd)
}

pub fn find_closest_stop(stops: &[Stop], lat: f64, lon: f64) -> Option<&Stop> {
    stops.iter().min_by(|stop_a, stop_b| {
        let dist_a = haversine_km(lat, lon, stop_a.lat, stop_a.lon);
        let dist_b = haversine_km(lat, lon, stop_b.lat, stop_b.lon);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
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

/*
#[tauri::command]
#[specta::specta]
pub async fn send_bustime(
    _state: State<'_, AppState>,
    lat: f64,
    lon: f64,
) -> Result<String, String> {
    let all_stops = fetch_all_ucsc_stops().await?;
    // let nearest_stop = find_closest_stop(&all_stops, lat, lon)
    //     .ok_or("No nearby bus stop was found".to_string())?;

    // Ok(format!("closest stop: {}", nearest_stop.stpid))
    todo!()
}
*/

pub async fn refresh_bus_widget() -> Result<Bus, String> {
    let key = "BUS_API_KEY";
    let key = match env::var(key) {
        Ok(data) => data,
        Err(error) => return Err(format!("failed to get bus api key: {error}")),
    };
    let lat: f64 = 37.000074;
    let lon: f64 = -122.062569;
    let stops = fetch_all_ucsc_stops(&key).await?;
    let stop = match find_closest_stop(&stops, lat, lon) {
        Some(stop) => stop,
        None => return Err("no stops nearby".to_string()),
    };
    let predictions_response = fetch_predictions(&key, stop).await?;
    let mut predictions = Vec::new();
    for prediction in predictions_response {
        let parsed: u16 = match prediction.prdctdn.parse::<u16>() {
            Ok(p) => p,
            Err(_) => return Err("parsed overflows u16".to_string()),
        };
        predictions.push((prediction.rt.clone(), prediction.rtdir.clone(), parsed));
    }
    let stop = (stop.stpnm.clone(), stop.stpid.clone());
    Ok(Bus { stop, predictions })
}
