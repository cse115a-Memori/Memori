use ble_host::*;
use memori_ui::{
    MemoriState,
    layout::MemoriLayout,
    widgets::{MemoriWidget, Weather, WidgetId},
};
use std::time::Duration;
use tokio::time::{self};
use transport::HostTransport;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = 2479;
    let (mut transport, (_rx, _tx)) = HostBLETransport::connect(&code.to_string())
        .await
        .unwrap_or_else(|e| {
            eprintln!("Fatal: failed to connect to BLE transport: {e:?}");
            std::process::exit(1);
        });

    time::sleep(Duration::from_secs(2)).await;

    let weather_widget = MemoriWidget::new(
        WidgetId(0),
        memori_ui::widgets::WidgetKind::Weather(Weather {
            city: "Santa Cruz".to_owned(),
            temp: "24".to_owned(),
            clouds: "cloudy".to_owned(),
            wind: "Windy".to_owned(),
            rain: "rainy".to_owned(),
            humidity: "humid".to_owned(),
            description: "very rainly".to_owned(),
        }),
        memori_ui::widgets::UpdateFrequency::Never,
        memori_ui::widgets::UpdateFrequency::Never,
    );
    loop {
        time::sleep(Duration::from_secs(1)).await;
        transport
            .set_state(MemoriState::new(
                0,
                vec![weather_widget.clone()],
                vec![MemoriLayout::Full(WidgetId(0))],
                5,
            ))
            .await
            .unwrap();

        // match transport.get_widget(WidgetId(12)).await {
        //     Ok(widget) => {
        //         println!("[logic] got widget, data: {:?}", widget)
        //     }
        //     Err(e) => println!("[logic] refresh failed: {:?}", e),
        // }

        // time::sleep(Duration::from_secs(1)).await;

        // match transport.get_battery_level().await {
        //     Ok(level) => {
        //         println!("[logic] got battery level: {level}")
        //     }
        //     Err(e) => println!("[logic] refresh failed: {:?}", e),
        // }
    }
}
