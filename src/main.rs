use dotenv::dotenv;
use indicatif::ProgressBar;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

// CLI flags -> allow-others-repos
// output_path -> default is ./backups
#[derive(Debug)]
struct Cli {
    allow_others_repos: bool,
    output_path: String,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            allow_others_repos: false,
            output_path: String::from("./backups"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = get_cli_args();
    let owner: Owner = get_owner_login().await?;

    let repos: Vec<Repository> = get_owner_repos(args.allow_others_repos, owner).await?;
    let len_repos: u64 = repos.len() as u64;
    let mut error_counts = 0;

    let pb = ProgressBar::new(len_repos);

    for repo in repos.iter() {
        let zipped_repo: ZippedRepository = get_zipped_repo(repo).await?;
        let created = create_zip_file(&zipped_repo.name, zipped_repo.zip, &args.output_path).await;
        match created {
            Ok(_) => {
                pb.inc(1);
            }
            Err(e) => {
                println!("Error downloading repo: {}. Err: {}", zipped_repo.name, e);
                error_counts += 1;
            }
        }
    }
    let downloaded_repos = len_repos - error_counts;
    let message = format!(
        "Downloaded {} repositories and saved them. Could not download {} repositories.",
        downloaded_repos, error_counts
    );
    pb.finish_with_message(message);

    Ok(())
}

fn get_cli_args() -> Cli {
    let mut args = Cli::default();
    if let Some(output_path) = env::args().nth(1) {
        args.output_path = output_path;
    }

    if let Some(allow_others_repos) = env::args().nth(2) {
        if allow_others_repos == "--allow-others-repos" {
            args.allow_others_repos = true;
        }
    }

    args
}

fn get_api_token() -> String {
    dotenv().ok();
    let api_token = env::var("GITHUB_AUTH_KEY").expect("$GITHUB_AUTH_KEY is not set");
    return api_token;
}

fn get_client() -> reqwest::Client {
    return reqwest::Client::new();
}

fn filter_logins(repositories: Vec<Repository>, owner: Owner) -> Vec<Repository> {
    repositories
        .into_iter()
        .filter(|repo| repo.owner.login == owner.login)
        .collect()
}

async fn get_owner_login() -> Result<Owner, Box<dyn std::error::Error>> {
    let api_token = get_api_token();
    let client = get_client();

    let res = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Accept", "application/vnd.github+json")
        .header(USER_AGENT, "Lorenzobattistela")
        .send()
        .await?;
    let owner: Owner = res.json().await?;
    Ok(owner)
}

async fn get_owner_repos(
    allow_others_repos: bool,
    owner: Owner,
) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let api_token = get_api_token();
    let client = get_client();

    let res = client
        .get("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Accept", "application/vnd.github+json")
        .header(USER_AGENT, owner.login.clone())
        .query(&[("per_page", "100")])
        .send()
        .await?;

    let mut repositories: Vec<Repository> = res.json().await?;
    if !allow_others_repos {
        repositories = filter_logins(repositories, owner);
    }
    Ok(repositories)
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

async fn create_zip_file(
    zip_name: &str,
    zip: Vec<u8>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let saving_path = format!("{}/{}.zip", output_path, zip_name);
    let mut file = File::create(&saving_path).await?;
    file.write_all(&zip).await?;
    println!("Saved zip at {}", saving_path);
    Ok(())
}
