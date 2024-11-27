use alloy::{
    providers::PendingTransactionError, rpc::json_rpc::ErrorPayload, sol,
    transports::TransportError,
};

use crate::{error::DecodeError, impl_decode_error};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Operator,
    "abi/Operator.json",
);

impl std::fmt::Debug for Operator::OperatorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::OperatorErrors::AllVaultsNotUnstakedFromDSS(_) => {
                write!(f, "AllVaultsNotUnstakedFromDSS")
            }
            Operator::OperatorErrors::DSSHookCallReverted(_) => {
                write!(f, "DSSHookCallReverted")
            }
            Operator::OperatorErrors::InvalidQueuedStakeUpdateInput(_) => {
                write!(f, "InvalidQueuedStakeUpdateInput")
            }
            Operator::OperatorErrors::MaxDSSCapacityReached(_) => {
                write!(f, "MaxDSSCapacityReached")
            }
            Operator::OperatorErrors::NotEnoughGas(_) => {
                write!(f, "NotEnoughGas")
            }
            Operator::OperatorErrors::OperatorAlreadyRegisteredToDSS(_) => {
                write!(f, "OperatorAlreadyRegisteredToDSS")
            }
            Operator::OperatorErrors::OperatorNotValidatingForDSS(_) => {
                write!(f, "OperatorNotValidatingForDSS")
            }
            Operator::OperatorErrors::OperatorStakeUpdateDelayNotPassed(_) => {
                write!(f, "OperatorStakeUpdateDelayNotPassed")
            }
            Operator::OperatorErrors::PendingStakeUpdateRequest(_) => {
                write!(f, "PendingStakeUpdateRequest")
            }
            Operator::OperatorErrors::VaultAlreadyStakedInDSS(_) => {
                write!(f, "VaultAlreadyStakedInDSS")
            }
            Operator::OperatorErrors::VaultNotAChildVault(_) => {
                write!(f, "VaultNotAChildVault")
            }
            Operator::OperatorErrors::VaultNotStakedInDSS(_) => {
                write!(f, "VaultNotStakedInDSS")
            }
        }
    }
}

impl std::fmt::Display for Operator::OperatorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::OperatorErrors::AllVaultsNotUnstakedFromDSS(_) => {
                write!(f, "All vaults not unstaked from DSS")
            }
            Operator::OperatorErrors::DSSHookCallReverted(_) => {
                write!(f, "DSS hook call reverted")
            }
            Operator::OperatorErrors::InvalidQueuedStakeUpdateInput(_) => {
                write!(f, "Invalid queued stake update input")
            }
            Operator::OperatorErrors::MaxDSSCapacityReached(_) => {
                write!(f, "Max DSS capacity reached")
            }
            Operator::OperatorErrors::NotEnoughGas(_) => {
                write!(f, "Not enough gas")
            }
            Operator::OperatorErrors::OperatorAlreadyRegisteredToDSS(_) => {
                write!(f, "Operator already registered to DSS")
            }
            Operator::OperatorErrors::OperatorNotValidatingForDSS(_) => {
                write!(f, "Operator not validating for DSS")
            }
            Operator::OperatorErrors::OperatorStakeUpdateDelayNotPassed(_) => {
                write!(f, "Operator stake update delay not passed")
            }
            Operator::OperatorErrors::PendingStakeUpdateRequest(_) => {
                write!(f, "Pending stake update request")
            }
            Operator::OperatorErrors::VaultAlreadyStakedInDSS(_) => {
                write!(f, "Vault already staked in DSS")
            }
            Operator::OperatorErrors::VaultNotAChildVault(_) => {
                write!(f, "Vault not a child vault")
            }
            Operator::OperatorErrors::VaultNotStakedInDSS(_) => {
                write!(f, "Vault not staked in DSS")
            }
        }
    }
}

impl_decode_error!(Operator::OperatorErrors);

#[derive(Debug, thiserror::Error)]
pub enum OperatorError<E: std::fmt::Debug> {
    #[error("Operator error: {0}")]
    Operator(Operator::OperatorErrors),
    #[error(transparent)]
    Inner(E),
}

impl From<ErrorPayload> for OperatorError<ErrorPayload> {
    fn from(value: ErrorPayload) -> Self {
        match Operator::OperatorErrors::decode_error(&value) {
            Some(error) => OperatorError::Operator(error),
            None => OperatorError::Inner(value),
        }
    }
}

impl From<TransportError> for OperatorError<TransportError> {
    fn from(value: TransportError) -> Self {
        match Operator::OperatorErrors::decode_error(&value) {
            Some(error) => OperatorError::Operator(error),
            _ => OperatorError::Inner(value),
        }
    }
}

impl From<alloy::contract::Error> for OperatorError<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match Operator::OperatorErrors::decode_error(&value) {
            Some(error) => OperatorError::Operator(error),
            _ => OperatorError::Inner(value),
        }
    }
}

impl From<PendingTransactionError> for OperatorError<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match Operator::OperatorErrors::decode_error(&value) {
            Some(error) => OperatorError::Operator(error),
            _ => OperatorError::Inner(value),
        }
    }
}
