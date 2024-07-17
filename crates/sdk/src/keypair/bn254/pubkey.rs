use std::{fmt::Display, ops::Deref};

use ark_bn254::{G1Affine, G2Affine};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};

pub struct G1Pubkey(G1Affine);
pub struct G2Pubkey(G2Affine);

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

impl From<G2Affine> for G2Pubkey {
    fn from(value: G2Affine) -> Self {
        Self(value)
    }
}

impl TryFrom<&G2Pubkey> for Vec<u8> {
    type Error = SerializationError;

    fn try_from(value: &G2Pubkey) -> Result<Self, Self::Error> {
        let mut bytes = vec![];

        value.serialize_with_mode(&mut bytes, Compress::Yes)?;

        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for G2Pubkey {
    type Error = SerializationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(G2Affine::deserialize_with_mode(value, Compress::Yes, Validate::Yes)?.into())
    }
}

impl TryFrom<&str> for G2Pubkey {
    type Error = SerializationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        bs58::decode(value)
            .into_vec()
            .map_err(|_| SerializationError::InvalidData)?
            .as_slice()
            .try_into()
    }
}

impl Display for G2Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pubkey_bytes = vec![];
        self.serialize_with_mode(&mut pubkey_bytes, Compress::Yes)
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
