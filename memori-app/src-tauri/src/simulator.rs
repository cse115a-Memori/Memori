use memori_tcp::{DeviceRequest, HostResponse, Sequenced};
use memori_ui::widgets::{MemoriWidget, Name, UpdateFrequency, WidgetId, WidgetKind};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use memori_ui::MemoriState;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn request_handler(
    memori: Arc<RwLock<Option<MemoriState>>>,
    mut dev_req_rx: UnboundedReceiver<Sequenced<DeviceRequest>>,
    host_resp_tx: UnboundedSender<Sequenced<HostResponse>>,
) {
    while let Some(req) = dev_req_rx.recv().await {
        println!("received request from device! {req:#?}");

        let resp = match req.msg_kind {
            DeviceRequest::RefreshData(id) => {
                let data = Box::new(MemoriWidget::new(
                    WidgetId(0),
                    WidgetKind::Name(Name::new("name")),
                    UpdateFrequency::Never,
                    UpdateFrequency::Never,
                ));
                HostResponse::UpdatedWidget(data)
            }
            DeviceRequest::Ping => HostResponse::Pong,
        };

        host_resp_tx
            .send(Sequenced::new(req.seq_num, resp))
            .unwrap();
    }
}
