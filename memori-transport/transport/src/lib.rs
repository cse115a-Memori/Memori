#![no_std]

pub mod ble_types;

use serde::Deserialize;
use serde::Serialize;

/// Helper type to define a byte array.
pub type ByteArray = heapless::Vec<u8, 1024>;

/// New type struct for a widget identifier.
#[derive(Serialize, Deserialize)]
pub struct WidgetId(u32);

/// Any errors risen during transport.
#[derive(Serialize, Deserialize)]
pub enum TransError {
    NoAck,
    WidgetNotFound,
}

/// Result type for transport errors.
pub type TransResult<T> = Result<T, TransError>;

/// The general information held by a widget.
#[derive(Serialize, Deserialize)]
pub struct Widget {
    id: u8,
    data: ByteArray,
}

/// Device configuration options
#[derive(Serialize, Deserialize)]
pub struct DeviceConfig {
    dark_mode: bool,
}

pub trait HostTransport {
    /// Set the widgets that are going to be displayed on the device.
    fn set_widgets(&mut self, widget: Widget) -> TransResult<()>;

    /// Get the data for a widget, given a device id.
    fn get_widget(&mut self, id: WidgetId) -> TransResult<Widget>;

    /// Get the battery level of the device.
    /// NOTE: using this on a simulator will always return 100
    fn get_battery_level(&mut self) -> TransResult<u8>;

    /// Set the configuration settings of the device.
    fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()>;
}

pub trait DeviceTransport {
    /// Ask the host for a refresh of widget data.
    fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray>;

    /// Ping the host to ensure they are still connected.
    fn ping(&mut self) -> TransResult<()>;
}
