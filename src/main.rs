use gauth::{get_refresh_token};

#[tokio::main]
async fn main() {
    get_refresh_token("").await.unwrap();
}
