use alloy::{
    providers::PendingTransactionError, rpc::json_rpc::ErrorPayload, sol,
    transports::TransportError,
};

use crate::{error::DecodeError, impl_decode_error};

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

impl_decode_error!(ERC20::ERC20Errors);

#[derive(Debug, thiserror::Error)]
pub enum ERC20Error<E: std::fmt::Debug> {
    #[error("ERC20 error: {0}")]
    ERC20(ERC20::ERC20Errors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<ERC20::ERC20Errors> for ERC20Error<E> {
    fn from(error: ERC20::ERC20Errors) -> Self {
        ERC20Error::ERC20(error)
    }
}

impl From<ErrorPayload> for ERC20Error<ErrorPayload> {
    fn from(value: ErrorPayload) -> Self {
        match value.as_decoded_error::<ERC20::ERC20Errors>(true) {
            Some(error) => ERC20Error::ERC20(error),
            None => ERC20Error::Inner(value),
        }
    }
}

impl From<TransportError> for ERC20Error<TransportError> {
    fn from(value: TransportError) -> Self {
        match ERC20::ERC20Errors::decode_error(&value) {
            Some(error) => ERC20Error::ERC20(error),
            _ => ERC20Error::Inner(value),
        }
    }
}

impl From<alloy::contract::Error> for ERC20Error<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match ERC20::ERC20Errors::decode_error(&value) {
            Some(error) => ERC20Error::ERC20(error),
            _ => ERC20Error::Inner(value),
        }
    }
}

impl From<PendingTransactionError> for ERC20Error<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match ERC20::ERC20Errors::decode_error(&value) {
            Some(error) => ERC20Error::ERC20(error),
            _ => ERC20Error::Inner(value),
        }
    }
}
