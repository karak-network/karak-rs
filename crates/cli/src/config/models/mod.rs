use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: ConfigVersion,
    pub chain: Option<Chain>,
    pub keystore: Keystore,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Chain {
    #[serde(rename = "evm")]
    Evm { id: u64, rpc_url: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Keystore {
    #[serde(rename = "local")]
    Local { path: PathBuf },
    #[serde(rename = "aws")]
    Aws, // For now, we just read the env but maybe we can make this more expressive?
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigVersion {
    #[serde(rename = "v0")]
    V0,
}
