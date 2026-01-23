#![no_std]

pub mod ble_types;

use serde::Deserialize;
use serde::Serialize;

use core::future::Future;
/// Helper type to define a byte array.
pub type ByteArray = heapless::Vec<u8, 1024>;

/// New type struct for a widget identifier.
#[derive(Serialize, Deserialize)]
pub struct WidgetId(pub u32);

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
    id: WidgetId,
    data: ByteArray,
}

/// Device configuration options
#[derive(Serialize, Deserialize)]
pub struct DeviceConfig {
    dark_mode: bool,
}

pub trait HostTransport {
    /// Set the widgets that are going to be displayed on the device.
    fn set_widgets(&mut self, widget: Widget) -> impl Future<Output = TransResult<()>> + Send;

    /// Get the data for a widget, given a device id.
    fn get_widget(&mut self, id: WidgetId) -> impl Future<Output = TransResult<Widget>> + Send;

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
    fn refresh_data(&mut self, widget_id: WidgetId)
    -> impl Future<Output = TransResult<ByteArray>>;

    /// Ping the host to ensure they are still connected.
    fn ping(&mut self) -> impl Future<Output = TransResult<()>>;
}
