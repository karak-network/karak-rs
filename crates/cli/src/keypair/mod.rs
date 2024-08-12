pub mod processor;

use clap::{Args, Parser, Subcommand};

use crate::shared::{Curve, Keystore};

#[derive(Parser)]
#[command(version, about = "Karak keypair CLI", long_about = None)]
#[command(propagate_version = true)]
pub struct KeypairRoot {
    #[command(subcommand)]
    pub subcommand: Keypair,
}

#[derive(Subcommand)]
pub enum Keypair {
    /// Generate keypair
    Generate {
        #[command(flatten)]
        keypair: KeypairArgs,

        /// Curve to use for key generation
        #[arg(short, long, value_enum)]
        curve: Curve,
    },
    /// View public key
    Pubkey {
        #[command(flatten)]
        keypair_location: KeypairLocationArgs,
        #[command(flatten)]
        keypair: KeypairArgs,
        /// Curve to use for key parsing
        #[arg(short, long, value_enum)]
        curve: Curve,
    },
}

#[derive(Args)]
pub struct KeypairArgs {
    /// Keystore to save the keypair
    #[arg(short = 's', long)]
    pub keystore: Keystore,

    /// Passphrase to encrypt keypair
    #[arg(short, long)]
    pub passphrase: Option<String>,
}

#[derive(Args)]
pub struct KeypairLocationArgs {
    /// Keypair ID/path to retrieve
    #[arg(short, long)]
    pub keypair: String,
}
