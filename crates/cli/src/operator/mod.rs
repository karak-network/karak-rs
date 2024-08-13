pub mod processor;

use alloy::primitives::Address;
use clap::Subcommand;
use url::Url;

use crate::{
    keypair::{KeypairArgs, KeypairLocationArgs},
    shared::Encoding,
};

#[derive(Subcommand)]
pub enum Operator {
    /// Perform BLS registration to DSS
    Register {
        #[command(flatten)]
        bn254_keypair_location: KeypairLocationArgs,
        #[command(flatten)]
        bn254_keypair: KeypairArgs,

        #[command(flatten)]
        secp256k1_keypair_location: KeypairLocationArgs,
        #[command(flatten)]
        secp256k1_keypair: KeypairArgs,

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
