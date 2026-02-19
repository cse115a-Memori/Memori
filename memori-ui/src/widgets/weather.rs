use alloc::{format, string::String, vec, vec::Vec};
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
pub struct Weather {
    pub temp: String,
    pub icon: String,
}

impl Weather {
    pub fn new(temp: impl Into<String>) -> Self {
        Self {
            temp: temp.into(),
            icon: "placeholder text".into(),
        }
    }
    pub fn update(&mut self) {
        info!("Updated name");
    }
}

// impl the function like this
impl Widget for &Weather {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let weather_block = Block::default()
            .title(Line::from(" Weather ").bold().centered())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        weather_block.clone().render(area, buf);

        let icon_lines = WeatherIcon::Sun.to_ascii();

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
