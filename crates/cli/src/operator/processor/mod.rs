pub mod dss;
pub mod vault;

use alloy::{network::EthereumWallet, providers::ProviderBuilder, signers::local::LocalSigner};
use color_eyre::eyre::{self, eyre};
use karak_contracts::Core::CoreInstance;

use crate::shared::Keystore;

use super::{OperatorArgs, OperatorCommand};

pub async fn process(args: OperatorArgs) -> eyre::Result<()> {
    let (operator_wallet, operator_address) = match args.secp256k1_keystore {
        Keystore::Local => {
            let Some(secp256k1_keypair_location) = args.secp256k1_keypair_location else {
                return Err(eyre!("SECP256k1 keypair location is required"));
            };
            let secp256k1_passphrase = match args.secp256k1_passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter SECP256k1 keypair passphrase: ")?,
            };

            let secp_256k1_signer =
                LocalSigner::decrypt_keystore(secp256k1_keypair_location, secp256k1_passphrase)?;

            let operator_address = secp_256k1_signer.address();
            let operator_wallet = EthereumWallet::from(secp_256k1_signer);
            (operator_wallet, operator_address)
        }
        Keystore::Aws => {
            todo!();
        }
    };

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(operator_wallet)
        .on_http(args.rpc_url);

    let core_instance = CoreInstance::new(args.core_address, provider.clone());

    match args.command {
        OperatorCommand::RegisterToDSS {
            bn254_keypair_location,
            bn254_keystore,
            bn254_passphrase,
            dss_address,
            message,
            message_encoding,
        } => {
            dss::process_registration(dss::DSSRegistrationArgs {
                bn254_keypair_location: &bn254_keypair_location,
                bn254_keystore: &bn254_keystore,
                bn254_passphrase: bn254_passphrase.as_deref(),
                core_instance: core_instance.clone(),
                dss_address,
                message: &message,
                message_encoding: &message_encoding,
            })
            .await?
        }
        OperatorCommand::CreateVault {
            asset_address,
            extra_data,
            vault_impl,
        } => {
            vault::process_vault_creation(
                asset_address,
                extra_data,
                operator_address,
                vault_impl,
                core_instance,
                provider,
            )
            .await?
        }
    }

    Ok(())
}
