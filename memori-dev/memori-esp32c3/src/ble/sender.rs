use ble_device::BLE_CMD_CHANNEL;
use memori_ui::widgets::WidgetId;
use transport::TransResult;
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
                let _result = send_ping(server, conn, msg_id).await;
            }
            DeviceBLECommand::RefreshData { widget_id } => {
                // return a host BLE request
                let _result = request_refresh(server, conn, msg_id, widget_id).await;
            }
        };
    }
}

pub async fn send_ping<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    msg_id: MessageID,
) -> TransResult<()> {
    send_packet(
        DeviceBLEPacket::Command(DeviceBLECommand::Ping),
        msg_id,
        server,
        conn,
    )
    .await
}

pub async fn request_refresh<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    msg_id: MessageID,
    widget_id: WidgetId,
) -> TransResult<()> {
    send_packet(
        DeviceBLEPacket::Command(DeviceBLECommand::RefreshData { widget_id }),
        msg_id,
        server,
        conn,
    )
    .await
}
