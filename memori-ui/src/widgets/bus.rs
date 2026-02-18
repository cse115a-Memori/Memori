use alloc::format;
use alloc::string::String;
use ratatui::{text::Text, widgets::Widget};
use serde::{Deserialize, Serialize};

/// Define a widget by its data
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Bus {
    pub route: String,
    pub prediction: String,
}

impl Bus {
    pub fn new(prediction: impl Into<String>, route: impl Into<String>) -> Self {
        Self {
            route: route.into(),
            prediction: prediction.into(),
        }
    }
}

// impl the function like this
impl Widget for &Bus {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let string = format!("prediction: {}\nroute: {}", self.prediction, self.route);
        Text::from(string).render(area, buf);
    }
}
