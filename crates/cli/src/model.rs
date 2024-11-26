use alloy::primitives::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowlistedAsset {
    pub asset: Address,
    pub chain_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct KarakBackendData<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct KarakBackendResult<T> {
    pub result: KarakBackendData<T>,
}
