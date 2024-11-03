use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;

use crate::config::models::Keystore;
use crate::operator::processor::stake::StakeUpdateType;
use crate::prompter;
use crate::shared::Encoding;

pub fn prompt_secp256k1_keystore_path() -> PathBuf {
    loop {
        let path = PathBuf::from(prompter::input::<String>(
            "Enter local SECP256k1 keystore path",
            None,
            None,
        ));

        match path.canonicalize() {
            Ok(canonical_path) => return canonical_path,
            Err(e) => println!("{} - Try again", e.to_string().red()),
        }
    }
}

pub fn prompt_secp256k1_passphrase() -> String {
    prompter::password("Enter SECP256k1 keypair passphrase: ")
}

pub fn prompt_secp256k1_keystore_type() -> Keystore {
    let (keystore, _) = prompter::select::<Keystore>("Select SECP256k1 keystore type", None);
    Keystore::from_repr(keystore).unwrap()
}

pub fn prompt_bn254_keystore_type() -> Keystore {
    let (keystore, _) = prompter::select::<Keystore>("Select BN254 keystore type", None);
    Keystore::from_repr(keystore).unwrap()
}

pub fn prompt_message_encoding() -> Encoding {
    let (encoding, _) = prompter::select::<Encoding>("Select message encoding", None);
    Encoding::from_repr(encoding).unwrap()
}

pub fn prompt_stake_update_type() -> StakeUpdateType {
    let (stake_update_type, _) =
        prompter::select::<StakeUpdateType>("Select stake update type", None);
    StakeUpdateType::from_repr(stake_update_type).unwrap()
}
