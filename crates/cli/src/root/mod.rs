pub mod processor;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::{config::Config, constants::*, keypair::Keypair, operator::OperatorArgs};

#[cfg(feature = "bls")]
use crate::bls::BLS;

#[derive(Parser, Debug)]
#[command(version = VERSION, about, long_about = None, propagate_version = true, subcommand_required = false)]
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
    #[command()]
    Operator(OperatorArgs),

    /// Config management
    #[command(subcommand)]
    Config(Config),

    /// Initialize Config
    #[command()]
    Configure,
}
