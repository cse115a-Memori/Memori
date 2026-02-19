use crate::simulator::request_handler;
use crate::state::{AppState, DeviceConnection, DeviceMode};
use ble_host::HostBLETransport;
use memori_tcp::HostTcpTransport;
use tauri::State;
use transport::HostTransport as _;

#[tauri::command]
#[specta::specta]
pub async fn connect_device(state: State<'_, AppState>, mode: DeviceMode) -> Result<(), String> {
    let mut guard = state.conn.lock().await;

    if !matches!(*guard, DeviceConnection::Disconnected) {
        return Err("Already connected. Disconnect first.".to_string());
    }

    match mode {
        DeviceMode::RealDevice => {
            let conn = HostBLETransport::connect()
                .await
                .map_err(|e| format!("Failed to connect to device: {e}"))?;

            *guard = DeviceConnection::RealDevice(conn);
            println!("Connected to real device over Bluetooth");
            Ok(())
        }
        DeviceMode::Simulator => {
            let transport = HostTcpTransport::default();
            let (conn, (dev_req_rx, host_resp_tx)) = transport
                .connect()
                .await
                .map_err(|e| format!("Failed to connect to simulator: {e}"))?;

            *guard = DeviceConnection::Simulator(conn);

            tokio::spawn(async move {
                request_handler(dev_req_rx, host_resp_tx).await;
            });

            println!("Connected to simulator over TCP");
            Ok(())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn disconnect_device(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.conn.lock().await;

    let old_connection = std::mem::replace(&mut *guard, DeviceConnection::Disconnected);

    match old_connection {
        DeviceConnection::RealDevice(transport) => {
            transport.disconnect().await;
        }
        DeviceConnection::Simulator(transport) => {
            transport.disconnect();
        }
        DeviceConnection::Disconnected => {}
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_device_mode(state: State<'_, AppState>) -> Result<Option<DeviceMode>, String> {
    let guard = state.conn.lock().await;
    Ok(match *guard {
        DeviceConnection::RealDevice(_) => Some(DeviceMode::RealDevice),
        DeviceConnection::Simulator(_) => Some(DeviceMode::Simulator),
        DeviceConnection::Disconnected => None,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn is_connected(state: State<'_, AppState>) -> Result<bool, String> {
    let guard = state.conn.lock().await;
    Ok(!matches!(*guard, DeviceConnection::Disconnected))
}

#[tauri::command]
#[specta::specta]
pub async fn get_battery(state: State<'_, AppState>) -> Result<u8, String> {
    let mut guard = state.conn.lock().await;

    match &mut *guard {
        DeviceConnection::RealDevice(transport) => transport
            .get_battery_level()
            .await
            .map_err(|e| format!("Failed to get battery: {e}")),
        DeviceConnection::Simulator(transport) => transport
            .get_battery_level()
            .await
            .map_err(|e| format!("Failed to get battery: {e}")),
        DeviceConnection::Disconnected => Err("Device is not connected".to_string()),
    }
}
