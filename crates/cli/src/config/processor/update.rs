use std::collections::HashMap;
use std::path::Path;

use color_eyre::eyre::{self, eyre};
use color_eyre::owo_colors::OwoColorize;

use crate::config::models::Config as ConfigModel;
use crate::config::models::Profile;
use crate::config::processor::prompt::profile_prompt;
use crate::config::{get_profile, read_config, write_config, ConfigError};
use crate::prompter;

pub fn process_update(
    profile_name: String,
    profile: Option<Profile>,
    config_path: String,
    reset: bool,
) -> eyre::Result<()> {
    let path = Path::new(&config_path);

    if path.exists() && reset {
        let confirm = prompter::confirm(
            "Existing config will be deleted. Do you want to proceed?",
            Some(false),
        )?;
        if !confirm {
            println!("Aborting configuration. Use `config update` command to update the config.");
            return Ok(());
        }
        std::fs::remove_file(path)?;
    }

    let mut config = match read_config(path) {
        Ok(cfg) => cfg,
        Err(ConfigError::ConfigNotFoundError(_)) => ConfigModel {
            profiles: HashMap::new(),
        },
        Err(e) => return Err(eyre!("Error reading config: {}", e)),
    };

    let profile = match profile {
        Some(p) => p,
        None => {
            println!("{}", "Configuring the Karak CLI\n".green().bold());
            println!("Profile: {}", profile_name.cyan());

            match get_profile(&config, &profile_name) {
                Ok(profile) => profile_prompt(Some(profile))?,
                Err(ConfigError::ProfileNotFound(profile_name)) => {
                    println!(
                        "Profile {} not found. Creating new profile...",
                        profile_name.cyan()
                    );
                    profile_prompt(None)?
                }
                Err(e) => return Err(e.into()),
            }
        }
    };

    config.profiles.insert(profile_name.to_string(), profile);

    write_config(config, path)?;

    Ok(())
}
