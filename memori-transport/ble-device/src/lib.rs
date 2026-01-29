#![no_std]
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_time::{Duration, with_timeout};
use log::{debug, error, info, trace, warn};
use transport::ble_types::*;
use transport::{ByteArray, DeviceTransport, TransError, TransResult, WidgetId};

pub const BLE_TIMEOUT_DUR: u64 = 5;
pub static BLE_CMD_CHANNEL: Channel<CriticalSectionRawMutex, DeviceBLECommand, 5> = Channel::new();
pub static BLE_RESP_CHANNEL: Channel<CriticalSectionRawMutex, HostBLEResponse, 5> = Channel::new();

pub static BLE_CONNECTED: AtomicBool = AtomicBool::new(false);

pub struct DeviceBLETransport {
    cmd_tx: Sender<'static, CriticalSectionRawMutex, DeviceBLECommand, 5>,
    resp_rx: Receiver<'static, CriticalSectionRawMutex, HostBLEResponse, 5>,
}


impl DeviceBLETransport {
    pub fn new() -> Self {
        Self {
            cmd_tx: BLE_CMD_CHANNEL.sender(),
            resp_rx: BLE_RESP_CHANNEL.receiver(),
        }
    }
}

impl DeviceTransport for DeviceBLETransport {
    async fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray> {
        if !BLE_CONNECTED.load(Ordering::SeqCst) {
            return Err(TransError::NotConnected);
        }

        self.cmd_tx
            .send(DeviceBLECommand::RefreshData { widget_id })
            .await;

        // TODO might want to keep the timeout here, but just make sure it's notably longer than
        // BLE_TIMEOUT_DUR to avoid channel fuckery
        //
        // match with_timeout(Duration::from_secs(BLE_TIMEOUT_DUR), self.resp_rx.receive()).await {
        //     Ok(HostBLEResponse::RefreshData { result }) => result,
        //     Ok(_) => Err(TransError::InvalidMessage),
        //     Err(_) => Err(TransError::Timeout),
        // }
        match self.resp_rx.receive().await {
            HostBLEResponse::RefreshData { result } => result,
            _ => Err(TransError::InvalidMessage),
        }
    }

    async fn ping(&mut self) -> TransResult<()> {
        if !BLE_CONNECTED.load(Ordering::SeqCst) {
            return Err(TransError::NotConnected);
        }

        self.cmd_tx.send(DeviceBLECommand::Ping).await;

        match self.resp_rx.receive().await {
            HostBLEResponse::Ping { result } => result,
            _ => Err(TransError::InvalidMessage),
        }

    }
}
