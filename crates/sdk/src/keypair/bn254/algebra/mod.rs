use ark_serialize::SerializationError;
use thiserror::Error;

pub mod g1;
pub mod g2;

#[derive(Debug, Error)]
pub enum AlgebraError {
    #[error("Serialization error: {0}")]
    SerializationError(SerializationError),
    #[error("Decoding error: {0}")]
    DecodingError(#[from] bs58::decode::Error),
}

impl From<SerializationError> for AlgebraError {
    fn from(error: SerializationError) -> Self {
        AlgebraError::SerializationError(error)
    }
}
