pub mod processor;

use clap::Parser;

use crate::{config::Config, keypair::Keypair, operator::Operator};

#[cfg(feature = "bls")]
use crate::bls::BLS;

#[derive(Parser)]
#[command(version, about = "Karak CLI", long_about = None)]
#[command(propagate_version = true)]
pub enum Root {
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
