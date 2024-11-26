use std::fmt::{Debug, Display};

use alloy::{
    providers::PendingTransactionError,
    rpc::json_rpc::ErrorPayload,
    sol,
    transports::{RpcError, TransportError},
};
use serde::{ser::SerializeStruct, Serialize};
use Operator::{QueuedStakeUpdate, StakeUpdateRequest};
use VaultLib::Config;

use crate::error::DecodeError;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Core,
    "abi/Core.json",
);

impl std::fmt::Debug for Core::CoreErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Core::CoreErrors::AlreadyInitialized(_) => write!(f, "AlreadyInitialized"),
            Core::CoreErrors::AssetNotAllowlisted(_) => write!(f, "AssetNotAllowlisted"),
            Core::CoreErrors::AttemptedPauseWhileUnpausing(_) => {
                write!(f, "AttemptedPauseWhileUnpausing")
            }
            Core::CoreErrors::AttemptedUnpauseWhilePausing(_) => {
                write!(f, "AttemptedUnpauseWhilePausing")
            }
            Core::CoreErrors::DSSAlreadyRegistered(_) => write!(f, "DSSAlreadyRegistered"),
            Core::CoreErrors::DSSHookCallReverted(_) => write!(f, "DSSHookCallReverted"),
            Core::CoreErrors::DSSNotRegistered(_) => write!(f, "DSSNotRegistered"),
            Core::CoreErrors::DuplicateSlashingVaults(_) => write!(f, "DuplicateSlashingVaults"),
            Core::CoreErrors::EmptyArray(_) => write!(f, "EmptyArray"),
            Core::CoreErrors::EnforcedPause(_) => write!(f, "EnforcedPause"),
            Core::CoreErrors::EnforcedPauseFunction(_) => write!(f, "EnforcedPauseFunction"),
            Core::CoreErrors::InvalidInitialization(_) => write!(f, "InvalidInitialization"),
            Core::CoreErrors::InvalidSlashingCount(_) => write!(f, "InvalidSlashingCount"),
            Core::CoreErrors::InvalidSlashingParams(_) => write!(f, "InvalidSlashingParams"),
            Core::CoreErrors::LengthsDontMatch(_) => write!(f, "LengthsDontMatch"),
            Core::CoreErrors::MathOverflowedMulDiv(_) => write!(f, "MathOverflowedMulDiv"),
            Core::CoreErrors::MaxSlashPercentageWadBreached(_) => {
                write!(f, "MaxSlashPercentageWadBreached")
            }
            Core::CoreErrors::MaxSlashableVaultsPerRequestBreached(_) => {
                write!(f, "MaxSlashableVaultsPerRequestBreached")
            }
            Core::CoreErrors::MaxVaultCapacityReached(_) => write!(f, "MaxVaultCapacityReached"),
            Core::CoreErrors::MinSlashingDelayNotPassed(_) => {
                write!(f, "MinSlashingDelayNotPassed")
            }
            Core::CoreErrors::NewOwnerIsZeroAddress(_) => write!(f, "NewOwnerIsZeroAddress"),
            Core::CoreErrors::NoHandoverRequest(_) => write!(f, "NoHandoverRequest"),
            Core::CoreErrors::NotEnoughGas(_) => write!(f, "NotEnoughGas"),
            Core::CoreErrors::NotInitializing(_) => write!(f, "NotInitializing"),
            Core::CoreErrors::NotSmartContract(_) => write!(f, "NotSmartContract"),
            Core::CoreErrors::OperatorNotValidatingForDSS(_) => {
                write!(f, "OperatorNotValidatingForDSS")
            }
            Core::CoreErrors::Reentrancy(_) => write!(f, "Reentrancy"),
            Core::CoreErrors::ReservedAddress(_) => write!(f, "ReservedAddress"),
            Core::CoreErrors::SlashingCooldownNotPassed(_) => {
                write!(f, "SlashingCooldownNotPassed")
            }
            Core::CoreErrors::Unauthorized(_) => write!(f, "Unauthorized"),
            Core::CoreErrors::VaultCreationFailedAddrMismatch(mismatch) => {
                write!(
                    f,
                    "VaultCreationFailedAddrMismatch {{ expected: {}, actual: {} }}",
                    mismatch.expected, mismatch.actual
                )
            }
            Core::CoreErrors::VaultImplNotAllowlisted(_) => write!(f, "VaultImplNotAllowlisted"),
            Core::CoreErrors::VaultNotAChildVault(_) => write!(f, "VaultNotAChildVault"),
            Core::CoreErrors::VaultNotStakedToDSS(_) => write!(f, "VaultNotStakedToDSS"),
            Core::CoreErrors::ZeroAddress(_) => write!(f, "ZeroAddress"),
            Core::CoreErrors::ZeroSlashPercentageWad(_) => write!(f, "ZeroSlashPercentageWad"),
        }
    }
}

impl Display for Core::CoreErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Core::CoreErrors::AlreadyInitialized(_) => write!(f, "Already initialized"),
            Core::CoreErrors::AssetNotAllowlisted(_) => {
                write!(f, "Asset not allowlisted")
            }
            Core::CoreErrors::AttemptedPauseWhileUnpausing(_) => {
                write!(f, "Attempted pause while unpausing")
            }
            Core::CoreErrors::AttemptedUnpauseWhilePausing(_) => {
                write!(f, "Attempted unpause while pausing")
            }
            Core::CoreErrors::DSSAlreadyRegistered(_) => write!(f, "DSS already registered"),
            Core::CoreErrors::DSSHookCallReverted(e) => {
                write!(f, "DSS hook call reverted with error: {}", e.revertReason)
            }
            Core::CoreErrors::DSSNotRegistered(_) => write!(f, "DSS not registered"),
            Core::CoreErrors::DuplicateSlashingVaults(_) => write!(f, "Duplicate slashing vaults"),
            Core::CoreErrors::EmptyArray(_) => write!(f, "Empty array"),
            Core::CoreErrors::EnforcedPause(_) => write!(f, "Enforced pause"),
            Core::CoreErrors::EnforcedPauseFunction(_) => write!(f, "Enforced pause function"),
            Core::CoreErrors::InvalidInitialization(_) => write!(f, "Invalid initialization"),
            Core::CoreErrors::InvalidSlashingCount(_) => write!(f, "Invalid slashing count"),
            Core::CoreErrors::InvalidSlashingParams(_) => write!(f, "Invalid slashing params"),
            Core::CoreErrors::LengthsDontMatch(_) => write!(f, "Lengths don't match"),
            Core::CoreErrors::MathOverflowedMulDiv(_) => write!(f, "Math overflowed mul div"),
            Core::CoreErrors::MaxSlashPercentageWadBreached(_) => {
                write!(f, "Max slash percentage wad breached")
            }
            Core::CoreErrors::MaxSlashableVaultsPerRequestBreached(_) => {
                write!(f, "Max slashable vaults per request breached")
            }
            Core::CoreErrors::MaxVaultCapacityReached(_) => write!(f, "Max vault capacity reached"),
            Core::CoreErrors::MinSlashingDelayNotPassed(_) => {
                write!(f, "Min slashing delay not passed")
            }
            Core::CoreErrors::NewOwnerIsZeroAddress(_) => write!(f, "New owner is zero address"),
            Core::CoreErrors::NoHandoverRequest(_) => write!(f, "No handover request"),
            Core::CoreErrors::NotEnoughGas(_) => write!(f, "Not enough gas"),
            Core::CoreErrors::NotInitializing(_) => write!(f, "Not initializing"),
            Core::CoreErrors::NotSmartContract(_) => write!(f, "Not smart contract"),
            Core::CoreErrors::OperatorNotValidatingForDSS(_) => {
                write!(f, "Operator not validating for DSS")
            }
            Core::CoreErrors::Reentrancy(_) => write!(f, "Reentrancy"),
            Core::CoreErrors::ReservedAddress(_) => write!(f, "Reserved address"),
            Core::CoreErrors::SlashingCooldownNotPassed(_) => {
                write!(f, "Slashing cooldown not passed")
            }
            Core::CoreErrors::Unauthorized(_) => write!(f, "Unauthorized"),
            Core::CoreErrors::VaultCreationFailedAddrMismatch(e) => {
                write!(
                    f,
                    "Vault creation failed address mismatch, expected {}, got {}",
                    e.expected, e.actual
                )
            }
            Core::CoreErrors::VaultImplNotAllowlisted(_) => write!(f, "Vault impl not allowlisted"),
            Core::CoreErrors::VaultNotAChildVault(_) => write!(f, "Vault not a child vault"),
            Core::CoreErrors::VaultNotStakedToDSS(_) => write!(f, "Vault not staked to DSS"),
            Core::CoreErrors::ZeroAddress(_) => write!(f, "Zero address"),
            Core::CoreErrors::ZeroSlashPercentageWad(_) => write!(f, "Zero slash percentage wad"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CoreError<E: std::fmt::Debug> {
    #[error("Core error: {0}")]
    Revert(Core::CoreErrors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<Core::CoreErrors> for CoreError<E> {
    fn from(error: Core::CoreErrors) -> Self {
        CoreError::Revert(error)
    }
}

impl From<ErrorPayload> for CoreError<ErrorPayload> {
    fn from(value: ErrorPayload) -> Self {
        match value.as_decoded_error::<Core::CoreErrors>(true) {
            Some(error) => CoreError::Revert(error),
            None => CoreError::Inner(value),
        }
    }
}

impl DecodeError<Core::CoreErrors> for TransportError {
    fn decode_error(&self) -> Option<Core::CoreErrors> {
        match self {
            RpcError::ErrorResp(error) => error.as_decoded_error::<Core::CoreErrors>(true),
            _ => None,
        }
    }
}

impl From<TransportError> for CoreError<TransportError> {
    fn from(value: TransportError) -> Self {
        match value.decode_error() {
            Some(error) => CoreError::Revert(error),
            _ => CoreError::Inner(value),
        }
    }
}

impl DecodeError<Core::CoreErrors> for alloy::contract::Error {
    fn decode_error(&self) -> Option<Core::CoreErrors> {
        match self {
            alloy::contract::Error::TransportError(transport_error) => {
                transport_error.decode_error()
            }
            _ => None,
        }
    }
}

impl From<alloy::contract::Error> for CoreError<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match value.decode_error() {
            Some(error) => CoreError::Revert(error),
            _ => CoreError::Inner(value),
        }
    }
}

impl DecodeError<Core::CoreErrors> for PendingTransactionError {
    fn decode_error(&self) -> Option<Core::CoreErrors> {
        match self {
            PendingTransactionError::TransportError(transport_error) => {
                transport_error.decode_error()
            }
            _ => None,
        }
    }
}

impl From<PendingTransactionError> for CoreError<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match value.decode_error() {
            Some(error) => CoreError::Revert(error),
            _ => CoreError::Inner(value),
        }
    }
}

impl Serialize for StakeUpdateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("StakeUpdateRequest", 3)?;
        state.serialize_field("vault", &self.vault)?;
        state.serialize_field("dss", &self.dss)?;
        state.serialize_field("toStake", &self.toStake)?;
        state.end()
    }
}

impl Serialize for QueuedStakeUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("QueuedStakeUpdate", 4)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("startTimestamp", &self.startTimestamp)?;
        state.serialize_field("operator", &self.operator)?;
        state.serialize_field("updateRequest", &self.updateRequest)?;
        state.end()
    }
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("VaultConfig", 5)?;
        state.serialize_field("asset", &self.asset)?;
        state.serialize_field("decimals", &self.decimals)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("symbol", &self.symbol)?;
        state.serialize_field("operator", &self.operator)?;
        state.end()
    }
}
