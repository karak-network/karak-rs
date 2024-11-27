use std::path::Path;

use bat::{Input, PrettyPrinter};

use crate::config::{get_profile, read_config};

pub fn process_get(
    profile: String,
    config_path: String,
    all: bool,
) -> color_eyre::eyre::Result<()> {
    let path = Path::new(&config_path);
    let config = read_config(path)?;
    let mut bytes = Vec::with_capacity(128);

    if all {
        serde_yaml::to_writer(&mut bytes, &config).unwrap();
        display_config(&bytes, "All Profiles");
        return Ok(());
    }

    let profile_config = get_profile(&config, &profile)?;

    serde_yaml::to_writer(&mut bytes, &profile_config).unwrap();
    display_config(&bytes, &format!("Profile: {}", profile));
    Ok(())
}

fn display_config(bytes: &[u8], title: &str) {
    PrettyPrinter::new()
        .language("yaml")
        .line_numbers(true)
        .grid(true)
        .header(true)
        .input(Input::from_bytes(bytes).title(title))
        .print()
        .unwrap();
}
