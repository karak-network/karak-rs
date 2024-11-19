use alloy::sol_types::SolInterface;

pub trait DecodeError<E: SolInterface> {
    fn decode_error(&self) -> Option<E>;
}
