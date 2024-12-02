use std::env;

use once_cell::sync::Lazy;

pub static DEFAULT_KARAK_DIR: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/.karak",
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    )
});

pub static DEFAULT_CONFIG_PATH: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/.karak/config.yaml",
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    )
});

pub const DEFAULT_PROFILE: &str = "default";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
