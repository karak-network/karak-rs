use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;

use crate::config::models::{Curve, Keystore, Profile};
use crate::keypair::processor::generate::generate_keystore;
use crate::operator::processor::stake::StakeUpdateType;
use crate::prompter;
use crate::shared::Encoding;

pub fn prompt_keystore_path() -> eyre::Result<PathBuf> {
    loop {
        let path = PathBuf::from(prompter::input::<String>(
            "Enter local keystore path",
            None,
            None,
        )?);

        match path.canonicalize() {
            Ok(canonical_path) => return Ok(canonical_path),
            Err(e) => println!("{} - Try again", e.to_string().red()),
        }
    }
}

pub fn prompt_secp256k1_passphrase() -> eyre::Result<String> {
    prompter::password("Enter SECP256k1 keypair passphrase")
}

pub async fn prompt_keystore_type(
    curve: Curve,
    profile: Profile,
    profile_name: &str,
    config_path: String,
) -> eyre::Result<Keystore> {
    let keystores = profile
        .keystores
        .get(&curve)
        .cloned()
        .unwrap_or_else(HashMap::new);

    let mut keystore_names = keystores.keys().cloned().collect::<Vec<String>>();
    keystore_names.push("Generate new".to_string());
    let names_refs: Vec<&str> = keystore_names.iter().map(|s| s.as_str()).collect();
    let (keystore_name_selection, _) =
        prompter::select_str(&names_refs, &format!("Select {} keystore", curve), None)?;
    // if length of keystore_names is greater than keystore_name_selection, then we need to generate a new keystore
    if keystore_name_selection == keystore_names.len() - 1 {
        println!("\nGenerating new keystore...");
        return generate_keystore(None, Some(curve), profile, profile_name, config_path).await;
    }
    let keystore_name = keystore_names[keystore_name_selection].clone();
    Ok(keystores.get(&keystore_name).unwrap().clone())
}

pub fn prompt_bn254_keystore_type() -> eyre::Result<Keystore> {
    let (keystore, _) = prompter::select_enum::<Keystore>("Select BN254 keystore type", None)?;
    // can unwrap safely here since keystore selection is generated from the enum itself
    Ok(Keystore::from_repr(keystore).unwrap())
}

pub fn prompt_message_encoding() -> eyre::Result<Encoding> {
    let (encoding, _) = prompter::select_enum::<Encoding>("Select message encoding", None)?;
    // can unwrap safely here since encoding selection is generated from the enum itself
    Ok(Encoding::from_repr(encoding).unwrap())
}

pub fn prompt_stake_update_type() -> eyre::Result<StakeUpdateType> {
    let (stake_update_type, _) =
        prompter::select_enum::<StakeUpdateType>("Select stake update type", None)?;
    // can unwrap safely here since encoding selection is generated from the enum itself
    Ok(StakeUpdateType::from_repr(stake_update_type).unwrap())
}
