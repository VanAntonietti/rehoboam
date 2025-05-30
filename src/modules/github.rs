use dotenvy::dotenv;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use reqwest::{Client, header};
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

pub async fn fetch_contributions(username: &str, token: &str) -> Result<Value, Box<dyn Error>> {
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
        .header(header::USER_AGENT, "RehoboamRustGithubClient/1.0")
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    println!("Status: {}", status);
    println!("Raw response body: {}", body);
    if !status.is_success() {
        return Err(format!("API error: Status{}, Body: {}", status, body).into());
    }
    let json: Value = serde_json::from_str(&body)?;
    Ok(json)
}

pub async fn github_layout() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let username = env::var("GITHUB_NAME")?;
    let token = env::var("GITHUB_TOKEN")?;
    let result = fetch_contributions(&username, &token).await?;
    println!("Raw Json: {}", result);
    Ok(())
}

pub fn github_draw(frame: &mut Frame, area: Rect) {
    /*let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .margin(1)
    .split(area);
    */
    let block = Block::bordered().title("Graphs");
    frame.render_widget(block, area);
}
