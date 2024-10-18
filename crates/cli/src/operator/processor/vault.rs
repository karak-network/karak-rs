use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
    transports::Transport,
};
use eyre::Result;
use karak_contracts::{
    core::contract::VaultLib,
    erc20::mintable::ERC20Mintable::ERC20MintableInstance,
    vault::Vault::VaultInstance,
    Core::{self, CoreInstance},
};

pub async fn process_vault_creation<T: Transport + Clone, P: Provider<T>>(
    asset_address: Address,
    extra_data: Option<Bytes>,
    operator_address: Address,
    vault_impl: Address,
    core_instance: CoreInstance<T, P>,
    erc20_instance: ERC20MintableInstance<T, P>,
) -> Result<()> {
    let extra_data = extra_data.unwrap_or_default();

    let decimals = erc20_instance.decimals().call().await?._0;
    let name = erc20_instance.name().call().await?._0;
    let symbol = erc20_instance.symbol().call().await?._0;

    let vault_config = VaultLib::Config {
        asset: asset_address,
        decimals,
        name,
        symbol,
        operator: operator_address,
        extraData: extra_data,
    };

    let receipt = core_instance
        .deployVaults(vec![vault_config], vault_impl)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("Vault deployed: {}", receipt.transaction_hash);

    let vault_address = receipt.inner.logs()[3]
        .log_decode::<Core::DeployedVault>()?
        .inner
        .data
        .vault;

    println!("Vault deployed at: {vault_address}");

    Ok(())
}

pub async fn deposit_to_vault<T: Transport + Clone, P: Provider<T>>(
    amount: U256,
    operator_address: Address,
    vault_address: Address,
    vault_instance: VaultInstance<T, P>,
    erc20_instance: ERC20MintableInstance<T, P>,
) -> Result<()> {
    let receipt = erc20_instance
        .approve(vault_address, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("Approved: {}", receipt.transaction_hash);

    let receipt = vault_instance
        .deposit_0(amount, operator_address)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("Deposited to vault: {}", receipt.transaction_hash);

    Ok(())
}
