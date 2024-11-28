use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::models::Keystore;
use crate::util;
use crate::{
    config::models::Curve,
    keypair::KeypairArgs,
    keypair::{AwsKeypairConfig, LocalKeypairConfig},
    prompter,
};

pub fn prompt_keypair_args(keypair_args: Option<KeypairArgs>) -> eyre::Result<KeypairArgs> {
    match keypair_args {
        Some(ka) => Ok(ka),
        None => {
            let (keystore_selection, _) =
                prompter::select_enum::<Keystore>("Select keypair keystore type", None)?;
            // Can unwrap safely since selection list is generated from the enum values itself
            let keystore = Keystore::from_repr(keystore_selection).unwrap();
            let keystore_name = prompter::input::<String>("Enter keystore name", None, None)?;
            let curve = prompt_curve(None)?;

            Ok(KeypairArgs {
                keystore_name: Some(keystore_name),
                keystore: Some(keystore),
                curve: Some(curve),
            })
        }
    }
}

pub fn prompt_new_keystore_name(keystore_name: Option<String>) -> eyre::Result<String> {
    match keystore_name {
        Some(kn) => Ok(kn),
        None => prompter::input::<String>("Enter new keystore name", None, None),
    }
}

pub fn prompt_keystore_type(keystore_type: Option<Keystore>) -> eyre::Result<Keystore> {
    match keystore_type {
        Some(k) => Ok(k),
        None => {
            let (keystore_selection, _) = prompter::select_enum::<Keystore>(
                "Select keystore type of the existing keypair",
                None,
            )?;
            // Can unwrap safely since selection list is generated from the enum values itself
            Ok(Keystore::from_repr(keystore_selection).unwrap())
        }
    }
}

pub async fn prompt_aws_config(
    aws_config: Option<AwsKeypairConfig>,
) -> eyre::Result<AwsKeypairConfig> {
    match aws_config {
        Some(ac) => Ok(ac),
        None => {
            let aws_profile = prompt_aws_profile().await?;
            let aws_secret_name = prompter::input::<String>("Enter AWS secret name", None, None)?;
            Ok(AwsKeypairConfig {
                secret_name: Some(aws_secret_name),
                profile: Some(aws_profile),
            })
        }
    }
}

pub async fn prompt_local_config(
    local_config: Option<LocalKeypairConfig>,
) -> eyre::Result<LocalKeypairConfig> {
    match local_config {
        Some(lc) => Ok(lc),
        None => {
            let keystore_path =
                PathBuf::from(prompter::input::<String>("Enter keypair path", None, None)?)
                    .canonicalize()?;
            Ok(LocalKeypairConfig {
                keystore_path: Some(keystore_path),
            })
        }
    }
}

pub fn prompt_passphrase(passphrase: Option<String>) -> eyre::Result<String> {
    match passphrase {
        Some(p) => Ok(p),
        None => prompter::password("Enter keypair passphrase"),
    }
}

pub fn prompt_keystore_name(
    keystore_name: Option<String>,
    keystores: HashMap<String, Keystore>,
) -> eyre::Result<String> {
    match keystore_name {
        Some(kn) => Ok(kn),
        None => {
            let keystore_names = keystores.keys().cloned().collect::<Vec<String>>();
            let names_refs: Vec<&str> = keystore_names.iter().map(|s| s.as_str()).collect();
            let (keystore_name_selection, _) =
                prompter::select_str(&names_refs, "Select keystore name", None)?;
            Ok(keystore_names[keystore_name_selection].clone())
        }
    }
}

pub fn prompt_curve(curve: Option<Curve>) -> eyre::Result<Curve> {
    match curve {
        Some(c) => Ok(c),
        None => {
            let (curve_selection, _) =
                prompter::select_enum::<Curve>("Select keypair curve", None)?;
            // Can unwrap safely since selection is being generated from the enum itself
            Ok(Curve::from_repr(curve_selection).unwrap())
        }
    }
}

pub async fn prompt_aws_profile() -> eyre::Result<String> {
    let profiles = util::get_aws_profiles().await?;
    let profiles_refs: Vec<&str> = profiles.iter().map(|s| s.as_str()).collect();
    let (profile_selection, _) = prompter::select_str(&profiles_refs, "Select AWS profile", None)?;
    Ok(profiles[profile_selection].clone())
}
