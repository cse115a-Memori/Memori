mod bus;
mod clock;
mod name;
mod weather;
mod github;
pub use bus::*;
pub use clock::*;
pub use name::*;
pub use weather::*;
pub use github::*;

use ratatui::widgets::Widget;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct WidgetId(pub u32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MemoriWidget {
    pub id: WidgetId,
    pub(crate) kind: WidgetKind,
    remote_update_frequency: UpdateFrequency,
    local_update_frequency: UpdateFrequency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UpdateFrequency {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Never,
}

impl UpdateFrequency {
    pub fn to_seconds(&self) -> Option<u32> {
        match self {
            Self::Seconds(s) => Some(*s),
            Self::Minutes(m) => Some(m * 60),
            Self::Hours(h) => Some(h * 3600),
            Self::Never => None,
        }
    }
}

impl MemoriWidget {
    pub fn new(
        id: WidgetId,
        kind: WidgetKind,
        remote_update_frequency: UpdateFrequency,
        local_update_frequency: UpdateFrequency,
    ) -> Self {
        Self {
            id,
            kind,
            remote_update_frequency,
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
    pub fn get_remote_update_frequency(&self) -> UpdateFrequency {
        self.remote_update_frequency
    }

    pub fn get_local_update_frequency(&self) -> UpdateFrequency {
        self.local_update_frequency
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WidgetKind {
    Name(Name),
    Clock(Clock),
    Github(Github),
    Weather(Weather),
    Bus(Bus),
}

impl WidgetKind {
    pub fn update(&mut self) {
        match self {
            Self::Clock(c) => c.update(),
            Self::Name(n) => n.update(),
            Self::Weather(w) => w.update(),
            Self::Bus(b) => b.update(),
            Self::Github(g) => g.update(),
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
            WidgetKind::Github(g) => g.render(area, buf),
            WidgetKind::Weather(w) => w.render(area, buf),
            WidgetKind::Bus(b) => b.render(area, buf),
        }
    }
}
