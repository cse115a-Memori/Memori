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
    update_frequency: UpdateFrequency,
    local_update_frequency: UpdateFrequency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UpdateFrequency {
    Never,
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

impl UpdateFrequency {
    pub fn to_seconds(&self) -> Option<u32> {
        match self {
            Self::Never => None,
            Self::Seconds(s) => Some(*s),
            Self::Minutes(m) => Some(m * 60),
            Self::Hours(h) => Some(h * 3600),
        }
    }
}

impl MemoriWidget {
    pub fn new(id: WidgetId, kind: WidgetKind, update_frequency: UpdateFrequency, local_update_frequency: UpdateFrequency) -> Self {
        Self {
            id,
            kind,
            update_frequency,
            local_update_frequency,
        }
    }
}

impl MemoriWidget {
    pub fn update(&mut self) {
        self.kind.update();
    }
}

impl MemoriWidget {
    pub fn get_update_frequency(&self) -> UpdateFrequency {
        self.update_frequency
    }
    
    pub fn get_local_update_frequency(&self) -> UpdateFrequency {
        self.local_update_frequency
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WidgetKind {
    Name(Name),
    Clock(Clock),
}

impl WidgetKind {
    pub fn update(&mut self) {
        match self {
            Self::Clock(c) => c.update(),
            Self::Name(n) => n.update(),
            _ => {}
        }
    }
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
