mod clock;
mod name;
pub use clock::*;
pub use name::*;

use ratatui::widgets::Widget;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct WidgetId(pub u32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MemoriWidget {
    pub(crate) id: WidgetId,
    pub(crate) kind: WidgetKind,
}

impl MemoriWidget {
    pub fn new(id: WidgetId, kind: WidgetKind) -> Self {
        Self { id, kind }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WidgetKind {
    Name(Name),
    Clock(Clock),
}

impl Widget for &MemoriWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match &self.kind {
            WidgetKind::Name(n) => n.render(area, buf),
            WidgetKind::Clock(c) => c.render(area, buf),
        }
    }
}
