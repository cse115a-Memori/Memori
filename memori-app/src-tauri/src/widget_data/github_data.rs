use memori_ui::widgets::Github;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;
use tauri::State;

use tauri_plugin_store::StoreExt;

fn get_github_token(app: &AppHandle) -> Result<String, String> {
    let store = app.store("auth").map_err(|e| e.to_string())?;
    let token = store
        .get("usersByProvider")
        .and_then(|v| v.get("github"))
        .and_then(|u| u.get("access_token"))
        .and_then(|t| t.as_str())
        .ok_or("No GitHub token found".to_string())?
        .to_string();
    Ok(token)
}

fn get_github_username(app: &AppHandle) -> Result<String, String> {
    let username = store
        .get("usersByProvider")
        .and_then(|v| v.get("github"))
        .and_then(|u| u.get("username"))
        .and_then(|t| t.as_str())
        .ok_or("No GitHub username found".to_string())?
        .to_string();
    Ok(username)
}

fn get_github_repo(app: &AppHandle) -> Result<String, String> {
    let repo = store
        .get("usersByProvider")
        .and_then(|v| v.get("github"))
        .and_then(|u| u.get("repo"))
        .and_then(|t| t.as_str())
        .ok_or("No GitHub repo found".to_string())?
        .to_string();
    Ok(repo)
}

fn get_num_prs(app: &AppHandle) -> Result<u32, String> {
    let token = get_github_token(app)?;
    let owner = get_github_username(app)?;
    let repo = get_github_repo(app)?;
    let client = Client::new();
    let response = client
        .get("https://api.github.com/repos/{owner}/{repo}/pulls?state=open&per_page=100")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    #[derive(Deserialize)]
    struct PullRequest {
        id: u64,
    }

    let prs = response
        .json::<Vec<PullRequest>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(prs.len() as u32)
}

fn get_num_stars(app: &AppHandle) -> Result<u32, String> {
    let token = get_github_token(app)?;
    let owner = get_github_username(app)?;
    let repo = get_github_repo(app)?;
    let client = Client::new();
    let response = client
        .get("https://api.github.com/repos/{owner}/{repo}")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    #[derive(Deserialize)]
    struct Repo {
        stargazers_count: u32,
    }

    let repo = response.json::<Repo>().await.map_err(|e| e.to_string())?;

    Ok(repo.stargazers_count)
}

pub async fn refresh_github_widget(app: &AppHandle) -> memori_ui::widgets::Github {
    let num_prs = get_num_prs(app).await?;
    let num_stars = get_num_stars(app).await?;

    Ok(memori_ui::widgets::Github { num_prs, num_stars })
}
