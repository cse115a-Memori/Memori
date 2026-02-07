use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
    DeviceRequest, HostResponse, HostTcpTransport, Sequenced,
};
use specta_typescript::Typescript;
use tauri::State;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use transport::{HostTransport as _, Widget, WidgetId};

enum Connection {
    Connected(HostTcpTransport<DeviceConnected>),
    Disconnected(HostTcpTransport<DeviceDisconnected>),
}

struct AppState {
    conn: Mutex<Connection>,
}

impl AppState {
    fn new() -> Self {
        Self {
            conn: Mutex::new(Connection::Disconnected(HostTcpTransport::default())),
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
async fn connect(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.conn.lock().await;

    if let Connection::Disconnected(transport) = &*guard {
        let (conn, (dev_req_rx, host_resp_tx)) =
            transport.connect().await.map_err(|e| e.to_string())?;
        *guard = Connection::Connected(conn);

        tokio::spawn(async move {
            request_handler(dev_req_rx, host_resp_tx).await;
        });

        return Ok(());
    }

    Err("Already connected or in invalid state".to_string())
}

#[tauri::command]
#[specta::specta]
async fn get_battery(state: State<'_, AppState>) -> Result<u8, String> {
    let mut guard = state.conn.lock().await;

    match &mut *guard {
        Connection::Connected(conn) => conn
            .get_battery_level()
            .await
            .map_err(|e| format!("Failed to get battery: {e}")),
        Connection::Disconnected(_) => Err("Device is not connected".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
async fn send_string(state: State<'_, AppState>, string: String) -> Result<(), String> {
    let widget = Widget::new(WidgetId(0), string).map_err(|e| e.to_string())?;

    let mut guard = state.conn.lock().await;

    match &mut *guard {
        Connection::Connected(conn) => conn
            .set_widgets(widget)
            .await
            .map_err(|e| format!("Failed to set widget: {e}")),
        Connection::Disconnected(_) => Err("Device is not connected".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        hello,
        connect,
        get_battery,
        send_string
    ]);

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
        .invoke_handler(tauri::generate_handler![
            hello,
            connect,
            get_battery,
            send_string
        ])
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
                // TODO: implement your refresh response
                todo!()
            }
            DeviceRequest::Ping => HostResponse::Pong,
        };

        host_resp_tx
            .send(Sequenced::new(req.seq_num, resp))
            .unwrap();
    }
}
