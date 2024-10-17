pub mod processor;

use std::path::PathBuf;

use alloy::primitives::{Address, Bytes};
use clap::{Args, Subcommand};
use url::Url;

use crate::shared::{Encoding, Keystore};

#[derive(Debug, Subcommand)]
pub enum OperatorCommand {
    /// Perform BLS registration to DSS
    RegisterToDSS {
        #[arg(long)]
        bn254_keypair_location: String,
        #[arg(long)]
        bn254_keystore: Keystore,
        #[arg(long)]
        bn254_passphrase: Option<String>,

        /// DSS address
        #[arg(short, long)]
        dss_address: Address,

        #[arg(long)]
        message: String,

        #[arg(long)]
        message_encoding: Encoding,
    },

    /// Perform Core registration
    CreateVault {
        #[arg(long)]
        asset_address: Address,

        #[arg(long)]
        extra_data: Option<Bytes>,
    },
}

#[derive(Debug, Args)]
pub struct OperatorArgs {
    #[command(subcommand)]
    pub command: OperatorCommand,

    #[arg(long)]
    secp256k1_keystore: Keystore,

    #[arg(long, required_if_eq("secp256k1_keystore", "Local"))]
    secp256k1_keypair_location: Option<PathBuf>,

    #[arg(long)]
    secp256k1_passphrase: Option<String>,

    #[arg(short, long, default_value = "http://localhost:8545")]
    /// RPC endpoint
    rpc_url: Url,
    #[arg(short, long)]
    /// Core addresss
    core_address: Address,
}
