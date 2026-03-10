use alloc::{format, string::String, vec, vec::Vec};
use ratatui::{
    layout::{Alignment, Rect},
    prelude::Buffer,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use serde::{Deserialize, Serialize};

/// Define a widget by its data
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Twitch {
    pub username: String,
    pub live_channels: Vec<(String, String, String, String)>,
}

impl Twitch {
    pub fn new(
        username: impl Into<String>,
        // channel, game, title, viewers
        live_channels: impl Into<Vec<(String, String, String, String)>>,
    ) -> Self {
        Self {
            username: username.into(),
            live_channels: live_channels.into(),
        }
    }
    pub fn update(&mut self) {
        self.live_channels.rotate_right(1);
    }
}

impl Default for Twitch {
    fn default() -> Self {
        Self {
            username: String::new(),
            live_channels: Vec::new(),
        }
    }
}

// impl the function like this
impl Widget for &Twitch {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let twitch_block = Block::default()
            .title(
                Line::from(format!("Twitch user {}", self.username))
                    .bold()
                    .centered(),
            )
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        twitch_block.clone().render(area, buf);
        let num_live = self.live_channels.len();
        let (mut p1, mut p2) = (Paragraph::default(), Paragraph::default());
        let live = Span::styled("● LIVE", Style::default().bg(Color::White).fg(Color::Black));
        let outer_inner = twitch_block.inner(area);
        if num_live >= 2 {
            let stream1 = self.live_channels.first().unwrap();
            let streamer1 = Span::from(format!(" {} ", stream1.0));
            let streamer1 = Line::from(vec![live.clone(), streamer1]);
            let game1 = Line::from(format!("playing {}", stream1.1));
            let title1 = Line::from(format!("Title: {}", stream1.2));
            let viewers1 = Line::from(format!("Viewers: {}", stream1.3));
            let temp1 = Text::from(vec![streamer1, game1, title1, viewers1]);
            p1 = Paragraph::new(temp1)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            let stream2 = self.live_channels.get(1).unwrap();
            let streamer2 = Span::from(format!(" {} ", stream2.0));
            let streamer2 = Line::from(vec![live, streamer2]);
            let game2 = Line::from(format!("playing {}", stream2.1));
            let title2 = Line::from(format!("Title: {}", stream2.2));
            let viewers2 = Line::from(format!("Viewers: {}", stream2.3));
            let temp2 = Text::from(vec![streamer2, game2, title2, viewers2]);
            p2 = Paragraph::new(temp2)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
        } else if num_live == 1 {
            let stream = self.live_channels.first().unwrap();
            let streamer = Span::from(format!(" {} ", stream.0));
            let streamer = Line::from(vec![live, streamer]);
            let game = Line::from(format!("playing {}", stream.1));
            let title = Line::from(format!("Title: {}", stream.2));
            let viewers = Line::from(format!("Viewers: {}", stream.3));
            let temp = Text::from(vec![streamer, game, viewers, title]);
            p1 = Paragraph::new(temp)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
        } else {
            Text::from("No channels you follow are currently live").render(outer_inner, buf);
            return;
        }
        match (outer_inner.width, outer_inner.height) {
            (w, h) if w < 30 && h < 6 => {
                // small
                p1.render(outer_inner, buf);
            }
            (w, h) if w < 30 => {
                // tall
                p1.render(Rect::new(outer_inner.x, outer_inner.y, w, h / 2), buf);
                p2.render(Rect::new(outer_inner.x, outer_inner.y + 6, w, h / 2), buf);
            }
            (w, h) if h < 6 => {
                // wide
                p1.render(Rect::new(outer_inner.x, outer_inner.y, w / 2, h), buf);
                p2.render(Rect::new(w / 2, outer_inner.y, w / 2, h), buf);
            }
            (w, h) => {
                // full
                let icon_lines = TwitchIcon::Logo.to_ascii();
                let icon_text =
                    Text::from(icon_lines.into_iter().map(Line::from).collect::<Vec<_>>())
                        .centered();
                let icon = Paragraph::new(icon_text)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                icon.render(Rect::new(outer_inner.x, outer_inner.y, w / 3, h), buf);
                p1.render(Rect::new(w / 2, outer_inner.y, w, h / 2), buf);
                p2.render(Rect::new(w / 2, outer_inner.y + 6, w, h / 2), buf);
            }
        }
    }
}

enum TwitchIcon {
    Logo,
}

impl TwitchIcon {
    pub fn to_ascii(&self) -> Vec<&'static str> {
        match self {
            TwitchIcon::Logo => vec![
                "███████████████",
                "██░░░░░░░░░░░██",
                "██░░░▄▄░░▄▄░░██",
                "██░░░██░░██░░██",
                "██░░░██░░██░░██",
                "██░░░▀▀░░▀▀░░██",
                "██░░░░░░░░░░▄██",
                "████░░░░▄████▀ ",
                "  ██░░▄████▀   ",
                "  ▀▀▀▀▀▀▀▀     ",
            ],
        }
    }
}
