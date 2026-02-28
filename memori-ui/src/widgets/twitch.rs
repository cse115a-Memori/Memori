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
#[cfg(feature = "specta")]
use specta::Type;

/// Define a widget by its data
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct Twitch {
    pub user: String,
}

impl Twitch {
    pub fn new(user: impl Into<String>) -> Self {
        Self { user: user.into() }
    }
    pub fn update(&mut self) {
        info!("Updated name");
    }
}

// impl the function like this
impl Widget for &Twitch {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let twitch_block = Block::default()
            .title(Line::from(" Weather ").bold().centered())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        twitch_block.clone().render(area, buf);

        let icon_lines = TwitchIcon::Logo.to_ascii();

        let text =
            Text::from(icon_lines.into_iter().map(Line::from).collect::<Vec<_>>()).centered();

        let icon_paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let user_text = format!("User: {}", self.user);
        let user_paragraph = Paragraph::new(Line::from(user_text))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let inner_area = twitch_block.inner(area);
        let left_width = inner_area.width / 3;
        let right_width = inner_area.width - left_width;

        let icon_area = Rect::new(inner_area.x, inner_area.y, left_width, inner_area.height);
        let userinfo_area = Rect::new(
            inner_area.x + left_width,
            inner_area.y,
            right_width,
            inner_area.height,
        );

        icon_paragraph.render(icon_area, buf);
        user_paragraph.render(userinfo_area, buf);
    }
}

enum TwitchIcon {
    Logo,
}

impl TwitchIcon {
    fn to_ascii(&self) -> Vec<&'static str> {
        match self {
            TwitchIcon::Logo => vec![
                "░░░███████████████████████████████████████",
                "░▄████████████████████████████████████████",
                "▄██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█████",
                "███████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█████",
                "███████░░░░░░░░░▄▄▄▄▄░░░░░░░▄▄▄▄░░░░░█████",
                "███████░░░░░░░░░█████░░░░░░░████░░░░░█████",
                "███████░░░░░░░░░█████░░░░░░░████░░░░░█████",
                "███████░░░░░░░░░█████░░░░░░░████░░░░░█████",
                "███████░░░░░░░░░█████░░░░░░░████░░░░░█████",
                "███████░░░░░░░░░▀▀▀▀▀░░░░░░░▀▀▀▀░░░░░█████",
                "███████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▄█████",
                "███████░░░░░░░░░░░░░░░░░░░░░░░░░░▄██████▀",
                "███████░░░░░░░░░░░░░░░░░░░░░░░░▄█████▀▀",
                "██████████████░░░░░▄███████████████▀",
                "██████████████░░▄███████████████▀▀",
                "▀▀▀▀▀▀▀▀▀▀████████████▀▀▀▀▀▀▀▀▀",
                "░░░░░░░░░░█████████▀▀",
                "░░░░░░░░░░███████▀",
            ],
        }
    }
}
