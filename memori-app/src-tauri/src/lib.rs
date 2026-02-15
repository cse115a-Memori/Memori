use ble_host::HostBLETransport;
use memori_tcp::{
    host::{DeviceConnected},
    DeviceRequest, HostResponse, HostTcpTransport, Sequenced,
};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{MemoriWidget, Name, WidgetId, WidgetKind},
    MemoriState,
};
use specta_typescript::Typescript;
use tauri::State;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use transport::HostTransport as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(specta::Type)]
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
    conn: Mutex<DeviceConnection>
}

impl AppState {
    fn new() -> Self {
        Self {
            conn: Mutex::new(DeviceConnection::Disconnected)
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
async fn connect_device(
    state: State<'_, AppState>,
    mode: DeviceMode,
) -> Result<(), String> {
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
        },
        DeviceConnection::Simulator(transport) => {
            transport.disconnect();
        },
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
#[specta::specta] // hi
async fn send_string(state: State<'_, AppState>, string: String) -> Result<(), String> {
    let mut state_guard = state.conn.lock().await;

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
            UpdateFrequency::Never,
        )],
        vec![MemoriLayout::Fourths {
            top_right: WidgetId(0),
            bottom_left: WidgetId(0),
            bottom_right: WidgetId(0),
            top_left: WidgetId(0),
        }],
        5,
    );
    if let DeviceConnection::Simulator(conn) = &mut *state_guard {
        return conn
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}"));
    }

    Err("Device is not connected on tcp".to_string())
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
        send_string
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
