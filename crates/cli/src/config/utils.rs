use std::{
    fs,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use color_eyre::owo_colors::OwoColorize;
use thiserror::Error;

use super::{
    models::{Config, Curve, Keystore, Profile},
    processor::update::process_update,
};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config not found error at {0}")]
    ConfigNotFoundError(PathBuf),
    #[error("Config parse error: {0}")]
    ConfigParseError(#[from] serde_json::Error),
    #[error("Config read error: {0}")]
    ReadConfigError(String),
    #[error("Config deserialize error: {0}")]
    DeserializeError(String),
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    #[error("Config write error: {0}")]
    WriteConfigError(String),
}

pub fn read_config(path: &Path) -> Result<Config, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::ConfigNotFoundError(path.to_path_buf()));
    }

    let file = fs::File::open(path).map_err(|e| ConfigError::ReadConfigError(e.to_string()))?;

    let reader = BufReader::new(file);

    let config = serde_yaml::from_reader(reader)
        .map_err(|e| ConfigError::DeserializeError(e.to_string()))?;

    Ok(config)
}

pub fn write_config(config: Config, path: &Path) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ConfigError::WriteConfigError(e.to_string()))?;
    }

    let file = fs::File::create(path).map_err(|e| ConfigError::WriteConfigError(e.to_string()))?;
    let writer = BufWriter::new(file);
    serde_yaml::to_writer(writer, &config)
        .map_err(|e| ConfigError::WriteConfigError(e.to_string()))?;

    println!("{}", "Configuration saved successfully!".green());
    Ok(())
}

pub fn get_profile(config: &Config, profile: &str) -> Result<Profile, ConfigError> {
    if let Some(profile) = config.profiles.get(profile) {
        Ok(profile.clone())
    } else {
        Err(ConfigError::ProfileNotFound(profile.to_string()))
    }
}

pub fn add_keystore_to_profile(
    profile_name: String,
    mut profile: Profile,
    curve: Curve,
    keystore: Keystore,
    keystore_name: &str,
    config_path: String,
) -> eyre::Result<()> {
    let keystores = profile.keystores.entry(curve).or_default();

    // Insert or update the keystore for the given name
    keystores.insert(keystore_name.to_string(), keystore);

    process_update(profile_name, Some(profile), config_path, false)?;
    Ok(())
}
