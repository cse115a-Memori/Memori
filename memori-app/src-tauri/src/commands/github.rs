use crate::commands::data::{call_api_json, set_memori_state};
use crate::{AppState, MemoriLayout, MemoriWidget};
use memori_ui::widgets::{Github, UpdateFrequency, WidgetId, WidgetKind};
use memori_ui::MemoriState;
use serde::Deserialize;
use serde_json::json;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn send_github() -> Result<Box<MemoriWidget>, String> {
    /*
    let url = "https://api.github.com/user";
    let client = Client::new();
    let token = "";
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "tauri-app")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let response = response.error_for_status().map_err(|e| e.to_string())?;
    let user_info: serde_json::Value = response.json().await.map_err(|err| err.to_string())?;
    let _ = user_info;
    */
    #[derive(Deserialize, Debug)]
    struct User {
        login: String,
        id: u64,
        node_id: String,
        avatar_url: String,
        gravatar_id: String,
        url: String,
        html_url: String,
        followers_url: String,
        following_url: String,
        gists_url: String,
        starred_url: String,
        subscriptions_url: String,
        organizations_url: String,
        repos_url: String,
        events_url: String,
        received_events_url: String,
        #[serde(rename = "type")]
        user_type: String,
        user_view_type: String,
        site_admin: bool,
        name: Option<String>,
        company: Option<String>,
        blog: String,
        location: Option<String>,
        email: Option<String>,
        hireable: Option<bool>,
        bio: Option<String>,
        twitter_username: Option<String>,
        notification_email: Option<String>,
        public_repos: u64,
        public_gists: u64,
        followers: u64,
        following: u64,
        created_at: String,
        updated_at: String,
        private_gists: u64,
        total_private_repos: u64,
        owned_private_repos: u64,
        disk_usage: u64,
        collaborators: u64,
        two_factor_authentication: bool,
        plan: Plan,
    }

    #[derive(Deserialize, Debug)]
    struct Plan {
        name: String,
        space: u64,
        collaborators: u64,
        private_repos: u64,
    }
    let args = json!({
        "provider": "github",
        "url": "https://api.github.com/user",
        // "headers": serde_json::Value::Object(headers),
    });

    let api_response: User = call_api_json(args).await?;
    let updated_widget = Box::new(MemoriWidget::new(
        WidgetId(0),
        WidgetKind::Github(Github::new(api_response.login, None)),
        UpdateFrequency::Never,
        UpdateFrequency::Never,
    ));
    Ok(updated_widget)
}

#[tauri::command]
#[specta::specta]
pub async fn init_github(state: State<'_, AppState>) -> Result<(), String> {
    let memori_state = MemoriState::new(
        0,
        vec![MemoriWidget::new(
            WidgetId(0),
            WidgetKind::Github(Github::new("TEMP".to_string(), None)),
            UpdateFrequency::Seconds(5),
            UpdateFrequency::Never,
        )],
        vec![MemoriLayout::Full(WidgetId(0))],
        5,
    );
    set_memori_state(&state, memori_state).await
}
