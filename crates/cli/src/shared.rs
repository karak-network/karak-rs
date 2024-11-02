use base64::Engine;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr, VariantNames};
use thiserror::Error;
use url::Url as Url_;

#[derive(Clone, ValueEnum, Debug, FromRepr, EnumString, Display, VariantNames)]
pub enum Curve {
    /// BN254 (also known as alt_bn128) is the curve used in Ethereum for BLS aggregation
    Bn254,

    /// secp256k1 is the curve used in Ethereum for ECDSA signatures
    Secp256k1,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Scheme {
    /// Boneh–Lynn–Shacham (BLS) signature scheme using BN254
    Bls,
}

// TODO: Move the encoding enum to the SDK crate
#[derive(Clone, ValueEnum, Debug)]
pub enum Encoding {
    Utf8,
    Hex,
    Base64,
    Base64URL,
    Base58,
}

#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Decoding UTF8 error")]
    DecodeUtf8Error,
    #[error("Decoding Hex error: {0}")]
    DecodeHexError(#[from] hex::FromHexError),
    #[error("Decoding Base64 error: {0}")]
    DecodeBase64Error(#[from] base64::DecodeError),
    #[error("Decoding Base58 error: {0}")]
    DecodeBase58Error(#[from] bs58::decode::Error),
}

impl Encoding {
    pub fn decode(&self, input: &str) -> Result<Vec<u8>, EncodingError> {
        let decoded = match self {
            Self::Utf8 => input.as_bytes().to_vec(),
            Self::Hex => hex::decode(input)?,
            Self::Base64 => base64::engine::general_purpose::STANDARD.decode(input)?,
            Self::Base64URL => base64::engine::general_purpose::URL_SAFE.decode(input)?,
            Self::Base58 => bs58::decode(input).into_vec()?,
        };

        Ok(decoded)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url(Url_);

impl Default for Url {
    fn default() -> Self {
        Url(Url_::parse("http://localhost:8545").unwrap())
    }
}

impl std::str::FromStr for Url {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url(Url_::parse(s)?))
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
