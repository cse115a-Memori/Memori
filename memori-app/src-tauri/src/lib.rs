use std::sync::Arc;

use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
    DeviceRequest, HostResponse, HostTcpTransport,
};
use tokio::sync::Mutex;
use transport::HostTransport as _;

#[taurpc::procedures(event_trigger = ApiEventTrigger, export_to = "../src/lib/bindings.ts")]
trait Api {
    // #[taurpc(event)]
    async fn hello(name: String) -> Result<String, String>;
    async fn get_battery() -> Result<u8, String>;
    async fn connect() -> Result<(), String>;
}

#[derive(Clone)]
struct ApiImpl {
    state: MyState,
}

enum Connection {
    Connected(HostTcpTransport<DeviceConnected>),
    Disconnected(HostTcpTransport<DeviceDisconnected>),
}

struct State {
    conn: Connection,
}

type MyState = Arc<Mutex<State>>;

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn hello(self, name: String) -> Result<String, String> {
        Ok(format!("hi there, {}", name))
    }

    async fn connect(self) -> Result<(), String> {
        let mut state_guard = self.state.lock().await;

        if let Connection::Disconnected(ref transport) = state_guard.conn {
            let connected_transport = transport.connect().await.map_err(|e| e.to_string())?;

            state_guard.conn = Connection::Connected(connected_transport);

            return Ok(());
        }

        Err("Already connected or in invalid state".to_string())
    }

    async fn get_battery(self) -> Result<u8, String> {
        let mut state_guard = self.state.lock().await;

        if let Connection::Connected(ref mut conn) = state_guard.conn {
            return conn
                .get_battery_level()
                .await
                .map_err(|e| format!("Failed to get battery: {e}"));
        }

        Err("Device is not connected".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    let conn = Connection::Disconnected(HostTcpTransport::new(async |req| match req {
        DeviceRequest::Ping => HostResponse::Pong,

        DeviceRequest::RefreshData(_id) => {
            todo!()
        }
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(taurpc::create_ipc_handler(
            ApiImpl {
                state: Arc::new(Mutex::new(State { conn })),
            }
            .into_handler(),
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
