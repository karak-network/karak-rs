pub mod processor;

use clap::{Parser, Subcommand};

use crate::{config::Config, keypair::Keypair, operator::Operator};

#[cfg(feature = "bls")]
use crate::bls::BLS;

const DEFAULT_CONFIG_PATH: &str = concat!(env!("HOME"), "/.config/karak/config.yaml");
const DEFAULT_PROFILE: &str = "default";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Root {
    #[arg(short = 'p', long, global = true, default_value = DEFAULT_PROFILE)]
    profile: Option<String>,

    #[arg(short = 'c', long, global = true, default_value = DEFAULT_CONFIG_PATH)]
    config_path: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Keypair management
    #[command(subcommand)]
    Keypair(Keypair),

    /// BLS operations
    #[cfg(feature = "bls")]
    #[command(subcommand)]
    BLS(BLS),

    /// Operator management
    #[command(subcommand)]
    Operator(Operator),

    /// Config management
    #[command(subcommand)]
    Config(Config),
}
