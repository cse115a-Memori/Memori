#![no_std]

pub mod ble_types;

use memori_ui::MemoriState;
use memori_ui::widgets::MemoriWidget;
use memori_ui::widgets::WidgetId;
use serde::Deserialize;
use serde::Serialize;

use core::error::Error;
use core::fmt::Display;
/// Helper type to define a byte array.
pub type ByteArray = heapless::Vec<u8, 256>;

/// Any errors risen during transport.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TransError {
    InternalError,
    NoAck,
    WidgetNotFound,
    SerializationFailure,
    NotConnected,
    Timeout,
    InvalidMessage,
    ProtocolIssue,
}

impl Display for TransError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TransError::InternalError => write!(f, "Internal Error!"),
            TransError::NoAck => write!(
                f,
                "No Ack, also know we might send these errors for no reason"
            ),
            TransError::WidgetNotFound => write!(f, "Widget not found! possible invalid WidgetID!"),
            TransError::SerializationFailure => {
                write!(f, "Failed to draw widget")
            }
            TransError::NotConnected => write!(f, "Transport is not connected!"),
            TransError::Timeout => write!(f, "Timeout reached on transport!"),
            TransError::InvalidMessage => write!(f, "Invalid message sent through transport!"),
            TransError::ProtocolIssue => write!(f, "Something wrong happened with the protocol!"),
        }
    }
}

impl Error for TransError {}

/// Result type for transport errors.
pub type TransResult<T> = Result<T, TransError>;

/// Device configuration options
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DeviceConfig {
    dark_mode: bool,
}

pub trait HostTransport {
    /// Set the widgets that are going to be displayed on the device.
    fn set_state(&mut self, state: MemoriState) -> impl Future<Output = TransResult<()>> + Send;

    /// Get the data for a widget, given a device id.
    fn get_widget(
        &mut self,
        id: WidgetId,
    ) -> impl Future<Output = TransResult<MemoriWidget>> + Send;

    /// Get the battery level of the device.
    /// NOTE: using this on a simulator will always return 100
    fn get_battery_level(&mut self) -> impl Future<Output = TransResult<u8>> + Send;

    /// Set the configuration settings of the device.
    fn set_device_config(
        &mut self,
        config: DeviceConfig,
    ) -> impl Future<Output = TransResult<()>> + Send;
}

pub trait DeviceTransport {
    /// Ask the host for a refresh of widget data.
    fn refresh_data(
        &mut self,
        widget_id: WidgetId,
    ) -> impl Future<Output = TransResult<MemoriWidget>>;

    /// Ping the host to ensure they are still connected.
    fn ping(&mut self) -> impl Future<Output = TransResult<()>>;
}
