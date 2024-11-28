pub mod models;
pub mod processor;
mod utils;

pub use utils::*;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Config {
    /// Upsert config
    Update {
        #[arg(long)]
        reset: bool,
    },
    /// Get config
    Get {
        #[arg(long)]
        all: bool,
    },
}
