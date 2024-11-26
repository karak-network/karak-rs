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

impl DecodeError<ErrorPayload> for SafeTransferLib::SafeTransferLibErrors {
    fn decode_error(error: &ErrorPayload) -> Option<SafeTransferLib::SafeTransferLibErrors> {
        error.as_decoded_error::<SafeTransferLib::SafeTransferLibErrors>(true)
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

impl DecodeError<TransportError> for SafeTransferLib::SafeTransferLibErrors {
    fn decode_error(error: &TransportError) -> Option<SafeTransferLib::SafeTransferLibErrors> {
        match error {
            RpcError::ErrorResp(error) => {
                error.as_decoded_error::<SafeTransferLib::SafeTransferLibErrors>(true)
            }
            _ => None,
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

impl DecodeError<alloy::contract::Error> for SafeTransferLib::SafeTransferLibErrors {
    fn decode_error(
        error: &alloy::contract::Error,
    ) -> Option<SafeTransferLib::SafeTransferLibErrors> {
        match error {
            alloy::contract::Error::TransportError(transport_error) => {
                SafeTransferLib::SafeTransferLibErrors::decode_error(transport_error)
            }
            _ => None,
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

impl DecodeError<PendingTransactionError> for SafeTransferLib::SafeTransferLibErrors {
    fn decode_error(
        error: &PendingTransactionError,
    ) -> Option<SafeTransferLib::SafeTransferLibErrors> {
        match error {
            PendingTransactionError::TransportError(transport_error) => {
                SafeTransferLib::SafeTransferLibErrors::decode_error(transport_error)
            }
            _ => None,
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
