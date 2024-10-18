pub mod dss;
#[cfg(feature = "testnet")]
pub mod erc20;
pub mod registry;
pub mod stake;
pub mod vault;

use alloy::{
    network::EthereumWallet,
    providers::ProviderBuilder,
    signers::{aws::AwsSigner, local::LocalSigner, Signer},
};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_secretsmanager::config::{Credentials, SharedCredentialsProvider};
use color_eyre::eyre::{self, eyre};
use karak_contracts::{
    erc20::mintable::ERC20Mintable, registry::RestakingRegistry, vault::Vault::VaultInstance,
    Core::CoreInstance,
};

use crate::shared::Keystore;

use super::{OperatorArgs, OperatorCommand};

pub async fn process(args: OperatorArgs) -> eyre::Result<()> {
    let (operator_wallet, operator_address) = match args.secp256k1_keystore_type {
        Keystore::Local => {
            let Some(secp256k1_keystore_path) = args.secp256k1_keystore_path else {
                return Err(eyre!("SECP256k1 keypair location is required"));
            };
            let secp256k1_passphrase = match args.secp256k1_passphrase {
                Some(passphrase) => passphrase,
                None => rpassword::prompt_password("Enter SECP256k1 keypair passphrase: ")?,
            };

            let secp_256k1_signer =
                LocalSigner::decrypt_keystore(secp256k1_keystore_path, secp256k1_passphrase)?;

            let operator_address = secp_256k1_signer.address();
            let operator_wallet = EthereumWallet::from(secp_256k1_signer);
            (operator_wallet, operator_address)
        }
        Keystore::Aws => {
            let region = args
                .aws_region
                .ok_or(eyre!("AWS region is required for AWS keystore"))?;
            let access_key_id = args
                .aws_access_key_id
                .ok_or(eyre!("AWS access key ID is required for AWS keystore"))?;
            let secret_access_key = args
                .aws_secret_access_key
                .ok_or(eyre!("AWS secret access key is required for AWS keystore"))?;
            let operator_key_id = args
                .aws_operator_key_id
                .ok_or(eyre!("AWS operator key ID is required for AWS keystore"))?;

            let credentials = Credentials::new(access_key_id, secret_access_key, None, None, "");
            let aws_config = aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new(region))
                .credentials_provider(SharedCredentialsProvider::new(credentials))
                .load()
                .await;

            let client = aws_sdk_kms::Client::new(&aws_config);
            let signer = AwsSigner::new(client, operator_key_id, None).await?;

            let operator_address = signer.address();
            let operator_wallet = EthereumWallet::from(signer);

            (operator_wallet, operator_address)
        }
    };

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(operator_wallet)
        .on_http(args.rpc_url);

    match args.command {
        OperatorCommand::RegisterToDSS {
            bn254_keypair_location,
            bn254_keystore,
            bn254_passphrase,
            dss_address,
            message,
            message_encoding,
            core_address,
        } => {
            let core_instance = CoreInstance::new(core_address, provider.clone());

            dss::process_registration(dss::DSSRegistrationArgs {
                bn254_keypair_location: &bn254_keypair_location,
                bn254_keystore: &bn254_keystore,
                bn254_passphrase: bn254_passphrase.as_deref(),
                core_instance: core_instance.clone(),
                dss_address,
                message: &message,
                message_encoding: &message_encoding,
                operator_address,
            })
            .await?
        }
        OperatorCommand::CreateVault {
            asset_address,
            extra_data,
            vault_impl,
            core_address,
        } => {
            let core_instance = CoreInstance::new(core_address, provider.clone());
            let erc20_instance = ERC20Mintable::new(asset_address, provider.clone());

            vault::process_vault_creation(
                asset_address,
                extra_data,
                operator_address,
                vault_impl,
                core_instance,
                erc20_instance,
            )
            .await?
        }
        OperatorCommand::RegisterToRegistry {
            registry_address,
            kns,
        } => {
            let registry_instance = RestakingRegistry::new(registry_address, provider);
            registry::process_registry_registration(kns, operator_address, registry_instance)
                .await?
        }
        OperatorCommand::RequestStakeUpdate {
            vault_address,
            dss_address,
            stake_update_type,
            core_address,
        } => {
            let core_instance = CoreInstance::new(core_address, provider.clone());
            stake::process_stake_update_request(
                vault_address,
                dss_address,
                stake_update_type,
                core_instance,
            )
            .await?
        }
        OperatorCommand::FinalizeStakeUpdate {
            vault_address,
            dss_address,
            stake_update_type,
            nonce,
            start_timestamp,
            core_address,
        } => {
            let core_instance = CoreInstance::new(core_address, provider);
            stake::process_finalize_stake_update_request(
                vault_address,
                dss_address,
                stake_update_type,
                nonce,
                start_timestamp,
                operator_address,
                core_instance,
            )
            .await?
        }
        OperatorCommand::DepositToVault {
            vault_address,
            amount,
        } => {
            let vault_instance = VaultInstance::new(vault_address, provider.clone());
            let asset_address = vault_instance.asset().call().await?._0;
            let erc20_instance = ERC20Mintable::new(asset_address, provider.clone());

            vault::deposit_to_vault(
                amount,
                operator_address,
                vault_address,
                vault_instance,
                erc20_instance,
            )
            .await?
        }
        #[cfg(feature = "testnet")]
        OperatorCommand::MintERC20 {
            asset_address,
            amount,
        } => {
            let erc20_instance = ERC20Mintable::new(asset_address, provider);
            erc20::mint(amount, operator_address, erc20_instance).await?
        }
    }

    Ok(())
}
