use crate::oauth::UserInfo;
use crate::widget_data::github_data::GITHUB_TIMEOUT_SECS;
use memori_ui::widgets::Twitch;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tauri::AppHandle;
use tauri_plugin_svelte::ManagerExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct TwitchResponse<T> {
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TwitchUser {
    pub display_name: String,
    pub email: Option<String>,
    pub id: String,
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveChannels {
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

async fn get_live_channels(
    token: &str,
    userid: &str,
) -> Result<Vec<(String, String, String, String)>, String> {
    let url = format!(
        "https://api.twitch.tv/helix/streams/followed?userid={}",
        userid
    );
    let data = twitch_get(&url, token).await?;
    let twitch_response: TwitchResponse<LiveChannels> =
        serde_json::from_value(data).map_err(|e| e.to_string())?;
    let live_channels: Vec<(String, String, String, String)> = twitch_response
        .data
        .iter()
        .map(|channel| {
            (
                channel.user_name.clone(),
                channel.game_name.clone(),
                channel.title.clone(),
                channel.viewer_count.to_string(),
            )
        })
        .collect();
    Ok(live_channels)
}

pub async fn refresh_twitch_widget(app: &AppHandle) -> Result<Twitch, String> {
    let auth_users = app
        .svelte()
        .get::<HashMap<String, UserInfo>>("auth", "usersByProvider")
        .unwrap();
    let twitch_user = auth_users
        .get("twitch")
        .ok_or("No Twitch user found".to_string())?;
    let token = &twitch_user.access_token;
    let userid = &twitch_user.id;
    let live_channels = get_live_channels(&token, &userid).await?;
    Ok(memori_ui::widgets::Twitch {
        username: "".to_string(),
        live_channels: live_channels,
    })
}

pub(crate) async fn fetch_twitch_widget(token: &str) -> Result<Twitch, String> {
    let url = "https://api.twitch.tv/helix/users";
    let client = Client::builder()
        .timeout(Duration::from_secs(GITHUB_TIMEOUT_SECS))
        .build()
        .map_err(|err| err.to_string())?;
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/json")
        .header("User-Agent", "tauri-app")
        .send()
        .await
        .map_err(|err| err.to_string())?
        .error_for_status()
        .map_err(|err| err.to_string())?;
    let user = response
        .json::<TwitchUser>()
        .await
        .map_err(|err| err.to_string())?;
    Ok(Twitch::new(user.display_name, vec![]))
}
