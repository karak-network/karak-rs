use alloy::{
    primitives::{Address, Bytes},
    providers::Provider,
    transports::Transport,
};
use eyre::Result;
use karak_contracts::Core::CoreInstance;

pub async fn process_vault_creation<T: Transport + Clone, P: Provider<T>>(
    _asset_address: &Address,
    _extra_data: Option<&Bytes>,
    _core_instance: CoreInstance<T, P>,
) -> Result<()> {
    todo!()
}
