use crate::commands::data::{call_api_json, set_memori_state};
use crate::{AppState, MemoriLayout, MemoriWidget};
use memori_ui::widgets::{UpdateFrequency, Weather, WidgetId, WidgetKind};
use memori_ui::MemoriState;
use serde::Deserialize;
use serde_json::json;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn refresh_temp() -> Result<Box<MemoriWidget>, String> {
    let lat = "11.0";
    let lon = "11.0";
    #[derive(Deserialize, Debug)]
    struct WeatherResponse {
        main: Main,
    }

    #[derive(Deserialize, Debug)]
    struct Main {
        temp: f32,
    }

    let request_body = json!({
        "provider": "weather",
        "url": format!(
            "https://api.openweathermap.org/data/2.5/weather?appid={{}}&lat={lat}&lon={lon}&units=metric"
        ),
        "lat": lat.to_string(),//lat.to_string().as_str(),
        "lon": lon.to_string(),// lon.to_string().as_str(),
    });
    let response: WeatherResponse = call_api_json(request_body).await?;
    let _weather = Weather {
        temp: response.main.temp.to_string(),
        icon: String::from("Twitch User Julian"),
    };
    // let json = serde_json::to_string(&weather);
    // let mut data: ByteArray = Default::default();
    // data.extend_from_slice(json.unwrap().as_bytes());
    // Ok(data)
    let updated_widget = Box::new(MemoriWidget::new(
        WidgetId(0),
        WidgetKind::Weather(Weather::new("")),
        UpdateFrequency::Never,
        UpdateFrequency::Never,
    ));
    Ok(updated_widget)
}

#[tauri::command]
#[specta::specta]
pub async fn init_temp(state: State<'_, AppState>) -> Result<(), String> {
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Weather(Weather::new("")),
            UpdateFrequency::Seconds(5),
            UpdateFrequency::Never,
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await
}
