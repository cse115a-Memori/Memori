use ble_host::HostBLETransport;
use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
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
    if let TCPConnection::Connected(conn) = &mut *state_guard {
        return conn
            .set_state(memori_state)
            .await
            .map_err(|e| format!("Failed to set state: {e}"));
    }

    Err("Device is not connected".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        hello,
        tcp_connect,
        ble_connect,
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
