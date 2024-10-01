pub mod registration;

use color_eyre::eyre;

use super::Operator;
use crate::config::models::{Chain, Profile};

pub async fn process(command: Operator, profile: Profile) -> eyre::Result<()> {
    match command {
        Operator::Register {
            bn254_keypair_location,
            bn254_keystore,
            bn254_passphrase,
            secp256k1_keypair_location,
            secp256k1_keystore,
            secp256k1_passphrase,
            rpc_url,
            dss_address,
            message,
            message_encoding,
        } => {
            registration::process_registration(registration::RegistrationArgs {
                bn254_keypair_location: &bn254_keypair_location,
                bn254_keystore: &bn254_keystore,
                bn254_passphrase: bn254_passphrase.as_deref(),
                secp256k1_keypair_location: &secp256k1_keypair_location,
                secp256k1_keystore: &secp256k1_keystore,
                secp256k1_passphrase: secp256k1_passphrase.as_deref(),
                rpc_url: rpc_url.unwrap_or(match profile.chain {
                    Chain::Evm { rpc_url, .. } => url::Url::parse(&rpc_url)?,
                }),
                core_address: profile.karak_address,
                dss_address,
                message: &message,
                message_encoding: &message_encoding,
            })
            .await?
        }
    }
    Ok(())
}
