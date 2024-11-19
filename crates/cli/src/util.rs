use std::ffi::CStr;

use alloy::{primitives::Bytes, sol_types::SolValue};
use eyre::Result;

pub fn parse_token_str(input: &Bytes) -> Result<String> {
    // Most token data (name, symbol) can be ABI decoded into a string
    // however, some tokens (eg, MKR) are not ABI encoded, so we try to parse them as a C string
    // if that fails, we return an error
    if let Ok(token_str) = String::abi_decode(input, true) {
        Ok(token_str)
    } else if let Ok(token_str) = CStr::from_bytes_until_nul(input.as_ref()) {
        Ok(token_str.to_str()?.to_string())
    } else {
        Err(eyre::eyre!("Failed to parse token string"))
    }
}
