use std::fs;

use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use color_eyre::eyre::{self, eyre};
use color_eyre::owo_colors::OwoColorize;
use karak_kms::{
    keypair::{bn254, traits::Keypair},
    keystore::{
        self,
        aws::AwsKeystoreParams,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
};

use crate::config::add_keystore_to_profile;
use crate::config::models::{Keystore, Profile};
use crate::keypair::processor::prompt;
use crate::{config::models::Curve, keypair::KeypairArgs};

pub async fn process_generate(
    keypair_args: Option<KeypairArgs>,
    curve: Option<Curve>,
    profile: Profile,
    profile_name: &str,
    config_path: String,
) -> eyre::Result<()> {
    generate_keystore(keypair_args, curve, profile, profile_name, config_path).await?;
    Ok(())
}

pub async fn generate_keystore(
    keypair_args: Option<KeypairArgs>,
    curve: Option<Curve>,
    profile: Profile,
    profile_name: &str,
    config_path: String,
) -> eyre::Result<Keystore> {
    let keypair_args = prompt::prompt_keypair_args(keypair_args)?;
    let curve = prompt::prompt_curve(curve)?;

    let KeypairArgs {
        keystore,
        passphrase,
        keystore_name,
    } = keypair_args;

    // values will be set by prompt
    let keystore = keystore.unwrap();
    let passphrase = passphrase.unwrap();
    let keystore_name = keystore_name.unwrap();

    let generation_folder = profile.clone().key_generation_folder;

    println!("Generating new keypair for curve: {:?}", curve);
    match curve {
        Curve::Bn254 => {
            let keypair = bn254::Keypair::generate();
            println!("Generated BN254 keypair with public key: {keypair}");

            match keystore {
                Keystore::Local { path: _ } => {
                    let output_path = generation_folder.join(format!("{keypair}.bls"));
                    fs::File::create(&output_path)?;

                    let local_keystore =
                        keystore::local::LocalEncryptedKeystore::new(output_path.clone());
                    local_keystore.store(&keypair, &passphrase)?;

                    let resolved_path = output_path.canonicalize()?;
                    let resolved_path_str =
                        resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                    println!("Saved keypair to {resolved_path_str}");
                    println!("\n{}", "Updating config profile...".blue());

                    let keystore = Keystore::Local {
                        path: resolved_path,
                    };

                    add_keystore_to_profile(
                        profile_name.to_string(),
                        profile,
                        curve,
                        keystore.clone(),
                        &keystore_name,
                        config_path,
                    )?;

                    Ok(keystore)
                }
                Keystore::Aws { secret: _ } => {
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
                    println!("\n{}", "Updating config profile...".blue());

                    let keystore = Keystore::Aws {
                        secret: secret_name,
                    };

                    add_keystore_to_profile(
                        profile_name.to_string(),
                        profile,
                        curve,
                        keystore.clone(),
                        &keystore_name,
                        config_path,
                    )?;

                    Ok(keystore)
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
            match keystore {
                Keystore::Local { path: _ } => {
                    let filename = format!("{}.json", private_key.address());

                    LocalSigner::encrypt_keystore(
                        generation_folder.clone(),
                        &mut rng,
                        private_key.to_bytes(),
                        passphrase,
                        Some(&filename),
                    )?;

                    let resolved_path = generation_folder.join(filename).canonicalize()?;
                    let resolved_path_str =
                        resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                    println!("Saved keypair to {resolved_path_str}");
                    println!("\n{}", "Updating config profile...".blue());

                    let keystore = Keystore::Local {
                        path: resolved_path,
                    };

                    add_keystore_to_profile(
                        profile_name.to_string(),
                        profile,
                        curve,
                        keystore.clone(),
                        &keystore_name,
                        config_path,
                    )?;

                    Ok(keystore)
                }
                Keystore::Aws { secret: _ } => {
                    todo!()
                }
            }
        }
    }
}
