use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
    transports::{http::reqwest, Transport},
};
use eyre::{eyre, Result};
use karak_contracts::{
    core::contract::VaultLib,
    erc20::mintable::ERC20Mintable::ERC20MintableInstance,
    vault::Vault::VaultInstance,
    Core::{self, CoreInstance},
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

use crate::{
    model::{AllowlistedAsset, KarakBackendResult},
    prompter,
};

use crate::util::parse_token_str;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Asset {
    pub address: Address,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{symbol} ({name}) - {address}",
            symbol = self.symbol,
            name = self.name,
            address = self.address,
        )
    }
}

async fn get_asset<T: Transport + Clone, P: Provider<T>>(
    asset_address: Address,
    provider: P,
) -> Result<Asset> {
    let erc20_instance = ERC20MintableInstance::new(asset_address, provider);
    let symbol = erc20_instance.symbol().call_raw().await?;
    let name = erc20_instance.name().call_raw().await?;
    let decimals = erc20_instance.decimals().call().await?._0;

    Ok(Asset {
        address: asset_address,
        symbol: parse_token_str(&symbol).unwrap_or_default(),
        name: parse_token_str(&name).unwrap_or_default(),
        decimals,
    })
}

async fn get_assets<T: Transport + Clone, P: Provider<T> + Clone + 'static>(
    asset_addresses: &[Address],
    operator_address: Address,
    core_instance: CoreInstance<T, P>,
    provider: P,
) -> Result<Vec<Asset>> {
    let deployed_assets =
        get_deployed_assets_for_operator(operator_address, core_instance, provider.clone()).await?;

    let mut join_set = JoinSet::new();
    for asset_address in asset_addresses {
        if deployed_assets.contains(asset_address) {
            println!("Skipping already deployed asset: {asset_address}");
            continue;
        }
        join_set.spawn(get_asset(*asset_address, provider.clone()));
    }

    let mut assets = join_set
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    assets.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    Ok(assets)
}

async fn get_deployed_assets_for_operator<
    T: Transport + Clone,
    P: Provider<T> + Clone + 'static,
>(
    operator_address: Address,
    core_instance: CoreInstance<T, P>,
    provider: P,
) -> Result<HashSet<Address>> {
    let deployed_vaults = core_instance
        .getOperatorVaults(operator_address)
        .call()
        .await?
        .vaults;

    let mut join_set = JoinSet::new();
    for vault in deployed_vaults {
        let provider = provider.clone();
        join_set.spawn(async move {
            let vault_instance = VaultInstance::new(vault, provider);
            vault_instance
                .asset()
                .call()
                .await
                .map(|vault| vault._0)
                .map_err(|e| eyre!(e))
        });
    }
    let deployed_assets = join_set
        .join_all()
        .await
        .into_iter()
        .collect::<Result<HashSet<Address>>>()?;

    Ok(deployed_assets)
}

async fn get_allowlisted_assets<T: Transport + Clone, P: Provider<T> + Clone + 'static>(
    chain_id: u64,
    operator_address: Address,
    core_instance: CoreInstance<T, P>,
    provider: P,
) -> Result<Vec<Asset>> {
    let response = reqwest::get("https://v2-backend.karak.network/trpc/getAllowlistedAssets")
        .await?
        .json::<KarakBackendResult<Vec<AllowlistedAsset>>>()
        .await?;
    let allowlisted_assets = response
        .result
        .data
        .into_iter()
        .filter(|asset| asset.chain_id == chain_id)
        .map(|asset| asset.asset)
        .collect::<Vec<_>>();

    let assets = get_assets(
        &allowlisted_assets,
        operator_address,
        core_instance,
        provider,
    )
    .await?;

    Ok(assets)
}

pub async fn process_vault_creation<T: Transport + Clone, P: Provider<T> + Clone + 'static>(
    asset_addresses: Option<Vec<Address>>,
    operator_address: Address,
    vault_impl: Option<Address>,
    core_instance: CoreInstance<T, P>,
    provider: P,
) -> Result<()> {
    let chain_id = provider.get_chain_id().await?;

    let assets = match &asset_addresses {
        Some(asset_addresses) => {
            get_assets(
                asset_addresses,
                operator_address,
                core_instance.clone(),
                provider.clone(),
            )
            .await?
        }
        None => {
            let assets = get_allowlisted_assets(
                chain_id,
                operator_address,
                core_instance.clone(),
                provider.clone(),
            )
            .await?;
            if assets.is_empty() {
                assets
            } else {
                let selection = prompter::multi_select("Select assets", &assets)?;
                selection
                    .into_iter()
                    .map(|index| assets[index].clone())
                    .collect::<Vec<_>>()
            }
        }
    };

    if assets.is_empty() {
        println!("No assets to deploy");
        return Ok(());
    }

    let vault_impl = vault_impl.unwrap_or_default();

    let mut vault_configs = Vec::new();
    for asset in &assets {
        println!("Creating vault for asset: {}", asset.symbol);

        let name = prompter::input("Please enter vault name", Some(asset.name.clone()), None)?;

        let symbol = prompter::input(
            "Please enter vault symbol",
            Some(asset.symbol.clone()),
            None,
        )?;

        let extra_data = prompter::input(
            "Please enter any extra data",
            Some(Bytes::default()),
            Some(prompter::InputOptions {
                allow_empty: true,
                initial_text: "0x".to_string(),
            }),
        )?;

        let vault_config = VaultLib::Config {
            asset: asset.address,
            decimals: asset.decimals,
            name,
            symbol,
            operator: operator_address,
            extraData: extra_data,
        };

        vault_configs.push(vault_config);
    }

    println!("Deploying the following vaults:");
    println!("{}", serde_json::to_string_pretty(&vault_configs)?);

    let confirm = prompter::confirm("Do you want to proceed with the deployment?", None)?;
    if !confirm {
        println!("Aborting deployment");
        return Ok(());
    }

    let receipt = core_instance
        .deployVaults(vault_configs, vault_impl)
        .send()
        .await?
        .get_receipt()
        .await?;

    let asset_map = assets
        .into_iter()
        .map(|asset| (asset.address, asset))
        .collect::<HashMap<_, _>>();

    println!("Vault(s) deployed in tx {}", receipt.transaction_hash);
    for logs in receipt.inner.logs().chunks(4) {
        let log = logs[3].log_decode::<Core::DeployedVault>()?.inner.data;
        let vault = log.vault;
        let asset = &asset_map[&log.asset];
        println!(
            "Vault: {vault}, Asset: {} ({})",
            asset.symbol, asset.address
        );
    }

    Ok(())
}

pub async fn deposit_to_vault<T: Transport + Clone, P: Provider<T>>(
    amount: U256,
    operator_address: Address,
    vault_address: Address,
    vault_instance: VaultInstance<T, P>,
    erc20_instance: ERC20MintableInstance<T, P>,
) -> Result<()> {
    let symbol = erc20_instance.symbol().call().await?._0;
    let receipt = erc20_instance
        .approve(vault_address, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Approved spending {} {} in tx {}",
        amount, symbol, receipt.transaction_hash
    );

    let receipt = vault_instance
        .deposit_0(amount, operator_address)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Deposited {} {} to vault {} in tx {}",
        amount, symbol, vault_address, receipt.transaction_hash
    );

    Ok(())
}
