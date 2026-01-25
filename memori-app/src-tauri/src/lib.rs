use std::{any::Any, sync::Arc};

use memori_tcp::{host::DeviceConnected, DeviceRequest, HostResponse, HostTcpTransport};
use tokio::sync::Mutex;
use transport::HostTransport as _;

#[taurpc::procedures(event_trigger = ApiEventTrigger, export_to = "../src/lib/bindings.ts")]
trait Api {
    // #[taurpc(event)]
    async fn hello(name: String) -> Result<String, String>;
    async fn ping() -> Result<String, String>;
}

#[derive(Clone)]
struct ApiImpl {
    state: MyState,
}

struct State {
    transport: HostTcpTransport<DeviceConnected>,
}

type MyState = Arc<Mutex<State>>;

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn hello(self, name: String) -> Result<String, String> {
        Ok(format!("hi there, {}", name))
    }

    async fn ping(self) -> Result<String, String> {
        let x = &mut *self.state.lock().await;

        let x = &mut x.transport;

        let batt = x.get_battery_level().await.unwrap();

        Ok(batt.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    let transport = HostTcpTransport::new(async |req| match req {
        DeviceRequest::Ping => HostResponse::Pong,

        DeviceRequest::RefreshData(_id) => {
            todo!()
        }
    })
    .connect()
    .await
    .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(taurpc::create_ipc_handler(
            ApiImpl {
                state: Arc::new(Mutex::new(State { transport })),
            }
            .into_handler(),
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
