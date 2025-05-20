use tokio::io::{self, AsyncBufReadExt, BufReader};
use reqwest::{self, Client};
use serde::{Serialize, Deserialize};
use serde_json::json;
use config::*;

mod config;

pub async fn test_fn() {
    config::load_config().await.unwrap();
}

// TODO: Make getting data from config
// TODO: Error handling

pub async fn get_auth_token() -> Result<String, std::io::Error> {

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

pub async fn get_refresh_token(code: &str, client_id: &str) -> Result<RefreshTokenOutput, std::io::Error> {
    let client = Client::new();

    println!("Write client secret:");

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut data = String::new();
    let bytes = reader.read_line(&mut data).await?;
    if bytes == 0 {
        println!("EOF")
    }

    let data = json!({
        "code": code,
        "client_id": client_id,
        "client_secret": data.trim(),
        "redirect_uri": "http://localhost",
        "grant_type": "authorization_code"
    });

    let request = client.post("https://oauth2.googleapis.com/token")
        .form(&data)
        .send().await.unwrap()
        .text().await.unwrap(); // remove unwrap   

    let parsestruct: RefreshTokenOutput = serde_json::from_str(&request).unwrap();

    Ok(parsestruct)
}

pub async fn get_access_token() {}
