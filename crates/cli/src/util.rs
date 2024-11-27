use std::ffi::CStr;

use alloy::{primitives::Bytes, sol_types::SolValue};
use aws_types::os_shim_internal::{Env, Fs};
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

#[macro_export]
macro_rules! clap_enum_variants {
    ($e: ty) => {{
        use clap::builder::TypedValueParser;
        use strum::VariantNames;
        clap::builder::PossibleValuesParser::new(<$e>::VARIANTS).map(|s| s.parse::<$e>().unwrap())
    }};
}

// TODO: Add options for custom files
pub async fn get_aws_profiles() -> eyre::Result<Vec<String>> {
    Ok(aws_config::profile::load(
        &Fs::real(),
        &Env::real(),
        &aws_runtime::env_config::file::EnvConfigFiles::default(),
        None,
    )
    .await?
    .profiles()
    .map(String::from)
    .collect::<Vec<String>>())
}
