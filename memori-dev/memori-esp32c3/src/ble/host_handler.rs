use log::info;
use memori_ui::widgets::{MemoriWidget, Name, WidgetId};
use transport::ble_types::*;
use trouble_host::prelude::*;

use crate::ble::{Server, send_packet};

/// Act on any host commands.
pub async fn handle_host_cmd<P: PacketPool>(
    cmd: HostBLECommand,
    msg_id: MessageID,
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) {
    info!("[transport] received cmd {:?}", cmd);

    match cmd {
        HostBLECommand::GetWidget { widget_id } => {
            get_widget_response(widget_id, msg_id, server, conn).await;
        }
        HostBLECommand::SetState { state: _ } => {
            todo!()
            
            // Iterate through each widget and spawn update tasks
            let widgets_to_update = {
                let locked_state = mem_state.lock().await;
                locked_state.widgets.iter()
                    .filter_map(|(widget_id, widget)| {
                        match widget.get_local_update_frequency() {
                            UpdateFrequency::Never => None,
                            UpdateFrequency::Seconds(s) if s < 60 => Some((*widget_id, s)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
            };
            
            for (widget_id, seconds) in widgets_to_update {
                spawner
                    .spawn(widget_update_task(mem_state, widget_id, seconds as u64))
                    .expect("Failed to spawn widget update task");
            }
        }
        HostBLECommand::SetConfig { config: _ } => {
            todo!()
        }
    }
}

async fn get_widget_response<P: PacketPool>(
    widget_id: WidgetId,
    msg_id: MessageID,
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) {
    let widget = MemoriWidget::new(
        widget_id,
        memori_ui::widgets::WidgetKind::Name(Name::new("Hi")),
        memori_ui::widgets::UpdateFrequency::Never,
        memori_ui::widgets::UpdateFrequency::Never,
    );

    let pkt = DeviceBLEPacket::Response(DeviceBLEResponse::WidgetGet { result: Ok(widget) });
    let _ = send_packet(pkt, msg_id, server, conn).await;
}
