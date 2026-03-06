use ble_host::HostBLETransport;
use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
    HostTcpTransport,
};
use memori_ui::{Memori, MemoriState};
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::sync::{Mutex, RwLock};
use std::sync::{Arc};

#[derive(Debug)]
pub(crate) enum TCPConnection {
    Connected(HostTcpTransport<DeviceConnected>),
    Disconnected(HostTcpTransport<DeviceDisconnected>),
}

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
    pub(crate) tcp_conn: Mutex<TCPConnection>,
    pub(crate) conn: Mutex<DeviceConnection>,
    pub(crate) memori: Arc<RwLock<Option<MemoriState>>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tcp_conn: Mutex::new(TCPConnection::Disconnected(HostTcpTransport::default())),
            conn: Mutex::new(DeviceConnection::Disconnected),
            memori: Arc::new(RwLock::new(None))
        }
    }
}
