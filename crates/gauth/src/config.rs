use serde::{Serialize, Deserialize};
use tokio::fs;
use dirs_next::config_dir;
use toml::toml;
use crate::Errors;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub access_token: String
}

pub async fn load_config() -> Result<Config, Errors> {

    let home = config_dir().unwrap();
    let home = home.to_string_lossy();

    // Default paths
    let path_dir = format!("{}/gsync", &home);
    let path_file = format!("{}/gsync/config.toml", &home);

    let default_config = toml! {
        client_id = ""
        client_secret = ""
        refresh_token = ""
        access_token = ""
    };

    let default_config_string = toml::to_string(&default_config)?;

    // Check if file exists
    if let Ok(file) = fs::try_exists(&path_file).await {
        if !file {
            fs::create_dir(&path_dir).await?;
            fs::File::create(&path_file).await?;
            fs::write(&path_file, default_config_string).await?;
        }
    }

    let context = fs::read_to_string(&path_file).await?;
    let loaded_config: Config = toml::from_str(&context)?;

    Ok(loaded_config)
}
