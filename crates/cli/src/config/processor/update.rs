use std::collections::HashMap;
use std::path::Path;

use color_eyre::eyre::{self, eyre};
use color_eyre::owo_colors::OwoColorize;

use crate::config::models::Config as ConfigModel;
use crate::config::{get_profile, profile_prompt, read_config, write_config, ConfigError};

pub fn process_update(profile_name: String, config_path: String, reset: bool) -> eyre::Result<()> {
    let path = Path::new(&config_path);

    if path.exists() && reset {
        std::fs::remove_file(path)?;
    }

    let mut config = match read_config(path) {
        Ok(cfg) => cfg,
        Err(ConfigError::ConfigNotFoundError(_)) => ConfigModel {
            profiles: HashMap::new(),
        },
        Err(e) => return Err(eyre!("Error reading config: {}", e)),
    };

    println!("{}", "Configuring the Karak CLI\n".green().bold());
    println!("Profile: {}", profile_name.cyan());

    let profile = match get_profile(&config, &profile_name) {
        Ok(_) => {
            // TODO: Add prompt for updating existing profile
            profile_prompt()
        }
        Err(_) => profile_prompt(),
    };

    config.profiles.insert(profile_name.to_string(), profile);

    write_config(config, path)?;

    Ok(())
}
