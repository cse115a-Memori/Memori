use crate::commands::data::{call_api_json, set_memori_state};
use crate::{AppState, MemoriLayout, MemoriWidget};
use memori_ui::widgets::{Bus, UpdateFrequency, WidgetId, WidgetKind};
use memori_ui::MemoriState;
use serde::Deserialize;
use serde_json::json;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn send_bustime() -> Result<Box<MemoriWidget>, String> {
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
    let lat = 0.0;
    let lon = 0.0;
    let _closest_stop = stops.iter().min_by(|a, b| {
        let a_dist = haversine_km(lat, lon, a.lat, a.lon);
        let b_dist = haversine_km(lat, lon, b.lat, b.lon);
        a_dist
            .partial_cmp(&b_dist)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    /*
    if let Some(stop) = closest_stop {
        Ok(format!("closest stop: {}", stop.stpid))
    } else {
        Err("No nearby bus stop was found".into())
    }
    */
    let updated_widget = Box::new(MemoriWidget::new(
        WidgetId(0),
        WidgetKind::Bus(Bus::new("", "")),
        UpdateFrequency::Never,
        UpdateFrequency::Never,
    ));
    Ok(updated_widget)
}

#[tauri::command]
#[specta::specta]
pub async fn init_bus(state: State<'_, AppState>) -> Result<(), String> {
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Bus(Bus::new("", "")),
            UpdateFrequency::Seconds(5),
            UpdateFrequency::Never,
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await
}
