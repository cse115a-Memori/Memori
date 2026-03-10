use ble_host::HostBLETransport;
use memori_ui::MemoriState;
use memori_tcp::{host::DeviceConnected, HostTcpTransport};
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::sync::{Mutex, RwLock};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum DeviceMode {
    RealDevice,
    Simulator,
}

pub(crate) enum DeviceConnection {
    RealDevice(HostBLETransport),
    Simulator(HostTcpTransport<DeviceConnected>),
    Disconnected,
}

pub struct AppState {
    pub(crate) conn: Mutex<DeviceConnection>,
    pub(crate) memori: Arc<RwLock<Option<MemoriState>>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            conn: Mutex::new(DeviceConnection::Disconnected),
            memori: Arc::new(RwLock::new(None)),
        }
    }
}
