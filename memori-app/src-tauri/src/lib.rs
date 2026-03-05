mod commands;
mod oauth;
mod simulator;
mod state;
mod widget_data;

use commands::{
    connect_device, disconnect_device, flash_memori_state, get_battery, get_device_mode,
    get_widget_kinds, is_connected, send_bustime, send_github, send_name, send_temp, send_twitch, test_github,
};
use memori_ui::{layout::MemoriLayout, widgets::MemoriWidget};
use oauth::{login_with_provider, start_oauth_server};
use specta_typescript::{BigIntExportBehavior, Typescript};
use state::AppState;
use tauri_specta::{collect_commands, collect_events, Builder, Event};

// use serde::{Deserialize, Serialize};
// use specta::Type;

// #[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
// pub struct UpdateIsConnected(pub bool);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            connect_device,
            disconnect_device,
            get_device_mode,
            is_connected,
            get_battery,
            get_widget_kinds,
            send_twitch,
            flash_memori_state,
            send_github,
            send_name,
            send_temp,
            send_bustime,
            start_oauth_server,
            login_with_provider,
            test_github,
        ])
        // .events(collect_events![UpdateIsConnected])
        .typ::<MemoriLayout>()
        .typ::<MemoriWidget>();

    #[cfg(all(debug_assertions, not(any(target_os = "ios", target_os = "android"))))]
    builder
        .export(
            Typescript::default().bigint(BigIntExportBehavior::Number),
            "../src/lib/tauri/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_store::Builder::new().build())
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

            // UpdateIsConnected::listen(app, |event| {
            //     println!("hi event: {:?}", event.payload);
            // });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
