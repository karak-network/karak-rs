use std::collections::HashMap;
use std::path::PathBuf;

use crate::shared::Url;
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr, VariantNames};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub chain: Chain,
    pub core_address: Address,
    pub bn254_keystore: Keystore,
    pub secp256k1_keystore: Keystore,
    pub key_generation_folder: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, VariantNames, FromRepr, Display)]
#[serde(tag = "type")]
pub enum Chain {
    #[serde(rename = "evm")]
    Evm { id: u64, rpc_url: Url },
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, VariantNames, FromRepr, Display)]
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
