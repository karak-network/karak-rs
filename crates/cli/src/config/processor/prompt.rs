use std::collections::HashMap;
use std::path::PathBuf;

use alloy::primitives::Address;
use color_eyre::owo_colors::OwoColorize;

use crate::config::models::{Chain, Profile};
use crate::constants::DEFAULT_KARAK_DIR;
use crate::prompter;
use crate::types::Url;

// Add validation to prompts
pub fn profile_prompt(profile: Option<Profile>) -> eyre::Result<Profile> {
    if let Some(profile) = profile {
        let chain = prompt_chain(Some(profile.chain))?;
        let key_generation_folder = PathBuf::from(prompter::input::<String>(
            "Enter key generation folder",
            // unwrap safe here since validity of the key_generation folder is checked when setting the config
            Some(profile.key_generation_folder.to_str().unwrap().to_string()),
            None,
        )?)
        .canonicalize()
        .unwrap_or_else(|e| {
            println!("{}", e.to_string().red());
            profile.key_generation_folder
        });
        let core_address = prompter::input::<Address>(
            "Enter Karak Core contract address",
            Some(profile.core_address),
            None,
        )?;

        return Ok(Profile {
            chain,
            core_address,
            keystores: profile.keystores,
            key_generation_folder,
        });
    }
    let chain = prompt_chain(None)?;
    let key_generation_folder = PathBuf::from(prompter::input::<String>(
        "Enter key generation folder",
        Some(DEFAULT_KARAK_DIR.to_owned()),
        None,
    )?)
    .canonicalize()
    .unwrap_or_else(|e| {
        println!(
            "{} - Using default karak folder: {}",
            e.to_string().red(),
            DEFAULT_KARAK_DIR.bold()
        );
        PathBuf::from(DEFAULT_KARAK_DIR)
    });
    let core_address = prompter::input::<Address>("Enter Karak Core contract address", None, None)?;

    return Ok(Profile {
        chain,
        core_address,
        keystores: HashMap::new(),
        key_generation_folder,
    });
}

fn prompt_chain(default: Option<Chain>) -> eyre::Result<Chain> {
    let (chain_index, is_default) =
        prompter::select_enum::<Chain>("Select chain type", default.clone())?;
    // unwrap since the variants were created from this enum itself
    let chain = Chain::from_repr(chain_index).unwrap();

    if is_default && default.is_some() {
        // unwrap is safe since default is Some according to the if condition
        match default.unwrap() {
            Chain::Evm { id, rpc_url } => {
                return Ok(Chain::Evm {
                    id: prompter::input::<u64>("Enter chain ID", Some(id), None)?,
                    rpc_url: prompter::input::<Url>("Enter RPC URL", Some(rpc_url), None)?,
                })
            }
        }
    }

    match chain {
        Chain::Evm { id: _, rpc_url: _ } => {
            let id = prompter::input::<u64>("Enter chain ID", None, None)?;
            let rpc_url = prompter::input::<Url>("Enter RPC URL", None, None)?;
            Ok(Chain::Evm { id, rpc_url })
        }
    }
}
