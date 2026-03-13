use crate::commands::{read_store_state, AuthState};
use memori_ui::widgets::{Twitch, WidgetId, WidgetKind, UpdateFrequency, MemoriWidget};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Deserialize, Serialize)]
pub struct TwitchResponse<T> {
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveStream {
    pub user_name: String,
    pub game_name: String,
    pub title: String,
    pub viewer_count: u32,
}

async fn twitch_get(url: &str, token: &str) -> Result<serde_json::Value, String> {
    let client = Client::new();
    client
        .get(url)
        .bearer_auth(token)
        .header("Client-Id", "halyhdsjvkw9jqbqk5h4s4ryj9hjbk")
        .header("User-Agent", "my-app")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())
}

async fn get_live_streams(userid: &str, token: &str) -> Result<Vec<LiveStream>, String> {
    let url = format!("https://api.twitch.tv/helix/streams/followed?user_id={userid}");
    let data = twitch_get(&url, token).await?;
    let live_streams: TwitchResponse<LiveStream> =
        serde_json::from_value(data).map_err(|e| e.to_string())?;
    Ok(live_streams.data)
}

pub async fn refresh_twitch_widget(app: &AppHandle) -> Result<Twitch, String> {
    let auth: AuthState = read_store_state(app, "auth");
    let twitch_user = auth.users_by_provider.twitch;
    if twitch_user.is_none() { return Ok(Twitch::new("Not logged in", vec![])); }
    let token = twitch_user.as_ref().unwrap().access_token.clone();
    let username = twitch_user.as_ref().unwrap().name.clone();
    let userid = twitch_user.as_ref().unwrap().id.trim_matches('"');
    let live_streams: Vec<LiveStream> = get_live_streams(userid, &token).await?;
    let live_streams_tuples: Vec<(String, String, String, String)> = live_streams
        .iter()
        .take(1)
        .map(|stream| {
            (
                stream.user_name.clone(),
                stream.game_name.clone(),
                stream.title.chars().filter(|c| c.is_ascii()).collect(),
                stream.viewer_count.to_string(),
            )
        })
        .collect();
    Ok(Twitch {
        username,
        live_channels: live_streams_tuples,
    })
}

pub async fn twitch_to_memori_widget(twitch_id: u32, twitch: Twitch) -> Result<MemoriWidget, String> {
    let widget = MemoriWidget {
        id: WidgetId(twitch_id),
        kind: WidgetKind::Twitch(twitch),
        remote_update_frequency: UpdateFrequency::Seconds(30),
        local_update_frequency: UpdateFrequency::Never,
    };
    Ok(widget)
}
