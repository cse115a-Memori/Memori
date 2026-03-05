use memori_ui::widgets::Clock;
use chrono::{Local, Timelike};

pub async fn refresh_clock_widget() -> Result<Clock, String> {
    Ok(Clock {
        seconds: Local::now().second() as u32,
        minutes: Local::now().minute() as u32,
        hours: Local::now().hour() as u32,
    })
}