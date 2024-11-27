use alloy::sol_types::SolInterface;

pub(crate) trait DecodeError<E>: SolInterface {
    fn decode_error(value: &E) -> Option<Self>;
}

#[macro_export]
macro_rules! impl_decode_error {
    ($name:path) => {
        impl $crate::error::DecodeError<::alloy::rpc::json_rpc::ErrorPayload> for $name {
            fn decode_error(error: &::alloy::rpc::json_rpc::ErrorPayload) -> Option<$name> {
                error.as_decoded_error::<$name>(true)
            }
        }

        impl $crate::error::DecodeError<::alloy::transports::TransportError> for $name {
            fn decode_error(error: &::alloy::transports::TransportError) -> Option<$name> {
                match error {
                    ::alloy::transports::RpcError::ErrorResp(error) => {
                        error.as_decoded_error::<$name>(true)
                    }
                    _ => None,
                }
            }
        }

        impl $crate::error::DecodeError<::alloy::contract::Error> for $name {
            fn decode_error(error: &::alloy::contract::Error) -> Option<$name> {
                match error {
                    ::alloy::contract::Error::TransportError(transport_error) => {
                        <$name as $crate::error::DecodeError<::alloy::transports::TransportError>>::decode_error(
                            transport_error,
                        )
                    }
                    _ => None,
                }
            }
        }

        impl $crate::error::DecodeError<::alloy::providers::PendingTransactionError> for $name {
            fn decode_error(error: &::alloy::providers::PendingTransactionError) -> Option<$name> {
                match error {
                    ::alloy::providers::PendingTransactionError::TransportError(
                        transport_error,
                    ) => <$name as $crate::error::DecodeError<
                        ::alloy::transports::TransportError,
                    >>::decode_error(transport_error),
                    _ => None,
                }
            }
        }
    };
}
