use std::fmt::Display;

use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
    transports::{http::reqwest, Transport},
};
use dialoguer::{theme::ColorfulTheme, Confirm};
use eyre::Result;
use karak_contracts::{
    core::contract::VaultLib,
    erc20::mintable::ERC20Mintable::ERC20MintableInstance,
    vault::Vault::VaultInstance,
    Core::{self, CoreInstance},
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use url::Url;

use crate::prompter;

use crate::util::parse_token_str;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowlistedAsset {
    pub asset: Address,
    pub chain_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct AllowlistedAssetsData {
    pub data: Vec<AllowlistedAsset>,
}

#[derive(Debug, Deserialize)]
pub struct AllowlistedAssets {
    pub result: AllowlistedAssetsData,
}

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
    provider: P,
) -> Result<Vec<Asset>> {
    let mut join_set = JoinSet::new();
    for asset_address in asset_addresses {
        join_set.spawn(get_asset(*asset_address, provider.clone()));
    }

    let assets = join_set
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    Ok(assets)
}

async fn get_allowlisted_assets<T: Transport + Clone, P: Provider<T> + Clone + 'static>(
    chain_id: u64,
    provider: P,
) -> Result<Vec<Asset>> {
    let url = Url::parse("https://v2-backend.karak.network/trpc/getAllowlistedAssets")?;
    let response = reqwest::get(url).await?.json::<AllowlistedAssets>().await?;
    let allowlisted_assets = response
        .result
        .data
        .iter()
        .filter(|asset| asset.chain_id == chain_id)
        .map(|asset| asset.asset)
        .collect::<Vec<_>>();

    let assets = get_assets(&allowlisted_assets, provider).await?;

    let selection = prompter::multi_select("Select assets", &assets);

    Ok(selection?
        .into_iter()
        .map(|index| assets[index].clone())
        .collect::<Vec<_>>())
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
        Some(asset_addresses) => get_assets(asset_addresses, provider.clone()).await?,
        None => get_allowlisted_assets(chain_id, provider.clone()).await?,
    };
    let vault_impl = vault_impl.unwrap_or_default();

    let mut vault_configs = Vec::new();
    for asset in assets {
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

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to proceed with the deployment?")
        .interact()?;
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

    println!("Vault(s) deployed in tx {}", receipt.transaction_hash);
    for logs in receipt.inner.logs().chunks(4) {
        let log = logs[3].log_decode::<Core::DeployedVault>()?.inner.data;
        let vault = log.vault;
        let asset = log.asset;
        println!("Deployed vault {vault} for asset {asset}");
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
