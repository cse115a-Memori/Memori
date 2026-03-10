use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use log::info;
use ratatui::{
    layout::{Alignment, Rect},
    prelude::Buffer,
    style::Stylize,
    text::Line,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use serde::{Deserialize, Serialize};

/// Define a widget by its data
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Weather {
    pub city: String,
    pub temp: String,
    pub clouds: String,
    pub wind: String,
    pub rain: String,
    pub humidity: String,
    pub description: String,
}

impl Weather {
    pub fn new(
        city: impl Into<String>,
        temp: impl Into<String>,
        humidity: impl Into<String>,
        wind: impl Into<String>,
        rain: impl Into<String>,
        clouds: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            city: city.into(),
            temp: temp.into(),
            humidity: humidity.into(),
            wind: wind.into(),
            clouds: clouds.into(),
            rain: rain.into(),
            description: description.into(),
        }
    }
    pub fn update(&mut self) {
        info!("Updated name");
    }
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            city: String::new(),
            temp: String::new(),
            humidity: String::new(),
            wind: String::new(),
            clouds: String::new(),
            rain: String::new(),
            description: String::new(),
        }
    }
}

// impl the function like this
impl Widget for &Weather {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let newline = Line::from("");
        let temp = Line::from(format!("Temp: {}°F", self.temp.clone()));
        let clouds = Line::from(format!("Clouds: {}%", self.clouds.clone()));
        let wind = Line::from(format!("Wind: {}mph", self.wind.clone()));
        let humidity = Line::from(format!("Humidity: {}%", self.humidity.clone()));
        let rain = Line::from(format!("Rain: {}mm/hr", self.rain.clone()));
        let weather_block = Block::default()
            .title(Line::from(self.city.clone()).bold().centered())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));
        weather_block.clone().render(area, buf);
        let outer_inner = weather_block.inner(area);
        match (outer_inner.width, outer_inner.height) {
            (w, h) if w < 30 && h < 6 => {
                // small
                let icon_lines = WeatherIcon::SunSmall.to_ascii();
                let icon_text =
                    Text::from(icon_lines.into_iter().map(Line::from).collect::<Vec<_>>())
                        .centered();
                let icon = Paragraph::new(icon_text)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                let temp = Paragraph::new(vec![temp, clouds, wind])
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                icon.render(Rect::new(outer_inner.x, outer_inner.y, w / 3, h), buf);
                temp.render(Rect::new((w / 3) + 2, outer_inner.y, w / 2, h), buf);
            }
            (w, h) if w < 30 => {
                // tall
                let icon_lines = WeatherIcon::SunSmall.to_ascii();
                let icon_text =
                    Text::from(icon_lines.into_iter().map(Line::from).collect::<Vec<_>>())
                        .centered();
                let icon = Paragraph::new(icon_text)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                let temp = Paragraph::new(vec![temp, clouds, wind, rain])
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                icon.render(Rect::new(outer_inner.x, outer_inner.y + 1, w, h / 2), buf);
                temp.render(Rect::new(outer_inner.x, outer_inner.y + 5, w, h / 2), buf);
            }
            (w, h) if h < 6 => {
                // wide
                let icon_lines = WeatherIcon::SunSmall.to_ascii();
                let icon_text =
                    Text::from(icon_lines.into_iter().map(Line::from).collect::<Vec<_>>())
                        .centered();
                let icon = Paragraph::new(icon_text)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                let temp = Paragraph::new(vec![temp, wind, clouds])
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                let temp2 = Paragraph::new(vec![rain, humidity])
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                icon.render(Rect::new(outer_inner.x, outer_inner.y, w / 3, h), buf);
                temp.render(Rect::new(w / 3, outer_inner.y, w / 3, h), buf);
                temp2.render(Rect::new((w / 3) * 2, outer_inner.y, w / 3, h), buf);
            }
            (w, h) => {
                // full
                let icon_lines = WeatherIcon::Sun.to_ascii();
                let icon_text =
                    Text::from(icon_lines.into_iter().map(Line::from).collect::<Vec<_>>())
                        .centered();
                let icon = Paragraph::new(icon_text)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                let temp = Paragraph::new(vec![newline, temp, wind, rain, clouds, humidity])
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                icon.render(Rect::new(outer_inner.x, outer_inner.y + 2, w / 2, h), buf);
                temp.render(Rect::new(w / 2, outer_inner.y, w / 2, h), buf);
            }
        }
    }
}

enum WeatherIcon {
    Sun,
    SunSmall,
    // Cloudy,
    // Rainy,
    // Snowy,
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
            WeatherIcon::SunSmall => vec![" \\ | / ", " - O - ", "/ | \\"],
            // WeatherIcon::Cloudy => vec!["   .-.   ", " .-(  )-.", "(   _   )", " `-` `-` "],
            /*
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
            */
        }
    }
}
