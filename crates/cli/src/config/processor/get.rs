use crate::config::{env::get_config_path, models::Config};
use color_eyre::eyre::eyre;

pub async fn process_get() -> color_eyre::eyre::Result<()> {
    let path = get_config_path()?;
    let path_str = path.to_str().expect("path should exist");
    let config_exists = tokio::fs::metadata(&path).await.is_ok();
    if !config_exists {
        return Err(eyre!("Config not found at {path_str}"));
    }

    let contents = tokio::fs::read_to_string(&path).await?;

    let config: Config = serde_json::from_str(&contents)?;

    println!("{:#?}", config);

    Ok(())
}
