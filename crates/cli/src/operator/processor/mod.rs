pub mod registration;

use std::{path::PathBuf, str::FromStr};

use color_eyre::eyre;

use super::Operator;
use crate::config::models::{Chain, Keystore, Profile};

pub async fn process(command: Operator, profile: Profile) -> eyre::Result<()> {
    match command {
        Operator::Register {
            bn254_keystore_type,
            bn254_keystore,
            bn254_passphrase,
            secp256k1_keystore_type,
            secp256k1_keystore,
            secp256k1_passphrase,
            rpc_url,
            dss_address,
            message,
            message_encoding,
        } => {
            let bn254_keystore = match bn254_keystore_type {
                Some(keystore) => match keystore {
                    Keystore::Aws { secret: _ } => Keystore::Aws {
                        secret: bn254_keystore.unwrap(),
                    },
                    Keystore::Local { path: _ } => Keystore::Local {
                        path: PathBuf::from_str(&bn254_keystore.unwrap()).unwrap(),
                    },
                },
                None => profile.bn254_keystore,
            };
            let secp256k1_keystore = match secp256k1_keystore_type {
                Some(keystore) => match keystore {
                    Keystore::Aws { secret: _ } => Keystore::Aws {
                        secret: secp256k1_keystore.unwrap(),
                    },
                    Keystore::Local { path: _ } => Keystore::Local {
                        path: PathBuf::from_str(&secp256k1_keystore.unwrap()).unwrap(),
                    },
                },
                None => profile.secp256k1_keystore,
            };

            registration::process_registration(registration::RegistrationArgs {
                bn254_keystore: &bn254_keystore,
                bn254_passphrase: bn254_passphrase.as_deref(),
                secp256k1_keystore: &secp256k1_keystore,
                secp256k1_passphrase: secp256k1_passphrase.as_deref(),
                rpc_url: rpc_url.unwrap_or(match profile.chain {
                    Chain::Evm { rpc_url, .. } => rpc_url,
                }),
                core_address: profile.karak_address,
                dss_address,
                message: &message,
                message_encoding: &message_encoding,
            })
            .await?
        }
    }
    Ok(())
}
