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
    keypair::{KeypairArgs, KeypairLocationArgs},
    shared::{Curve, KeystoreType},
};

pub async fn process_pubkey(
    keypair_args: KeypairArgs,
    keypair_location_args: KeypairLocationArgs,
    curve: Curve,
) -> eyre::Result<()> {
    let KeypairArgs {
        keystore,
        passphrase,
    } = keypair_args;
    let KeypairLocationArgs { keypair } = keypair_location_args;

    match curve {
        Curve::Bn254 => {
            let passphrase = match passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter keypair passphrase: ")?,
            };

            match keystore {
                KeystoreType::Local => {
                    let local_keystore =
                        keystore::local::LocalEncryptedKeystore::new(PathBuf::from(keypair));

                    let keypair: bn254::Keypair = local_keystore.retrieve(&passphrase)?;

                    println!("Public Key (retrieved from local keystore): {keypair}");
                }
                KeystoreType::Aws => {
                    let config = aws_config::load_from_env().await;
                    let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);

                    let secret_name = format!("{keypair}.bls");

                    let keypair: bn254::Keypair = aws_keystore
                        .retrieve(&passphrase, &AwsKeystoreParams { secret_name })
                        .await?;

                    println!("Public Key (retrieved from AWS Secrets Manager): {keypair}");
                }
            }
        }
        Curve::Secp256k1 => {
            let passphrase = match passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter keypair passphrase: ")?,
            };
            match keystore {
                KeystoreType::Local => {
                    let keypath = PathBuf::from_str(&keypair)?;
                    let private_key = LocalSigner::decrypt_keystore(keypath, passphrase)?;
                    println!(
                        "Address (retrieved from local keystore): {}",
                        private_key.address()
                    );
                }
                KeystoreType::Aws => todo!(),
            }
        }
    }
    Ok(())
}
