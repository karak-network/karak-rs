use aes::cipher::{NewCipher, StreamCipher};
use aes::Aes128Ctr;
use hex::encode;
use rand::RngCore;
use scrypt::{scrypt, Params};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoJSON {
    pub cipher: String,
    pub cipher_text: String,
    pub cipher_params: HashMap<String, String>,
    pub kdf: String,
    pub kdf_params: HashMap<String, String>,
    pub mac: String,
}

pub fn encrypt_data_v3(
    data: &[u8],
    auth: &[u8],
    scrypt_log_n: u8,
    scrypt_p: u32,
) -> Result<CryptoJSON, Box<dyn std::error::Error>> {
    let scrypt_r = 8;
    let scrypt_dklen = 32;

    let mut salt = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);

    let mut derived_key = vec![0u8; scrypt_dklen];
    let scrypt_params = Params::new(scrypt_log_n, scrypt_r, scrypt_p)?;
    scrypt(auth, &salt, &scrypt_params, &mut derived_key)?;

    let encrypt_key = &derived_key[..16];

    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut iv);

    let mut cipher = Aes128Ctr::new_from_slices(encrypt_key, &iv).unwrap();
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
    scrypt_params_json.insert("dklen".to_string(), scrypt_dklen.to_string());
    scrypt_params_json.insert("salt".to_string(), encode(salt));

    let mut cipher_params_json = HashMap::new();
    cipher_params_json.insert("iv".to_string(), encode(iv));

    let crypto_struct = CryptoJSON {
        cipher: "aes-128-ctr".to_string(),
        cipher_text: encode(cipher_text),
        cipher_params: cipher_params_json,
        kdf: "scrypt".to_string(),
        kdf_params: scrypt_params_json,
        mac: encode(mac.to_vec()),
    };

    Ok(crypto_struct)
}

pub fn decrypt_data_v3(
    crypto: &CryptoJSON,
    auth: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Extract parameters from the crypto struct
    let scrypt_log_n: u8 = crypto.kdf_params["log_n"].parse()?;
    let scrypt_r: u32 = crypto.kdf_params["r"].parse()?;
    let scrypt_p: u32 = crypto.kdf_params["p"].parse()?;
    let scrypt_dklen: usize = crypto.kdf_params["dklen"].parse()?;
    let salt = hex::decode(&crypto.kdf_params["salt"])?;
    let iv = hex::decode(&crypto.cipher_params["iv"])?;

    // Derive the key using scrypt
    let scrypt_params = Params::new(scrypt_log_n, scrypt_r, scrypt_p)?;
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
        return Err("MAC verification failed".into());
    }

    // Decrypt the cipher text
    let mut cipher = Aes128Ctr::new_from_slices(decrypt_key, &iv).unwrap();
    let mut plain_text = cipher_text.to_vec();
    cipher.apply_keystream(&mut plain_text);

    Ok(plain_text)
}

pub fn save_to_file(crypto: &CryptoJSON, file_path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(crypto)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_from_file(file_path: &Path) -> io::Result<CryptoJSON> {
    let mut file = File::open(file_path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let crypto: CryptoJSON = serde_json::from_str(&json)?;
    Ok(crypto)
}
