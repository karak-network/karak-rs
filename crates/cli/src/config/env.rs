use std::path::PathBuf;

use color_eyre::eyre::eyre;

pub fn get_config_path() -> color_eyre::eyre::Result<std::path::PathBuf> {
    match std::env::var("KARAK_CONFIG_PATH") {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => Ok(dirs_next::home_dir()
            .ok_or(eyre!("Home directory not found"))?
            .join(".config")
            .join("karak")
            .join("config.json")),
    }
}
