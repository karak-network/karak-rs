use std::str::FromStr;

use color_eyre::eyre;
use karak_sdk::{
    keypair::bn254::G2Pubkey,
    signer::bls::{keypair_signer::verify_signature, signature::Signature},
};
use sha3::{Digest, Keccak256};

use crate::bls::MessageArgs;

pub fn process_verify(
    message_args: MessageArgs,
    pubkey: String,
    signature: String,
) -> eyre::Result<()> {
    let MessageArgs {
        message,
        message_encoding,
    } = message_args;

    let message_bytes = message_encoding.decode(&message)?;

    // We Keccak256 hash the message to a 32 bytes hash

    let mut hasher = Keccak256::new();
    hasher.update(message_bytes);
    let result = hasher.finalize();

    let mut hash_buffer = [0u8; 32];
    hash_buffer.copy_from_slice(&result);

    let public_key = G2Pubkey::from_str(&pubkey)?;
    let signature = Signature::from_str(&signature)?;

    match verify_signature(&public_key, &signature, hash_buffer) {
        Ok(_) => println!("Signature is valid"),
        _ => println!("Signature verification failed!"),
    };

    Ok(())
}
