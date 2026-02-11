use crate::alloc::string::ToString;
use alloc::format;
use alloc::vec;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Widget};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Type)]
pub struct Clock {
    pub seconds: u32,
    pub minutes: u32,
    pub hours: u32,
}

impl Default for Clock {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Clock {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Clock {
            seconds: seconds,
            minutes: minutes,
            hours: hours,
        }
    }

    pub fn update(&mut self) {
        self.seconds += 1;
        if self.seconds == 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
        if self.minutes == 60 {
            self.minutes = 0;
            self.hours += 1;
        }
        if self.hours == 13 {
            self.hours = 1;
        }
    }
}

impl Widget for &Clock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut hours_string = self.hours.to_string();
        let mut minutes_string = self.minutes.to_string();
        let mut seconds_string = self.seconds.to_string();

        if self.hours < 10 {
            hours_string = format!("0{}", self.hours);
        }
        if self.minutes < 10 {
            minutes_string = format!("0{}", self.minutes);
        }
        if self.seconds < 10 {
            seconds_string = format!("0{}", self.seconds);
        }

        let string = format!("{}:{}:{}", hours_string, minutes_string, seconds_string);

        let border_set = border::Set {
            top_left: "+",
            top_right: "+",
            bottom_left: "+",
            bottom_right: "+",
            vertical_left: "|",
            vertical_right: "|",
            horizontal_top: "=",
            horizontal_bottom: "=",
        };

        // Render the block with borders
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border_set);
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Calculate center position
        let text_len = string.len() as u16;
        let center_x = inner_area.x + (inner_area.width.saturating_sub(text_len)) / 2;
        let center_y = inner_area.y + inner_area.height / 2;

        // Render the centered text
        buf.set_string(center_x, center_y, string, Style::default());
    }
}
