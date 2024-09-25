use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{collections::HashMap, fs::File};

use alloy::primitives::Address;
use clap::{Arg, ArgAction, ArgMatches, Command};
use color_eyre::owo_colors::OwoColorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};

use crate::CliError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub chain: Option<Chain>,
    pub keystore: Keystore,
    pub karak_address: Address,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Chain {
    #[serde(rename = "evm")]
    Evm { id: u64, rpc_url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Keystore {
    #[serde(rename = "local")]
    Local { path: PathBuf },
    #[serde(rename = "aws")]
    Aws,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(flatten)]
    profiles: HashMap<String, Profile>,
}

impl Config {
    pub fn read_config(path: &Path) -> Result<Config, CliError> {
        if !path.exists() {
            return Err(CliError::NotConfigured);
        }

        let file = File::open(path).map_err(|e| CliError::ConfigError(e.to_string()))?;
        let reader = BufReader::new(file);
        let config =
            serde_yaml::from_reader(reader).map_err(|e| CliError::ConfigError(e.to_string()))?;
        Ok(config)
    }

    pub fn write_config(&self, path: &Path) -> Result<(), CliError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| CliError::ConfigError(e.to_string()))?;
        }

        let file = File::create(path).map_err(|e| CliError::ConfigError(e.to_string()))?;
        let writer = BufWriter::new(file);
        serde_yaml::to_writer(writer, self).map_err(|e| CliError::ConfigError(e.to_string()))?;

        println!("{}", "Configuration saved successfully!".green());
        Ok(())
    }

    pub fn get_profile(&self, profile: &str) -> Result<Profile, CliError> {
        if let Some(profile) = self.profiles.get(profile) {
            Ok(profile.clone())
        } else {
            Err(CliError::ProfileNotFound(profile.to_string()))
        }
    }

    pub fn profile_prompt(config: Option<Profile>) -> Profile {
        let chain = prompt_chain();
        let keystore = prompt_keystore();
        let karak_address = Address::from_str(&prompt_address()).unwrap();

        Profile {
            chain,
            keystore,
            karak_address,
        }
    }

    pub fn upsert_profile(&mut self, profile: &str) {
        let config = match self.get_profile(profile) {
            Ok(existing_profile) => Self::profile_prompt(Some(existing_profile)),
            Err(_) => Self::profile_prompt(None),
        };

        self.profiles.insert(profile.to_string(), config);
    }

    pub fn add_commands(cmd: Command) -> Command {
        cmd.subcommand(
            Command::new("configure")
                .about("Configure the Karak CLI")
                .arg(
                    Arg::new("reset")
                        .short('r')
                        .long("reset")
                        .value_name("RESET")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .help("Resets the config file"),
                ),
        )
    }

    pub fn run(args: &ArgMatches) -> Result<(), CliError> {
        let profile = args.get_one::<String>("profile").unwrap();
        let path = Path::new(args.get_one::<String>("config").unwrap());

        let &reset_flag = args.get_one::<bool>("reset").unwrap();

        if path.exists() && reset_flag {
            std::fs::remove_file(path).map_err(|e| CliError::ConfigError(e.to_string()))?;
        }

        let mut config = match Self::read_config(path) {
            Ok(cfg) => cfg,
            Err(CliError::NotConfigured) => Config {
                profiles: HashMap::new(),
            },
            Err(e) => return Err(e),
        };

        println!("{}", "Configuring the Karak CLI\n".green().bold());
        println!("Profile: {}", profile.cyan());

        config.upsert_profile(profile);
        config.write_config(path)?;

        Ok(())
    }
}

fn prompt_chain() -> Option<Chain> {
    let theme = ColorfulTheme::default();

    let chain_options = vec!["EVM", "None"];
    let chain_index = Select::with_theme(&theme)
        .with_prompt("Select chain type")
        .default(0)
        .items(&chain_options)
        .interact()
        .unwrap();

    match chain_index {
        0 => {
            let id: u64 = Input::with_theme(&theme)
                .with_prompt("Enter chain ID")
                .interact_text()
                .unwrap();
            let rpc_url: String = Input::with_theme(&theme)
                .with_prompt("Enter RPC URL")
                .interact_text()
                .unwrap();
            Some(Chain::Evm { id, rpc_url })
        }
        _ => None,
    }
}

fn prompt_keystore() -> Keystore {
    let theme = ColorfulTheme::default();

    let keystore_options = vec!["Local", "AWS"];
    let keystore_index = Select::with_theme(&theme)
        .with_prompt("Select keystore type")
        .default(0)
        .items(&keystore_options)
        .interact()
        .unwrap();

    match keystore_index {
        0 => {
            let path: String = Input::with_theme(&theme)
                .with_prompt("Enter local keystore path")
                .interact_text()
                .unwrap();
            Keystore::Local {
                path: PathBuf::from(path),
            }
        }
        _ => Keystore::Aws,
    }
}

fn prompt_address() -> String {
    // TODO: Add address validation
    let address: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Karak Core contract address")
        .interact_text()
        .unwrap();

    address
}
