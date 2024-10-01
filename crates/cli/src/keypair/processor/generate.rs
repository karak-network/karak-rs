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

use crate::shared::KeystoreType;
use crate::{keypair::KeypairArgs, shared::Curve};

pub async fn process_generate(
    keypair_args: KeypairArgs,
    curve: Curve,
    generation_folder: PathBuf,
) -> eyre::Result<()> {
    let KeypairArgs {
        keystore,
        passphrase,
    } = keypair_args;

    println!("Generating new keypair for curve: {:?}", curve);
    match curve {
        Curve::Bn254 => {
            let keypair = bn254::Keypair::generate();
            println!("Generated BN254 keypair with public key: {keypair}");

            let output_path = generation_folder.join(format!("{keypair}.bls"));

            let passphrase = match passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter keypair passphrase: ")?,
            };

            match keystore {
                KeystoreType::Local => {
                    fs::File::create(&output_path)?;

                    let local_keystore =
                        keystore::local::LocalEncryptedKeystore::new(output_path.clone());
                    local_keystore.store(&keypair, &passphrase)?;

                    let resolved_path = output_path.canonicalize()?;
                    let resolved_path_str =
                        resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                    println!("Saved keypair to {resolved_path_str}");
                }
                KeystoreType::Aws => {
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
                KeystoreType::Local => {
                    let filename = "secp256k1.json";
                    let keypath = generation_folder.join(filename);

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
                KeystoreType::Aws => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}
