use crate::commands::{read_store_state, AuthState};
use memori_ui::widgets::Twitch;
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
    /*
    let live_streams: Vec<(String, String, String, String)> = twitch_response
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
    */
    Ok(live_streams.data)
}

pub async fn refresh_twitch_widget(app: &AppHandle) -> Result<Twitch, String> {
    println!("Refresh twitch widget called");
    /*
    let auth_users = app
        .svelte()
        .get::<HashMap<String, UserInfo>>("auth", "usersByProvider")
        .unwrap();
    let twitch_user = auth_users
        .get("twitch")
        .ok_or("No Twitch user found".to_string())?;
    */
    let auth: AuthState = read_store_state(app, "auth");
    let twitch_user = auth.users_by_provider.twitch;
    if twitch_user.is_none() {
        println!("twitch user is none");
        return Ok(Twitch::new("Not logged in", vec![]));
    }
    let token = twitch_user.as_ref().unwrap().access_token.clone();
    let userid = twitch_user.as_ref().unwrap().id.trim_matches('"');
    println!("{token}");
    println!("{userid}");
    let live_streams: Vec<LiveStream> = get_live_streams(userid, &token).await?;
    let live_streams_tuples: Vec<(String, String, String, String)> = live_streams
        .iter()
        .map(|stream| {
            (
                stream.user_name.clone(),
                stream.game_name.clone(),
                stream.title.clone(),
                stream.viewer_count.to_string(),
            )
        })
        .collect();
    println!("LIVE STREAMERS RIGHT NOW {:?}", live_streams_tuples);
    Ok(memori_ui::widgets::Twitch {
        username: "".to_string(),
        live_channels: live_streams_tuples,
    })
}
