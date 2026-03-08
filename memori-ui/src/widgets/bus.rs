use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::cmp::min;
use ratatui::{
    layout::Direction,
    prelude::{Buffer, Line, Rect, Style},
    symbols::border,
    text::Text,
    widgets::{BarChart, Block, Borders, Widget},
};
use serde::{Deserialize, Serialize};

/// Define a widget by its data
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Bus {
    // stop name and either id or km for second element
    pub stop: (String, String),
    pub predictions: Vec<(String, String, u16)>,
}

impl Bus {
    pub fn new(
        stop: impl Into<(String, String)>,
        prediction: impl Into<Vec<(String, String, u16)>>,
    ) -> Self {
        Self {
            stop: stop.into(),
            predictions: prediction.into(),
        }
    }
    pub fn update(&mut self) {
        self.predictions.rotate_right(1);
    }
    pub fn render2(&self, buf: &mut Buffer, num_routes: usize, w: u16) {
        let predictions = self.predictions.clone();
        let bars: Vec<(String, u64)> = predictions
            .into_iter()
            .map(|(label, _, value)| {
                let label2 = format!(" {}  ", label);
                (label2, value as u64)
            })
            .collect();
        let bars: Vec<(&str, u64)> = bars
            .iter()
            .map(|(label, value)| (label.as_str(), *value))
            .collect();

        for i in 0..min(num_routes, bars.len()) {
            let bar = vec![bars[i].clone()];
            let bar_chart = BarChart::default()
                .block(Block::default())
                .data(&bar)
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal);
            bar_chart.render(Rect::new(1, 2 * ((i + 1) as u16), w, 1), buf);
            let t = self.predictions[i].1.clone();
            Text::from(format!(" {}", t))
                .render(Rect::new(1, (2 * ((i + 1) as u16)) + 1, w, 1), buf);
        }
    }
}

// impl the function like this
impl Widget for &Bus {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let t = truncate(self.stop.1.as_str(), (area.width - 2) as usize);
        let bus_block = Block::default()
            .title(Line::from(t).centered())
            .borders(Borders::ALL)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(ratatui::style::Color::White));

        let outer_inner = bus_block.inner(area);
        bus_block.clone().render(area, buf);
        Text::from("Route  min left").render(
            Rect::new(1, 1, outer_inner.width - 1, outer_inner.height - 1),
            buf,
        );
        match (outer_inner.width, outer_inner.height) {
            (w, h) if w < 30 && h < 6 => {
                self.render2(buf, 1, w);
            }
            (w, h) if w < 30 => {
                self.render2(buf, 4, w);
            }
            (w, h) if h < 6 => {
                self.render2(buf, 1, w);
            }
            (w, _) => {
                self.render2(buf, 4, w);
            }
        }
    }
}

fn truncate(title: &str, max_length: usize) -> String {
    if title.len() > max_length {
        let truncated = &title[..max_length];
        truncated.into()
    } else {
        title.to_string()
    }
}
