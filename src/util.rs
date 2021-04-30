use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "invalid_token")]
    pub token: String,
    #[serde(default = "database_path")]
    pub database_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: invalid_token(),
            database_path: database_path(),
        }
    }
}

fn invalid_token() -> String {
    "INVALID_TOKEN".to_string()
}

fn database_path() -> PathBuf {
    "data.db".into()
}

impl Config {
    pub const FILENAME: &'static str = "settings.toml";

    pub async fn load() -> crate::Result<Self> {
        let path: PathBuf = Self::FILENAME.into();

        if !path.exists() {
            Self::default().save().await?;
        }

        let mut file = fs::OpenOptions::new().read(true).open(path).await?;
        let mut content = String::with_capacity(256);
        file.read_to_string(&mut content).await?;

        Ok(toml::from_str(&content)?)
    }

    pub async fn save(self) -> crate::Result<()> {
        let path: PathBuf = Self::FILENAME.into();

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .await?;

        let content = toml::to_string_pretty(&self)?;
        file.write_all(content.as_ref()).await?;
        Ok(())
    }
}
