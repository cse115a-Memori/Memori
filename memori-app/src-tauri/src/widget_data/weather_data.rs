use crate::oauth::cloudflare;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<Weather>,
    wind: Wind,
    cloud: Cloud,
    rain: Rain,
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
struct Weather {
    main: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    mmph: f32,
}

pub async fn call_api_json<T>(req_body: serde_json::Value) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let api_res = cloudflare("call_api", req_body)
        .await
        .map_err(|err| format!("cloudflare error: {err}"))?;
    serde_json::from_value(api_res).map_err(|err| err.to_string())
}

pub(crate) async fn fetch_weather_temp(lat: f64, lon: f64) -> Result<f32, String> {
    let req_body = json!({
        "provider": "weather",
        "url": format!(
            "https://api.openweathermap.org/data/2.5/weather?appid={{}}&lat={lat}&lon={lon}&units=metric"
        ),
        "lat": lat.to_string(),
        "lon": lon.to_string(),
    });
    let weather_res: WeatherResponse = call_api_json(req_body).await?;
    Ok(weather_res.main.temp)
}
