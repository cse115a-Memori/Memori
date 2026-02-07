use std::sync::Arc;

use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
    DeviceRequest, HostResponse, HostTcpTransport, Sequenced,
};
use memori_ui::{
    layout::MemoriLayout,
    widgets::{MemoriWidget, Name, UpdateFrequency, WidgetId, WidgetKind},
    MemoriState,
};
use tauri_plugin_tracing::{tracing::error, Builder, LevelFilter};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
use transport::HostTransport as _;

#[taurpc::procedures(event_trigger = ApiEventTrigger, export_to = "../src/lib/bindings.ts")]
trait Api {
    // #[taurpc(event)]
    async fn hello(name: String) -> Result<String, String>;
    async fn get_battery() -> Result<u8, String>;
    async fn connect() -> Result<(), String>;
    async fn send_string(string: String) -> Result<(), String>;
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
            let (conn, (dev_req_rx, host_resp_tx)) =
                transport.connect().await.map_err(|e| e.to_string())?;

            state_guard.conn = Connection::Connected(conn);

            tokio::spawn(async { request_handler(dev_req_rx, host_resp_tx).await });

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

    async fn send_string(self, string: String) -> Result<(), String> {
        let mut state_guard = self.state.lock().await;

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
        if let Connection::Connected(ref mut conn) = state_guard.conn {
            return conn
                .set_state(memori_state)
                .await
                .map_err(|e| format!("Failed to get battery: {e}"));
        }

        Err("Device is not connected".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    let conn = Connection::Disconnected(HostTcpTransport::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            Builder::new()
                .with_max_level(LevelFilter::DEBUG)
                .with_default_subscriber()
                .build(),
        )
        .invoke_handler(taurpc::create_ipc_handler(
            ApiImpl {
                state: Arc::new(Mutex::new(State { conn })),
            }
            .into_handler(),
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// tokio::spawn(async { request_handler(dev_req_rx, host_resp_tx) });
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

        println!("responding with :{resp:?}");
        host_resp_tx
            .send(Sequenced::new(req.seq_num, resp))
            .inspect_err(|e| error!("expected to send response successfully: {e}"))
            .unwrap()
    }
}
