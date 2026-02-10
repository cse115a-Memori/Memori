use ble_host::HostBLETransport;
use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
    DeviceRequest, HostResponse, HostTcpTransport, Sequenced,
};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{MemoriWidget, Name, UpdateFrequency, Weather, WidgetId, WidgetKind},
    MemoriState,
};
use reqwest::Client;
use serde::Deserialize;
use specta_typescript::Typescript;
use std::{env, fmt::format};
use tauri::State;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use transport::HostTransport as _;

enum TCPConnection {
    Connected(HostTcpTransport<DeviceConnected>),
    Disconnected(HostTcpTransport<DeviceDisconnected>),
}

enum BLEConnection {
    Connected(HostBLETransport),
    Disconnected(HostBLETransport),
}

struct AppState {
    tcp_conn: Mutex<TCPConnection>,
    // ble_conn: Mutex<BLEConnection>,
}

impl AppState {
    fn new() -> Self {
        Self {
            tcp_conn: Mutex::new(TCPConnection::Disconnected(HostTcpTransport::default())),
            // ble_conn: Mutex::new(BLEConnection::Disconnected(HostBLETransport::default())),
        }
    }
}

#[tauri::command]
#[specta::specta]
async fn hello(name: String) -> Result<String, String> {
    Ok(format!("hi there, {}", name))
}

#[tauri::command]
#[specta::specta]
async fn tcp_connect(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.tcp_conn.lock().await;

    if let TCPConnection::Disconnected(transport) = &*guard {
        let (conn, (dev_req_rx, host_resp_tx)) =
            transport.connect().await.map_err(|e| e.to_string())?;
        *guard = TCPConnection::Connected(conn);

        tokio::spawn(async move {
            request_handler(dev_req_rx, host_resp_tx).await;
        });

        return Ok(());
    }

    Err("Already connected or in invalid state".to_string())
}

#[tauri::command]
#[specta::specta]
async fn ble_connect(_state: State<'_, AppState>) -> Result<(), String> {
    Err("BLE connect is not implemented yet".to_string())
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
async fn get_battery(state: State<'_, AppState>) -> Result<u8, String> {
    let mut guard = state.tcp_conn.lock().await;

    match &mut *guard {
        TCPConnection::Connected(conn) => conn
            .get_battery_level()
            .await
            .map_err(|e| format!("Failed to get battery: {e}")),
        TCPConnection::Disconnected(_) => Err("Device is not connected".to_string()),
    }
}

#[tauri::command]
#[specta::specta] // hi
async fn send_string(state: State<'_, AppState>, string: String) -> Result<(), String> {
    let mut state_guard = state.tcp_conn.lock().await;

    // let memori_state = MemoriState::new(
    //     0,
    //     vec![MemoriWidget::new(
    //         WidgetId(0),
    //         WidgetKind::Name(Name::new(string)),
    //     )],
    //     vec![MemoriLayout::Full(WidgetId(0))],
    //     5,
    // );

    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Name(Name::new(string)),
            Some(UpdateFrequency::Seconds(1)),
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
async fn send_temp(state: State<'_, AppState>, city: String) -> Result<(), String> {
    let mut state_guard = state.tcp_conn.lock().await;
    #[derive(Deserialize, Debug)]
    struct WeatherResponse {
        main: Main,
        weather: Vec<WeatherDetail>,
    }
    #[derive(Deserialize, Debug)]
    struct WeatherDetail {
        main: String,
    }
    #[derive(Deserialize, Debug)]
    struct Main {
        temp: f32,
    }
    // add to .bashrc -> $ export ~/.bashrc
    // export API_KEY='api key goes here'
    let api_key = env::var("API_KEY_W").map_err(|e| format!("API_KEY not set: {e}"))?;
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
            Some(UpdateFrequency::Seconds(1)),
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
async fn send_bustime(location: String) -> Result<String, String> {
    // let mut state_guard = state.tcp_conn.lock().await;
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
        stpnm: String,
        lat: f64,
        lon: f64,
    }
    #[derive(Debug, Deserialize)]
    struct Predictions {
        prd: Prediction,
    }
    #[derive(Debug, Deserialize)]
    struct Prediction {
        prdctdn: String,
    }
    let api_key = env::var("API_KEY").map_err(|e| format!("API_KEY not set: {e}"))?;
    let client = Client::new();
    let url = format!(
        "https://rt.scmetro.org/bustime/api/v3/getroutes?key={}&format=json",
        api_key
    );
    let response: BustimeResponse<Routes> = client
        .get(&url)
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
        let url = format!(
            "https://rt.scmetro.org/bustime/api/v3/getdirections?key={}&rt={}&format=json",
            api_key, route.rt
        );
        let response: BustimeResponse<Directions> = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("request err: {e}"))?
            .json()
            .await
            .map_err(|e| format!("deserialize err: {e}"))?;
        for direction in response.bustime_response.directions {
            let url = format!(
                "https://rt.scmetro.org/bustime/api/v3/getstops?key={}&rt={}&dir={}&format=json",
                api_key, route.rt, direction.id
            );
            let response: BustimeResponse<Stops> = client
                .get(&url)
                .send()
                .await
                .map_err(|e| format!("request err: {e}"))?
                .json()
                .await
                .map_err(|e| format!("deserialize err: {e}"))?;

            stops.extend(response.bustime_response.stops);
        }
    }
    let (lat, lon): (f64, f64) = match location.as_str() {
        "1" => (36.999934, -122.062213),
        "2" => (36.977296, -122.053627),
        "3" => (36.974099, -122.024405),
        _ => return Err("Invalid location".into()),
    };
    let mut closest_stop: Option<&Stop> = None;
    let mut min_distance = f64::MAX;
    fn hsine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371.0;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        r * c
    }
    for stop in &stops {
        let distance = hsine(lat, lon, stop.lat, stop.lon);
        if distance < min_distance {
            min_distance = distance;
            closest_stop = Some(stop);
        }
    }
    let closest_stop_id: String;
    if let Some(stop) = closest_stop {
        closest_stop_id = stop.stpid.clone();
        println!("closest_stop: {}", closest_stop_id);
    } else {
        return Err("1111".into());
    }
    let url = format!(
        "https://rt.scmetro.org/bustime/api/v3/getpredictions?key={}&stpid={}&format=json",
        api_key, closest_stop_id
    );
    let response: BustimeResponse<Predictions> = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("request err: {e}"))?
        .json()
        .await
        .map_err(|e| format!("deserialize err: {e}"))?;
    Ok(format!(
        "num min until arrival: {}",
        response.bustime_response.prd.prdctdn
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        hello,
        tcp_connect,
        ble_connect,
        get_battery,
        send_string,
        send_temp,
        send_bustime
    ]);
    // hi
    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/lib/tauri/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .manage(AppState::new())
        // .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_svelte::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub async fn request_handler(
    mut dev_req_rx: UnboundedReceiver<Sequenced<DeviceRequest>>,
    host_resp_tx: UnboundedSender<Sequenced<HostResponse>>,
) {
    while let Some(req) = dev_req_rx.recv().await {
        println!("received request from device! {req:#?}");

        let resp = match req.msg_kind {
            DeviceRequest::RefreshData(_id) => {
                todo!()
            }
            DeviceRequest::Ping => HostResponse::Pong,
        };

        host_resp_tx
            .send(Sequenced::new(req.seq_num, resp))
            .unwrap();
    }
}
