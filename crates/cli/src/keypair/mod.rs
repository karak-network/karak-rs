pub mod processor;

use clap::{Args, Parser, Subcommand};

use crate::config::models::{Curve, Keystore};

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
        #[arg(long, value_parser = crate::clap_enum_variants!(Curve))]
        curve: Option<Curve>,
    },
    /// List keypairs
    List {
        /// Curve to list keypairs for
        #[arg(long, value_parser = crate::clap_enum_variants!(Curve))]
        curve: Option<Curve>,
    },
    /// View public key
    Pubkey {
        /// Keystore name to use
        #[arg(long)]
        keystore_name: Option<String>,

        /// Passphrase to decrypt keypair
        #[arg(long)]
        passphrase: Option<String>,

        /// Curve to use for key parsing
        #[arg(long, value_parser = crate::clap_enum_variants!(Curve))]
        curve: Option<Curve>,
    },
}

#[derive(Args, Debug)]
pub struct KeypairArgs {
    /// Keystore name
    #[arg(long)]
    pub keystore_name: Option<String>,

    /// Keystore to save the keypair
    #[arg(long, value_parser = crate::clap_enum_variants!(Keystore))]
    pub keystore: Option<Keystore>,

    /// Passphrase to encrypt keypair
    #[arg(long)]
    pub passphrase: Option<String>,
}
