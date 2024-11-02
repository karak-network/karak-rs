use std::{path::PathBuf, str::FromStr};

use alloy::signers::local::LocalSigner;
use color_eyre::eyre;
use karak_kms::{
    keypair::bn254,
    keystore::{
        self,
        aws::AwsKeystoreParams,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
};

use crate::{
    config::models::Keystore,
    keypair::{KeypairArgs, KeypairLocationArgs},
    shared::Curve,
};

use super::prompt;

pub async fn process_pubkey(
    keypair_args: Option<KeypairArgs>,
    keypair_location_args: Option<KeypairLocationArgs>,
    curve: Option<Curve>,
) -> eyre::Result<()> {
    let keypair_args = prompt::prompt_keypair_args(keypair_args);
    let curve = prompt::prompt_curve(curve);
    let keypair_location_args = prompt::prompt_keypair_location_args(keypair_location_args);

    let KeypairArgs {
        keystore,
        passphrase,
    } = keypair_args;
    let KeypairLocationArgs { keypair } = keypair_location_args;

    // value will be set by prompts
    let keystore = keystore.unwrap();
    let keypair = keypair.unwrap();
    let passphrase = passphrase.unwrap();

    match curve {
        Curve::Bn254 => match keystore {
            Keystore::Local { path: _ } => {
                let local_keystore =
                    keystore::local::LocalEncryptedKeystore::new(PathBuf::from(keypair));

                let keypair: bn254::Keypair = local_keystore.retrieve(&passphrase)?;

                println!("Public Key (retrieved from local keystore): {keypair}");
            }
            Keystore::Aws { secret: _ } => {
                let config = aws_config::load_from_env().await;
                let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);

                let secret_name = format!("{keypair}.bls");

                let keypair: bn254::Keypair = aws_keystore
                    .retrieve(&passphrase, &AwsKeystoreParams { secret_name })
                    .await?;

                println!("Public Key (retrieved from AWS Secrets Manager): {keypair}");
            }
        },
        Curve::Secp256k1 => match keystore {
            Keystore::Local { path: _ } => {
                let keypath = PathBuf::from_str(&keypair)?;
                let private_key = LocalSigner::decrypt_keystore(keypath, passphrase)?;
                println!(
                    "Address (retrieved from local keystore): {}",
                    private_key.address()
                );
            }
            Keystore::Aws { secret: _ } => todo!(),
        },
    }
    Ok(())
}
