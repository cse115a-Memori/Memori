use crate::commands::data::{call_api_json, set_memori_state};
use crate::{AppState, MemoriLayout, MemoriWidget};
use memori_ui::widgets::{Twitch, UpdateFrequency, WidgetId, WidgetKind};
use memori_ui::MemoriState;
use serde::Deserialize;
use serde_json::json;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn init_twitch(state: State<'_, AppState>) -> Result<(), String> {
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Twitch(Twitch::new("No User Data")),
            UpdateFrequency::Seconds(5),
            UpdateFrequency::Never,
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_twitch() -> Result<Box<MemoriWidget>, String> {
    #[derive(Debug, Deserialize)]
    struct Broadcaster {
        broadcaster_type: String,
        created_at: String,
        description: String,
        display_name: String,
        email: Option<String>,
        id: String,
        login: String,
        view_count: u64,
    }
    #[derive(Debug, Deserialize)]
    struct TwitchResponse {
        data: Vec<Broadcaster>,
    }
    // let mut headers = serde_json::Map::new();
    /*
    let client_id = std::env::var("TWITCH_CLIENT_ID")
        .ok()
        .or_else(|| option_env!("TWITCH_CLIENT_ID").map(ToString::to_string))
        .ok_or("TWITCH_CLIENT_ID is not configured".to_string())?;
    let token = "token";
    headers.insert(
        "Authorization".to_string(),
        serde_json::Value::String(format!("Bearer {}", token)),
    );
    headers.insert(
        "Client-ID".to_string(),
        serde_json::Value::String(client_id),
    );
    */
    let args = json!({
        "provider": "twitch",
        "url": "https://api.twitch.tv/helix/users",
        // "headers": serde_json::Value::Object(headers),
    });

    let api_response: TwitchResponse = call_api_json(args).await?;
    println!("api resp: {:?}", api_response);
    let broadcaster = match api_response.data.get(0) {
        Some(first_element) => first_element,
        None => return Err("Twitch response contained no user".to_string()),
    };
    /*
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Twitch(Twitch::new(broadcaster.display_name.clone())),
            UpdateFrequency::Seconds(1),
            UpdateFrequency::Seconds(1),
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await


    let user = Twitch {
        name: String::from("Twitch User Julian"),
    };
    let json = serde_json::to_string(&user);
    // let byte_array = json.as_bytes();
    let mut data: ByteArray = Default::default();
    data.extend_from_slice(json.unwrap().as_bytes()); // .unwrap()// .map_err(|_| format!("err"))
                                                      /*
                                                      if let DeviceConnection::Simulator(conn) = &mut *state_guard {
                                                          return conn
                                                              .refresh_widget(WidgetId(0), data)
                                                              .await
                                                              .map_err(|e| format!("Failed to refresh widget: {e}"));
                                                      }
                                                      */
    // Err("Device is not connected".to_string())
    Ok(data)
    */
    let updated_widget = Box::new(MemoriWidget::new(
        WidgetId(0),
        WidgetKind::Twitch(Twitch::new(broadcaster.display_name.clone())),
        UpdateFrequency::Seconds(5),
        UpdateFrequency::Never,
    ));
    Ok(updated_widget)
}
