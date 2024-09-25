#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("cli is not configured. Please run the configure command first.")]
    NotConfigured,

    #[error("Unknown command")]
    UnknownCommand,

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("OS error: {0}")]
    OsError(String),

    // TODO: Refactor component errors
    #[error("Component error: {0}")]
    ComponentError(String),
}
