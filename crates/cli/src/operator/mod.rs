pub mod processor;

use std::path::PathBuf;

use alloy::primitives::{aliases::U48, Address, U256};
use clap::{Args, Subcommand};
use processor::stake::StakeUpdateType;

use crate::config::models::Keystore;
use crate::shared::Encoding;

#[derive(Debug, Subcommand)]
pub enum OperatorCommand {
    /// Perform vault creation
    CreateVault {
        #[arg(long, required(false), value_delimiter(','))]
        assets: Option<Vec<Address>>,

        #[arg(long, required(false))]
        vault_impl: Option<Address>,
    },

    /// Perform registration with the registry
    RegisterToRegistry {
        #[arg(long)]
        registry_address: Option<Address>,

        #[arg(long)]
        kns: Option<String>,
    },

    /// Request a stake update
    RequestStakeUpdate {
        #[arg(long)]
        vault_address: Option<Address>,

        #[arg(long)]
        dss_address: Option<Address>,

        #[arg(long)]
        stake_update_type: Option<StakeUpdateType>,
    },

    /// Finalize a stake update
    FinalizeStakeUpdate {
        #[arg(long)]
        vault_address: Option<Address>,

        #[arg(long)]
        dss_address: Option<Address>,

        #[arg(long)]
        stake_update_type: Option<StakeUpdateType>,

        #[arg(long)]
        nonce: Option<U48>,

        #[arg(long)]
        start_timestamp: Option<U48>,
    },

    /// Perform BLS registration to DSS
    RegisterToDSS {
        #[arg(long)]
        bn254_keypair_location: Option<String>,

        #[arg(long)]
        bn254_keystore: Option<Keystore>,

        #[arg(long)]
        bn254_passphrase: Option<String>,

        /// DSS address
        #[arg(short, long)]
        dss_address: Option<Address>,

        #[arg(long)]
        message: Option<String>,

        #[arg(long)]
        message_encoding: Option<Encoding>,
    },

    /// Deposit to vault
    DepositToVault {
        #[arg(long)]
        vault_address: Option<Address>,

        #[arg(long)]
        amount: Option<U256>,
    },

    #[cfg(feature = "testnet")]
    /// Mint ERC20 tokens
    MintERC20 {
        #[arg(long)]
        asset_address: Option<Address>,

        #[arg(long)]
        amount: Option<U256>,
    },
}

#[derive(Debug, Args)]
pub struct OperatorArgs {
    #[command(subcommand)]
    pub command: OperatorCommand,

    #[arg(long, global(true))]
    secp256k1_keystore_type: Option<Keystore>,

    #[arg(long, required_if_eq("secp256k1_keystore_type", "local"), global(true))]
    secp256k1_keystore_path: Option<PathBuf>,

    #[arg(long, global(true))]
    secp256k1_passphrase: Option<String>,

    #[arg(long, required_if_eq("secp256k1_keystore_type", "aws"), global(true))]
    aws_region: Option<String>,

    #[arg(long, required_if_eq("secp256k1_keystore_type", "aws"), global(true))]
    aws_access_key_id: Option<String>,

    #[arg(long, required_if_eq("secp256k1_keystore_type", "aws"), global(true))]
    aws_secret_access_key: Option<String>,

    #[arg(long, required_if_eq("secp256k1_keystore_type", "aws"), global(true))]
    aws_operator_key_id: Option<String>,
}
