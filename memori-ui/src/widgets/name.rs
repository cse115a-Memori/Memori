use alloc::format;
use alloc::string::String;
use ratatui::{text::Text, widgets::Widget};
use serde::{Deserialize, Serialize};

/// Define a widget by its data
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

// impl the function like this
impl Widget for &Name {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let string = format!("Hello {}!", self.name);
        Text::from(string).render(area, buf);
    }
}
