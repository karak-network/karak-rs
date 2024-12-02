use std::env;

use once_cell::sync::Lazy;

pub static DEFAULT_KARAK_DIR: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/.karak",
        env::var("HOME").expect("HOME environment variable not set")
    )
});

pub static DEFAULT_CONFIG_PATH: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/.karak/config.yaml",
        env::var("HOME").expect("HOME environment variable not set")
    )
});

pub const DEFAULT_PROFILE: &str = "default";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
