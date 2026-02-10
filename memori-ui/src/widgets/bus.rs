use alloc::format;
use alloc::string::String;
use ratatui::{text::Text, widgets::Widget};
use serde::{Deserialize, Serialize};

/// Define a widget by its data
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Bus {
    pub string: String,
}

impl Bus {
    pub fn new(string: impl Into<String>) -> Self {
        Self { string: string.into() }
    }
}

// impl the function like this
impl Widget for &Bus {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let string = format!("Hello {}!", self.string);
        Text::from(string).render(area, buf);
    }
}
