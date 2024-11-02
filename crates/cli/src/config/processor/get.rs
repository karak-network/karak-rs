use std::path::Path;

use crate::config::{get_profile, read_config};

pub fn process_get(profile: String, config_path: String) -> color_eyre::eyre::Result<()> {
    let path = Path::new(&config_path);
    let config = read_config(path)?;

    let profile = get_profile(&config, &profile)?;

    let yaml = serde_yaml::to_string(&profile)?;
    println!("{}", yaml);

    Ok(())
}
