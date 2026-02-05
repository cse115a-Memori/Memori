use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::sync::Mutex;
use alloc::sync::Arc;

pub trait Updatable: Send {
    fn update(&mut self);
}

#[embassy_executor::task]
pub async fn widget_update_task(
    widget: Arc<Mutex<dyn Updatable>>,
    seconds: u64,
) {
    loop {
        widget.lock(|w| w.update());
        Timer::after(Duration::from_secs(seconds)).await;
    }
}