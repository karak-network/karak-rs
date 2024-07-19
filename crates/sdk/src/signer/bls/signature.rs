use std::{fmt::Display, str::FromStr};

use ark_bn254::G1Affine;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use thiserror::Error;

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct Signature(pub(crate) G1Affine);

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Serialization error: {0}")]
    SerializationError(SerializationError),
    #[error("Invalid signature")]
    DecodingError(#[from] bs58::decode::Error),
}

impl From<SerializationError> for SignatureError {
    fn from(value: SerializationError) -> Self {
        Self::SerializationError(value)
    }
}

impl FromStr for Signature {
    type Err = SignatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec()?;
        let sig = Signature::try_from(bytes.as_slice())?;
        Ok(sig)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = vec![];
        self.serialize_compressed(&mut bytes)
            .map_err(|_| std::fmt::Error)?;
        write!(f, "{}", bs58::encode(bytes).into_string())
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = SignatureError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let sig = Signature::deserialize_compressed(value)?;
        Ok(sig)
    }
}
