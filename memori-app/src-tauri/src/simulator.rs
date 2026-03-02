use crate::commands::bus::send_bustime;
use crate::commands::github::send_github;
use crate::commands::twitch::send_twitch;
use crate::commands::weather::send_temp;
use memori_tcp::{DeviceRequest, HostResponse, Sequenced};
use memori_ui::widgets::WidgetKind;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub async fn request_handler(
    mut dev_req_rx: UnboundedReceiver<Sequenced<DeviceRequest>>,
    host_resp_tx: UnboundedSender<Sequenced<HostResponse>>,
) {
    while let Some(req) = dev_req_rx.recv().await {
        println!("received request from device! {req:#?}");

        let resp = match req.msg_kind {
            DeviceRequest::RefreshData(kind) => {
                let data = match kind {
                    WidgetKind::Twitch(_) => send_twitch().await,
                    WidgetKind::Github(_) => send_github().await,
                    WidgetKind::Bus(_) => send_bustime().await,
                    WidgetKind::Weather(_) => send_temp().await,
                    _ => Err("branch does not exist for refresh kind".to_string()),
                };
                HostResponse::UpdatedWidget(data)
            }
            DeviceRequest::Ping => HostResponse::Pong,
        };

        host_resp_tx
            .send(Sequenced::new(req.seq_num, resp))
            .unwrap();
    }
}
