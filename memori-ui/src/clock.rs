use alloc::format;
use alloc::string::String;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::text::Text;
use crate::alloc::string::ToString;

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
        if self.hours == 24 {
            self.hours = 0;
        }
    }
}

impl Widget for &Clock{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut hours_string = String::from(self.hours.to_string());        
        let mut minutes_string = String::from(self.minutes.to_string());
        let mut seconds_string = String::from(self.seconds.to_string());
        
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
        Text::from(string).render(area, buf);
    }
}