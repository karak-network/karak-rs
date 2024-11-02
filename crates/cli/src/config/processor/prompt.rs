use std::path::PathBuf;

use alloy::primitives::Address;
use color_eyre::owo_colors::OwoColorize;

use crate::config::models::{Chain, Keystore, Profile};
use crate::constants::DEFAULT_KARAK_DIR;
use crate::prompter;
use crate::shared::Url;

// Add validation to prompts
pub fn profile_prompt(profile: Option<Profile>) -> Profile {
    if let Some(profile) = profile {
        let chain = prompt_chain(Some(profile.chain));
        let bn254_keystore = prompt_keystore("BLS", Some(profile.bn254_keystore));
        let secp256k1_keystore = prompt_keystore("ECDSA", Some(profile.secp256k1_keystore));
        let key_generation_folder = PathBuf::from(prompter::input::<String>(
            "Enter key generation folder",
            Some(profile.key_generation_folder.to_str().unwrap().to_string()),
        ))
        .canonicalize()
        .unwrap_or_else(|e| {
            println!("{}", e.to_string().red());
            profile.key_generation_folder
        });
        let core_address = prompter::input::<Address>(
            "Enter Karak Core contract address",
            Some(profile.core_address),
        );

        return Profile {
            chain,
            core_address,
            bn254_keystore,
            secp256k1_keystore,
            key_generation_folder,
        };
    }
    let chain = prompt_chain(None);
    let bn254_keystore = prompt_keystore("BLS", None);
    let secp256k1_keystore = prompt_keystore("ECDSA", None);
    let key_generation_folder = PathBuf::from(prompter::input::<String>(
        "Enter key generation folder",
        None,
    ))
    .canonicalize()
    .unwrap_or_else(|e| {
        println!(
            "{} - Using default karak folder: {}",
            e.to_string().red(),
            DEFAULT_KARAK_DIR.bold()
        );
        PathBuf::from(DEFAULT_KARAK_DIR)
    });
    let core_address = prompter::input::<Address>("Enter Karak Core contract address", None);

    Profile {
        chain,
        core_address,
        bn254_keystore,
        secp256k1_keystore,
        key_generation_folder,
    }
}

fn prompt_chain(default: Option<Chain>) -> Chain {
    let (chain_index, is_default) = prompter::select::<Chain>("Select chain type", default.clone());
    // unwrap since the variants were created from this enum itself
    let chain = Chain::from_repr(chain_index).unwrap();

    if is_default && default.is_some() {
        match default.unwrap() {
            Chain::Evm { id, rpc_url } => {
                return Chain::Evm {
                    id: prompter::input::<u64>("Enter chain ID", Some(id)),
                    rpc_url: prompter::input::<Url>("Enter RPC URL", Some(rpc_url)),
                };
            }
        }
    }

    match chain {
        Chain::Evm { id: _, rpc_url: _ } => {
            let id = prompter::input::<u64>("Enter chain ID", None);
            let rpc_url = prompter::input::<Url>("Enter RPC URL", None);
            Chain::Evm { id, rpc_url }
        }
    }
}

fn prompt_keystore(keystore_type: &str, default: Option<Keystore>) -> Keystore {
    let (keystore_index, is_default) = prompter::select::<Keystore>(
        format!("Select {} keystore type", keystore_type).as_str(),
        default.clone(),
    );
    // unwrap since the variants were created from this enum itself
    let keystore = Keystore::from_repr(keystore_index).unwrap();

    if is_default && default.is_some() {
        match default.unwrap() {
            Keystore::Local { path } => {
                return Keystore::Local {
                    path: PathBuf::from(prompter::input::<String>(
                        format!("Enter local {} keystore path", keystore_type).as_str(),
                        Some(path.to_str().unwrap().to_string()),
                    ))
                    .canonicalize()
                    .unwrap_or_else(|e| {
                        println!(
                            "{}. Using default keystore path: {}",
                            e.to_string().red(),
                            DEFAULT_KARAK_DIR.bold()
                        );
                        PathBuf::from(DEFAULT_KARAK_DIR)
                    }),
                }
            }
            Keystore::Aws { secret } => {
                return Keystore::Aws {
                    secret: prompter::input::<String>(
                        format!("Enter {} aws keystore secret", keystore_type).as_str(),
                        Some(secret),
                    ),
                }
            }
        }
    }

    match keystore {
        Keystore::Local { path } => Keystore::Local {
            path: PathBuf::from(prompter::input::<String>(
                format!("Enter local {} keystore path", keystore_type).as_str(),
                Some(path.to_str().unwrap().to_string()),
            ))
            .canonicalize()
            .unwrap_or_else(|e| {
                println!(
                    "{} - Using default keystore path: {}",
                    e.to_string().red(),
                    DEFAULT_KARAK_DIR.bold()
                );
                PathBuf::from(DEFAULT_KARAK_DIR)
            }),
        },
        Keystore::Aws { secret: _ } => Keystore::Aws {
            secret: prompter::input::<String>(
                format!("Enter {} aws keystore secret", keystore_type).as_str(),
                None,
            ),
        },
    }
}
