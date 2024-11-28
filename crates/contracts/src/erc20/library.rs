use alloy::{
    providers::PendingTransactionError, rpc::json_rpc::ErrorPayload, sol,
    transports::TransportError,
};

use crate::{error::DecodeError, impl_decode_error};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SafeTransferLib,
    "abi/SafeTransferLib.json",
);

impl std::fmt::Debug for SafeTransferLib::SafeTransferLibErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SafeTransferLib::SafeTransferLibErrors::ApproveFailed(_) => {
                write!(f, "ApproveFailed")
            }
            SafeTransferLib::SafeTransferLibErrors::ETHTransferFailed(_) => {
                write!(f, "ETHTransferFailed")
            }
            SafeTransferLib::SafeTransferLibErrors::Permit2AmountOverflow(_) => {
                write!(f, "Permit2AmountOverflow")
            }
            SafeTransferLib::SafeTransferLibErrors::Permit2Failed(_) => {
                write!(f, "Permit2Failed")
            }
            SafeTransferLib::SafeTransferLibErrors::TransferFailed(_) => {
                write!(f, "TransferFailed")
            }
            SafeTransferLib::SafeTransferLibErrors::TransferFromFailed(_) => {
                write!(f, "TransferFromFailed")
            }
        }
    }
}

impl std::fmt::Display for SafeTransferLib::SafeTransferLibErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SafeTransferLib::SafeTransferLibErrors::ApproveFailed(_) => {
                write!(f, "Approve failed")
            }
            SafeTransferLib::SafeTransferLibErrors::ETHTransferFailed(_) => {
                write!(f, "ETH transfer failed")
            }
            SafeTransferLib::SafeTransferLibErrors::Permit2AmountOverflow(_) => {
                write!(f, "Permit2 amount overflow")
            }
            SafeTransferLib::SafeTransferLibErrors::Permit2Failed(_) => {
                write!(f, "Permit2 failed")
            }
            SafeTransferLib::SafeTransferLibErrors::TransferFailed(_) => {
                write!(f, "Transfer failed")
            }
            SafeTransferLib::SafeTransferLibErrors::TransferFromFailed(_) => {
                write!(
                    f,
                    "Transfer failed. Check that you have enough token balance for the transfer"
                )
            }
        }
    }
}

impl_decode_error!(SafeTransferLib::SafeTransferLibErrors);

#[derive(Debug, thiserror::Error)]
pub enum SafeTransferLibError<E: std::fmt::Debug> {
    #[error("SafeTransferLib error: {0}")]
    SafeTransferLib(SafeTransferLib::SafeTransferLibErrors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<SafeTransferLib::SafeTransferLibErrors> for SafeTransferLibError<E> {
    fn from(error: SafeTransferLib::SafeTransferLibErrors) -> Self {
        SafeTransferLibError::SafeTransferLib(error)
    }
}

impl From<ErrorPayload> for SafeTransferLibError<ErrorPayload> {
    fn from(value: ErrorPayload) -> Self {
        match SafeTransferLib::SafeTransferLibErrors::decode_error(&value) {
            Some(error) => SafeTransferLibError::SafeTransferLib(error),
            None => SafeTransferLibError::Inner(value),
        }
    }
}

impl From<TransportError> for SafeTransferLibError<TransportError> {
    fn from(value: TransportError) -> Self {
        match SafeTransferLib::SafeTransferLibErrors::decode_error(&value) {
            Some(error) => SafeTransferLibError::SafeTransferLib(error),
            _ => SafeTransferLibError::Inner(value),
        }
    }
}

impl From<alloy::contract::Error> for SafeTransferLibError<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match SafeTransferLib::SafeTransferLibErrors::decode_error(&value) {
            Some(error) => SafeTransferLibError::SafeTransferLib(error),
            _ => SafeTransferLibError::Inner(value),
        }
    }
}

impl From<PendingTransactionError> for SafeTransferLibError<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match SafeTransferLib::SafeTransferLibErrors::decode_error(&value) {
            Some(error) => SafeTransferLibError::SafeTransferLib(error),
            _ => SafeTransferLibError::Inner(value),
        }
    }
}
