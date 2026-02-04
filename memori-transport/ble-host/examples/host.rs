use std::time::Duration;
use tokio::time::{self};
use transport::{HostTransport, WidgetId};
use ble_host::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut transport = HostBLETransport::connect().await.unwrap_or_else(|e| {
        eprintln!("Fatal: failed to connect to BLE transport: {e:?}");
        std::process::exit(1);
    });

    // time::sleep(Duration::from_secs(5)).await;


    loop {
        time::sleep(Duration::from_secs(1)).await;

        match transport.get_widget(WidgetId(12)).await {
            Ok(widget) =>{
                println!("[logic] got widget, data: {:?}", widget)
            },
            Err(e) => println!("[logic] refresh failed: {:?}", e)
        }

        time::sleep(Duration::from_secs(1)).await;

        match transport.get_battery_level().await {
            Ok(level) =>{
                println!("[logic] got battery level: {level}")
            },
            Err(e) => println!("[logic] refresh failed: {:?}", e)
        }
    }
    // Ok(())
}
