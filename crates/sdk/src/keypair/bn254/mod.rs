use std::fmt::Display;

use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::UniformRand;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};
use rand::thread_rng;

use super::traits::Keypair as KeypairTrait;

mod pubkey;
pub use pubkey::*;

pub struct Keypair {
    secret_key: Fr,
    public_key: PublicKey,
}

impl TryFrom<&Keypair> for Vec<u8> {
    type Error = SerializationError;

    fn try_from(value: &Keypair) -> Result<Self, Self::Error> {
        let mut bytes = vec![];

        value
            .secret_key
            .serialize_with_mode(&mut bytes, Compress::No)?;

        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for Keypair {
    type Error = SerializationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let secret_key = Fr::deserialize_with_mode(value, Compress::No, Validate::Yes)?;

        let g1_public_key = (G1Affine::generator() * secret_key).into_affine();
        let g2_public_key = (G2Affine::generator() * secret_key).into_affine();

        Ok(Self {
            secret_key,
            public_key: PublicKey {
                g1: g1_public_key.into(),
                g2: g2_public_key.into(),
            },
        })
    }
}

impl KeypairTrait for Keypair {
    type SecretKey = Fr;
    type PublicKey = PublicKey;

    fn generate() -> Self {
        let mut rng = thread_rng();
        let secret_key = Fr::rand(&mut rng);
        let g1_public_key = (G1Affine::generator() * secret_key).into_affine();
        let g2_public_key = (G2Affine::generator() * secret_key).into_affine();

        Self {
            secret_key,
            public_key: PublicKey {
                g1: g1_public_key.into(),
                g2: g2_public_key.into(),
            },
        }
    }

    fn secret_key(&self) -> &Self::SecretKey {
        &self.secret_key
    }

    fn public_key(&self) -> &Self::PublicKey {
        &self.public_key
    }
}

impl Display for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.public_key.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    // TODO: More exhaustive tests with pre-loaded keypairs
    //       The above will need methods to load keypairs from bytes

    use super::*;

    #[test]
    fn test_generate_bn254_keypair() {
        let keypair = Keypair::generate();
        let mut secret_key_bytes = vec![];
        let mut public_key_bytes = vec![];

        keypair
            .secret_key()
            .0
            .serialize_with_mode(&mut secret_key_bytes, Compress::Yes)
            .unwrap();

        keypair
            .public_key()
            .g2
            .serialize_with_mode(&mut public_key_bytes, Compress::Yes)
            .unwrap();

        assert_eq!(secret_key_bytes.len(), 32);
        assert_eq!(public_key_bytes.len(), 32);
    }
}
