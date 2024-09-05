use std::fmt::Display;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Serialize};

use super::algebra::{g1::G1Point, g2::G2Point};

pub type G1Pubkey = G1Point;
pub type G2Pubkey = G2Point;

#[derive(
    Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize, Serialize, Deserialize,
)]
pub struct PublicKey {
    pub g1: G1Pubkey,
    pub g2: G2Pubkey,
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.g2.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::keypair::{bn254::Keypair as Bn254Keypair, traits::Keypair};

    use super::*;

    #[test]
    fn test_serialize() {
        let keypair = Bn254Keypair::generate();
        let pubkey = keypair.public_key().clone();
        let serialized = serde_json::to_string(&pubkey).unwrap();
        let deserialized = serde_json::from_str::<PublicKey>(&serialized).unwrap();
        assert_eq!(pubkey, deserialized);
    }
}
