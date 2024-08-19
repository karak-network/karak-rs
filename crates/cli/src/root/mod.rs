pub mod processor;

use clap::Parser;

use crate::{bls::BLS, config::Config, keypair::Keypair, operator::Operator};

#[derive(Parser)]
#[command(version, about = "Karak CLI", long_about = None)]
#[command(propagate_version = true)]
pub enum Root {
    /// Keypair management
    #[command(subcommand)]
    Keypair(Keypair),
    /// BLS operations
    #[command(subcommand)]
    BLS(BLS),
    /// Operator management
    #[command(subcommand)]
    Operator(Operator),
    /// Config management
    #[command(subcommand)]
    Config(Config),
}
