pub mod processor;

use clap::Parser;

use crate::{bls::BLS, keypair::Keypair, operator::Operator};

#[derive(Parser)]
#[command(version, about = "Karak CLI", long_about = None)]
#[command(propagate_version = true)]
pub enum Root {
    /// Manage keypairs
    #[command(subcommand)]
    Keypair(Keypair),
    /// Perform BLS operation
    #[command(subcommand)]
    BLS(BLS),
    /// Operator management
    #[command(subcommand)]
    Operator(Operator),
}
