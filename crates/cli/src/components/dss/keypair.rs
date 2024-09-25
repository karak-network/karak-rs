use std::str::FromStr;
use std::{fs, path::PathBuf};

use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use color_eyre::eyre::{self, eyre};
use karak_kms::{
    keypair::{bn254, traits::Keypair},
    keystore::{
        self,
        aws::AwsKeystoreParams,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
};

use crate::common::{Curve, Keystore};

pub struct KeypairArgs {
    pub keystore: Keystore,
    pub passphrase: Option<String>,
}

pub struct KeypairLocationArgs {
    pub keypair: String,
}

pub async fn process_generate(keypair_args: KeypairArgs, curve: Curve) -> eyre::Result<()> {
    let KeypairArgs {
        keystore,
        passphrase,
    } = keypair_args;

    println!("Generating new keypair for curve: {:?}", curve);
    match curve {
        Curve::Bn254 => {
            let keypair = bn254::Keypair::generate();
            println!("Generated BN254 keypair with public key: {keypair}");

            let passphrase = match passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter keypair passphrase: ")?,
            };

            match keystore {
                Keystore::Local => {
                    let output = PathBuf::from(format!("{keypair}.bls"));

                    if let Some(parent) = output.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::File::create(&output)?;

                    let local_keystore =
                        keystore::local::LocalEncryptedKeystore::new(output.clone());
                    local_keystore.store(&keypair, &passphrase)?;

                    let resolved_path = output.canonicalize()?;
                    let resolved_path_str =
                        resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                    println!("Saved keypair to {resolved_path_str}");
                }
                Keystore::Aws => {
                    let config = aws_config::load_from_env().await;
                    let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);

                    let secret_name = format!("{keypair}.bls");

                    aws_keystore
                        .store(
                            &keypair,
                            &passphrase,
                            &AwsKeystoreParams {
                                secret_name: secret_name.clone(),
                            },
                        )
                        .await?;

                    println!("Saved keypair to {secret_name} in AWS Secrets Manager");
                }
            }
        }
        Curve::Secp256k1 => {
            let private_key = PrivateKeySigner::random();
            println!(
                "Generated SECP256k1 keypair with address: {}",
                private_key.address()
            );
            let mut rng = rand::thread_rng();
            let passphrase = match passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter keypair passphrase: ")?,
            };
            match keystore {
                Keystore::Local => {
                    let keypath = dirs_next::home_dir()
                        .ok_or(eyre!("Home directory not found"))?
                        .join(".config")
                        .join("karak");
                    let filename = "secp256k1.json";

                    fs::create_dir_all(&keypath)?;

                    let resolved_path = keypath.join(filename).canonicalize()?;

                    LocalSigner::encrypt_keystore(
                        keypath,
                        &mut rng,
                        private_key.to_bytes(),
                        passphrase,
                        Some(filename),
                    )?;

                    let resolved_path_str =
                        resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                    println!("Saved keypair to {resolved_path_str}");
                }
                Keystore::Aws => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}

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
                Keystore::Local => {
                    let local_keystore =
                        keystore::local::LocalEncryptedKeystore::new(PathBuf::from(keypair));

                    let keypair: bn254::Keypair = local_keystore.retrieve(&passphrase)?;

                    println!("Public Key (retrieved from local keystore): {keypair}");
                }
                Keystore::Aws => {
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
                Keystore::Local => {
                    let keypath = PathBuf::from_str(&keypair)?;
                    let private_key = LocalSigner::decrypt_keystore(keypath, passphrase)?;
                    println!(
                        "Address (retrieved from local keystore): {}",
                        private_key.address()
                    );
                }
                Keystore::Aws => todo!(),
            }
        }
    }
    Ok(())
}
