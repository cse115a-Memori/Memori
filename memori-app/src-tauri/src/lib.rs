mod oauth;
use crate::oauth::*;
use ble_host::HostBLETransport;
use memori_tcp::{
    host::DeviceConnected, host::DeviceDisconnected, DeviceRequest, HostResponse, HostTcpTransport,
    Sequenced,
};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{Bus, MemoriWidget, Name, UpdateFrequency, Weather, WidgetId, WidgetKind},
    MemoriState,
};
use reqwest::Client;
use serde::Deserialize;
use specta_typescript::Typescript;
use std::{env, fmt::format};
use tauri::{AppHandle, State};
use tauri_plugin_svelte::ManagerExt;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use transport::HostTransport as _;

#[derive(Debug)]
enum TCPConnection {
    Connected(HostTcpTransport<DeviceConnected>),
    Disconnected(HostTcpTransport<DeviceDisconnected>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum DeviceMode {
    RealDevice,
    Simulator,
}

enum DeviceConnection {
    RealDevice(HostBLETransport),
    Simulator(HostTcpTransport<DeviceConnected>),
    Disconnected,
}

struct AppState {
    tcp_conn: Mutex<TCPConnection>,
    // ble_conn: Mutex<BLEConnection>,
    conn: Mutex<DeviceConnection>,
}

impl AppState {
    fn new() -> Self {
        Self {
            tcp_conn: Mutex::new(TCPConnection::Disconnected(HostTcpTransport::default())),
            // ble_conn: Mutex::new(BLEConnection::Disconnected(HostBLETransport::default())),
            conn: Mutex::new(DeviceConnection::Disconnected),
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
async fn connect_device(state: State<'_, AppState>, mode: DeviceMode) -> Result<(), String> {
    let mut guard = state.conn.lock().await;

    if !matches!(*guard, DeviceConnection::Disconnected) {
        return Err("Already connected. Disconnect first.".to_string());
    }

    match mode {
        DeviceMode::RealDevice => {
            let conn = HostBLETransport::connect()
                .await
                .map_err(|e| format!("Failed to connect to device: {e}"))?;

            *guard = DeviceConnection::RealDevice(conn);
            println!("Connected to real device over Bluetooth");
            Ok(())
        }
        DeviceMode::Simulator => {
            let transport = HostTcpTransport::default();
            let (conn, (dev_req_rx, host_resp_tx)) = transport
                .connect()
                .await
                .map_err(|e| format!("Failed to connect to simulator: {e}"))?;

            *guard = DeviceConnection::Simulator(conn);

            tokio::spawn(async move {
                request_handler(dev_req_rx, host_resp_tx).await;
            });

            println!("Connected to simulator over TCP");
            Ok(())
        }
    }
}

#[tauri::command]
#[specta::specta]
async fn disconnect_device(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.conn.lock().await;

    let old_connection = std::mem::replace(&mut *guard, DeviceConnection::Disconnected);

    match old_connection {
        DeviceConnection::RealDevice(transport) => {
            transport.disconnect().await;
        }
        DeviceConnection::Simulator(transport) => {
            transport.disconnect();
        }
        DeviceConnection::Disconnected => {}
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn get_device_mode(state: State<'_, AppState>) -> Result<Option<DeviceMode>, String> {
    let guard = state.conn.lock().await;
    Ok(match *guard {
        DeviceConnection::RealDevice(_) => Some(DeviceMode::RealDevice),
        DeviceConnection::Simulator(_) => Some(DeviceMode::Simulator),
        DeviceConnection::Disconnected => None,
    })
}

#[tauri::command]
#[specta::specta]
async fn is_connected(state: State<'_, AppState>) -> Result<bool, String> {
    let guard = state.conn.lock().await;
    Ok(!matches!(*guard, DeviceConnection::Disconnected))
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
async fn get_battery(state: State<'_, AppState>) -> Result<u8, String> {
    let mut guard = state.conn.lock().await;

    match &mut *guard {
        DeviceConnection::RealDevice(transport) => transport
            .get_battery_level()
            .await
            .map_err(|e| format!("Failed to get battery: {e}")),
        DeviceConnection::Simulator(transport) => transport
            .get_battery_level()
            .await
            .map_err(|e| format!("Failed to get battery: {e}")),
        DeviceConnection::Disconnected => Err("Device is not connected".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
async fn send_twitch(_state: State<'_, AppState>, token: String) -> Result<String, String> {
    println!("token: {}", token);
    Ok(format!("access token: {}", token))
}

#[tauri::command]
#[specta::specta] // hi
async fn send_name(state: State<'_, AppState>, name: String) -> Result<(), String> {
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
    let api_key = env!("API_KEY_W");
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
        // if let DeviceConnection::Simulator(conn) = &mut *state_guard {

        return conn
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}"));
    }
    Err("Device is not connected".to_string())
}

#[tauri::command]
#[specta::specta]
async fn send_bustime(state: State<'_, AppState>, lat: f64, lon: f64) -> Result<String, String> {
    // Ok(format!("location: {}, {}", lat, lon))

    let mut state_guard = state.tcp_conn.lock().await;
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
        // stpnm: String,
        lat: f64,
        lon: f64,
    }
    #[derive(Debug, Deserialize)]
    struct Predictions {
        prd: Vec<Prediction>,
    }
    #[derive(Debug, Deserialize)]
    struct Prediction {
        rt: String,
        prdctdn: String,
    }
    let api_key = env!("API_KEY");
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
    //let (lat, lon): (f64, f64) = match location.as_str() {
    //     "1" => (36.999934, -122.062213),
    //    "2" => (36.977296, -122.053627),
    //     "3" => (36.974099, -122.024405),
    //    _ => return Err("Invalid location".into()),
    // };
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
    } else {
        return Err("1111".into());
    }
    Ok(format!("closest stop: {}", closest_stop_id))
    /*
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
    let closest_bus: String;
    let busstop_name: String;
    if let Some(first_prediction) = response.bustime_response.prd.first() {
        closest_bus = first_prediction.prdctdn.clone();
        busstop_name = first_prediction.rt.clone();
    } else {
        return Err("prediction err".into());
    }

    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Bus(Bus::new(closest_bus, busstop_name)), //response.bustime_response.prd)),
            Some(UpdateFrequency::Seconds(60)),
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
    // Err("Device is not connected on tcp".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        hello,
        connect_device,
        disconnect_device,
        get_device_mode,
        is_connected,
        get_battery,
        send_twitch,
        send_name,
        send_temp,
        send_bustime,
        start_oauth_server,
        login_with_provider
    ]);
    #[cfg(all(debug_assertions, not(any(target_os = "ios", target_os = "android"))))]
    builder
        .export(Typescript::default(), "../src/lib/tauri/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .manage(AppState::new())
        // .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_svelte::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_oauth::init())
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
