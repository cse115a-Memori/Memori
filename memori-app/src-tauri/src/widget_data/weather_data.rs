use crate::commands::call_api_json;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<Weather>,
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
struct Weather {
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

pub(crate) async fn fetch_weather_temp(
    lat: f64,
    lon: f64,
) -> Result<(String, String, String, String, String, String, String), String> {
    let req_body = json!({
        "provider": "weather",
        "url": format!(
            "https://api.openweathermap.org/data/2.5/weather?appid={{}}&lat={lat}&lon={lon}&units=metric"
        ),
        "lat": lat.to_string(),
        "lon": lon.to_string(),
    });
    let weather_res: WeatherResponse = call_api_json(req_body).await?;
    let description = weather_res.weather.first().unwrap().main.clone();
    let rain: String = match weather_res.rain {
        Some(res) => res.mmph.to_string(),
        None => "no rain".to_string(),
    };
    let weather_text: (String, String, String, String, String, String, String) = (
        "Santa Cruz".to_string(),
        weather_res.main.temp.to_string(),
        weather_res.main.humidity.to_string(),
        weather_res.wind.speed.to_string(),
        rain,
        weather_res.clouds.all.to_string(),
        description.to_string(),
    );
    Ok(weather_text)
}
