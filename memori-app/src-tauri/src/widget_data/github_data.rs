use reqwest::Client;
use serde::Deserialize;
use chrono::{Local, Datelike};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;


async fn get_num_prs(
    token: &str,
    username: &str,
    repo: &str,
) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls?state=open&per_page=100",
        username, repo
    );
    let response = client
        .get(&url)
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

async fn get_num_stars(
    token: &str,
    username: &str,
    repo: &str,
) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}",
        username, repo
    );
    let response = client
        .get(&url)
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

async fn get_num_issues(token: &str, username: &str) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=is:open+is:issue+author:{}",
        username
    );
    let response = client
        .get(&url)
        .bearer_auth(token)
        .header("User-Agent", "my-app")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    #[derive(Deserialize)]
    struct SearchResult {
        total_count: u32,
    }

    let result = response
        .json::<SearchResult>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.total_count)
}

async fn get_num_notifications(token: &str) -> Result<u32, String> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/notifications")
        .bearer_auth(token)
        .header("User-Agent", "my-app")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let notifications = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;
    let count = notifications
        .as_array()
        .map(|a| a.len() as u32)
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_commit_frequency(
    token: &str,
    owner: &str,
    repo: &str,
) -> Result<[u32; 7], String> {
    let client = Client::new();
    let mut commits_per_day = Vec::new();

    for i in 0..7 {
        let since = (Local::now() - chrono::Duration::days(i + 1))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        let until = (Local::now() - chrono::Duration::days(i))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        let url = format!(
            "https://api.github.com/repos/{}/{}/commits?since={}&until={}",
            owner, repo, since, until
        );
        let response = client
            .get(&url)
            .bearer_auth(&token)
            .header("User-Agent", "my-app")
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let commits = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let count = commits.as_array().map(|a| a.len() as u32).unwrap_or(0);
        commits_per_day.push(count);
    }
    
    let commits_arr: [u32; 7] = commits_per_day.try_into().unwrap();
    
    Ok(commits_arr)
}

pub async fn refresh_github_widget(app: &AppHandle) -> Result<memori_ui::widgets::Github, String> {
    let store = app.store("auth").map_err(|e| e.to_string())?;
    let users = store.get("usersByProvider").ok_or("No authenticated users found")?;
    let github_user = users["github"].as_object().ok_or("No GitHub user found".to_string())?;
    
    let token = github_user["accessToken"].as_str().ok_or("No access token found".to_string())?.to_string();
    let username = github_user["username"].as_str().ok_or("No username found".to_string())?.to_string();
    let repo = "Memori".to_string();
    
    Ok(memori_ui::widgets::Github {
        username: username.clone(),
        repo: repo.clone(),
        open_issues: get_num_issues(&token, &username).await?,
        open_prs: get_num_prs(&token, &username, &repo).await?,
        stars: get_num_stars(&token, &username, &repo).await?,
        notifications: get_num_notifications(&token).await?,
        commits: get_commit_frequency(&token, &username, &repo).await?,
        weekday: Local::now().weekday().num_days_from_sunday() as usize,
    })
}
