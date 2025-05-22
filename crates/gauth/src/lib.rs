use tokio::io::{self, AsyncBufReadExt, BufReader};
use reqwest::{self, Client};
use serde::{Serialize, Deserialize};
use serde_json::json;
use config::*;
use errors::Errors;

mod config;
mod errors;

pub async fn test_fn() {
    config::load_config().await.unwrap();
}

pub async fn get_auth_token() -> Result<String, Errors> {

    println!("https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri=http://localhost&response_type=code&scope=https://www.googleapis.com/auth/drive&access_type=offline", 1);

    let stdin = io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut data = String::new();

    println!("Write output token:");
    let token_bytes = reader.read_line(&mut data).await?;
    if token_bytes == 0 {
        println!("EOF")
    }

    Ok(data.trim().to_string())
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenOutput {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String 
}

pub async fn get_refresh_token(code: &str) -> Result<RefreshTokenOutput, Errors> {
    let client = Client::new();
    let config = load_config().await?;

    if config.client_id.is_empty() || config.client_secret.is_empty() {
        println!("Please add the client secret/ID to the configuration.");
        return Err(Errors::EmptyField())
    }

    let data = json!({
        "code": code,
        "client_id": config.client_id,
        "client_secret": config.client_secret,
        "redirect_uri": "http://localhost",
        "grant_type": "authorization_code"
    });

    let request = client.post("https://oauth2.googleapis.com/token")
        .form(&data)
        .send().await?
        .text().await?;

    let parsestruct: RefreshTokenOutput = serde_json::from_str(&request)?;

    Ok(parsestruct)
}

pub async fn get_access_token() {}
