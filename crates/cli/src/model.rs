use alloy::primitives::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowlistedAsset {
    pub asset: Address,
    pub chain_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    pub address: Address,
    pub chain_id: u64,
    pub vaults: Vec<Vault>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vault {
    pub asset_address: Address,
}

#[derive(Debug, Deserialize)]
pub struct KarakBackendData<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct KarakBackendResult<T> {
    pub result: KarakBackendData<T>,
}
