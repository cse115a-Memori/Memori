// if we dont have the feature "std", we are going to compile without the std libary
#[cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

// uncomment out later
// #[cfg(feature = "websocket")]
pub mod websocket;

#[cfg(feature = "ble")]
pub mod ble;

use serde::Deserialize;
use serde::Serialize;

pub type ByteArray = heapless::Vec<u8, 1024>;

#[derive(Serialize, Deserialize)]
pub struct WidgetId(u32);

#[derive(Serialize, Deserialize)]
pub enum TransError {
    NoAck,
    WidgetNotFound,
}

/// Result type for transport errors
pub type TransResult<T> = Result<T, TransError>;

#[derive(Serialize, Deserialize)]
// #[derive(Serialize, Deserialize)]
/// The general information held by a widget
pub struct Widget {
    id: u8,
    data: ByteArray,
}

#[derive(Serialize, Deserialize)]
/// Device configuration options
pub struct DeviceConfig {
    dark_mode: bool,
}

pub trait HostTransport {
    fn set_widgets(&mut self, widget: Widget) -> TransResult<()>;

    fn get_widget(&mut self, id: WidgetId) -> TransResult<Widget>;

    // fn send_widget_data(&mut self, id: WidgetId, data: impl AsRef<[u8]>);

    fn get_battery_level(&mut self) -> TransResult<u8>;

    fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()>;
}

pub trait DeviceTransport {
    fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray>;

    fn ping(&mut self) -> TransResult<()>;
}
