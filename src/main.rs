use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use std::env;

#[derive(Debug, Deserialize)]
struct Repository {
    owner: Owner,
    name: String,
    default_branch: String,
}

#[derive(Debug, Deserialize)]
struct Owner {
    login: String,
}

struct ZippedRepository {
    name: String,
    zip: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repos: Vec<Repository> = get_owner_repos().await?;
    let first_repo = &repos[0];
    get_zipped_repo(first_repo).await?;
    Ok(())
}

fn get_api_token() -> String {
    dotenv().ok();
    let api_token = env::var("GITHUB_AUTH_KEY").expect("$GITHUB_AUTH_KEY is not set");
    return api_token;
}

fn get_client() -> reqwest::Client {
    return reqwest::Client::new();
}

fn filter_logins(repositories: Vec<Repository>) -> Vec<Repository> {
    repositories
        .into_iter()
        .filter(|repo| repo.owner.login == "Lorenzobattistela")
        .collect()
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

async fn get_zipped_repo(
    repository: &Repository,
) -> Result<ZippedRepository, Box<dyn std::error::Error>> {
    let api_token = get_api_token();
    let client = get_client();

    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/zipball/{default_branch}",
        owner = repository.owner.login,
        repo = repository.name,
        default_branch = repository.default_branch
    );

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Accept", "application/vnd.github+json")
        .header(USER_AGENT, "Lorenzobattistela")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;

    let zip_bytes = res.bytes().await?;
    let zip = zip_bytes.to_vec();
    let name = repository.name.clone();
    let zipped_repo = ZippedRepository { name, zip };
    Ok::<ZippedRepository, Box<dyn std::error::Error>>(zipped_repo)
}

