use std::env;

pub fn default_karak_dir() -> String {
    format!(
        "{}/.karak",
        env::var("HOME").expect("HOME environment variable not set")
    )
}

pub fn default_config_path() -> String {
    format!("{}/config.yaml", default_karak_dir())
}

pub const DEFAULT_PROFILE: &str = "default";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
