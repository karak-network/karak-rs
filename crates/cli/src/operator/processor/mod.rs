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
    erc20::contract::ERC20::ERC20Instance, registry::RestakingRegistry,
    vault::Vault::VaultInstance, Core::CoreInstance,
};

#[cfg(feature = "testnet")]
use karak_contracts::erc20::mintable::ERC20Mintable::ERC20MintableInstance;

use crate::config::models::{Curve, Keystore, Profile};
use crate::prompter;
use prompt::*;

use super::{OperatorArgs, OperatorCommand};

pub async fn process(
    args: OperatorArgs,
    profile: Profile,
    profile_name: &str,
    config_path: String,
) -> eyre::Result<()> {
    let secp256k1_keystore_type = match args.secp256k1_keystore_type {
        Some(Keystore::Local { path: _ }) => {
            let secp256k1_keystore_path = match args.secp256k1_keystore_path {
                Some(path) => path,
                None => prompt_keystore_path()?,
            };

            Keystore::Local {
                path: secp256k1_keystore_path,
            }
        }
        // TODO: Update config to handle AWS secret and access keys
        Some(Keystore::Aws { secret: s }) => Keystore::Aws { secret: s },
        None => {
            prompt_keystore_type(
                Curve::Secp256k1,
                profile.clone(),
                profile_name,
                config_path.clone(),
            )
            .await?
        }
    };

    let (operator_wallet, operator_address) = match secp256k1_keystore_type {
        Keystore::Local { path } => {
            let secp256k1_passphrase = match args.secp256k1_passphrase {
                Some(passphrase) => passphrase,
                None => prompt_secp256k1_passphrase()?,
            };

            let secp_256k1_signer = LocalSigner::decrypt_keystore(path, secp256k1_passphrase)?;

            let operator_address = secp_256k1_signer.address();
            let operator_wallet = EthereumWallet::from(secp_256k1_signer);
            (operator_wallet, operator_address)
        }
        // TODO: Update config to handle AWS secret and access keys
        Keystore::Aws { secret: _ } => {
            let region = match args.aws_region {
                Some(r) => r,
                None => prompter::input::<String>("Enter AWS region", None, None)?,
            };
            let access_key_id = match args.aws_access_key_id {
                Some(ak) => ak,
                None => prompter::input::<String>("Enter AWS access key ID", None, None)?,
            };
            let secret_access_key = match args.aws_secret_access_key {
                Some(sk) => sk,
                None => prompter::input::<String>("Enter AWS secret access key", None, None)?,
            };
            let operator_key_id = match args.aws_operator_key_id {
                Some(ok) => ok,
                None => prompter::input::<String>("Enter AWS operator key ID", None, None)?,
            };

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

            let bn254_keystore = match bn254_keystore {
                Some(Keystore::Local { path: _ }) => {
                    let bn254_keypair_location = match bn254_keypair_location {
                        Some(path) => path,
                        None => prompt_keystore_path()?,
                    };

                    Keystore::Local {
                        path: bn254_keypair_location,
                    }
                }
                // TODO: Update config to handle AWS secret and access keys
                Some(Keystore::Aws { secret: s }) => Keystore::Aws { secret: s },
                None => {
                    prompt_keystore_type(
                        Curve::Bn254,
                        profile.clone(),
                        profile_name,
                        config_path.clone(),
                    )
                    .await?
                }
            };

            let bn254_passphrase = match bn254_passphrase {
                Some(bp) => bp,
                None => prompter::password("Enter BN254 keypair passphrase: ")?,
            };
            let dss_address = match dss_address {
                Some(da) => da,
                None => prompter::input::<Address>("Enter DSS address", None, None)?,
            };
            let message = match message {
                Some(m) => m,
                None => prompter::input::<String>("Enter message", None, None)?,
            };
            let message_encoding = match message_encoding {
                Some(me) => me,
                None => prompt_message_encoding()?,
            };

            dss::process_registration(dss::DSSRegistrationArgs {
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
            let registry_address = match registry_address {
                Some(ra) => ra,
                None => prompter::input::<Address>("Enter registry address", None, None)?,
            };
            let kns = match kns {
                Some(k) => k,
                None => prompter::input::<String>("Enter KNS", None, None)?,
            };

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

            let stake_update_type = match stake_update_type {
                Some(su) => su,
                None => prompt_stake_update_type()?,
            };
            let vault_address = match vault_address {
                Some(va) => va,
                None => prompter::input::<Address>("Enter vault address", None, None)?,
            };
            let dss_address = match dss_address {
                Some(da) => da,
                None => prompter::input::<Address>("Enter DSS address", None, None)?,
            };

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

            let stake_update_type = match stake_update_type {
                Some(su) => su,
                None => prompt_stake_update_type()?,
            };
            let vault_address = match vault_address {
                Some(va) => va,
                None => prompter::input::<Address>("Enter vault address", None, None)?,
            };
            let dss_address = match dss_address {
                Some(da) => da,
                None => prompter::input::<Address>("Enter DSS address", None, None)?,
            };
            let nonce = match nonce {
                Some(n) => n,
                None => prompter::input::<U48>("Enter nonce", None, None)?,
            };
            let start_timestamp = match start_timestamp {
                Some(st) => st,
                None => prompter::input::<U48>("Enter start timestamp", None, None)?,
            };

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
            let vault_address = match vault_address {
                Some(va) => va,
                None => prompter::input::<Address>("Enter vault address", None, None)?,
            };
            let amount = match amount {
                Some(a) => a,
                None => prompter::input::<U256>("Enter amount", None, None)?,
            };
            let vault_instance = VaultInstance::new(vault_address, provider.clone());
            let asset_address = vault_instance.asset().call().await?._0;
            let erc20_instance = ERC20Instance::new(asset_address, provider.clone());

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
            let asset_address = match asset_address {
                Some(aa) => aa,
                None => prompter::input::<Address>("Enter asset address", None, None)?,
            };
            let amount = match amount {
                Some(a) => a,
                None => prompter::input::<U256>("Enter amount", None, None)?,
            };

            let erc20_instance = ERC20MintableInstance::new(asset_address, provider);
            erc20::mint(amount, operator_address, erc20_instance).await?
        }
    }

    Ok(())
}
