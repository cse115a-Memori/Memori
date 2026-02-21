mod commands;
mod oauth;
mod simulator;
mod state;

use commands::{
    connect_device, disconnect_device, get_battery, get_device_mode, get_widget_kinds, hello,
    is_connected, send_bustime, send_name, send_temp, send_twitch,
};
use memori_ui::{layout::MemoriLayout, widgets::MemoriWidget};
use oauth::{login_with_provider, start_oauth_server};
use specta_typescript::Typescript;
use state::AppState;
use tauri_specta::{collect_commands, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            hello,
            connect_device,
            disconnect_device,
            get_device_mode,
            is_connected,
            get_battery,
            get_widget_kinds,
            send_twitch,
            send_name,
            send_temp,
            send_bustime,
            start_oauth_server,
            login_with_provider,
        ])
        .typ::<MemoriLayout>()
        .typ::<MemoriWidget>();

    #[cfg(all(debug_assertions, not(any(target_os = "ios", target_os = "android"))))]
    builder
        .export(Typescript::default(), "../src/lib/tauri/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_svelte::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_oauth::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
