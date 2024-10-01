use std::collections::HashMap;
use std::path::PathBuf;

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub chain: Chain,
    pub bn254_keystore: Keystore,
    pub secp256k1_keystore: Keystore,
    pub karak_address: Address,
    pub key_generation_folder: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Chain {
    #[serde(rename = "evm")]
    Evm { id: u64, rpc_url: url::Url },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Keystore {
    #[serde(rename = "local")]
    Local { path: PathBuf },
    #[serde(rename = "aws")]
    Aws { secret: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}
