// use crate::commands::call_api_json;
use memori_ui::widgets::Bus;
use serde::Deserialize;
use serde_json::json;
use tauri::State;
use std::env;

use crate::state::AppState;

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
    pub(crate) rtdd: String,
    pub(crate) prdctdn: String,
}

async fn fetch_ucsc_routes() -> Result<Vec<Route>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": "https://rt.scmetro.org/bustime/api/v3/getroutes?key={}&format=json",
    });
    let routes_res: BustimeResponse<Route> = call_api_json(req_body).await?;

    Ok(routes_res
        .bustime_response
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
    let directions_res: BustimeResponse<Directions> = call_api_json(req_body).await?;
    Ok(directions_res.bustime_response.directions)
}

async fn fetch_route_stops(route_id: &str, direction_id: &str) -> Result<Vec<Stop>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!(
            "https://rt.scmetro.org/bustime/api/v3/getstops?key={{}}&rt={route_id}&dir={direction_id}&format=json"
        ),
    });
    let stops_res: BustimeResponse<Stops> = call_api_json(req_body).await?;
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

pub(crate) async fn fetch_predictions(stop: &Stop) -> Result<Vec<Prediction>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!("https://rt.scmetro.org/bustime/api/v3/getpredictions?key={{}}&stpid={}&format=json", stop.stpid),
    });
    let predictions: BustimeResponse<Predictions> = call_api_json(req_body).await?;
    Ok(predictions.bustime_response.prd)
}

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

pub async fn refresh_bus_widget() -> Result<Bus, String> {
    let api_key = "4pCLUzKbqkXLDKmsbP5akEDiv";
    let lat: f64 = 37.000074;
    let lon: f64 = -122.062569;
    let stops = fetch_all_ucsc_stops().await?;
    let stop = match find_closest_stop(&stops, lat, lon) {
        Some(stop) => stop,
        None => return Err("build bustime error".to_string()),
    };
    let predictions_response = fetch_predictions(stop).await?;
    let mut predictions = Vec::new();
    for prediction in predictions_response {
        let parsed: u16 = match prediction.prdctdn.parse::<u16>() {
            Ok(p) => p,
            Err(_) => return Err("parsed overflows u16".to_string()),
        };
        predictions.push((prediction.rt.clone(), prediction.rtdir.clone(), parsed));
    }
    Ok(Bus {
        stop: (stop.stpnm, stop.stpid),
        predictions: predictions,
    })
}
