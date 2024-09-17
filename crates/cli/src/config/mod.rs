pub mod env;
pub mod models;
pub mod processor;
mod utils;

pub use utils::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Config {
    /// Initialize config
    Init {
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long, action)]
        overwrite: bool,
    },
    /// Get config
    Get,
}
