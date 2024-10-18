use alloy::{primitives::Address, providers::Provider, transports::Transport};
use eyre::Result;
use karak_contracts::registry::RestakingRegistry::RestakingRegistryInstance;

pub async fn process_registry_registration<T: Transport + Clone, P: Provider<T>>(
    kns: String,
    operator_address: Address,
    registry_instance: RestakingRegistryInstance<T, P>,
) -> Result<()> {
    let receipt = registry_instance
        .register(kns, operator_address, operator_address)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "Registered operator {} to registry in tx {}",
        operator_address, receipt.transaction_hash
    );

    Ok(())
}
