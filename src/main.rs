use dotenvy::dotenv;
use rehoboam::modules::github::fetch_contributions;
use std::env;
mod app;
mod modules;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // app::run()
    dotenv().ok();
    let username = env::var("GITHUB_NAME")?;
    let token = env::var("GITHUB_TOKEN")?;

    let result = fetch_contributions(username, token).await?;
    println!("Raw Json: {}", result);
    Ok(())
}
