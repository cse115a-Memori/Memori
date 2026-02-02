use alloc::format;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::StatefulWidget;

pub struct Clock {
    pub seconds: u32,
    pub minutes: u32,
    pub hours: u32,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            seconds: 0,
            minutes: 0,
            hours: 0,
        }
    }

    pub fn tick(&mut self) {
        self.seconds += 1;
        if self.seconds == 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
        if self.minutes == 60 {
            self.minutes = 0;
            self.hours += 1;
        }
    }
}

pub struct TimeWidget;

impl StatefulWidget for TimeWidget {
    type State = Clock;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.hours < 10 || state.minutes < 10 || state.seconds < 10 {
            let string = format!("0{}:0{}:0{}", state.hours, state.minutes, state.seconds);
            buf.set_string(area.x, area.y, string, Style::default());
        } else {
            let string = format!("{}:{}:{}", state.hours, state.minutes, state.seconds);
            buf.set_string(area.x, area.y, string, Style::default());
        }
    }
}
