use std::fmt::Display;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use super::algebra::{g1::G1Point, g2::G2Point};

pub type G1Pubkey = G1Point;
pub type G2Pubkey = G2Point;

#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct PublicKey {
    pub g1: G1Pubkey,
    pub g2: G2Pubkey,
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.g2.fmt(f)
    }
}
