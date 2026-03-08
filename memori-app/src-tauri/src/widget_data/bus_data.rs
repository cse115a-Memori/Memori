use crate::widget_data::weather_data::call_api_json;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct BustimeResponse<T> {
    #[serde(rename = "bustime-response")]
    bustime_response: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct Route {
    rt: String,
    rtnm: String,
}

#[derive(Debug, Deserialize)]
struct Direction {
    id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Stop {
    pub(crate) stpid: String,
    pub(crate) stpnm: String,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Prediction {
    pub(crate) rt: String,
    pub(crate) rtdd: String,
    pub(crate) prdctdn: String,
}

async fn fetch_route_directions(route_id: &str) -> Result<Vec<Direction>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!(
            "https://rt.scmetro.org/bustime/api/v3/getdirections?key={{}}&rt={route_id}&format=json"
        ),
    });
    let directions_res: BustimeResponse<Direction> = call_api_json(req_body).await?;
    Ok(directions_res.bustime_response)
}

async fn fetch_route_stops(route_id: &str, direction_id: &str) -> Result<Vec<Stop>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!(
            "https://rt.scmetro.org/bustime/api/v3/getstops?key={{}}&rt={route_id}&dir={direction_id}&format=json"
        ),
    });
    let stops_res: BustimeResponse<Stop> = call_api_json(req_body).await?;
    Ok(stops_res.bustime_response)
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

pub(crate) async fn fetch_predictions(stop: &Stop) -> Result<Vec<Prediction>, String> {
    let req_body = json!({
        "provider": "bustime",
        "url": format!("https://rt.scmetro.org/bustime/api/v3/getpredictions?key={{}}&stpid={}&format=json", stop.stpid),
    });
    let predictions: BustimeResponse<Prediction> = call_api_json(req_body).await?;
    Ok(predictions.bustime_response)
}

pub(crate) async fn fetch_all_ucsc_stops() -> Result<Vec<Stop>, String> {
    let ucsc_routes = fetch_ucsc_routes().await?;
    fetch_all_route_stops(&ucsc_routes).await
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
