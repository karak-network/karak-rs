pub mod processor;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::{config::Config, keypair::Keypair, operator::Operator};

#[cfg(feature = "bls")]
use crate::bls::BLS;

const DEFAULT_CONFIG_PATH: &str = concat!(env!("HOME"), "/.config/karak/config.yaml");
const DEFAULT_PROFILE: &str = "default";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, propagate_version = true, subcommand_required = false)]
pub struct Root {
    #[arg(short = 'p', long, global = true, default_value = DEFAULT_PROFILE)]
    pub profile: Option<String>,

    #[arg(short = 'c', long, global = true, default_value = DEFAULT_CONFIG_PATH)]
    pub config_path: Option<String>,

    #[arg(long = "completions", value_enum)]
    pub generator: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
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