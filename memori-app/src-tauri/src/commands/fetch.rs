use crate::oauth::cloudflare;
use memori_ui::widgets::{Github, Twitch};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;
use std::time::Duration;

const GITHUB_TIMEOUT_SECS: u64 = 3;
const DEFAULT_GITHUB_REPO: &str = "Memori";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GithubUser {
    login: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TwitchUser {
    login: String,
}

pub async fn call_api_json<T>(req_body: serde_json::Value) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let api_res = cloudflare("call_api", req_body)
        .await
        .map_err(|err| format!("cloudflare error: {err}"))?;
    serde_json::from_value(api_res).map_err(|err| err.to_string())
}

/*
pub(crate) async fn fetch_twitch_display_name(token: &str) -> Result<String, String> {
    let client_id = "".to_string();
    let mut headers = serde_json::Map::new();
    headers.insert(
        "Authorization".to_string(),
        serde_json::Value::String(format!("Bearer {token}")),
    );
    headers.insert(
        "Client-ID".to_string(),
        serde_json::Value::String(client_id),
    );

    let req_body = json!({
        "provider": "twitch",
        "url": "https://api.twitch.tv/helix/users",
        "headers": serde_json::Value::Object(headers),
    });
    let twitch_res: TwitchResponse = call_api_json(req_body).await?;
    let broadcaster = twitch_res
        .data
        .first()
        .ok_or("Twitch response contained no user".to_string())?;
    Ok(broadcaster.display_name.clone())
}
*/
pub(crate) async fn fetch_twitch_widget(token: &str) -> Result<Twitch, String> {
    let url = "https://api.twitch.tv/helix/users";
    let client = Client::builder()
        .timeout(Duration::from_secs(GITHUB_TIMEOUT_SECS))
        .build()
        .map_err(|err| err.to_string())?;
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Client-ID", "")
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
    Ok(Twitch::new(user.login, vec![]))
}

pub(crate) async fn fetch_github_widget(token: &str) -> Result<Github, String> {
    let url = "https://api.github.com/user";
    let client = Client::builder()
        .timeout(Duration::from_secs(GITHUB_TIMEOUT_SECS))
        .build()
        .map_err(|err| err.to_string())?;
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "tauri-app")
        .send()
        .await
        .map_err(|err| err.to_string())?
        .error_for_status()
        .map_err(|err| err.to_string())?;
    let user = response
        .json::<GithubUser>()
        .await
        .map_err(|err| err.to_string())?;

    Ok(Github::new(
        user.login,
        Some(DEFAULT_GITHUB_REPO.to_string()),
    ))
}
