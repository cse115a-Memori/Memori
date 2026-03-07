use reqwest::Client;
use chrono::{Local, Datelike};
use tauri::AppHandle;
use memori_ui::widgets::Github;
use crate::commands::data::{AuthState, read_store_state};
use serde::Deserialize;

//Need this struct so that read_store_state can deserialize the stored state
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct GithubState {
    repo: Option<String>,
}

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

//This is the frontend call to list GitHub repositories for the authenticated user on the github widget tile.
#[tauri::command]
#[specta::specta]
pub async fn get_github_repos(app: AppHandle) -> Result<Vec<String>, String> {
    let auth: AuthState = read_store_state(&app, "auth");
    let github_user = auth.users_by_provider.github;

    let token = match github_user {
        Some(user) => user.access_token,
        None => return Err("Not logged in with GitHub".to_string()),
    };

    get_user_repos(&token).await
}

pub async fn get_user_repos(token: &str) -> Result<Vec<String>, String> {
    let client = Client::new();
    let mut repos = Vec::new();

    // Fetch user's own repos
    let mut page = 1;
    loop {
        let url = format!(
            "https://api.github.com/user/repos?per_page=100&page={}&sort=updated",
            page
        );
        let data = github_get(&client, &url, token).await?;
        let arr = match data.as_array() {
            Some(a) if !a.is_empty() => a,
            _ => break,
        };
        for repo in arr {
            if let Some(name) = repo["full_name"].as_str() {
                repos.push(name.to_string());
            }
        }
        if arr.len() < 100 { break; }
        page += 1;
    }

    // Fetch user's orgs
    let orgs_data = github_get(&client, "https://api.github.com/user/orgs", token).await?;
    if let Some(orgs) = orgs_data.as_array() {
        for org in orgs {
            if let Some(org_login) = org["login"].as_str() {
                let mut page = 1;
                loop {
                    let url = format!(
                        "https://api.github.com/orgs/{}/repos?per_page=100&page={}",
                        org_login, page
                    );
                    let data = github_get(&client, &url, token).await?;
                    let arr = match data.as_array() {
                        Some(a) if !a.is_empty() => a,
                        _ => break,
                    };
                    for repo in arr {
                        if let Some(name) = repo["full_name"].as_str() {
                            if !repos.contains(&name.to_string()) {
                                repos.push(name.to_string());
                            }
                        }
                    }
                    if arr.len() < 100 { break; }
                    page += 1;
                }
            }
        }
    }

    Ok(repos)
}

async fn get_num_prs(
    token: &str,
    repo: &str,
) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=repo:{}+type:pr+state:open&per_page=1",
        repo
    );
    let data = github_get(&client, &url, token).await?;
    let count = data["total_count"].as_u64().ok_or_else(|| "total_count not found".to_string())? as u32;

    Ok(count)
}

async fn get_num_stars(token: &str, repo: &str) -> Result<u32, String> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}", repo);
    let data = github_get(&client, &url, token).await?;
    let stars = data["stargazers_count"].as_u64().unwrap_or(0) as u32;
    Ok(stars)
}

async fn get_num_issues(token: &str, repo: &str) -> Result<u32, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=repo:{}+type:issue+state:open&per_page=1",
        repo
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

async fn get_commit_frequency(token: &str, repo: &str) -> Result<[u32; 7], String> {
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
            "https://api.github.com/repos/{}/commits?since={}&until={}",
            repo, since, until
        );

        let data = github_get(&client, &url, token).await?;
        let count = data.as_array().map(|a| a.len() as u32).unwrap_or(0);
        commits_per_day.push(count);
    }

    let commits_arr: [u32; 7] = commits_per_day.try_into().unwrap();
    Ok(commits_arr)
}

pub async fn refresh_github_widget(app: &AppHandle) -> Result<Github, String> {
    println!("Refresh github widget called");
    let auth: AuthState = read_store_state(app, "auth");
    let github_user = auth.users_by_provider.github;
    
    if github_user.is_none() {
        println!("Github user is none");
        return Ok(Github::new("Not logged in...".to_string(), None))
    }
    
    let token = github_user.as_ref().unwrap().access_token.clone();
    let username = github_user.as_ref().unwrap().name.clone();
    
    let github_store: GithubState = read_store_state(app, "github");
    let repo = match github_store.repo {
        Some(repo) => repo,
        None => return Ok(Github::new(username, None)),
    };
    
    // Call each async function concurrently
    let (open_issues, open_prs, stars, notifications, commits) = tokio::join!(
        get_num_issues(&token, &repo),
        get_num_prs(&token, &repo),
        get_num_stars(&token, &repo),
        get_num_notifications(&token),
        get_commit_frequency(&token, &repo),
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
