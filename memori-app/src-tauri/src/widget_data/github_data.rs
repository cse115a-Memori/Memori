use reqwest::Client;
use serde::Deserialize;
use chrono::{Local, Datelike};
use tauri::AppHandle;
use tauri_plugin_svelte::ManagerExt;
use crate::oauth::UserInfo;
use std::collections::HashMap;
use memori_ui::widgets::Github;

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
        .header("User-Agent", "my-app")
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
        .header("User-Agent", "my-app")
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

async fn get_num_issues(token: &str, username: &str, repo: &str) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=repo:{}/{}+type:issue+state:open&per_page=1",
        username, repo
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
    
    let status = response.status();
    if !status.is_success() {
        return Err(format!("Failed to fetch issues: status {}", status));
    }
    
    let result = response.json::<SearchResult>().await.map_err(|e| e.to_string())?;
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

    for i in (1..=7).rev() {
        let since = (Local::now() - chrono::Duration::days(i))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        let until = (Local::now() - chrono::Duration::days(i - 1))
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

pub async fn refresh_github_widget(app: &AppHandle) -> Result<Github, String> {
    let auth_users = app.svelte().get::<HashMap<String, UserInfo>>("auth", "usersByProvider").unwrap();
    let github_user = auth_users.get("github").ok_or("No GitHub user found".to_string())?;
    
    let token = &github_user.access_token;
    let username = &github_user.name;
    let repo = "Memori".to_string();
    let owner = "cse115a-Memori";
    
    // Call each async function concurrently
    let (open_issues, open_prs, stars, notifications, commits) = tokio::join!(
        get_num_issues(&token, &owner, &repo),
        get_num_prs(&token, &owner, &repo),
        get_num_stars(&token, &owner, &repo),
        get_num_notifications(&token),
        get_commit_frequency(&token, &owner, &repo),
    );
    
    Ok(memori_ui::widgets::Github {
        username: username.clone(),
        repo: repo.clone(),
        open_issues: open_issues?,
        open_prs: open_prs?,
        stars: stars?,
        notifications: notifications?,
        commits: commits?,
        weekday: Local::now().weekday().num_days_from_sunday() as usize,
    })
}
