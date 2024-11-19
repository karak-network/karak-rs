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
pub enum Error<E: std::fmt::Debug> {
    #[error("ERC20Mintable error: {0}")]
    ERC20MintableError(ERC20Mintable::ERC20MintableErrors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<ERC20Mintable::ERC20MintableErrors> for Error<E> {
    fn from(err: ERC20Mintable::ERC20MintableErrors) -> Self {
        Error::ERC20MintableError(err)
    }
}

impl DecodeError<ERC20Mintable::ERC20MintableErrors> for ErrorPayload {
    fn decode_error(&self) -> Option<ERC20Mintable::ERC20MintableErrors> {
        self.as_decoded_error::<ERC20Mintable::ERC20MintableErrors>(true)
    }
}

impl From<ErrorPayload> for Error<ErrorPayload> {
    fn from(err: ErrorPayload) -> Self {
        match err.decode_error() {
            Some(err) => Error::ERC20MintableError(err),
            None => Error::Inner(err),
        }
    }
}

impl DecodeError<ERC20Mintable::ERC20MintableErrors> for TransportError {
    fn decode_error(&self) -> Option<ERC20Mintable::ERC20MintableErrors> {
        match self {
            alloy::transports::RpcError::ErrorResp(error) => error.decode_error(),
            _ => None,
        }
    }
}

impl From<TransportError> for Error<TransportError> {
    fn from(err: TransportError) -> Self {
        match err.decode_error() {
            Some(err) => Error::ERC20MintableError(err),
            None => Error::Inner(err),
        }
    }
}

impl DecodeError<ERC20Mintable::ERC20MintableErrors> for alloy::contract::Error {
    fn decode_error(&self) -> Option<ERC20Mintable::ERC20MintableErrors> {
        match self {
            alloy::contract::Error::TransportError(error) => error.decode_error(),
            _ => None,
        }
    }
}

impl From<alloy::contract::Error> for Error<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match value.decode_error() {
            Some(err) => Error::ERC20MintableError(err),
            None => Error::Inner(value),
        }
    }
}

impl DecodeError<ERC20Mintable::ERC20MintableErrors> for PendingTransactionError {
    fn decode_error(&self) -> Option<ERC20Mintable::ERC20MintableErrors> {
        match self {
            PendingTransactionError::TransportError(error) => error.decode_error(),
            _ => None,
        }
    }
}

impl From<PendingTransactionError> for Error<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match value.decode_error() {
            Some(err) => Error::ERC20MintableError(err),
            None => Error::Inner(value),
        }
    }
}
