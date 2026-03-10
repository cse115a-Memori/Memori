mod bus;
mod clock;
mod github;
mod name;
mod pair;
mod twitch;
mod weather;
pub use bus::*;
pub use clock::*;
pub use github::*;
pub use name::*;
pub use pair::*;
pub use twitch::*;
pub use weather::*;

use alloc::vec;
use alloc::vec::Vec;
use ratatui::widgets::Widget;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WidgetId(pub u32);

impl From<u32> for WidgetId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", specta(rename_all = "camelCase"))]
pub struct MemoriWidget {
    pub id: WidgetId,
    pub(crate) kind: WidgetKind,
    remote_update_frequency: UpdateFrequency,
    local_update_frequency: UpdateFrequency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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

    pub fn with_never_update_frequency(id: impl Into<WidgetId>, kind: WidgetKind) -> Self {
        Self {
            id: id.into(),
            kind,
            remote_update_frequency: UpdateFrequency::Never,
            local_update_frequency: UpdateFrequency::Never,
        }
    }

    pub fn with_second_update_frequency(
        id: impl Into<WidgetId>,
        kind: WidgetKind,
        seconds: u32,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            remote_update_frequency: UpdateFrequency::Seconds(seconds),
            local_update_frequency: UpdateFrequency::Seconds(seconds),
        }
    }

    pub fn with_minute_update_frequency(
        id: impl Into<WidgetId>,
        kind: WidgetKind,
        minutes: u32,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            remote_update_frequency: UpdateFrequency::Minutes(minutes),
            local_update_frequency: UpdateFrequency::Minutes(minutes),
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
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum WidgetKind {
    Name(Name),
    Clock(Clock),
    Github(Github),
    Weather(Weather),
    Bus(Bus),
    Twitch(Twitch),
    Pair(Pair),
}

impl WidgetKind {
    pub fn update(&mut self) {
        match self {
            Self::Clock(c) => c.update(),
            Self::Name(n) => n.update(),
            Self::Weather(w) => w.update(),
            Self::Bus(b) => b.update(),
            Self::Twitch(t) => t.update(),
            Self::Github(g) => g.update(),
            Self::Pair(_) => {}
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
            WidgetKind::Twitch(t) => t.render(area, buf),
            WidgetKind::Pair(p) => p.render(area, buf),
        }
    }
}
