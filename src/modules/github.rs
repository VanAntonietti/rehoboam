use dotenvy::dotenv;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct GrapghQLQuery {
    query: String,
    variables: Variables,
}

#[derive(Serialize)]
struct Variables {
    #[serde(rename = "userName")]
    user_name: String,
}

pub async fn fetch_contributions(username: String, token: String) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let query = r#"
        query($userName: String!) { 
          user(login: $userName) {
            contributionsCollection {
              contributionCalendar {
                totalContributions
                weeks {
                  contributionDays {
                    contributionCount
                    date
                  }
                }
              }
            }
          }
        }
    "#;
    let payload = GrapghQLQuery {
        query: query.to_string(),
        variables: Variables {
            user_name: username.to_string(),
        },
    };
    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;
    let json: Value = response.json().await?;
    Ok(json)
}

pub async fn github_layout() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let username = env::var("GITHUB_NAME")?;
    let token = env::var("GITHUB_TOKEN")?;

    let result = fetch_contributions(username, token).await?;
    println!("Raw Json: {}", result);
    Ok(())
}
