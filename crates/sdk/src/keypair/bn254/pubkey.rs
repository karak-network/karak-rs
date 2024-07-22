use std::{
    fmt::Display,
    ops::{Add, Deref, Sub},
    str::FromStr,
};

use ark_bn254::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use thiserror::Error;

#[derive(Default, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct G1Pubkey(pub(crate) G1Affine);
#[derive(Default, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct G2Pubkey(pub(crate) G2Affine);

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct PublicKey {
    pub g1: G1Pubkey,
    pub g2: G2Pubkey,
}

impl Deref for G1Pubkey {
    type Target = G1Affine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for &G1Pubkey {
    type Output = G1Pubkey;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO: Can we optimize this by just storing the projective representations?
        G1Pubkey((self.0 + rhs.0).into())
    }
}

impl Sub for &G1Pubkey {
    type Output = G1Pubkey;

    fn sub(self, rhs: Self) -> Self::Output {
        // TODO: Can we optimize this by just storing the projective representations?
        G1Pubkey((self.0 - rhs.0).into())
    }
}

impl From<G1Affine> for G1Pubkey {
    fn from(value: G1Affine) -> Self {
        Self(value)
    }
}

impl Deref for G2Pubkey {
    type Target = G2Affine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for &G2Pubkey {
    type Output = G2Pubkey;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO: Can we optimize this by just storing the projective representations?
        G2Pubkey((self.0 + rhs.0).into())
    }
}

impl Sub for &G2Pubkey {
    type Output = G2Pubkey;

    fn sub(self, rhs: Self) -> Self::Output {
        // TODO: Can we optimize this by just storing the projective representations?
        G2Pubkey((self.0 - rhs.0).into())
    }
}

impl From<G2Affine> for G2Pubkey {
    fn from(value: G2Affine) -> Self {
        Self(value)
    }
}

#[derive(Debug, Error)]
pub enum PublicKeyError {
    #[error("Serialization error: {0}")]
    SerializationError(SerializationError),
    #[error("Decoding error: {0}")]
    DecodingError(#[from] bs58::decode::Error),
}

impl From<SerializationError> for PublicKeyError {
    fn from(value: SerializationError) -> Self {
        Self::SerializationError(value)
    }
}

impl FromStr for G2Pubkey {
    type Err = PublicKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(G2Pubkey::deserialize_compressed(
            bs58::decode(value).into_vec()?.as_slice(),
        )?)
    }
}

impl Display for G2Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pubkey_bytes = vec![];
        self.serialize_compressed(&mut pubkey_bytes)
            .map_err(|_| std::fmt::Error)?;

        write!(f, "{}", bs58::encode(pubkey_bytes).into_string())?;

        Ok(())
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.g2.fmt(f)
    }
}
