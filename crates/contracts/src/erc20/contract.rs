use alloy::{
    providers::PendingTransactionError,
    rpc::json_rpc::ErrorPayload,
    sol,
    transports::{RpcError, TransportError},
};

use crate::error::DecodeError;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "abi/ERC20.json",
);

impl std::fmt::Debug for ERC20::ERC20Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERC20::ERC20Errors::ERC20InsufficientAllowance(_) => {
                write!(f, "ERC20InsufficientAllowance")
            }
            ERC20::ERC20Errors::ERC20InsufficientBalance(_) => {
                write!(f, "ERC20InsufficientBalance")
            }
            ERC20::ERC20Errors::ERC20InvalidApprover(_) => {
                write!(f, "ERC20InvalidApprover")
            }
            ERC20::ERC20Errors::ERC20InvalidReceiver(_) => {
                write!(f, "ERC20InvalidReceiver")
            }
            ERC20::ERC20Errors::ERC20InvalidSender(_) => {
                write!(f, "ERC20InvalidSender")
            }
            ERC20::ERC20Errors::ERC20InvalidSpender(_) => {
                write!(f, "ERC20InvalidSpender")
            }
        }
    }
}

impl std::fmt::Display for ERC20::ERC20Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERC20::ERC20Errors::ERC20InsufficientAllowance(_) => {
                write!(f, "ERC20 insufficient allowance")
            }
            ERC20::ERC20Errors::ERC20InsufficientBalance(_) => {
                write!(f, "ERC20 insufficient balance")
            }
            ERC20::ERC20Errors::ERC20InvalidApprover(_) => {
                write!(f, "ERC20 invalid approver")
            }
            ERC20::ERC20Errors::ERC20InvalidReceiver(_) => {
                write!(f, "ERC20 invalid receiver")
            }
            ERC20::ERC20Errors::ERC20InvalidSender(_) => {
                write!(f, "ERC20 invalid sender")
            }
            ERC20::ERC20Errors::ERC20InvalidSpender(_) => {
                write!(f, "ERC20 invalid spender")
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ERC20Error<E: std::fmt::Debug> {
    #[error("ERC20 error: {0}")]
    Revert(ERC20::ERC20Errors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<ERC20::ERC20Errors> for ERC20Error<E> {
    fn from(error: ERC20::ERC20Errors) -> Self {
        ERC20Error::Revert(error)
    }
}

impl From<ErrorPayload> for ERC20Error<ErrorPayload> {
    fn from(value: ErrorPayload) -> Self {
        match value.as_decoded_error::<ERC20::ERC20Errors>(true) {
            Some(error) => ERC20Error::Revert(error),
            None => ERC20Error::Inner(value),
        }
    }
}

impl DecodeError<ERC20::ERC20Errors> for TransportError {
    fn decode_error(&self) -> Option<ERC20::ERC20Errors> {
        match self {
            RpcError::ErrorResp(error) => error.as_decoded_error::<ERC20::ERC20Errors>(true),
            _ => None,
        }
    }
}

impl From<TransportError> for ERC20Error<TransportError> {
    fn from(value: TransportError) -> Self {
        match value.decode_error() {
            Some(error) => ERC20Error::Revert(error),
            _ => ERC20Error::Inner(value),
        }
    }
}

impl DecodeError<ERC20::ERC20Errors> for alloy::contract::Error {
    fn decode_error(&self) -> Option<ERC20::ERC20Errors> {
        match self {
            alloy::contract::Error::TransportError(transport_error) => {
                transport_error.decode_error()
            }
            _ => None,
        }
    }
}

impl From<alloy::contract::Error> for ERC20Error<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match value.decode_error() {
            Some(error) => ERC20Error::Revert(error),
            _ => ERC20Error::Inner(value),
        }
    }
}

impl DecodeError<ERC20::ERC20Errors> for PendingTransactionError {
    fn decode_error(&self) -> Option<ERC20::ERC20Errors> {
        match self {
            PendingTransactionError::TransportError(transport_error) => {
                transport_error.decode_error()
            }
            _ => None,
        }
    }
}

impl From<PendingTransactionError> for ERC20Error<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match value.decode_error() {
            Some(error) => ERC20Error::Revert(error),
            _ => ERC20Error::Inner(value),
        }
    }
}
