use core::sync::atomic::{AtomicU32, Ordering};

use alloc::vec::Vec;
use ble_device::DeviceBLETransport;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};
use log::{error, info};
use memori_ui::{MemoriState, widgets::MemoriWidget};
use transport::{DeviceTransport, TransError, ble_types::*};
use trouble_host::prelude::*;

use crate::ble::{Server, send_packet};

/// Keeps track of the generation of refresh tasks, basically
/// a way to tell older tasks to die when we update the state with
/// new update-able widgets.
static REFRESH_GENERATION: AtomicU32 = AtomicU32::new(0);

/// Act on any host commands.
pub(super) async fn handle_host_cmd<P: PacketPool>(
    cmd: HostBLECommand,
    msg_id: MessageID,
    server: &Server<'_>,
    state: &'static Mutex<CriticalSectionRawMutex, MemoriState>,
    transport: &'static Mutex<CriticalSectionRawMutex, DeviceBLETransport>,
    spawner: Spawner,
    conn: &GattConnection<'_, '_, P>,
) {
    info!("[transport] received cmd {:?}", cmd);

    let mut state_guard = state.lock().await;
    let mem_state = &mut *state_guard;
    let resp = match cmd {
        HostBLECommand::GetWidget { widget_id } => DeviceBLEResponse::WidgetGet {
            result: mem_state
                .widgets
                .get(&widget_id)
                .cloned()
                .ok_or(TransError::WidgetNotFound),
        },
        HostBLECommand::SetState { state: new_state } => {
            *mem_state = new_state;

            let current_gen = REFRESH_GENERATION.load(Ordering::Relaxed);
            let new_gen = current_gen.wrapping_add(1);
            REFRESH_GENERATION.store(new_gen, Ordering::Relaxed);

            let widgets_needing_refresh = mem_state
                .widgets
                .iter()
                // Filter for widgets that have an update frequency
                .filter_map(|(_, w)| {
                    if w.update_frequency.is_some() {
                        Some(w)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for widget in widgets_needing_refresh {
                let _ = spawner
                    .spawn(refresh_widget_task(widget.clone(), transport, state,new_gen))
                    .inspect_err(|e| error!("Error with spawning refresh task: {e:#?}, aborting spawning refresh for this task, may not work as intended."));
            }

            DeviceBLEResponse::SetState { result: Ok(()) }
        }
        HostBLECommand::SetConfig { config: _ } => {
            todo!()
        }
    };

    // Release mutex as soon as possible.
    drop(state_guard);

    let pkt = DeviceBLEPacket::Response(resp);

    let _ = send_packet(pkt, msg_id, server, conn).await;
}

/// Refreshes widget data from the host on the interval specified in the widget.
#[embassy_executor::task]
async fn refresh_widget_task(
    widget: MemoriWidget,
    transport: &'static Mutex<CriticalSectionRawMutex, DeviceBLETransport>,
    state: &'static Mutex<CriticalSectionRawMutex, MemoriState>,
    my_generation: u32,
) {
    // Watches for the cancellation watch to be updated, and returns when it does so.
    let Some(wait_period) = widget
        .update_frequency
        .map(|f| f.to_seconds().expect("Should convert to seconds"))
    else {
        // Called this function on a widget that doesn't have an update frequency, just return.
        return;
    };

    loop {
        Timer::after(Duration::from_secs(wait_period.into())).await;

        // If the generation of tasks has passed the generation for this one, we just kill ourself lol.
        if REFRESH_GENERATION.load(Ordering::Relaxed) != my_generation {
            info!("Generation increased! killing myself!");
            return;
        }

        let widget_id = widget.id;

        let mut transport = transport.lock().await;

        let Ok(data) = transport
            .refresh_data(widget_id)
            .await
            .inspect_err(|e| error!("Failed to refresh data for widget: {e:#?}"))
        else {
            continue;
        };

        // Drop guard as soon as possible.
        drop(transport);

        let mut state = state.lock().await;
        state.widgets.insert(widget_id, data);
    }
}
