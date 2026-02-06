#![no_std]
// use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::usize;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, with_timeout};
use log::{debug, error, info, trace, warn};
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use transport::ble_types::*;
use transport::{ByteArray, DeviceTransport, TransError, TransResult, WidgetId};

const MAX_INFLIGHT: usize = 4;

pub const BLE_TIMEOUT_DUR: u64 = 5;
pub static BLE_CMD_CHANNEL: Channel<CriticalSectionRawMutex, OutgoingCommand, 5> = Channel::new();
pub static BLE_HOST_RESPONSE: [Signal<CriticalSectionRawMutex, HostBLEResponse>; MAX_INFLIGHT] =
    [const { Signal::new() }; MAX_INFLIGHT];

pub static BLE_CONNECTED: AtomicBool = AtomicBool::new(false);
pub static MESSAGE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

fn get_next_id() -> MessageID {
    MESSAGE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub struct OutgoingCommand {
    pub cmd: DeviceBLECommand,
    pub id: MessageID,
}

pub struct DeviceBLETransport {
    cmd_tx: Sender<'static, CriticalSectionRawMutex, OutgoingCommand, 5>,
}

impl DeviceBLETransport {
    pub fn new() -> Self {
        Self {
            cmd_tx: BLE_CMD_CHANNEL.sender(),
        }
    }

    async fn handle_command(&self, cmd: DeviceBLECommand) -> TransResult<HostBLEResponse> {
        if !BLE_CONNECTED.load(Ordering::SeqCst) {
            return Err(TransError::InternalError);
        }
        let id = get_next_id();
        let outgoing = OutgoingCommand { cmd, id };

        self.cmd_tx.send(outgoing).await;

        let id = id as usize;
        match with_timeout(
            Duration::from_secs(BLE_TIMEOUT_DUR + 2),
            BLE_HOST_RESPONSE[id % MAX_INFLIGHT].wait(),
        )
        .await
        {
            Ok(host_response) => Ok(host_response),
            Err(_) => Err(TransError::InternalError),
        }
    }
}

impl DeviceTransport for DeviceBLETransport {
    async fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray> {
        let command = DeviceBLECommand::RefreshData { widget_id };

        match self.handle_command(command).await {
            Ok(HostBLEResponse::RefreshData { result }) => result,
            Ok(_) => Err(TransError::InternalError),
            Err(e) => Err(e),
        }
    }

    async fn ping(&mut self) -> TransResult<()> {
        let command = DeviceBLECommand::Ping;

        match self.handle_command(command).await {
            Ok(HostBLEResponse::Ping { result }) => result,
            Ok(_) => Err(TransError::InternalError),
            Err(e) => Err(e),
        }
    }
}
