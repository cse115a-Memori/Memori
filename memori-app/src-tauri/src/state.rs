use ble_host::HostBLETransport;
use memori_tcp::{
    host::{DeviceConnected, DeviceDisconnected},
    HostTcpTransport,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::sync::Mutex;

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
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tcp_conn: Mutex::new(TCPConnection::Disconnected(HostTcpTransport::default())),
            conn: Mutex::new(DeviceConnection::Disconnected),
        }
    }
}
