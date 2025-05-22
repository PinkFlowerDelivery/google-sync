use std::fmt;

#[derive(Debug)]
pub enum Errors {
    IO(tokio::io::Error),
    Http(reqwest::Error),
    TomlSerialize(toml::ser::Error),
    TomlDeserialize(toml::de::Error),
    Json(serde_json::Error),
    EmptyField()
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::IO(e) => write!(f, "IO Error: {}", e),
            Errors::Http(e) => write!(f, "Http error: {}", e),
            Errors::TomlSerialize(e) => write!(f, "Toml serialize error: {}", e),
            Errors::TomlDeserialize(e) => write!(f, "Toml deserialize error: {}", e),
            Errors::Json(e) => write!(f, "Json parsing error: {}", e),
            Errors::EmptyField() => write!(f, "Empty field in config")

        }
    }
}

impl From<std::io::Error> for Errors {
    fn from(value: std::io::Error) -> Self {
        Errors::IO(value) 
    }
}

impl From<reqwest::Error> for Errors {
    fn from(value: reqwest::Error) -> Self {
        Errors::Http(value)
    }
}

impl From<toml::ser::Error> for Errors {
    fn from(value: toml::ser::Error) -> Self {
        Errors::TomlSerialize(value)
    }
}

impl From<toml::de::Error> for Errors {
    fn from(value: toml::de::Error) -> Self {
        Errors::TomlDeserialize(value)
    }
}

impl From<serde_json::Error> for Errors {
    fn from(value: serde_json::Error) -> Self {
        Errors::Json(value)
    }
}
