use alloy::{
    primitives::{aliases::U48, Address},
    providers::Provider,
    transports::Transport,
};
use clap::ValueEnum;
use eyre::Result;
use karak_contracts::{
    core::contract::Operator::{QueuedStakeUpdate, StakeUpdateRequest},
    Core::{self, CoreInstance},
};
use strum_macros::{Display, EnumString, FromRepr, VariantNames};

#[derive(Debug, Clone, ValueEnum, EnumString, FromRepr, VariantNames, Display)]
pub enum StakeUpdateType {
    Stake,
    Unstake,
}

impl From<StakeUpdateType> for bool {
    fn from(stake_update_type: StakeUpdateType) -> bool {
        match stake_update_type {
            StakeUpdateType::Stake => true,
            StakeUpdateType::Unstake => false,
        }
    }
}

pub async fn process_stake_update_request<T: Transport + Clone, P: Provider<T>>(
    vault_address: Address,
    dss_address: Address,
    stake_update_type: StakeUpdateType,
    core_instance: CoreInstance<T, P>,
) -> Result<()> {
    let stake_update_request = StakeUpdateRequest {
        vault: vault_address,
        dss: dss_address,
        toStake: stake_update_type.into(),
    };

    let receipt = core_instance
        .requestUpdateVaultStakeInDSS(stake_update_request)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("Requested stake update in tx {}", receipt.transaction_hash);

    let queued_stake_update = receipt.inner.logs()[1]
        .log_decode::<Core::RequestedStakeUpdate>()?
        .inner
        .updateRequest
        .clone();

    println!(
        "Queued stake update: {}",
        serde_json::to_string_pretty(&queued_stake_update)?
    );

    Ok(())
}

pub async fn process_finalize_stake_update_request<T: Transport + Clone, P: Provider<T>>(
    vault_address: Address,
    dss_address: Address,
    stake_update_type: StakeUpdateType,
    nonce: U48,
    start_timestamp: U48,
    operator_address: Address,
    core_instance: CoreInstance<T, P>,
) -> Result<()> {
    let queued_stake_update = QueuedStakeUpdate {
        nonce,
        startTimestamp: start_timestamp,
        operator: operator_address,
        updateRequest: StakeUpdateRequest {
            vault: vault_address,
            dss: dss_address,
            toStake: stake_update_type.into(),
        },
    };
    let receipt = core_instance
        .finalizeUpdateVaultStakeInDSS(queued_stake_update)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("Finalized stake update in tx {}", receipt.transaction_hash);

    Ok(())
}
