use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
    transports::{http::reqwest, Transport},
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect};
use eyre::Result;
use karak_contracts::{
    core::contract::VaultLib,
    erc20::mintable::ERC20Mintable::ERC20MintableInstance,
    vault::Vault::VaultInstance,
    Core::{self, CoreInstance},
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct AllowlistedAsset {
    pub asset: Address,
}

#[derive(Debug, Deserialize)]
pub struct AllowlistedAssetsData {
    pub data: Vec<AllowlistedAsset>,
}

#[derive(Debug, Deserialize)]
pub struct AllowlistedAssets {
    pub result: AllowlistedAssetsData,
}

async fn get_allowlisted_assets() -> Result<Vec<Address>> {
    let url = Url::parse("https://v2-backend.karak.network/trpc/getAllowlistedAssets")?;
    let response = reqwest::get(url).await?.json::<AllowlistedAssets>().await?;
    let allowlisted_assets = response
        .result
        .data
        .iter()
        .map(|asset| asset.asset)
        .collect::<Vec<_>>();

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select assets")
        .items(&allowlisted_assets)
        .interact()?;

    Ok(selection
        .into_iter()
        .map(|index| allowlisted_assets[index])
        .collect::<Vec<_>>())
}

pub async fn process_vault_creation<T: Transport + Clone, P: Provider<T> + Clone>(
    assets: Option<Vec<Address>>,
    operator_address: Address,
    vault_impl: Address,
    core_instance: CoreInstance<T, P>,
    provider: P,
) -> Result<()> {
    let assets = match &assets {
        Some(assets) => assets.clone(),
        None => get_allowlisted_assets().await?,
    };
    let erc20_instances = assets
        .iter()
        .map(|asset| ERC20MintableInstance::new(*asset, provider.clone()))
        .collect::<Vec<_>>();

    let mut vault_configs = Vec::new();
    for erc20_instance in erc20_instances {
        let decimals = erc20_instance.decimals().call().await?._0;
        let name = erc20_instance.name().call().await?._0;
        let symbol = erc20_instance.symbol().call().await?._0;
        let asset = erc20_instance.address();

        let extra_data = Input::<Bytes>::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Asset: {asset}, Name: {name}, Symbol: {symbol}, Decimals: {decimals}\nPlease enter any extra data:",
            ))
            .with_initial_text("0x")
            .default(Bytes::default())
            .allow_empty(true)
            .interact()?;

        let vault_config = VaultLib::Config {
            asset: *asset,
            decimals,
            name,
            symbol,
            operator: operator_address,
            extraData: extra_data,
        };

        vault_configs.push(vault_config);
    }

    println!("Deploying the following vaults:");
    for vault_config in &vault_configs {
        println!(
            "Asset: {}, Name: {}, Symbol: {}, extraData: {}",
            vault_config.asset, vault_config.name, vault_config.symbol, vault_config.extraData
        );
    }

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

    let vault_address = receipt.inner.logs()[3]
        .log_decode::<Core::DeployedVault>()?
        .inner
        .data
        .vault;

    println!(
        "Vault deployed at {} in tx {}",
        vault_address, receipt.transaction_hash
    );

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
