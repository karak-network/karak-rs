use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    transports::Transport,
};
use eyre::Result;
use karak_contracts::erc20::mintable::ERC20Mintable::ERC20MintableInstance;

pub async fn mint<T: Transport + Clone, P: Provider<T>>(
    amount: U256,
    operator_address: Address,
    erc20_instance: ERC20MintableInstance<T, P>,
) -> Result<()> {
    let symbol = erc20_instance.symbol().call().await?._0;
    let receipt = erc20_instance
        .mint(operator_address, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Minted {} {} to {} in tx {}",
        amount, symbol, operator_address, receipt.transaction_hash
    );

    Ok(())
}
