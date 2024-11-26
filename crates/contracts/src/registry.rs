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
    RestakingRegistry,
    "abi/RestakingRegistry.json",
);

impl std::fmt::Debug for RestakingRegistry::RestakingRegistryErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestakingRegistry::RestakingRegistryErrors::AddressEmptyCode(_) => {
                write!(f, "AddressEmptyCode")
            }
            RestakingRegistry::RestakingRegistryErrors::AddressZero(_) => {
                write!(f, "AddressZero")
            }
            RestakingRegistry::RestakingRegistryErrors::AlreadyInitialized(_) => {
                write!(f, "AlreadyInitialized")
            }
            RestakingRegistry::RestakingRegistryErrors::ERC1967InvalidImplementation(_) => {
                write!(f, "ERC1967InvalidImplementation")
            }
            RestakingRegistry::RestakingRegistryErrors::ERC1967NonPayable(_) => {
                write!(f, "ERC1967NonPayable")
            }
            RestakingRegistry::RestakingRegistryErrors::FailedInnerCall(_) => {
                write!(f, "FailedInnerCall")
            }
            RestakingRegistry::RestakingRegistryErrors::InvalidFourthSegment(_) => {
                write!(f, "InvalidFourthSegment")
            }
            RestakingRegistry::RestakingRegistryErrors::InvalidInitialization(_) => {
                write!(f, "InvalidInitialization")
            }
            RestakingRegistry::RestakingRegistryErrors::InvalidUrlFormat(_) => {
                write!(f, "InvalidUrlFormat")
            }
            RestakingRegistry::RestakingRegistryErrors::NewOwnerIsZeroAddress(_) => {
                write!(f, "NewOwnerIsZeroAddress")
            }
            RestakingRegistry::RestakingRegistryErrors::NoHandoverRequest(_) => {
                write!(f, "NoHandoverRequest")
            }
            RestakingRegistry::RestakingRegistryErrors::NotInitializing(_) => {
                write!(f, "NotInitializing")
            }
            RestakingRegistry::RestakingRegistryErrors::NotKnsOwner(_) => {
                write!(f, "NotKnsOwner")
            }
            RestakingRegistry::RestakingRegistryErrors::UUPSUnauthorizedCallContext(_) => {
                write!(f, "UUPSUnauthorizedCallContext")
            }
            RestakingRegistry::RestakingRegistryErrors::UUPSUnsupportedProxiableUUID(_) => {
                write!(f, "UUPSUnsupportedProxiableUUID")
            }
            RestakingRegistry::RestakingRegistryErrors::Unauthorized(_) => {
                write!(f, "Unauthorized")
            }
            RestakingRegistry::RestakingRegistryErrors::UnexpectedAmtOfDots(_) => {
                write!(f, "UnexpectedAmtOfDots")
            }
        }
    }
}

impl std::fmt::Display for RestakingRegistry::RestakingRegistryErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestakingRegistry::RestakingRegistryErrors::AddressEmptyCode(_) => {
                write!(f, "Address empty code")
            }
            RestakingRegistry::RestakingRegistryErrors::AddressZero(_) => {
                write!(f, "Address zero")
            }
            RestakingRegistry::RestakingRegistryErrors::AlreadyInitialized(_) => {
                write!(f, "Already initialized")
            }
            RestakingRegistry::RestakingRegistryErrors::ERC1967InvalidImplementation(_) => {
                write!(f, "ERC1967 invalid implementation")
            }
            RestakingRegistry::RestakingRegistryErrors::ERC1967NonPayable(_) => {
                write!(f, "ERC1967 non-payable")
            }
            RestakingRegistry::RestakingRegistryErrors::FailedInnerCall(_) => {
                write!(f, "Failed inner call")
            }
            RestakingRegistry::RestakingRegistryErrors::InvalidFourthSegment(_) => {
                write!(f, "Invalid fourth segment")
            }
            RestakingRegistry::RestakingRegistryErrors::InvalidInitialization(_) => {
                write!(f, "Invalid initialization")
            }
            RestakingRegistry::RestakingRegistryErrors::InvalidUrlFormat(_) => {
                write!(f, "Invalid URL format")
            }
            RestakingRegistry::RestakingRegistryErrors::NewOwnerIsZeroAddress(_) => {
                write!(f, "New owner is zero address")
            }
            RestakingRegistry::RestakingRegistryErrors::NoHandoverRequest(_) => {
                write!(f, "No handover request")
            }
            RestakingRegistry::RestakingRegistryErrors::NotInitializing(_) => {
                write!(f, "Not initializing")
            }
            RestakingRegistry::RestakingRegistryErrors::NotKnsOwner(_) => {
                write!(f, "Not KNS owner")
            }
            RestakingRegistry::RestakingRegistryErrors::UUPSUnauthorizedCallContext(_) => {
                write!(f, "UUPS unauthorized call context")
            }
            RestakingRegistry::RestakingRegistryErrors::UUPSUnsupportedProxiableUUID(_) => {
                write!(f, "UUPS unsupported proxiable UUID")
            }
            RestakingRegistry::RestakingRegistryErrors::Unauthorized(_) => {
                write!(f, "Unauthorized")
            }
            RestakingRegistry::RestakingRegistryErrors::UnexpectedAmtOfDots(_) => {
                write!(f, "Unexpected amount of dots")
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RestakingRegistryError<E: std::fmt::Debug> {
    #[error("RestakingRegistry error: {0}")]
    Revert(RestakingRegistry::RestakingRegistryErrors),
    #[error(transparent)]
    Inner(E),
}

impl<E: std::fmt::Debug> From<RestakingRegistry::RestakingRegistryErrors>
    for RestakingRegistryError<E>
{
    fn from(error: RestakingRegistry::RestakingRegistryErrors) -> Self {
        RestakingRegistryError::Revert(error)
    }
}

impl From<ErrorPayload> for RestakingRegistryError<ErrorPayload> {
    fn from(value: ErrorPayload) -> Self {
        match value.as_decoded_error::<RestakingRegistry::RestakingRegistryErrors>(true) {
            Some(error) => RestakingRegistryError::Revert(error),
            None => RestakingRegistryError::Inner(value),
        }
    }
}

impl DecodeError<RestakingRegistry::RestakingRegistryErrors> for TransportError {
    fn decode_error(&self) -> Option<RestakingRegistry::RestakingRegistryErrors> {
        match self {
            RpcError::ErrorResp(error) => {
                error.as_decoded_error::<RestakingRegistry::RestakingRegistryErrors>(true)
            }
            _ => None,
        }
    }
}

impl From<TransportError> for RestakingRegistryError<TransportError> {
    fn from(value: TransportError) -> Self {
        match value.decode_error() {
            Some(error) => RestakingRegistryError::Revert(error),
            _ => RestakingRegistryError::Inner(value),
        }
    }
}

impl DecodeError<RestakingRegistry::RestakingRegistryErrors> for alloy::contract::Error {
    fn decode_error(&self) -> Option<RestakingRegistry::RestakingRegistryErrors> {
        match self {
            alloy::contract::Error::TransportError(transport_error) => {
                transport_error.decode_error()
            }
            _ => None,
        }
    }
}

impl From<alloy::contract::Error> for RestakingRegistryError<alloy::contract::Error> {
    fn from(value: alloy::contract::Error) -> Self {
        match value.decode_error() {
            Some(error) => RestakingRegistryError::Revert(error),
            _ => RestakingRegistryError::Inner(value),
        }
    }
}

impl DecodeError<RestakingRegistry::RestakingRegistryErrors> for PendingTransactionError {
    fn decode_error(&self) -> Option<RestakingRegistry::RestakingRegistryErrors> {
        match self {
            PendingTransactionError::TransportError(transport_error) => {
                transport_error.decode_error()
            }
            _ => None,
        }
    }
}

impl From<PendingTransactionError> for RestakingRegistryError<PendingTransactionError> {
    fn from(value: PendingTransactionError) -> Self {
        match value.decode_error() {
            Some(error) => RestakingRegistryError::Revert(error),
            _ => RestakingRegistryError::Inner(value),
        }
    }
}
