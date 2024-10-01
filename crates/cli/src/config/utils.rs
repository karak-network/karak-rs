use std::{
    fs,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    str::FromStr,
};

use alloy::primitives::Address;
use color_eyre::owo_colors::OwoColorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use thiserror::Error;

use super::models::{Chain, Config, Keystore, Profile};

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

// Add validation to prompts
pub fn profile_prompt() -> Profile {
    // TODO: take optional config if already configured and set defaults in prompts
    let chain = prompt_chain();
    let bn254_keystore = prompt_keystore("BLS");
    let secp256k1_keystore = prompt_keystore("ECDSA");
    let karak_address = loop {
        match Address::from_str(&prompt_address()) {
            Ok(address) => break address,
            Err(e) => {
                println!("Invalid address format - {:?}", e.to_string().red());
                continue;
            }
        }
    };

    let key_generation_folder = prompt_key_generation_folder();

    Profile {
        chain,
        bn254_keystore,
        secp256k1_keystore,
        karak_address,
        key_generation_folder,
    }
}

fn prompt_key_generation_folder() -> PathBuf {
    let folder: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter key generation folder path")
        .interact_text()
        .unwrap();

    // TODO: get absolute path
    PathBuf::from(folder)
}

fn prompt_chain() -> Chain {
    let theme = ColorfulTheme::default();

    let chain_options = vec!["EVM"];
    // Used when more chains are supported
    let _ = Select::with_theme(&theme)
        .with_prompt("Select chain type")
        .default(0)
        .items(&chain_options)
        .interact()
        .unwrap();

    let id: u64 = Input::with_theme(&theme)
        .with_prompt("Enter chain ID")
        .interact_text()
        .unwrap();
    let rpc_url: url::Url = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Enter RPC URL")
            .interact_text()
            .unwrap();
        match url::Url::parse(&input) {
            Ok(url) => break url,
            Err(_) => {
                println!("Invalid URL format. Please try again.");
                continue;
            }
        }
    };
    Chain::Evm { id, rpc_url }
}

fn prompt_keystore(name: &str) -> Keystore {
    let theme = ColorfulTheme::default();

    let keystore_options = vec!["Local", "AWS"];
    let keystore_index = Select::with_theme(&theme)
        .with_prompt(format!("Select {} keystore type", name))
        .default(0)
        .items(&keystore_options)
        .interact()
        .unwrap();

    match keystore_index {
        0 => {
            let path: String = Input::with_theme(&theme)
                .with_prompt(format!("Enter {} local keystore path", name))
                .interact_text()
                .unwrap();
            Keystore::Local {
                path: PathBuf::from(path),
            }
        }
        _ => {
            let secret: String = Input::with_theme(&theme)
                .with_prompt(format!("Enter {} aws key location url", name))
                .interact_text()
                .unwrap();
            Keystore::Aws { secret }
        }
    }
}

fn prompt_address() -> String {
    let address: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Karak Core contract address")
        .interact_text()
        .unwrap();

    address
}
