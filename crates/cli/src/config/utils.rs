use std::{fs, path::PathBuf};

use thiserror::Error;

use super::{env::get_config_path, models::Config};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config path not found")]
    ConfigPathNotFound,
    #[error("Config not found error at {0}")]
    ConfigNotFoundError(PathBuf),
    #[error("Config parse error: {0}")]
    ConfigParseError(#[from] serde_json::Error),
}

pub fn get_config() -> Result<Config, ConfigError> {
    let path = get_config_path().map_err(|_| ConfigError::ConfigPathNotFound)?;
    fs::metadata(&path).map_err(|_| ConfigError::ConfigNotFoundError(path.clone()))?;

    let contents = fs::read_to_string(&path).map_err(|_| ConfigError::ConfigNotFoundError(path))?;

    Ok(serde_json::from_str(&contents)?)
}
