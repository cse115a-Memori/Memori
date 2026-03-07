use reqwest::Client;
use chrono::{Local, Datelike};
use tauri::AppHandle;
use tauri_plugin_svelte::ManagerExt;
use crate::oauth::UserInfo;
use std::collections::HashMap;
use memori_ui::widgets::Github;
use crate::commands::data::AuthState;

async fn github_get(client: &Client, url: &str, token: &str) -> Result<serde_json::Value, String> {
    client
        .get(url)
        .bearer_auth(token)
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

async fn get_num_prs(
    token: &str,
    username: &str,
    repo: &str,
) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=repo:{}/{}+type:pr+state:open&per_page=1",
        username, repo
    );
    let data = github_get(&client, &url, token).await?;
    let count = data["total_count"].as_u64().ok_or_else(|| "total_count not found".to_string())? as u32;

    Ok(count)
}

async fn get_num_stars(token: &str, username: &str, repo: &str) -> Result<u32, String> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}", username, repo);
    let data = github_get(&client, &url, token).await?;
    let stars = data["stargazers_count"].as_u64().unwrap_or(0) as u32;
    Ok(stars)
}

async fn get_num_issues(token: &str, username: &str, repo: &str) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=repo:{}/{}+type:issue+state:open&per_page=1",
        username, repo
    );
    let data = github_get(&client, &url, token).await?;
    let count = data["total_count"].as_u64().unwrap_or(0) as u32;
    Ok(count)
}


async fn get_num_notifications(token: &str) -> Result<u32, String> {
    let client = Client::new();
    let data = github_get(&client, "https://api.github.com/notifications", token).await?;
    let count = data.as_array().map(|a| a.len() as u32).unwrap_or(0);
    Ok(count)
}

async fn get_commit_frequency(token: &str, owner: &str, repo: &str) -> Result<[u32; 7], String> {
    let mut commits_per_day = Vec::new();
    let client = Client::new();

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

        let data = github_get(&client, &url, token).await?;
        let count = data.as_array().map(|a| a.len() as u32).unwrap_or(0);
        commits_per_day.push(count);
    }

    let commits_arr: [u32; 7] = commits_per_day.try_into().unwrap();
    Ok(commits_arr)
}

pub async fn refresh_github_widget(auth_store: &AuthState) -> Result<Github, String> {
    println!("Refresh github widget called");
    let github_user = &auth_store.users_by_provider.github;
    
    if github_user.is_none() {
        println!("Github user is none");
        return Ok(Github::new("Not logged in...".to_string(), None))
    }
    
    let token = &github_user.as_ref().unwrap().access_token;
    let username = &github_user.as_ref().unwrap().name;
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
    
    println!("Refresh github widget done");
    
    Ok(memori_ui::widgets::Github {
        username: username.clone(),
        repo: Some(repo.clone()),
        open_issues: open_issues?,
        open_prs: open_prs?,
        stars: stars?,
        notifications: notifications?,
        commits: commits?,
        weekday: Local::now().weekday().num_days_from_sunday() as usize,
    })
}
