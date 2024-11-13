use std::collections::HashMap;

use crate::config::models::Keystore;
use crate::{
    config::models::Curve,
    keypair::{KeypairArgs, KeypairLocationArgs},
    prompter,
};

pub fn prompt_keypair_args(keypair_args: Option<KeypairArgs>) -> KeypairArgs {
    if keypair_args.is_none() {
        let (keystore_selection, _) =
            prompter::select_enum::<Keystore>("Select keypair keystore type", None);
        let keystore = Keystore::from_repr(keystore_selection).unwrap();
        let keystore_name = prompter::input::<String>("Enter keystore name", None, None);
        let passphrase =
            prompter::password("Enter keypair passphrase, leave blank for no passphrase");

        return KeypairArgs {
            keystore_name: Some(keystore_name),
            keystore: Some(keystore),
            passphrase: Some(passphrase),
        };
    }
    keypair_args.unwrap()
}

pub fn prompt_passphrase(passphrase: Option<String>) -> String {
    if passphrase.is_none() {
        return prompter::password("Enter keypair passphrase, leave blank for no passphrase");
    }
    passphrase.unwrap()
}

pub fn prompt_keystore_name(
    keystore_name: Option<String>,
    keystores: HashMap<String, Keystore>,
) -> String {
    if keystore_name.is_none() {
        let keystore_names = keystores.keys().cloned().collect::<Vec<String>>();
        let names_refs: Vec<&str> = keystore_names.iter().map(|s| s.as_str()).collect();
        let (keystore_name_selection, _) =
            prompter::select_str(&names_refs, "Select keystore name", None);
        return keystore_names[keystore_name_selection].clone();
    }
    keystore_name.unwrap()
}

pub fn prompt_curve(curve: Option<Curve>) -> Curve {
    if curve.is_none() {
        let (curve_selection, _) = prompter::select_enum::<Curve>("Select keypair curve", None);
        return Curve::from_repr(curve_selection).unwrap();
    }
    curve.unwrap()
}

pub fn prompt_keypair_location_args(
    keypair_location_args: Option<KeypairLocationArgs>,
) -> KeypairLocationArgs {
    if keypair_location_args.is_none() {
        let keypair = prompter::input::<String>("Enter keypair ID/path to retrieve", None, None);
        return KeypairLocationArgs {
            keypair: Some(keypair),
        };
    }
    keypair_location_args.unwrap()
}
