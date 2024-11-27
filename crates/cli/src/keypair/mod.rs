pub mod processor;

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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

        /// Passphrase to encrypt keypair
        #[arg(long)]
        passphrase: Option<String>,
    },
    /// List keypairs
    List {
        /// Curve to list keypairs for
        #[arg(long, value_parser = crate::clap_enum_variants!(Curve))]
        curve: Option<Curve>,
    },
    /// Add existing keypair to keystore
    Add {
        #[command(flatten)]
        keypair_args: Option<KeypairArgs>,

        #[command(flatten)]
        aws_config: Option<AwsKeypairConfig>,

        #[command(flatten)]
        local_config: Option<LocalKeypairConfig>,
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
pub struct LocalKeypairConfig {
    /// Path to the keystore, if using local keystore
    #[arg(long, required_if_eq("keystore", "local"), global(true))]
    keystore_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct AwsKeypairConfig {
    /// AWS profile to use, if using AWS keystore
    #[arg(long, required_if_eq("keystore", "aws"), global(true))]
    profile: Option<String>,

    /// AWS secret name to use, if using AWS keystore
    #[arg(long, required_if_eq("keystore", "aws"), global(true))]
    secret_name: Option<String>,
}

#[derive(Args, Debug)]
pub struct KeypairArgs {
    /// Keystore name
    #[arg(long)]
    pub keystore_name: Option<String>,

    /// Keystore to save the keypair
    #[arg(long, value_parser = crate::clap_enum_variants!(Keystore))]
    pub keystore: Option<Keystore>,

    /// Curve to use for key generation
    #[arg(long, value_parser = crate::clap_enum_variants!(Curve))]
    pub curve: Option<Curve>,
}
