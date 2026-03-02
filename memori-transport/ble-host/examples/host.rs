use ble_host::*;
use transport::ble_types::*;
use std::time::Duration;
use tokio::time::{self};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use transport::HostTransport;
use memori_ui::{
    layout::MemoriLayout,
    widgets::{Bus, MemoriWidget, Name, UpdateFrequency, Weather, WidgetId, WidgetKind},
    MemoriState,
};

pub async fn ble_request_handler(
    mut dev_cmd_rx: UnboundedReceiver<DeviceBLECommand>,
    host_resp_tx: UnboundedSender<HostBLEResponse>,
) {
    while let Some(cmd) = dev_cmd_rx.recv().await {
        println!("received command from device! {cmd:#?}");
        let resp = match cmd {
            DeviceBLECommand::RefreshData { widget_id } => {
                todo!()
            }
            DeviceBLECommand::Ping => HostBLEResponse::Ping { result: Ok(()) },
        };
        host_resp_tx.send(resp).unwrap();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (mut conn, (dev_req_rx, host_resp_tx)) = HostBLETransport::connect()
        .await
        .expect("gyuur");

    tokio::spawn(async move {
        ble_request_handler(dev_req_rx, host_resp_tx).await;
    });

    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Name(Name::new("hi")),
            UpdateFrequency::Never,
            UpdateFrequency::Never,
        )],
        vec![MemoriLayout::Fourths {
            top_right: WidgetId(0),
            bottom_left: WidgetId(0),
            bottom_right: WidgetId(0),
            top_left: WidgetId(0),
        }],
        5,
    );

    time::sleep(Duration::from_secs(2)).await;
    conn.set_state(memori_state).await;

    loop {
        time::sleep(Duration::from_secs(1)).await;

        match conn.get_widget(WidgetId(0)).await {
            Ok(widget) => {
                println!("[logic] got widget, data: {:?}", widget)
            }
            Err(e) => println!("[logic] get failed: {:?}", e),
        }

        time::sleep(Duration::from_secs(1)).await;

        match conn.get_battery_level().await {
            Ok(level) => {
                println!("[logic] got battery level: {level}")
            }
            Err(e) => println!("[logic] refresh failed: {:?}", e),
        }
    }
}

