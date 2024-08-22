use alloy::{
    primitives::{Address, Bytes, TxHash},
    providers::Provider,
    sol,
    sol_types::SolValue,
    transports::Transport,
};
use karak_kms::{
    keypair::bn254::{G1Pubkey, G2Pubkey},
    signer::bls::signature::Signature,
};

use crate::Core::CoreInstance;

sol!(
    #[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use alloy::primitives::{keccak256, U256};

    #[test]
    fn test_registration_abi_encode() -> eyre::Result<()> {
        let g1_x =
            U256::from_str("0xb77a215b4f5cdd99f5ba438cc4996175bc729449b2cb250c3fb74eed2aaee62")?;
        let g1_y =
            U256::from_str("0x2158bc12a3fd6ab600d7a48a8b99a5defb35ac5a5bf652e368aeda7b97234cb6")?;
        let g2_x0 =
            U256::from_str("0x13de1b6acc2713a59bb7b57b71dcc47bb51e92bb5ce2ea27c22545b09ceb9e9")?;
        let g2_x1 =
            U256::from_str("0x80ce0e41fd5f198b867e8541cbc00b059d6e93e3611c2e2d3b55a7f2b519265")?;
        let g2_y0 =
            U256::from_str("0x2c2970386ad634cd092b2fb6308af892adee32057f139b3a7b3c3fe9559ded4e")?;
        let g2_y1 =
            U256::from_str("0x1873fc36edfb182034b6e1bbe05375c36f5bbbc39903fb431185787f28cb533d")?;
        let signature_x =
            U256::from_str("0x1294534ba6b2dc743ea1e8050e9e62526b7cb6d094ac5383bdd496568b1be0ba")?;
        let signature_y =
            U256::from_str("0x220338c1a0124231fa2ed8b56305a670b0c1f2d2ab43ceb6f69c3c5f56e109cf")?;

        let g1_pubkey = G1Pubkey::from((g1_x, g1_y));
        let g2_pubkey = G2Pubkey::from(([g2_x1, g2_x0], [g2_y1, g2_y0]));
        let signature = Signature::from((signature_x, signature_y));
        let msg_hash = keccak256(b"test message");
        let registration = BlsRegistration {
            g1_pubkey,
            g2_pubkey,
            signature,
            msg_hash,
        };

        let encoded = registration.abi_encode();

        let expected = (
            (g1_x, g1_y),
            ([g2_x1, g2_x0], [g2_y1, g2_y0]),
            (signature_x, signature_y),
            msg_hash,
        )
            .abi_encode();

        assert_eq!(encoded, expected);

        let decoded = BlsRegistration::abi_decode(&encoded, true)?;
        assert_eq!(decoded, registration);
        Ok(())
    }
}
