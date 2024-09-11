use alloy::{
    consensus::{SignableTransaction, TxLegacy},
    network::TxSigner,
    primitives::{Address, Bytes, TxKind, U256},
    rpc::client::{ClientBuilder, ReqwestClient},
    signers::Signature,
};
use async_trait::async_trait;
use serde::Serialize;
use url::Url;

/// A signer that sends an RPC request to sign a transaction remotely
/// Implements `eth_signTransaction` method of Consensys Web3 Signer
/// Reference: https://docs.web3signer.consensys.io/reference/api/json-rpc#eth_signtransaction
#[derive(Debug)]
pub struct Web3Signer {
    /// Client used to send an RPC request
    pub client: ReqwestClient,
    /// Address of the account that intends to sign a transaction.
    /// It must match the `from` field in the transaction.
    pub address: Address,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SignTransactionParams {
    from: Address,
    #[serde(default, skip_serializing_if = "TxKind::is_create")]
    to: TxKind,
    value: U256,
    #[serde(with = "alloy_serde::quantity")]
    gas: u128,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "alloy_serde::quantity::opt"
    )]
    gas_price: Option<u128>,
    #[serde(with = "alloy_serde::quantity")]
    nonce: u64,
    data: Bytes,
}

impl Web3Signer {
    pub fn new(address: Address, url: Url) -> Self {
        Web3Signer {
            client: ClientBuilder::default().http(url),
            address,
        }
    }
}

#[async_trait]
impl TxSigner<Signature> for Web3Signer {
    fn address(&self) -> Address {
        self.address
    }

    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> alloy::signers::Result<Signature> {
        let params = SignTransactionParams {
            from: self.address,
            to: tx.to(),
            value: tx.value(),
            gas: tx.gas_limit(),
            gas_price: tx.gas_price(),
            nonce: tx.nonce(),
            data: Bytes::copy_from_slice(tx.input()),
        };

        let response = self
            .client
            .request::<Vec<SignTransactionParams>, Bytes>("eth_signTransaction", vec![params])
            .await
            .map_err(alloy::signers::Error::other)?;

        let signed_tx = TxLegacy::decode_signed_fields(&mut response.as_ref())
            .map_err(alloy::signers::Error::other)?;

        Ok(*signed_tx.signature())
    }
}
