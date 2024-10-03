use std::{fmt::Display, str::FromStr};

use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::UniformRand;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Valid, Validate,
};
use rand::thread_rng;
use serde::Deserialize;
use thiserror::Error;

use super::traits::Keypair as KeypairTrait;

pub mod algebra;
pub mod bls;
mod encryption;
mod pubkey;
pub use encryption::*;
pub use pubkey::*;

#[derive(Clone, Debug)]
pub struct Keypair {
    secret_key: Fr,
    public_key: PublicKey,
}

#[derive(Debug, Error)]
pub enum Bn254Error {
    #[error("Serialization error: {0}")]
    SerializationError(SerializationError),
    #[error("Decoding error: {0}")]
    DecodingError(#[from] bs58::decode::Error),
    #[error("Invalid public key")]
    InvalidPublicKey,
}

impl Keypair {
    pub fn new(secret_key: Fr, public_key: PublicKey) -> Result<Self, Bn254Error> {
        let keypair = Self {
            secret_key,
            public_key,
        };

        keypair.check()?;

        Ok(keypair)
    }
}

impl From<SerializationError> for Bn254Error {
    fn from(error: SerializationError) -> Self {
        Bn254Error::SerializationError(error)
    }
}

impl CanonicalSerialize for Keypair {
    fn serialize_with_mode<W: ark_serialize::Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.secret_key.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.secret_key.serialized_size(compress)
    }
}

impl CanonicalDeserialize for Keypair {
    fn deserialize_with_mode<R: ark_serialize::Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let secret_key = Fr::deserialize_with_mode(reader, compress, validate)?;

        let g1_public_key = (G1Affine::generator() * secret_key).into_affine();
        let g2_public_key = (G2Affine::generator() * secret_key).into_affine();

        let public_key = PublicKey {
            g1: g1_public_key.into(),
            g2: g2_public_key.into(),
        };

        let keypair = Keypair {
            secret_key,
            public_key,
        };

        if let Validate::Yes = validate {
            keypair.check()?;
        }

        Ok(keypair)
    }
}

impl Valid for Keypair {
    fn check(&self) -> Result<(), SerializationError> {
        self.secret_key.check()?;
        self.public_key.check()?;

        Ok(())
    }
}

impl Keypair {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Bn254Error> {
        let mut bytes = Vec::new();
        self.serialize_uncompressed(&mut bytes)?;
        Ok(bytes)
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, Bn254Error> {
        Ok(Keypair::deserialize_uncompressed(bytes.as_ref())?)
    }
}

impl TryFrom<&[u8]> for Keypair {
    type Error = Bn254Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Keypair::deserialize_uncompressed(value)?)
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

impl FromStr for Keypair {
    type Err = Bn254Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec()?;
        Keypair::from_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Keypair {
    fn deserialize<D>(deserializer: D) -> Result<Keypair, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Keypair::from_str(&s).map_err(serde::de::Error::custom)
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
        assert_eq!(public_key_bytes.len(), 64);
    }
}
