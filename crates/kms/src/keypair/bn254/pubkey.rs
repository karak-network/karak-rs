use std::fmt::Display;

use ark_bn254::{G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::One;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Valid, Validate};
use serde::{Deserialize, Serialize};

use super::{
    algebra::{g1::G1Point, g2::G2Point},
    Bn254Error,
};

pub type G1Pubkey = G1Point;
pub type G2Pubkey = G2Point;

#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, Serialize, Deserialize, Hash)]
pub struct PublicKey {
    pub g1: G1Pubkey,
    pub g2: G2Pubkey,
}

impl PublicKey {
    pub fn new(g1: G1Pubkey, g2: G2Pubkey) -> Result<Self, Bn254Error> {
        let pubkey = Self { g1, g2 };
        pubkey.check()?;
        Ok(pubkey)
    }
}

impl Valid for PublicKey {
    fn check(&self) -> Result<(), ark_serialize::SerializationError> {
        self.g1.0.check()?;
        self.g2.0.check()?;

        let g1 = self.g1.0;
        let g2 = self.g2.0;
        let gen_g1 = G1Affine::generator();
        let gen_g2 = G2Affine::generator();

        let multi_pairing = ark_bn254::Bn254::multi_pairing([g1, -gen_g1], [gen_g2, g2]);

        if !multi_pairing.0.is_one() {
            return Err(ark_serialize::SerializationError::InvalidData);
        }

        Ok(())
    }
}

impl CanonicalDeserialize for PublicKey {
    fn deserialize_with_mode<R: ark_serialize::Read>(
        mut reader: R,
        compress: ark_serialize::Compress,
        validate: ark_serialize::Validate,
    ) -> Result<Self, ark_serialize::SerializationError> {
        let g1 = G1Pubkey::deserialize_with_mode(&mut reader, compress, validate)?;
        let g2 = G2Pubkey::deserialize_with_mode(&mut reader, compress, validate)?;
        let pubkey = Self { g1, g2 };

        if let Validate::Yes = validate {
            pubkey.check()?;
        }

        Ok(pubkey)
    }
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

    #[test]
    fn pubkeys_from_same_secret_key_should_pass_check() {
        let keypair = Bn254Keypair::generate();
        let pubkey = keypair.public_key();
        assert!(pubkey.check().is_ok());
    }

    #[test]
    fn pubkeys_from_different_secret_key_should_fail_check() {
        let keypair1 = Bn254Keypair::generate();
        let keypair2 = Bn254Keypair::generate();
        let pubkey1 = keypair1.public_key();
        let pubkey2 = keypair2.public_key();
        let pubkey = PublicKey {
            g1: pubkey1.g1,
            g2: pubkey2.g2,
        };
        assert!(pubkey.check().is_err());
    }
}
