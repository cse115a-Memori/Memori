use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use transport::{ByteArray, DeviceConfig, Widget, WidgetId};

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
pub struct Message {
    seq_num: u32,
    kind: MessageKind,
}

#[derive(Debug)]
pub struct Sequenced<T> {
    /// Sequence number for this message. A response's sequence number is always
    /// equal to its requests' sequence number. Additionally requests sent from the
    /// device always have odd sequence numbers, while events sent from the host
    /// always have even sequence numbers
    pub seq_num: u32,
    /// will be an enum variant of message kind
    pub msg_kind: T,
}

impl<T> Sequenced<T> {
    pub fn new(seq_num: u32, msg_kind: T) -> Self {
        Self { seq_num, msg_kind }
    }
}

macro_rules! impl_sequenced_to_message {
    // for types that need to be boxed
    ($inner_type:ty, $variant: ident, boxed) => {
        impl From<Sequenced<$inner_type>> for Message {
            fn from(value: Sequenced<$inner_type>) -> Self {
                Message {
                    seq_num: value.seq_num,
                    kind: MessageKind::$variant(Box::new(value.msg_kind)),
                }
            }
        }
    };
    // for non-boxed types
    ($inner_type:ty, $variant: ident) => {
        impl From<Sequenced<$inner_type>> for Message {
            fn from(value: Sequenced<$inner_type>) -> Self {
                Message {
                    seq_num: value.seq_num,
                    kind: MessageKind::$variant(value.msg_kind),
                }
            }
        }
    };
}

impl_sequenced_to_message!(DeviceRequest, DeviceRequest);
impl_sequenced_to_message!(DeviceResponse, DeviceResponse, boxed);
impl_sequenced_to_message!(HostRequest, HostRequest);
impl_sequenced_to_message!(HostResponse, HostResponse);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MessageKind {
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

#[derive(Debug)]
pub struct HostTcpTransport<State> {
    state: State,
}

#[derive(Debug)]
pub struct DeviceTcpTransport<State> {
    state: State,
}
