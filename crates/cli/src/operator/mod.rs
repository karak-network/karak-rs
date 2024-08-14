pub mod processor;

use alloy::primitives::Address;
use clap::Subcommand;
use url::Url;

use crate::shared::{Encoding, Keystore};

#[derive(Subcommand)]
pub enum Operator {
    /// Perform BLS registration to DSS
    Register {
        #[arg(long)]
        bn254_keypair_location: String,
        #[arg(long)]
        bn254_keystore: Keystore,
        #[arg(long)]
        bn254_passphrase: Option<String>,

        #[arg(long)]
        secp256k1_keypair_location: String,
        #[arg(long)]
        secp256k1_keystore: Keystore,
        #[arg(long)]
        secp256k1_passphrase: Option<String>,

        #[arg(short, long, default_value = "http://localhost:8545")]
        /// RPC endpoint
        rpc_url: Url,

        #[arg(short, long)]
        /// Core addresss
        core_address: Address,

        #[arg(short, long)]
        /// DSS address
        dss_address: Address,

        #[arg(long)]
        message: String,

        #[arg(long)]
        message_encoding: Encoding,
    },
}
