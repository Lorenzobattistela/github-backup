use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use bytes::Bytes;

fn get_api_token() -> String {
    dotenv().ok();
    let api_token = env::var("GITHUB_AUTH_KEY").expect("$GITHUB_AUTH_KEY is not set");
    return api_token;
}

fn get_client() -> reqwest::Client {
    return reqwest::Client::new();
}

async fn get_owner_repos() -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let api_token = get_api_token();
    let client = get_client();

    let res = client
        .get("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Accept", "application/vnd.github+json")
        .header(USER_AGENT, "Lorenzobattistela")
        .query(&[("per_page", "100")])
        .send()
        .await?;

    let repositories: Vec<Repository> = res.json().await?;
    let filtered_repos = filter_logins(repositories);
    Ok(filtered_repos)
}
