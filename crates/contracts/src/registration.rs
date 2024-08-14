use alloy::{
    primitives::{Address, Bytes, TxHash},
    providers::Provider,
    sol,
    sol_types::SolValue,
    transports::Transport,
};
use karak_sdk::keypair::bn254::algebra::{g1::G1Point, g2::G2Point};

use crate::Core::CoreInstance;

sol!(
    struct BlsRegistration {
        G1Point g1_pubkey;
        G2Point g2_pubkey;
        bytes32 msg_hash;
        G1Point signature;
    }
);

#[trait_variant::make(Send)]
pub trait OperatorRegistration {
    async fn register_operator_to_dss_with_data<B: Into<Bytes> + Send + Sync>(
        &self,
        dss: Address,
        data: B,
    ) -> eyre::Result<TxHash>;

    async fn register_operator_to_dss_with_bls(
        &self,
        dss: Address,
        registration: &BlsRegistration,
    ) -> eyre::Result<TxHash> {
        self.register_operator_to_dss_with_data(dss, registration.abi_encode())
    }
}

impl<T: Transport + Clone, P: Provider<T>> OperatorRegistration for CoreInstance<T, P> {
    async fn register_operator_to_dss_with_data<B: Into<Bytes> + Send + Sync>(
        &self,
        dss: Address,
        data: B,
    ) -> eyre::Result<TxHash> {
        let receipt = self
            .registerOperatorToDSS(dss, data.into())
            .send()
            .await?
            .get_receipt()
            .await?;
        Ok(receipt.transaction_hash)
    }
}
