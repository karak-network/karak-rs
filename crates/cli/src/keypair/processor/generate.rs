use std::{fs, path::PathBuf};

use color_eyre::eyre::{self, eyre};
use karak_sdk::{
    keypair::{bn254, traits::Keypair},
    keystore::{
        self,
        aws::AwsKeystoreParams,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
};

use crate::{
    keypair::KeypairArgs,
    shared::{Curve, Keystore},
};

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

                    Ok(())
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

                    Ok(())
                }
            }
        }
    }
}
