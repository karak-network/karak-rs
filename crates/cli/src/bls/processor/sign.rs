use std::path::PathBuf;

use color_eyre::eyre;
use karak_sdk::{
    keypair::bn254,
    keystore::{
        self,
        aws::AwsKeystoreParams,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
    signer::{bls, traits::Signer},
};
use sha3::{Digest, Keccak256};

use crate::{
    bls::MessageArgs,
    keypair::{KeypairArgs, KeypairLocationArgs},
    shared::Keystore,
};

pub async fn process_sign(
    keypair_location: KeypairLocationArgs,
    keypair: KeypairArgs,
    message: MessageArgs,
) -> eyre::Result<()> {
    let KeypairArgs {
        keystore,
        passphrase,
        ..
    } = keypair;
    let KeypairLocationArgs { keypair } = keypair_location;
    let MessageArgs {
        message,
        message_encoding,
    } = message;

    let message_bytes = message_encoding.decode(&message)?;

    // We Keccak256 hash the message to a 32 bytes hash

    let mut hasher = Keccak256::new();
    hasher.update(message_bytes);
    let result = hasher.finalize();

    let mut hash_buffer = [0u8; 32];
    hash_buffer.copy_from_slice(&result);

    let passphrase = match passphrase {
        Some(passphrase) => passphrase,
        None => rpassword::prompt_password("Enter keypair passphrase: ")?,
    };

    let keypair: bn254::Keypair = {
        match keystore {
            Keystore::Local => {
                let local_keystore =
                    keystore::local::LocalEncryptedKeystore::new(PathBuf::from(keypair));
                local_keystore.retrieve(&passphrase)?
            }
            Keystore::Aws => {
                let config = aws_config::load_from_env().await;
                let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);
                let secret_name = format!("{keypair}.bls");
                aws_keystore
                    .retrieve(&passphrase, &AwsKeystoreParams { secret_name })
                    .await?
            }
        }
    };

    println!("Signing with BN254 keypair: {keypair}");

    let signer = bls::keypair_signer::KeypairSigner::from(keypair);

    let signature = signer.sign_message(hash_buffer)?;
    println!("Signature: {signature}");

    Ok(())
}
