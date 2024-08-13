use std::str::FromStr;

use base64::Engine;
use karak_sdk::{
    keypair::bn254::G2Pubkey,
    signer::bls::{keypair_signer::verify_signature, signature::Signature},
};
use sha3::{Digest, Keccak256};

use crate::{bls::MessageArgs, shared::Encoding};

pub fn process_verify(
    message_args: MessageArgs,
    pubkey: String,
    signature: String,
) -> color_eyre::Result<()> {
    let MessageArgs {
        message,
        message_encoding,
    } = message_args;

    let message_bytes = match message_encoding {
        Encoding::Utf8 => message.as_bytes().to_vec(),
        Encoding::Hex => hex::decode(message)?,
        Encoding::Base64 => base64::engine::general_purpose::STANDARD.decode(message)?,
        Encoding::Base64URL => base64::engine::general_purpose::URL_SAFE.decode(message)?,
        Encoding::Base58 => bs58::decode(message).into_vec()?,
    };

    // We Keccak256 hash the message to a 32 bytes hash

    let mut hasher = Keccak256::new();
    hasher.update(message_bytes);
    let result = hasher.finalize();

    let mut hash_buffer = [0u8; 32];
    hash_buffer.copy_from_slice(&result);

    let public_key = G2Pubkey::from_str(&pubkey)?;
    let signature = Signature::from_str(&signature)?;

    match verify_signature(&public_key, &signature, &hash_buffer) {
        Ok(_) => println!("Signature is valid"),
        _ => println!("Signature verification failed!"),
    };

    Ok(())
}
