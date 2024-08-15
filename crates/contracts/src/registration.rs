use alloy::{
    primitives::{Address, Bytes, TxHash},
    providers::Provider,
    sol,
    sol_types::SolValue,
    transports::Transport,
};
use karak_sdk::{
    keypair::bn254::{G1Pubkey, G2Pubkey},
    signer::bls::signature::Signature,
};

use crate::Core::CoreInstance;

sol!(
    struct BlsRegistration {
        G1Pubkey g1_pubkey;
        G2Pubkey g2_pubkey;
        Signature signature;
        bytes32 msg_hash;
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
