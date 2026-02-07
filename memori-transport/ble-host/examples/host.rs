use ble_host::*;
use memori_ui::widgets::WidgetId;
use std::time::Duration;
use tokio::time::{self};
use transport::HostTransport;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut transport = HostBLETransport::connect().await.unwrap_or_else(|e| {
        eprintln!("Fatal: failed to connect to BLE transport: {e:?}");
        std::process::exit(1);
    });

    loop {
        time::sleep(Duration::from_secs(1)).await;

        match transport.get_widget(WidgetId(12)).await {
            Ok(widget) => {
                println!("[logic] got widget, data: {:?}", widget)
            }
            Err(e) => println!("[logic] refresh failed: {:?}", e),
        }

        time::sleep(Duration::from_secs(1)).await;

        match transport.get_battery_level().await {
            Ok(level) => {
                println!("[logic] got battery level: {level}")
            }
            Err(e) => println!("[logic] refresh failed: {:?}", e),
        }
    }
}
