pub mod processor;

use alloy::primitives::Address;
use clap::Subcommand;
use url::Url;

use crate::shared::{Encoding, KeystoreType};

#[derive(Subcommand, Debug)]
pub enum Operator {
    /// Perform BLS registration to DSS
    Register {
        #[arg(long, help = "Type of the BN254 keystore")]
        bn254_keystore_type: Option<KeystoreType>,

        #[arg(long, help = "Path to the BN254 keystore, either a file or an url")]
        bn254_keystore: Option<String>,

        #[arg(long)]
        bn254_passphrase: Option<String>,

        #[arg(long, help = "Type of the SECP256k1 keystore")]
        secp256k1_keystore_type: Option<KeystoreType>,

        #[arg(long, help = "Path to the SECP256k1 keystore, either a file or an url")]
        secp256k1_keystore: Option<String>,

        #[arg(long, help = "Passphrase for the SECP256k1 keystore")]
        secp256k1_passphrase: Option<String>,

        #[arg(short, long, help = "RPC endpoint overriding config rpc_url")]
        rpc_url: Option<Url>,

        #[arg(short, long, help = "DSS contract address")]
        dss_address: Address,

        #[arg(long)]
        message: String,

        #[arg(long)]
        message_encoding: Encoding,
    },
}
