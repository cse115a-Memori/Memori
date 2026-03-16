use memori_ui::widgets::Weather;
use memori_ui::widgets::{MemoriWidget, UpdateFrequency, WidgetId, WidgetKind};
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<WeatherDescription>,
    wind: Wind,
    clouds: Cloud,
    rain: Option<Rain>,
}

#[derive(Debug, Deserialize)]
struct Cloud {
    all: u32,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherDescription {
    main: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    mmph: f32,
}

pub(crate) async fn refresh_weather_widget(lat: f64, lon: f64) -> Result<Weather, String> {
    println!("Refresh weather widget called");
    let appid = "WEATHER_API_KEY";
    let appid = match env::var(appid) {
        Ok(key) => key,
        Err(error) => return Err(format!("Weather api key missing: {error}")),
    };
    let url = format!("https://api.openweathermap.org/data/2.5/weather?appid={appid}&lat={lat}&lon={lon}&units=imperial");
    let response = Client::new()
        .get(url)
        .header("User-Agent", "my-app")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string());
    let response = match response {
        Ok(data) => data,
        Err(error) => return Err(format!("failed to call weather api: {error}")),
    };
    let weather: WeatherResponse = match serde_json::from_value(response) {
        Ok(data) => data,
        Err(error) => {
            return Err(format!(
                "failed to convert json response to weather struct: {error}"
            ))
        }
    };
    println!("weather: {:?}", weather);
    let description: String = match weather.weather.first() {
        Some(data) => data.main.clone(),
        None => String::from("No weather description"),
    };
    let rain: String = match weather.rain {
        Some(res) => res.mmph.to_string(),
        None => "no rain".to_string(),
    };
    let city = String::from("Santa Cruz");
    // let temp = weather.main.temp.to_string();
    let temp = "59".to_string();
    // let humidity = weather.main.humidity.to_string();
    let humidity = "74".to_string();
    // let wind = weather.wind.speed.to_string();
    let wind = "4".to_string();
    // let clouds = weather.clouds.all.to_string();
    let clouds = "sunny".to_string();
    Ok(Weather {
        city,
        temp,
        humidity,
        wind,
        clouds,
        description,
        rain,
    })
}

pub async fn weather_to_memori_widget(
    widget_id: u32,
    weather_struct: Weather,
) -> Result<MemoriWidget, String> {
    let widget = MemoriWidget {
        id: WidgetId(widget_id),
        kind: WidgetKind::Weather(weather_struct),
        remote_update_frequency: UpdateFrequency::Minutes(30),
        local_update_frequency: UpdateFrequency::Never,
    };
    Ok(widget)
}
