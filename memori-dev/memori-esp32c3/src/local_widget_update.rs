use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use memori_ui::MemoriState;
use memori_ui::widgets::WidgetId;
use log::info;

pub trait Updatable: Send {
    fn update(&mut self);
}

#[embassy_executor::task]
pub async fn widget_update_task(
    state: &'static Mutex<CriticalSectionRawMutex, MemoriState>,
    widget_id: WidgetId,
    seconds: u64,
) {
    loop {  // This loop runs FOREVER
        {
            let mut locked_state = state.lock().await;
            if let Some(widget) = locked_state.widgets.get_mut(&widget_id) {
                widget.update();
            }
        }
        Timer::after(Duration::from_secs(seconds)).await;
    }
}