pub mod env;
pub mod models;
pub mod processor;
mod utils;

pub use utils::*;

use clap::Subcommand;
use std::path::PathBuf;

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
    /// Set config
    Set {
        #[arg(long)]
        chain_id: Option<u64>,
        #[arg(long)]
        rpc_url: Option<String>,
        #[arg(long, group = "keystore")]
        local_keystore: Option<PathBuf>,
        #[arg(long, group = "keystore")]
        aws_keystore: bool,
    },
}
