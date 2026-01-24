use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpStream, tcp::OwnedWriteHalf},
    sync::{
        Mutex,
        mpsc::{UnboundedReceiver, UnboundedSender},
    },
};

/// Marker types for Host.
pub struct Host;
/// Marker type for Device.
pub struct Device;

const TCP_ADDR: &str = "127.0.0.1:6942";
const TCP_PACKET_LEN: usize = 32;

pub mod device;

pub mod host;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TcpMessageKind {
    Request(TcpRequest),
    Response(TcpResponse),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TcpRequest {
    GetBatteryLevel,
}

#[derive(Debug)]
pub enum TcpTransportError {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TcpResponse {
    RespondBatteryLevel(u8),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct TcpMessage {
    /// This id should be used during sending and receiving, so the processor on
    /// either side knows which message relates to what thingy
    // id: u32,
    kind: TcpMessageKind,
}

impl TcpMessage {
    fn new(kind: TcpMessageKind) -> Self {
        Self { kind }
    }
}

pub struct TcpTransport<Kind> {
    _kind: std::marker::PhantomData<Kind>,

    writer: UnboundedSender<Vec<u8>>,
    responses: UnboundedReceiver<TcpResponse>,
    // request_map: HashMap<u32, TcpRequest>,
}
