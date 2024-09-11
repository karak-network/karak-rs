use aes::{
    cipher::{KeyIvInit, StreamCipher},
    Aes128,
};
use ctr::Ctr64BE;
use hex::encode;
use rand::RngCore;
use scrypt::{scrypt, Params};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use thiserror::Error;

type Aes128Ctr64BE = Ctr64BE<Aes128>;
const SCRYPT_DKLEN: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptDataV3Payload {
    pub cipher: String,
    pub cipher_text: String,
    pub cipher_params: HashMap<String, String>,
    pub kdf: String,
    pub kdf_params: HashMap<String, String>,
    pub mac: String,
}

#[derive(Debug, Error)]
pub enum EncryptDataV3Error {
    #[error("Invalid output length")]
    InvalidOutputLen(#[from] scrypt::errors::InvalidOutputLen),
    #[error("Invalid scrypt parameters")]
    InvalidParams(#[from] scrypt::errors::InvalidParams),
    #[error("Invalid cipher length")]
    InvalidCipherLength(aes::cipher::InvalidLength),
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Failed to decode hex string: {0}")]
    HexDecodeError(#[from] hex::FromHexError),
    #[error("MAC verification failed")]
    MacVerificationFailed,
}

impl From<aes::cipher::InvalidLength> for EncryptDataV3Error {
    fn from(_: aes::cipher::InvalidLength) -> Self {
        EncryptDataV3Error::InvalidCipherLength(aes::cipher::InvalidLength)
    }
}

pub fn encrypt_data_v3(
    data: &[u8],
    auth: &[u8],
    scrypt_log_n: u8,
    scrypt_p: u32,
) -> Result<EncryptDataV3Payload, EncryptDataV3Error> {
    let scrypt_r: u32 = 8;

    let mut salt = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);

    let mut derived_key = [0u8; SCRYPT_DKLEN];
    let scrypt_params = Params::new(scrypt_log_n, scrypt_r, scrypt_p, SCRYPT_DKLEN)?;
    scrypt(auth, &salt, &scrypt_params, &mut derived_key)?;

    let encrypt_key = &derived_key[..16];

    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut iv);

    let mut cipher = Aes128Ctr64BE::new_from_slices(encrypt_key, &iv)?;
    let mut cipher_text = data.to_vec();
    cipher.apply_keystream(&mut cipher_text);

    let mac = {
        let mut hasher = Keccak256::new();
        hasher.update(&derived_key[16..32]);
        hasher.update(&cipher_text);
        hasher.finalize()
    };

    let mut scrypt_params_json = HashMap::new();
    scrypt_params_json.insert("log_n".to_string(), scrypt_log_n.to_string());
    scrypt_params_json.insert("r".to_string(), scrypt_r.to_string());
    scrypt_params_json.insert("p".to_string(), scrypt_p.to_string());
    scrypt_params_json.insert("dklen".to_string(), SCRYPT_DKLEN.to_string());
    scrypt_params_json.insert("salt".to_string(), encode(salt));

    let mut cipher_params_json = HashMap::new();
    cipher_params_json.insert("iv".to_string(), encode(iv));

    let crypto_struct = EncryptDataV3Payload {
        cipher: "aes-128-ctr".to_string(),
        cipher_text: encode(cipher_text),
        cipher_params: cipher_params_json,
        kdf: "scrypt".to_string(),
        kdf_params: scrypt_params_json,
        mac: encode(mac),
    };

    Ok(crypto_struct)
}

pub fn decrypt_data_v3(
    crypto: &EncryptDataV3Payload,
    auth: &[u8],
) -> Result<Vec<u8>, EncryptDataV3Error> {
    // Extract parameters from the crypto struct
    let scrypt_log_n: u8 = crypto.kdf_params["log_n"].parse()?;
    let scrypt_r: u32 = crypto.kdf_params["r"].parse()?;
    let scrypt_p: u32 = crypto.kdf_params["p"].parse()?;
    let scrypt_dklen: usize = crypto.kdf_params["dklen"].parse()?;
    let salt = hex::decode(&crypto.kdf_params["salt"])?;
    let iv = hex::decode(&crypto.cipher_params["iv"])?;

    // Derive the key using scrypt
    let scrypt_params = Params::new(scrypt_log_n, scrypt_r, scrypt_p, scrypt_dklen)?;
    let mut derived_key = vec![0u8; scrypt_dklen];
    scrypt(auth, &salt, &scrypt_params, &mut derived_key)?;

    let decrypt_key = &derived_key[..16];
    let mac_key = &derived_key[16..32];

    // Verify the MAC
    let cipher_text = hex::decode(&crypto.cipher_text)?;
    let mut hasher = Keccak256::new();
    hasher.update(mac_key);
    hasher.update(&cipher_text);
    let expected_mac = hasher.finalize();

    if expected_mac.as_slice() != hex::decode(&crypto.mac)? {
        return Err(EncryptDataV3Error::MacVerificationFailed);
    }

    // Decrypt the cipher text
    let mut cipher = Aes128Ctr64BE::new_from_slices(decrypt_key, &iv)?;
    let mut plain_text = cipher_text.to_vec();
    cipher.apply_keystream(&mut plain_text);

    Ok(plain_text)
}
