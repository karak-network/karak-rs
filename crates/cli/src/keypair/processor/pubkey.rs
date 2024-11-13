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

use crate::config::models::{Curve, Keystore, Profile};

use super::prompt;

pub async fn process_pubkey(
    profile: Profile,
    keystore_name: Option<String>,
    passphrase: Option<String>,
    curve: Option<Curve>,
) -> eyre::Result<()> {
    let curve = prompt::prompt_curve(curve);
    if profile.keystores.is_none() {
        return Err(eyre::eyre!("No keystores found in profile"));
    }
    let keystores = profile.keystores.unwrap();
    let keystore_name =
        prompt::prompt_keystore_name(keystore_name, keystores.get(&curve).unwrap().clone());
    let passphrase = prompt::prompt_passphrase(passphrase);

    let keystore = keystores.get(&curve).unwrap().get(&keystore_name);
    if keystore.is_none() {
        return Err(eyre::eyre!("Keystore not found"));
    }

    let keystore = keystore.unwrap();

    match curve {
        Curve::Bn254 => match keystore {
            Keystore::Local { path: p } => {
                let local_keystore = keystore::local::LocalEncryptedKeystore::new(p.to_owned());

                let keypair: bn254::Keypair = local_keystore.retrieve(&passphrase)?;

                println!("Public Key (retrieved from local keystore): {keypair}");
            }
            Keystore::Aws { secret: s } => {
                let config = aws_config::load_from_env().await;
                let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);

                let keypair: bn254::Keypair = aws_keystore
                    .retrieve(
                        &passphrase,
                        &AwsKeystoreParams {
                            secret_name: s.to_owned(),
                        },
                    )
                    .await?;

                println!("Public Key (retrieved from AWS Secrets Manager): {keypair}");
            }
        },
        Curve::Secp256k1 => match keystore {
            Keystore::Local { path: p } => {
                let private_key = LocalSigner::decrypt_keystore(p, passphrase)?;
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
