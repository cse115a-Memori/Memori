mod bus;
mod clock;
mod name;
mod weather;
pub use bus::*;
pub use clock::*;
pub use name::*;
pub use weather::*;

use ratatui::widgets::Widget;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct WidgetId(pub u32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MemoriWidget {
    pub id: WidgetId,
    pub(crate) kind: WidgetKind,
    pub update_frequency: Option<UpdateFrequency>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UpdateFrequency {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

impl UpdateFrequency {
    pub fn to_seconds(&self) -> Option<u32> {
        match self {
            Self::Seconds(s) => Some(*s),
            Self::Minutes(m) => Some(m * 60),
            Self::Hours(h) => Some(h * 3600),
        }
    }
}

impl MemoriWidget {
    pub fn new(id: WidgetId, kind: WidgetKind, update_frequency: Option<UpdateFrequency>) -> Self {
        Self {
            id,
            kind,
            update_frequency,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WidgetKind {
    Name(Name),
    Clock(Clock),
    Weather(Weather),
    Bus(Bus),
}

impl Widget for &MemoriWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match &self.kind {
            WidgetKind::Name(n) => n.render(area, buf),
            WidgetKind::Clock(c) => c.render(area, buf),
            WidgetKind::Weather(w) => w.render(area, buf),
            WidgetKind::Bus(b) => b.render(area, buf),
        }
    }
}
