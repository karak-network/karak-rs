use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: ConfigVersion,
    pub chain: Option<Chain>,
    pub keypair_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub enum Chain {
    Evm { id: u64, rpc_url: String },
}

#[derive(Serialize, Deserialize)]
pub enum ConfigVersion {
    #[serde(rename = "v0")]
    V0,
}
