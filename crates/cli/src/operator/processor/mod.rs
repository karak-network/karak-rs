pub mod registration;

use color_eyre::eyre;

use super::Operator;

pub async fn process(command: Operator) -> eyre::Result<()> {
    match command {
        Operator::Register {
            bn254_keypair_location,
            bn254_keystore,
            bn254_passphrase,
            secp256k1_keypair_location,
            secp256k1_keystore,
            secp256k1_passphrase,
            rpc_url,
            core_address,
            dss_address,
            message,
            message_encoding,
        } => {
            registration::process_registration(
                bn254_keypair_location,
                bn254_keystore,
                bn254_passphrase,
                secp256k1_keypair_location,
                secp256k1_keystore,
                secp256k1_passphrase,
                rpc_url,
                core_address,
                dss_address,
                message,
                message_encoding,
            )
            .await?
        }
    }
    Ok(())
}
