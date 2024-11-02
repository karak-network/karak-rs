pub mod processor;

use clap::{Args, Parser, Subcommand};

use crate::config::models::Keystore;
use crate::shared::Curve;

#[derive(Parser)]
#[command(version, about = "Karak keypair CLI", long_about = None)]
#[command(propagate_version = true)]
pub struct KeypairRoot {
    #[command(subcommand)]
    pub subcommand: Keypair,
}

#[derive(Subcommand, Debug)]
pub enum Keypair {
    /// Generate keypair
    Generate {
        #[command(flatten)]
        keypair: Option<KeypairArgs>,

        /// Curve to use for key generation
        #[arg(long, value_enum)]
        curve: Option<Curve>,
    },
    /// View public key
    Pubkey {
        #[command(flatten)]
        keypair_location: Option<KeypairLocationArgs>,
        #[command(flatten)]
        keypair: Option<KeypairArgs>,
        /// Curve to use for key parsing
        #[arg(long, value_enum)]
        curve: Option<Curve>,
    },
}

#[derive(Args, Debug)]
pub struct KeypairArgs {
    /// Keystore to save the keypair
    #[arg(long)]
    pub keystore: Option<Keystore>,

    /// Passphrase to encrypt keypair
    #[arg(long)]
    pub passphrase: Option<String>,
}

#[derive(Args, Debug)]
pub struct KeypairLocationArgs {
    /// Keypair ID/path to retrieve
    #[arg(long)]
    pub keypair: Option<String>,
}
