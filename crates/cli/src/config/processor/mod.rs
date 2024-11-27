pub mod get;
pub mod prompt;
pub mod update;

use color_eyre::eyre;

use super::{get_profile, models::Profile, read_config, Config};
use get::process_get;
use update::process_update;

pub async fn process(command: Config, profile: String, config_path: String) -> eyre::Result<()> {
    match command {
        Config::Update { reset } => process_update(profile, None, config_path, reset),
        Config::Get { all } => process_get(profile, config_path, all),
    }
}

pub async fn process_configure(profile: String, config_path: String) -> eyre::Result<()> {
    process_update(profile, None, config_path, true)
}

pub fn pre_run(profile_name: String, config_path: String) -> eyre::Result<Profile> {
    let config = read_config(std::path::Path::new(&config_path))?;
    let profile = get_profile(&config, &profile_name)?;

    // TODO: check latest version against current version and warn if outdated
    Ok(profile)
}
