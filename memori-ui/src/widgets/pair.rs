use alloc::{string::String, vec, format};

use ratatui::{
    layout::{Constraint, HorizontalAlignment},
    style::Style,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Pair {
    code: String,
}

impl Pair {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}
impl Widget for &Pair {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let area = area.centered(Constraint::Ratio(3, 4), Constraint::Ratio(3, 4));

        let text = Text::raw(format!("\n\n{}", self.code.as_str().to_uppercase())).style(Style::new().bold());

        Paragraph::new(text)
            .alignment(HorizontalAlignment::Center)
            .block(
                Block::bordered()
                    .title_top("Pairing Code")
                    .title_alignment(HorizontalAlignment::Center),
            )
            .render(area, buf);
    }
}
