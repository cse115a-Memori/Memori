use std::io;

use serde::{Deserialize, Serialize};
use transport::{ByteArray, DeviceConfig, Widget, WidgetId};

use thiserror::Error;

pub mod device;
pub mod host;

const TCP_ADDR: &str = "127.0.0.1:6942";

#[derive(Debug, Error)]
pub enum TcpTransportError {
    #[error("IO Error!")]
    IOError(#[from] io::Error),
}

pub type TcpTransportResult<T> = Result<T, TcpTransportError>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Message {
    DeviceRequest(DeviceRequest),
    DeviceResponse(Box<DeviceResponse>),
    HostRequest(HostRequest),
    HostResponse(HostResponse),
}

/// These are requests a device can send
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DeviceRequest {
    RefreshData(WidgetId),
    Ping,
}

/// These are responses a device can receive
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum HostResponse {
    UpdatedWidget(Box<ByteArray>),
    Pong,
}

/// These are requests a host can send
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum HostRequest {
    GetBatteryLevel,
    Ping,
    SetDeviceConfig(DeviceConfig),
    //NOTE: this will change in the future
    SetWidgets(Box<Widget>),
    GetWidget(WidgetId),
}
/// These are responses a host can receive
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DeviceResponse {
    BatteryLevel(u8),
    Widget(Box<Widget>),
    Pong,
    /// General success message for any updates sent by host
    Success,
}

// #[derive(Debug)]
pub struct HostTcpTransport<State> {
    state: State,
}
// pub struct HostTcpTransport<State> {
//     _connection_status: PhantomData<ConnStatus>,
//     request_handler: HostRequestHandler,
//     msg_sender: Option<UnboundedSender<Message>>,
//     responses: Option<UnboundedReceiver<DeviceResponse>>,
//     send_task: Option<JoinHandle<()>>,
//     recv_task: Option<JoinHandle<()>>,
// }

#[derive(Debug)]
pub struct DeviceTcpTransport<State> {
    state: State,
}
// #[derive(Debug)]
// pub struct DeviceTcpTransport<State> {
//     _connection_status: PhantomData<ConnStatus>,
//     msg_sender: UnboundedSender<Message>,
//     responses: UnboundedReceiver<HostResponse>,
//     send_task: JoinHandle<()>,
//     recv_task: JoinHandle<()>,
// }
