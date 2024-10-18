pub mod processor;

use std::path::PathBuf;

use alloy::{
    primitives::{aliases::U48, Address, Bytes},
    signers::local::PrivateKeySigner,
};
use clap::{Args, Subcommand};
use processor::stake::StakeUpdateType;
use url::Url;

use crate::shared::{Encoding, Keystore};

#[derive(Debug, Subcommand)]
pub enum OperatorCommand {
    /// Perform vault creation
    CreateVault {
        #[arg(long)]
        asset_address: Address,

        #[arg(long)]
        extra_data: Option<Bytes>,

        #[arg(long)]
        vault_impl: Address,

        /// Core address
        #[arg(short, long)]
        core_address: Address,
    },

    /// Perform registration with the registry
    RegisterToRegistry {
        #[arg(long)]
        registry_address: Address,

        #[arg(long)]
        kns: String,
    },

    /// Request a stake update
    RequestStakeUpdate {
        #[arg(long)]
        vault_address: Address,

        #[arg(long)]
        dss_address: Address,

        #[arg(long)]
        stake_update_type: StakeUpdateType,

        /// Core address
        #[arg(short, long)]
        core_address: Address,
    },

    /// Finalize a stake update
    FinalizeStakeUpdate {
        #[arg(long)]
        vault_address: Address,

        #[arg(long)]
        dss_address: Address,

        #[arg(long)]
        stake_update_type: StakeUpdateType,

        #[arg(long)]
        nonce: U48,

        #[arg(long)]
        start_timestamp: U48,

        /// Core address
        #[arg(short, long)]
        core_address: Address,
    },

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

        /// Core address
        #[arg(short, long)]
        core_address: Address,
    },
}

#[derive(Debug, Args)]
pub struct OperatorArgs {
    #[command(subcommand)]
    pub command: OperatorCommand,

    #[arg(long, global(true), default_value = "local")]
    secp256k1_keystore_type: Keystore,

    #[arg(long, required_if_eq("secp256k1_keystore_type", "local"), global(true))]
    secp256k1_keystore_path: Option<PathBuf>,

    #[arg(long, global(true))]
    secp256k1_passphrase: Option<String>,

    #[arg(long, global(true), required(false), hide(true))]
    secp256k1_keypair: PrivateKeySigner,

    /// RPC endpoint
    #[arg(short, long, global(true), default_value = "http://localhost:8545")]
    rpc_url: Url,
}
