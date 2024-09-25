use std::{path::PathBuf, str::FromStr};

use color_eyre::eyre;
use karak_bls::{
    keypair_signer::{verify_signature, KeypairSigner},
    signature::Signature,
};
use karak_kms::{
    keypair::bn254::{self, Bn254Error, G2Pubkey},
    keystore::{
        self,
        aws::AwsKeystoreParams,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
    signer::traits::Signer,
};
use sha3::{Digest, Keccak256};

use crate::{
    common::{Encoding, Keystore},
    components::dss::keypair::{KeypairArgs, KeypairLocationArgs},
};

pub enum AggregateParams {
    Signatures(Vec<String>),
    Pubkeys(Vec<String>),
}

pub struct MessageArgs {
    pub message: String,
    pub message_encoding: Encoding,
}

pub fn process_aggregate(params: AggregateParams) -> eyre::Result<()> {
    match params {
        AggregateParams::Signatures(signatures) => {
            let signatures: Vec<Signature> = signatures
                .iter()
                .map(|signature| Signature::from_str(signature))
                .collect::<Result<Vec<Signature>, Bn254Error>>()?;

            let agg_signature: Signature = signatures.iter().sum();

            println!("Aggregated signature: {agg_signature}");

            Ok(())
        }
        AggregateParams::Pubkeys(pubkeys) => {
            let pubkeys: Vec<G2Pubkey> = pubkeys
                .iter()
                .map(|pubkey| G2Pubkey::from_str(pubkey))
                .collect::<Result<Vec<G2Pubkey>, Bn254Error>>()?;

            let agg_key: G2Pubkey = pubkeys.iter().sum();

            println!("Aggregated key: {agg_key}");

            Ok(())
        }
    }
}

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

    let signer = KeypairSigner::from(keypair);

    let signature = signer.sign_message(hash_buffer)?;
    println!("Signature: {signature}");

    Ok(())
}

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
