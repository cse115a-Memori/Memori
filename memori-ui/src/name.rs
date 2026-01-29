use alloc::format;
use alloc::string::String;
use ratatui::{
    text::Text,
    widgets::{StatefulWidget, Widget},
};

/// Define a widget by its data
pub struct Name {
    pub name: String,
}

// impl the function like this
impl Widget for &Name {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let string = format!("Hello {}!", self.name);
        Text::from(string).render(area, buf);
    }
}
