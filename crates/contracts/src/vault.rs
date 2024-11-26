use alloy::{
    providers::PendingTransactionError, rpc::json_rpc::ErrorPayload, sol,
    transports::TransportError,
};

use crate::{erc20::library::SafeTransferLib, error::DecodeError};

sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    Vault,
    "abi/Vault.json",
);

impl std::fmt::Debug for Vault::VaultErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vault::VaultErrors::AllowanceOverflow(_) => {
                write!(f, "AllowanceOverflow")
            }
            Vault::VaultErrors::AllowanceUnderflow(_) => {
                write!(f, "AllowanceUnderflow")
            }
            Vault::VaultErrors::AlreadyInitialized(_) => {
                write!(f, "AlreadyInitialized")
            }
            Vault::VaultErrors::AttemptedPauseWhileUnpausing(_) => {
                write!(f, "AttemptedPauseWhileUnpausing")
            }
            Vault::VaultErrors::AttemptedUnpauseWhilePausing(_) => {
                write!(f, "AttemptedUnpauseWhilePausing")
            }
            Vault::VaultErrors::DepositMoreThanMax(_) => {
                write!(f, "DepositMoreThanMax")
            }
            Vault::VaultErrors::EnforcedPause(_) => {
                write!(f, "EnforcedPause")
            }
            Vault::VaultErrors::EnforcedPauseFunction(_) => {
                write!(f, "EnforcedPauseFunction")
            }
            Vault::VaultErrors::InsufficientAllowance(_) => {
                write!(f, "InsufficientAllowance")
            }
            Vault::VaultErrors::InsufficientBalance(_) => {
                write!(f, "InsufficientBalance")
            }
            Vault::VaultErrors::InvalidInitialization(_) => {
                write!(f, "InvalidInitialization")
            }
            Vault::VaultErrors::InvalidPermit(_) => {
                write!(f, "InvalidPermit")
            }
            Vault::VaultErrors::MinWithdrawDelayNotPassed(_) => {
                write!(f, "MinWithdrawDelayNotPassed")
            }
            Vault::VaultErrors::MintMoreThanMax(_) => {
                write!(f, "MintMoreThanMax")
            }
            Vault::VaultErrors::NewOwnerIsZeroAddress(_) => {
                write!(f, "NewOwnerIsZeroAddress")
            }
            Vault::VaultErrors::NoHandoverRequest(_) => {
                write!(f, "NoHandoverRequest")
            }
            Vault::VaultErrors::NotEnoughShares(_) => {
                write!(f, "NotEnoughShares")
            }
            Vault::VaultErrors::NotImplemented(_) => {
                write!(f, "NotImplemented")
            }
            Vault::VaultErrors::NotInitializing(_) => {
                write!(f, "NotInitializing")
            }
            Vault::VaultErrors::PermitExpired(_) => {
                write!(f, "PermitExpired")
            }
            Vault::VaultErrors::RedeemMoreThanMax(_) => {
                write!(f, "RedeemMoreThanMax")
            }
            Vault::VaultErrors::Reentrancy(_) => {
                write!(f, "Reentrancy")
            }
            Vault::VaultErrors::TotalSupplyOverflow(_) => {
                write!(f, "TotalSupplyOverflow")
            }
            Vault::VaultErrors::Unauthorized(_) => {
                write!(f, "Unauthorized")
            }
            Vault::VaultErrors::WithdrawMoreThanMax(_) => {
                write!(f, "WithdrawMoreThanMax")
            }
            Vault::VaultErrors::WithdrawalNotFound(_) => {
                write!(f, "WithdrawalNotFound")
            }
            Vault::VaultErrors::ZeroAddress(_) => {
                write!(f, "ZeroAddress")
            }
            Vault::VaultErrors::ZeroAmount(_) => {
                write!(f, "ZeroAmount")
            }
            Vault::VaultErrors::ZeroShares(_) => {
                write!(f, "ZeroShares")
            }
        }
    }
}

impl std::fmt::Display for Vault::VaultErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vault::VaultErrors::AllowanceOverflow(_) => {
                write!(f, "Allowance overflow")
            }
            Vault::VaultErrors::AllowanceUnderflow(_) => {
                write!(f, "Allowance underflow")
            }
            Vault::VaultErrors::AlreadyInitialized(_) => {
                write!(f, "Already initialized")
            }
            Vault::VaultErrors::AttemptedPauseWhileUnpausing(_) => {
                write!(f, "Attempted pause while unpausing")
            }
            Vault::VaultErrors::AttemptedUnpauseWhilePausing(_) => {
                write!(f, "Attempted unpause while pausing")
            }
            Vault::VaultErrors::DepositMoreThanMax(_) => {
                write!(f, "Deposit more than max")
            }
            Vault::VaultErrors::EnforcedPause(_) => {
                write!(f, "Enforced pause")
            }
            Vault::VaultErrors::EnforcedPauseFunction(_) => {
                write!(f, "Enforced pause function")
            }
            Vault::VaultErrors::InsufficientAllowance(_) => {
                write!(f, "Insufficient allowance")
            }
            Vault::VaultErrors::InsufficientBalance(_) => {
                write!(f, "Insufficient balance")
            }
            Vault::VaultErrors::InvalidInitialization(_) => {
                write!(f, "Invalid initialization")
            }
            Vault::VaultErrors::InvalidPermit(_) => {
                write!(f, "Invalid permit")
            }
            Vault::VaultErrors::MinWithdrawDelayNotPassed(_) => {
                write!(f, "Min withdraw delay not passed")
            }
            Vault::VaultErrors::MintMoreThanMax(_) => {
                write!(f, "Mint more than max")
            }
            Vault::VaultErrors::NewOwnerIsZeroAddress(_) => {
                write!(f, "New owner is zero address")
            }
            Vault::VaultErrors::NoHandoverRequest(_) => {
                write!(f, "No handover request")
            }
            Vault::VaultErrors::NotEnoughShares(_) => {
                write!(f, "Not enough shares")
            }
            Vault::VaultErrors::NotImplemented(_) => {
                write!(f, "Not implemented")
            }
            Vault::VaultErrors::NotInitializing(_) => {
                write!(f, "Not initializing")
            }
            Vault::VaultErrors::PermitExpired(_) => {
                write!(f, "Permit expired")
            }
            Vault::VaultErrors::RedeemMoreThanMax(_) => {
                write!(f, "Redeem more than max")
            }
            Vault::VaultErrors::Reentrancy(_) => {
                write!(f, "Reentrancy")
            }
            Vault::VaultErrors::TotalSupplyOverflow(_) => {
                write!(f, "Total supply overflow")
            }
            Vault::VaultErrors::Unauthorized(_) => {
                write!(f, "Unauthorized")
            }
            Vault::VaultErrors::WithdrawMoreThanMax(_) => {
                write!(f, "Withdraw more than max")
            }
            Vault::VaultErrors::WithdrawalNotFound(_) => {
                write!(f, "Withdrawal not found")
            }
            Vault::VaultErrors::ZeroAddress(_) => {
                write!(f, "Zero address")
            }
            Vault::VaultErrors::ZeroAmount(_) => {
                write!(f, "Zero amount")
            }
            Vault::VaultErrors::ZeroShares(_) => {
                write!(f, "Zero shares")
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VaultError<E: std::fmt::Debug> {
    #[error("Vault error: {0}")]
    Vault(Vault::VaultErrors),
    #[error("Transfer error: {0}")]
    SafeTransferLib(SafeTransferLib::SafeTransferLibErrors),
    #[error(transparent)]
    Inner(E),
}

impl DecodeError<ErrorPayload> for Vault::VaultErrors {
    fn decode_error(error: &ErrorPayload) -> Option<Vault::VaultErrors> {
        error.as_decoded_error::<Vault::VaultErrors>(true)
    }
}

impl From<ErrorPayload> for VaultError<ErrorPayload> {
    fn from(error: ErrorPayload) -> Self {
        match Vault::VaultErrors::decode_error(&error) {
            Some(error) => VaultError::Vault(error),
            None => match SafeTransferLib::SafeTransferLibErrors::decode_error(&error) {
                Some(error) => VaultError::SafeTransferLib(error),
                None => VaultError::Inner(error),
            },
        }
    }
}

impl DecodeError<TransportError> for Vault::VaultErrors {
    fn decode_error(error: &TransportError) -> Option<Vault::VaultErrors> {
        match error {
            alloy::transports::RpcError::ErrorResp(error) => {
                Vault::VaultErrors::decode_error(error)
            }
            _ => None,
        }
    }
}

impl From<TransportError> for VaultError<TransportError> {
    fn from(error: TransportError) -> Self {
        match Vault::VaultErrors::decode_error(&error) {
            Some(error) => VaultError::Vault(error),
            None => match SafeTransferLib::SafeTransferLibErrors::decode_error(&error) {
                Some(error) => VaultError::SafeTransferLib(error),
                None => VaultError::Inner(error),
            },
        }
    }
}

impl DecodeError<alloy::contract::Error> for Vault::VaultErrors {
    fn decode_error(error: &alloy::contract::Error) -> Option<Vault::VaultErrors> {
        match error {
            alloy::contract::Error::TransportError(error) => {
                Vault::VaultErrors::decode_error(error)
            }
            _ => None,
        }
    }
}

impl From<alloy::contract::Error> for VaultError<alloy::contract::Error> {
    fn from(error: alloy::contract::Error) -> Self {
        match Vault::VaultErrors::decode_error(&error) {
            Some(error) => VaultError::Vault(error),
            None => match SafeTransferLib::SafeTransferLibErrors::decode_error(&error) {
                Some(error) => VaultError::SafeTransferLib(error),
                None => VaultError::Inner(error),
            },
        }
    }
}

impl DecodeError<PendingTransactionError> for Vault::VaultErrors {
    fn decode_error(error: &PendingTransactionError) -> Option<Vault::VaultErrors> {
        match error {
            PendingTransactionError::TransportError(error) => {
                Vault::VaultErrors::decode_error(error)
            }
            _ => None,
        }
    }
}

impl From<PendingTransactionError> for VaultError<PendingTransactionError> {
    fn from(error: PendingTransactionError) -> Self {
        match Vault::VaultErrors::decode_error(&error) {
            Some(error) => VaultError::Vault(error),
            None => match SafeTransferLib::SafeTransferLibErrors::decode_error(&error) {
                Some(error) => VaultError::SafeTransferLib(error),
                None => VaultError::Inner(error),
            },
        }
    }
}
