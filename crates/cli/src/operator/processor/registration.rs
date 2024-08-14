use std::{path::PathBuf, str::FromStr};

use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, Address},
    providers::ProviderBuilder,
    signers::local::LocalSigner,
};
use color_eyre::eyre;
use karak_contracts::{
    registration::{BlsRegistration, OperatorRegistration},
    Core::CoreInstance,
};
use karak_sdk::{
    keypair::{bn254, traits::Keypair},
    keystore::{self, traits::EncryptedKeystore},
    signer::{bls::keypair_signer::KeypairSigner, traits::Signer},
};
use url::Url;

use crate::shared::{Encoding, Keystore};

// TODO: Move args to a struct and remove the clippy lint
#[allow(clippy::too_many_arguments)]
pub async fn process_registration(
    bn254_keypair_location: String,
    bn254_keystore: Keystore,
    bn254_passphrase: Option<String>,
    secp256k1_keypair_location: String,
    secp256k1_keystore: Keystore,
    secp256k1_passphrase: Option<String>,
    rpc_url: Url,
    core_address: Address,
    dss_address: Address,
    message: String,
    message_encoding: Encoding,
) -> eyre::Result<()> {
    let bn254_passphrase = match bn254_passphrase {
        Some(passphrase) => passphrase,
        None => rpassword::prompt_password("Enter BN254 keypair passphrase: ")?,
    };
    let bn254_keypair: bn254::Keypair = match bn254_keystore {
        Keystore::Local => {
            let local_keystore = keystore::local::LocalEncryptedKeystore::new(PathBuf::from_str(
                &bn254_keypair_location,
            )?);
            local_keystore.retrieve(&bn254_passphrase)?
        }
        Keystore::Aws => todo!(),
    };

    let secp256k1_passphrase = match secp256k1_passphrase {
        Some(passphrase) => passphrase,
        None => rpassword::prompt_password("Enter SECP256k1 keypair passphrase: ")?,
    };

    let secp256k1_keypair = match secp256k1_keystore {
        Keystore::Local => {
            LocalSigner::decrypt_keystore(secp256k1_keypair_location, secp256k1_passphrase)?
        }
        Keystore::Aws => todo!(),
    };

    let wallet = EthereumWallet::from(secp256k1_keypair.clone());
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);
    let core = CoreInstance::new(core_address, provider.clone());

    let msg_bytes = message_encoding.decode(&message)?;
    let msg_hash = keccak256(msg_bytes);
    let signer = KeypairSigner::from(bn254_keypair.clone());
    let signature = signer.sign_message(msg_hash)?;
    let registration = BlsRegistration {
        g1_pubkey: bn254_keypair.public_key().g1,
        g2_pubkey: bn254_keypair.public_key().g2,
        signature,
        msg_hash,
    };
    core.register_operator_to_dss_with_bls(dss_address, &registration)
        .await?;

    Ok(())
}
