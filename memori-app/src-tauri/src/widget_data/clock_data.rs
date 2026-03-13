use memori_ui::widgets::Clock;
use chrono::{Local, Timelike};
use memori_ui::widgets::{WidgetId, UpdateFrequency};

pub async fn refresh_clock_widget() -> Result<Clock, String> {
    Ok(Clock {
        seconds: Local::now().second() as u32,
        minutes: Local::now().minute() as u32,
        hours: Local::now().hour() as u32,
    })
}

pub async fn clock_to_memori_widget(id: u32, clock: Clock) -> Result<memori_ui::widgets::MemoriWidget, String> {
    Ok(memori_ui::widgets::MemoriWidget {
        id: WidgetId(id),
        kind: memori_ui::widgets::WidgetKind::Clock(clock),
        remote_update_frequency: memori_ui::widgets::UpdateFrequency::Hours(1),
        local_update_frequency: memori_ui::widgets::UpdateFrequency::Seconds(1),
    })
}