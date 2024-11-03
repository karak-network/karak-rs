pub mod dss;
#[cfg(feature = "testnet")]
pub mod erc20;
pub mod prompt;
pub mod registry;
pub mod stake;
pub mod vault;

use alloy::{
    network::EthereumWallet,
    primitives::{aliases::U48, Address, U256},
    providers::ProviderBuilder,
    signers::{aws::AwsSigner, local::LocalSigner, Signer},
};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_secretsmanager::config::{Credentials, SharedCredentialsProvider};
use karak_contracts::{
    erc20::mintable::ERC20Mintable, registry::RestakingRegistry, vault::Vault::VaultInstance,
    Core::CoreInstance,
};

use crate::config::models::Keystore;
use crate::config::models::Profile;
use crate::prompter;
use prompt::*;

use super::{OperatorArgs, OperatorCommand};

pub async fn process(args: OperatorArgs, profile: Profile) -> eyre::Result<()> {
    let secp256k1_keystore_type = args
        .secp256k1_keystore_type
        .unwrap_or_else(|| prompt_secp256k1_keystore_type());

    let (operator_wallet, operator_address) = match secp256k1_keystore_type {
        Keystore::Local { path: _ } => {
            let secp256k1_keystore_path = match args.secp256k1_keystore_path {
                Some(path) => path,
                None => prompt_secp256k1_keystore_path(),
            };
            let secp256k1_passphrase = match args.secp256k1_passphrase {
                Some(passphrase) => passphrase,
                None => prompt_secp256k1_passphrase(),
            };

            let secp_256k1_signer =
                LocalSigner::decrypt_keystore(secp256k1_keystore_path, secp256k1_passphrase)?;

            let operator_address = secp_256k1_signer.address();
            let operator_wallet = EthereumWallet::from(secp_256k1_signer);
            (operator_wallet, operator_address)
        }
        Keystore::Aws { secret: _ } => {
            let region = args
                .aws_region
                .unwrap_or_else(|| prompter::input::<String>("Enter AWS region", None, None));
            let access_key_id = args.aws_access_key_id.unwrap_or_else(|| {
                prompter::input::<String>("Enter AWS access key ID", None, None)
            });
            let secret_access_key = args.aws_secret_access_key.unwrap_or_else(|| {
                prompter::input::<String>("Enter AWS secret access key", None, None)
            });
            let operator_key_id = args.aws_operator_key_id.unwrap_or_else(|| {
                prompter::input::<String>("Enter AWS operator key ID", None, None)
            });

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
        .on_http(profile.chain.rpc_url().into());

    match args.command {
        OperatorCommand::RegisterToDSS {
            bn254_keypair_location,
            bn254_keystore,
            bn254_passphrase,
            dss_address,
            message,
            message_encoding,
        } => {
            let core_instance = CoreInstance::new(profile.core_address, provider.clone());

            let bn254_keypair_location = bn254_keypair_location
                .unwrap_or_else(|| prompter::input("Enter BN254 keypair location", None, None));
            let bn254_keystore = bn254_keystore.unwrap_or_else(|| prompt_bn254_keystore_type());
            let bn254_passphrase = bn254_passphrase
                .unwrap_or_else(|| prompter::password("Enter BN254 keypair passphrase: "));
            let dss_address = dss_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter DSS address", None, None));
            let message =
                message.unwrap_or_else(|| prompter::input::<String>("Enter message", None, None));
            let message_encoding = message_encoding.unwrap_or_else(|| prompt_message_encoding());

            dss::process_registration(dss::DSSRegistrationArgs {
                bn254_keypair_location: &bn254_keypair_location,
                bn254_keystore: &bn254_keystore,
                bn254_passphrase: &bn254_passphrase,
                core_instance: core_instance.clone(),
                dss_address,
                message: &message,
                message_encoding: &message_encoding,
                operator_address,
            })
            .await?
        }
        OperatorCommand::CreateVault { assets, vault_impl } => {
            let core_instance = CoreInstance::new(profile.core_address, provider.clone());

            vault::process_vault_creation(
                assets,
                operator_address,
                vault_impl,
                core_instance,
                provider.clone(),
            )
            .await?

            // TODO: Direct to registy registration link
        }
        OperatorCommand::RegisterToRegistry {
            registry_address,
            kns,
        } => {
            let registry_address = registry_address.unwrap_or_else(|| {
                prompter::input::<Address>("Enter registry address", None, None)
            });
            let kns = kns.unwrap_or_else(|| prompter::input::<String>("Enter KNS", None, None));

            let registry_instance = RestakingRegistry::new(registry_address, provider);
            registry::process_registry_registration(kns, operator_address, registry_instance)
                .await?
        }
        OperatorCommand::RequestStakeUpdate {
            vault_address,
            dss_address,
            stake_update_type,
        } => {
            let core_instance = CoreInstance::new(profile.core_address, provider.clone());

            let stake_update_type = stake_update_type.unwrap_or_else(|| prompt_stake_update_type());
            let vault_address = vault_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter vault address", None, None));
            let dss_address = dss_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter DSS address", None, None));

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
        } => {
            let core_instance = CoreInstance::new(profile.core_address, provider.clone());

            let stake_update_type = stake_update_type.unwrap_or_else(|| prompt_stake_update_type());
            let vault_address = vault_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter vault address", None, None));
            let dss_address = dss_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter DSS address", None, None));
            let nonce = nonce.unwrap_or_else(|| prompter::input::<U48>("Enter nonce", None, None));
            let start_timestamp = start_timestamp
                .unwrap_or_else(|| prompter::input::<U48>("Enter start timestamp", None, None));

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
            let vault_address = vault_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter vault address", None, None));
            let amount =
                amount.unwrap_or_else(|| prompter::input::<U256>("Enter amount", None, None));

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
            let asset_address = asset_address
                .unwrap_or_else(|| prompter::input::<Address>("Enter asset address", None, None));
            let amount =
                amount.unwrap_or_else(|| prompter::input::<U256>("Enter amount", None, None));

            let erc20_instance = ERC20Mintable::new(asset_address, provider);
            erc20::mint(amount, operator_address, erc20_instance).await?
        }
    }

    Ok(())
}
