use crate::{DeviceConfig, TransResult, WidgetId};
use memori_ui::{MemoriState, widgets::MemoriWidget};
use serde::{Deserialize, Serialize};

pub const NUS_SERVICE_UUID: u128 = 0x6e400001b5a3f393e0a9e50e24dcca9e;
pub const NUS_RX_CHAR_UUID: u128 = 0x6e400002b5a3f393e0a9e50e24dcca9e;
pub const NUS_TX_CHAR_UUID: u128 = 0x6e400003b5a3f393e0a9e50e24dcca9e;

pub const BATTERY_SERVICE_UUID: u16 = 0x180f;
pub const BATTERY_LEVEL_CHAR_UUID: u16 = 0x2a19;
pub const BATTERY_NOTIFY_CHAR_UUID: u128 = 0x408813df5dd41f87ec11cdb001100000;

pub const BLE_CHAR_SIZE: usize = 128;

pub type MessageID = u32;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BLEPacket {
    pub payload: BLEPacketPayload,
    pub id: MessageID,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BLEPacketPayload {
    DevicePacket(DeviceBLEPacket),
    HostPacket(HostBLEPacket),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeviceBLEPacket {
    Command(DeviceBLECommand),
    Response(DeviceBLEResponse),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum HostBLEPacket {
    Command(HostBLECommand),
    Response(HostBLEResponse),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeviceBLECommand {
    RefreshData { widget_id: WidgetId },
    Ping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum HostBLEResponse {
    RefreshData { result: TransResult<MemoriWidget> },
    Ping { result: TransResult<()> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum HostBLECommand {
    SetState { state: MemoriState },
    GetWidget { widget_id: WidgetId },
    SetConfig { config: DeviceConfig },
    // Ping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeviceBLEResponse {
    WidgetSet { result: TransResult<()> },
    WidgetGet { result: TransResult<MemoriWidget> },
    DeviceConfigSet { result: TransResult<()> },
    // Ping { result: TransResult<()> },
}
