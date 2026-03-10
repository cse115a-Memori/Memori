//use crate::commands::call_api_json;
use memori_ui::widgets::Weather;
use serde::Deserialize;
use serde_json::json;

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
    // let api_key = "7fd5a9bc8b2d2753007ca6740cfc8917";
    // 
    // let req_body = json!({
    //     "provider": "weather",
    //     "url": format!(
    //         "https://api.openweathermap.org/data/2.5/weather?appid={{}}&lat={lat}&lon={lon}&units=metric"
    //     ),
    //     "lat": lat.to_string(),
    //     "lon": lon.to_string(),
    // });
    // let weather: WeatherResponse = call_api_json(req_body).await?;
    // let description = weather_res.weather.first().unwrap().main.clone();
    // let rain: String = match weather_res.rain {
    //     Some(res) => res.mmph.to_string(),
    //     None => "no rain".to_string(),
    // };
    // let city = "Santa Cruz".to_string();
    // let temp = weather.main.temp.to_string();
    // let humidity = weather.main.humidity.to_string();
    // let wind = weather.wind.speed.to_string();
    // let clouds = weather.clouds.all.to_string();
    // Ok(Weather {
    //     city,
    //     temp,
    //     humidity,
    //     wind,
    //     clouds,
    //     description,
    //     rain,
    // })
    todo!()
}
