use alloy::{
    providers::PendingTransactionError, rpc::json_rpc::ErrorPayload, sol,
    transports::TransportError,
};

use crate::error::DecodeError;

sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Mintable,
    "abi/ERC20Mintable.json",
);

impl std::fmt::Debug for ERC20Mintable::ERC20MintableErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERC20Mintable::ERC20MintableErrors::AddressZero(_) => {
                write!(f, "AddressZero")
            }
            ERC20Mintable::ERC20MintableErrors::AllowanceOverflow(_) => {
                write!(f, "AllowanceOverflow")
            }
            ERC20Mintable::ERC20MintableErrors::AllowanceUnderflow(_) => {
                write!(f, "AllowanceUnderflow")
            }
            ERC20Mintable::ERC20MintableErrors::InsufficientAllowance(_) => {
                write!(f, "InsufficientAllowance")
            }
            ERC20Mintable::ERC20MintableErrors::InsufficientBalance(_) => {
                write!(f, "InsufficientBalance")
            }
            ERC20Mintable::ERC20MintableErrors::InvalidPermit(_) => {
                write!(f, "InvalidPermit")
            }
            ERC20Mintable::ERC20MintableErrors::PermitExpired(_) => {
                write!(f, "PermitExpired")
            }
            ERC20Mintable::ERC20MintableErrors::TotalSupplyOverflow(_) => {
                write!(f, "TotalSupplyOverflow")
            }
        }
    }
}

impl std::fmt::Display for ERC20Mintable::ERC20MintableErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERC20Mintable::ERC20MintableErrors::AddressZero(_) => {
                write!(f, "Address is zero")
            }
            ERC20Mintable::ERC20MintableErrors::AllowanceOverflow(_) => {
                write!(f, "Allowance overflow")
            }
            ERC20Mintable::ERC20MintableErrors::AllowanceUnderflow(_) => {
                write!(f, "Allowance underflow")
            }
            ERC20Mintable::ERC20MintableErrors::InsufficientAllowance(_) => {
                write!(f, "Insufficient allowance")
            }
            ERC20Mintable::ERC20MintableErrors::InsufficientBalance(_) => {
                write!(f, "Insufficient balance")
            }
            ERC20Mintable::ERC20MintableErrors::InvalidPermit(_) => {
                write!(f, "Invalid permit")
            }
            ERC20Mintable::ERC20MintableErrors::PermitExpired(_) => {
                write!(f, "Permit expired")
            }
            ERC20Mintable::ERC20MintableErrors::TotalSupplyOverflow(_) => {
                write!(f, "Total supply overflow")
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ERC20MintableError<E: std::fmt::Debug> {
    #[error("ERC20Mintable error: {0}")]
    Revert(ERC20Mintable::ERC20MintableErrors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<ERC20Mintable::ERC20MintableErrors> for ERC20MintableError<E> {
    fn from(err: ERC20Mintable::ERC20MintableErrors) -> Self {
        ERC20MintableError::Revert(err)
    }
}

impl DecodeError<ErrorPayload> for ERC20Mintable::ERC20MintableErrors {
    fn decode_error(error: &ErrorPayload) -> Option<ERC20Mintable::ERC20MintableErrors> {
        error.as_decoded_error::<ERC20Mintable::ERC20MintableErrors>(true)
    }
}

impl From<ErrorPayload> for ERC20MintableError<ErrorPayload> {
    fn from(error: ErrorPayload) -> Self {
        match ERC20Mintable::ERC20MintableErrors::decode_error(&error) {
            Some(err) => ERC20MintableError::Revert(err),
            None => ERC20MintableError::Inner(error),
        }
    }
}

impl DecodeError<TransportError> for ERC20Mintable::ERC20MintableErrors {
    fn decode_error(error: &TransportError) -> Option<ERC20Mintable::ERC20MintableErrors> {
        match error {
            alloy::transports::RpcError::ErrorResp(error) => {
                ERC20Mintable::ERC20MintableErrors::decode_error(error)
            }
            _ => None,
        }
    }
}

impl From<TransportError> for ERC20MintableError<TransportError> {
    fn from(error: TransportError) -> Self {
        match ERC20Mintable::ERC20MintableErrors::decode_error(&error) {
            Some(err) => ERC20MintableError::Revert(err),
            None => ERC20MintableError::Inner(error),
        }
    }
}

impl DecodeError<alloy::contract::Error> for ERC20Mintable::ERC20MintableErrors {
    fn decode_error(error: &alloy::contract::Error) -> Option<ERC20Mintable::ERC20MintableErrors> {
        match error {
            alloy::contract::Error::TransportError(error) => {
                ERC20Mintable::ERC20MintableErrors::decode_error(error)
            }
            _ => None,
        }
    }
}

impl From<alloy::contract::Error> for ERC20MintableError<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match ERC20Mintable::ERC20MintableErrors::decode_error(&value) {
            Some(err) => ERC20MintableError::Revert(err),
            None => ERC20MintableError::Inner(value),
        }
    }
}

impl DecodeError<PendingTransactionError> for ERC20Mintable::ERC20MintableErrors {
    fn decode_error(error: &PendingTransactionError) -> Option<ERC20Mintable::ERC20MintableErrors> {
        match error {
            PendingTransactionError::TransportError(error) => {
                ERC20Mintable::ERC20MintableErrors::decode_error(error)
            }
            _ => None,
        }
    }
}

impl From<PendingTransactionError> for ERC20MintableError<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match ERC20Mintable::ERC20MintableErrors::decode_error(&value) {
            Some(err) => ERC20MintableError::Revert(err),
            None => ERC20MintableError::Inner(value),
        }
    }
}
