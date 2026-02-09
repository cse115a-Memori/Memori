use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::error;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::Buffer,
    style::{Style, Stylize},
    text::Line,
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
};
use reqwest::blocking;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<WeatherDetail>,
}

#[derive(Deserialize, Debug)]
struct WeatherDetail {
    main: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
}

enum WeatherIcon {
    Sun,
    Cloudy,
    Rainy,
    Snowy,
}

impl WeatherIcon {
    fn to_ascii(&self) -> Vec<&'static str> {
        match self {
            WeatherIcon::Sun => vec![
                "  \\   |   /  ",
                "     .-'-.    ",
                " ―   (   )   ― ",
                "      '― '    ",
                "   /   |   \\  ",
            ],
            WeatherIcon::Cloudy => vec![
                "     .--.     ",
                "  .-(    )-.  ",
                " (__  .  __)  ",
                "  `--` `--`   ",
            ],
            WeatherIcon::Rainy => vec![
                "    .--.     ",
                "  .-(    )-.  ",
                " (  .  .  )   ",
                "  `--|--`     ",
                "  |  / \\  |  ",
            ],
            WeatherIcon::Snowy => vec![
                "    .-^-._    ",
                "   (     )    ",
                "  . `-^-`*    ",
                "   * . * .    ",
            ],
        }
    }
}

pub struct Weather {
    pub temp: f32,
    pub icon: WeatherIcon,
}

impl Weather {
    pub fn new(api_key: &str, lat: f64, lon: f64) -> Result<Weather, Box<dyn error::Error>> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            lat, lon, api_key
        );

        let response: WeatherResponse = blocking::get(&url)?.json()?; // Blocking call

        let icon = match response.weather.get(0).map(|w| w.main.as_str()) {
            Some("Clear") => WeatherIcon::Sun,
            Some("Clouds") => WeatherIcon::Cloudy,
            Some("Rain") => WeatherIcon::Rainy,
            Some("Snow") => WeatherIcon::Snowy,
            _ => WeatherIcon::Cloudy,
        };

        Ok(Weather {
            temp: response.main.temp,
            icon,
        })
    }
}

impl Widget for &Weather {
    /*
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let string = format!("Temp: {}", self.temp);
        Text::from(string).render(area, buf);
    }
    */
    fn render(self, area: Rect, buf: &mut Buffer) {
        let weather_block = Block::default()
            .title(Line::from(" Weather ").bold().centered())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        weather_block.clone().render(area, buf);

        let icon_lines = self.icon.to_ascii();

        let text = Text::from(
            icon_lines
                .into_iter()
                .map(|line| Line::from(line))
                .collect::<Vec<_>>(),
        )
        .centered();

        let icon_paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let temp_text = format!("Temp: {:.1}°C", self.temp);
        let temp_paragraph = Paragraph::new(Line::from(temp_text))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let inner_area = weather_block.inner(area);
        let left_width = inner_area.width / 3;
        let right_width = inner_area.width - left_width;

        let icon_area = Rect::new(inner_area.x, inner_area.y, left_width, inner_area.height);
        let temp_area = Rect::new(
            inner_area.x + left_width,
            inner_area.y,
            right_width,
            inner_area.height,
        );

        icon_paragraph.render(icon_area, buf);
        temp_paragraph.render(temp_area, buf);
    }
}
