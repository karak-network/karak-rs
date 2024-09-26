use std::{path::PathBuf, str::FromStr};

use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, Address},
    providers::ProviderBuilder,
    signers::local::LocalSigner,
};
use color_eyre::eyre;
use karak_bls::{
    keypair_signer::KeypairSigner,
    registration::{BlsRegistration, OperatorRegistration},
};
use karak_contracts::Core::CoreInstance;
use karak_kms::{
    keypair::{bn254, traits::Keypair},
    keystore::{self, traits::EncryptedKeystore},
};
use signature::SignerMut;
use url::Url;

use crate::shared::{Encoding, Keystore};

pub struct RegistrationArgs<'a> {
    pub bn254_keypair_location: &'a str,
    pub bn254_keystore: &'a Keystore,
    pub bn254_passphrase: Option<&'a str>,
    pub secp256k1_keypair_location: &'a str,
    pub secp256k1_keystore: &'a Keystore,
    pub secp256k1_passphrase: Option<&'a str>,
    pub rpc_url: Url,
    pub core_address: Address,
    pub dss_address: Address,
    pub message: &'a str,
    pub message_encoding: &'a Encoding,
}

pub async fn process_registration(args: RegistrationArgs<'_>) -> eyre::Result<()> {
    let bn254_passphrase = match args.bn254_passphrase {
        Some(passphrase) => passphrase,
        None => &rpassword::prompt_password("Enter BN254 keypair passphrase: ")?,
    };
    let bn254_keypair: bn254::Keypair = match args.bn254_keystore {
        Keystore::Local => {
            let local_keystore = keystore::local::LocalEncryptedKeystore::new(PathBuf::from_str(
                args.bn254_keypair_location,
            )?);
            local_keystore.retrieve(bn254_passphrase)?
        }
        Keystore::Aws => todo!(),
    };

    let secp256k1_passphrase = match args.secp256k1_passphrase {
        Some(passphrase) => passphrase,
        None => &rpassword::prompt_password("Enter SECP256k1 keypair passphrase: ")?,
    };

    let secp256k1_keypair = match args.secp256k1_keystore {
        Keystore::Local => {
            LocalSigner::decrypt_keystore(args.secp256k1_keypair_location, secp256k1_passphrase)?
        }
        Keystore::Aws => todo!(),
    };

    let wallet = EthereumWallet::from(secp256k1_keypair.clone());
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(args.rpc_url);
    let core = CoreInstance::new(args.core_address, provider.clone());

    let msg_bytes = args.message_encoding.decode(args.message)?;
    let msg_hash = keccak256(msg_bytes);
    let mut signer = KeypairSigner::from(bn254_keypair.clone());
    let signature = signer.try_sign(&msg_hash.as_ref())?;
    let registration = BlsRegistration {
        g1_pubkey: bn254_keypair.public_key().g1,
        g2_pubkey: bn254_keypair.public_key().g2,
        signature,
        msg_hash,
    };
    core.register_operator_to_dss_with_bls(args.dss_address, &registration)
        .await?;

    Ok(())
}
