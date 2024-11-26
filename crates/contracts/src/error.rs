use alloy::sol_types::SolInterface;

pub(crate) trait DecodeError<E>: SolInterface {
    fn decode_error(value: &E) -> Option<Self>;
}
