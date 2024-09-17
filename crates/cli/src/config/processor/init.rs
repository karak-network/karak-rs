use std::path::PathBuf;

use color_eyre::eyre::eyre;
use tokio::io::AsyncWriteExt;

use crate::config::{
    env::get_config_path,
    models::{Config, ConfigVersion, Keystore},
};

pub async fn process_init(path: Option<String>, overwrite: bool) -> color_eyre::eyre::Result<()> {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => get_config_path()?,
    };

    // TODO: Better way to do this?
    let path_str = path.to_str().expect("path should exist");

    let config_exists = tokio::fs::metadata(&path).await.is_ok();
    if config_exists && !overwrite {
        return Err(eyre!("Config file already exists at {path_str}"));
    }

    let default_config = Config {
        version: ConfigVersion::V0,
        chain: None,
        keystore: Keystore::Local {
            path: PathBuf::from(
                dirs_next::home_dir()
                    .ok_or(eyre!("Home directory not found"))?
                    .join(".config")
                    .join("karak")
                    .join("keypairs"),
            ),
        },
    };

    let config_str = serde_json::to_string_pretty(&default_config)? + "\n";

    tokio::fs::create_dir_all(&path.parent().expect("expected config path parent")).await?;
    let mut file = tokio::fs::File::create(&path).await?;
    file.write_all(config_str.as_bytes()).await?;

    println!(
        "Config file initialized at {path_str}. If your path is not the default, make sure to export the custom path in the KARAK_CONFIG_PATH environment variable."
    );

    Ok(())
}
