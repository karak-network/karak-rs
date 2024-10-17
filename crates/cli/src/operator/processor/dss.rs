use std::{path::PathBuf, str::FromStr};

use crate::shared::{Encoding, Keystore};
use alloy::primitives::{keccak256, Address};
use alloy::providers::Provider;
use alloy::signers::k256::ecdsa::signature::SignerMut;
use alloy::transports::Transport;
use color_eyre::eyre;
use karak_contracts::Core::CoreInstance;
use karak_kms::{
    keypair::{
        bn254::{
            self,
            bls::registration::{BlsRegistration, OperatorRegistration},
        },
        traits::Keypair,
    },
    keystore::{self, traits::EncryptedKeystore},
};

pub struct DSSRegistrationArgs<'a, T: Transport + Clone, P: Provider<T>> {
    pub bn254_keypair_location: &'a str,
    pub bn254_keystore: &'a Keystore,
    pub bn254_passphrase: Option<&'a str>,
    pub core_instance: CoreInstance<T, P>,
    pub dss_address: Address,
    pub message: &'a str,
    pub message_encoding: &'a Encoding,
}

pub async fn process_registration<T: Transport + Clone, P: Provider<T>>(
    args: DSSRegistrationArgs<'_, T, P>,
) -> eyre::Result<()> {
    let bn254_passphrase = match args.bn254_passphrase {
        Some(passphrase) => passphrase,
        None => &rpassword::prompt_password("Enter BN254 keypair passphrase: ")?,
    };
    let mut bn254_keypair: bn254::Keypair = match args.bn254_keystore {
        Keystore::Local => {
            let local_keystore = keystore::local::LocalEncryptedKeystore::new(PathBuf::from_str(
                args.bn254_keypair_location,
            )?);
            local_keystore.retrieve(bn254_passphrase)?
        }
        Keystore::Aws => todo!(),
    };

    // TODO: Get this value from the DSS contract not from the args
    let msg_bytes = args.message_encoding.decode(args.message)?;
    let msg_hash = keccak256(msg_bytes);
    let signature = bn254_keypair.sign(msg_hash.as_ref());
    let registration = BlsRegistration {
        g1_pubkey: bn254_keypair.public_key().g1,
        g2_pubkey: bn254_keypair.public_key().g2,
        signature,
    };
    let tx_hash = args
        .core_instance
        .register_operator_to_dss_with_bls(args.dss_address, &registration)
        .await?;

    println!("Operator registered to DSS: {}", tx_hash);

    Ok(())
}
