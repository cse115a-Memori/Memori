// use crate::widget_data::refresh_bus_widget;
use crate::widget_data::github_data::refresh_github_widget;
// use crate::widget_data::refresh_twitch_widget;
// use crate::widget_data::refresh_weather_widget;
use memori_ui::widgets::MemoriWidget;
use memori_ui::widgets::WidgetId;
use memori_ui::widgets::WidgetKind;
use memori_ui::MemoriState;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;
use transport::ble_types::*;
use transport::TransError;

// async task that runs in the background when we have an active connection and
// lets us see and choose how to handle incoming requests from the device
pub async fn ble_request_handler(
    memori: Arc<RwLock<Option<MemoriState>>>,
    mut dev_cmd_rx: UnboundedReceiver<DeviceBLECommand>,
    host_resp_tx: UnboundedSender<HostBLEResponse>,
    app_handle: &AppHandle,
) {
    while let Some(cmd) = dev_cmd_rx.recv().await {
        println!("received command from device {cmd:#?}");
        let resp = match cmd {
            DeviceBLECommand::RefreshData { widget_id } => {
                handle_refresh_data(&memori, widget_id, app_handle).await
            }
            DeviceBLECommand::Ping => HostBLEResponse::Ping { result: Ok(()) },
        };
        host_resp_tx.send(resp).unwrap();
    }
}

// takes a widgetid and updates it based on the current state, returing a
// response object to send back to the device.
async fn handle_refresh_data(
    memori: &RwLock<Option<MemoriState>>,
    widget_id: WidgetId,
    app_handle: &AppHandle,
) -> HostBLEResponse {
    let guard = memori.read().await;

    let state = match &*guard {
        Some(s) => s,
        None => {
            return HostBLEResponse::RefreshData {
                result: Err(TransError::InternalError),
            }
        }
    };

    let widget = match state.widgets.get(&widget_id) {
        Some(w) => w,
        None => {
            return HostBLEResponse::RefreshData {
                result: Err(TransError::InternalError),
            }
        }
    };

    // log
    let refresh_result: Result<Box<MemoriWidget>, String> = match widget.kind {
        //WidgetKind::Twitch(_) => refresh_twitch_widget().await,
        WidgetKind::Github(_) => Ok(Box::new(MemoriWidget::new(
            widget.id,
            WidgetKind::Github(refresh_github_widget(app_handle).await.unwrap()),
            widget.get_remote_update_frequency(),
            widget.get_local_update_frequency(),
        ))),
        //   WidgetKind::Bus(_) => refresh_bus_widget().await,
        // WidgetKind::Weather(_) => refresh_temp().await,
        _ => Err("invalid refresh branch".to_string()),
    };

    match refresh_result {
        Ok(new_widget) => {
            let updated_kind = new_widget.kind;

            let updated_widget = MemoriWidget::new(
                widget.id,
                updated_kind,
                widget.get_remote_update_frequency(),
                widget.get_local_update_frequency(),
            );

            HostBLEResponse::RefreshData {
                result: Ok(updated_widget),
            }
        }
        Err(_) => HostBLEResponse::RefreshData {
            result: Err(TransError::InternalError),
        },
    }
}
