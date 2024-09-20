use anyhow::Context;
use std::path::PathBuf;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub workshop: Workshop,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Workshop {
    pub id: i64,
    pub name: String,
    pub authors: Vec<String>,
}

impl ConfigFile {
    pub fn from_file(path: PathBuf) -> anyhow::Result<ConfigFile> {
        let contents =
            std::fs::read_to_string(path).context("Failed to read contents of config file.")?;
        let config_file = toml::from_str(&contents).context("Failed to parse config file")?;

        Ok(config_file)
    }
}
