use std::collections::HashMap;
use std::path::PathBuf;

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr, VariantNames};

use crate::types::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub chain: Chain,
    pub core_address: Address,
    pub keystores: HashMap<Curve, HashMap<String, Keystore>>,
    pub key_generation_folder: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, VariantNames, FromRepr, Display)]
#[serde(tag = "type")]
pub enum Chain {
    #[serde(rename = "evm")]
    Evm { id: u64, rpc_url: Url },
}

impl Chain {
    pub fn rpc_url(&self) -> Url {
        match self {
            Chain::Evm { rpc_url, .. } => rpc_url.clone(),
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    EnumString,
    VariantNames,
    FromRepr,
    Display,
    Hash,
    Eq,
    PartialEq,
)]
pub enum Curve {
    /// BN254 (also known as alt_bn128) is the curve used in Ethereum for BLS aggregation
    #[serde(rename = "bn254")]
    #[strum(serialize = "bn254")]
    Bn254,

    /// secp256k1 is the curve used in Ethereum for ECDSA signatures
    #[serde(rename = "secp256k1")]
    #[strum(serialize = "secp256k1")]
    Secp256k1,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, VariantNames, FromRepr, Display)]
#[serde(tag = "type")]
pub enum Keystore {
    #[serde(rename = "local")]
    #[strum(serialize = "local")]
    Local { path: PathBuf },

    #[serde(rename = "aws")]
    #[strum(serialize = "aws")]
    Aws { secret: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}
