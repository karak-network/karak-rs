use crate::config::models::Keystore;
use crate::{
    keypair::{KeypairArgs, KeypairLocationArgs},
    prompter,
    shared::Curve,
};

pub fn prompt_keypair_args(keypair_args: Option<KeypairArgs>) -> KeypairArgs {
    if keypair_args.is_none() {
        let (keystore_selection, _) =
            prompter::select::<Keystore>("Select keypair keystore type", None);
        let keystore = Keystore::from_repr(keystore_selection).unwrap();
        let passphrase =
            prompter::password("Enter keypair passphrase, leave blank for no passphrase");

        return KeypairArgs {
            keystore: Some(keystore),
            passphrase: Some(passphrase),
        };
    }
    keypair_args.unwrap()
}

pub fn prompt_curve(curve: Option<Curve>) -> Curve {
    if curve.is_none() {
        let (curve_selection, _) = prompter::select::<Curve>("Select keypair curve", None);
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
