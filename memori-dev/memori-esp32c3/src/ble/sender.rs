use ble_device::BLE_CMD_CHANNEL;
use log::error;
use transport::ble_types::*;
use trouble_host::prelude::*;

use crate::ble::{Server, send_packet};

pub async fn sender_task<P: PacketPool>(server: &Server<'_>, conn: &GattConnection<'_, '_, P>) {
    let cmd_rx = BLE_CMD_CHANNEL.receiver();

    loop {
        let outgoing = cmd_rx.receive().await;
        let msg_id = outgoing.id;

        match outgoing.cmd {
            DeviceBLECommand::Ping => {
                let _ = send_packet(
                    DeviceBLEPacket::Command(DeviceBLECommand::Ping),
                    msg_id,
                    server,
                    conn,
                )
                .await
                .inspect_err(|e| error!("failed to send packet, {e:#?}"));
            }
            DeviceBLECommand::RefreshData { widget_id } => {
                let _ = send_packet(
                    DeviceBLEPacket::Command(DeviceBLECommand::RefreshData { widget_id }),
                    msg_id,
                    server,
                    conn,
                )
                .await
                .inspect_err(|e| error!("failed to send packet, {e:#?}"));
            }
        };
    }
}
