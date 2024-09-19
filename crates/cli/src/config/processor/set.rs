use std::path::PathBuf;

use crate::config::env::get_config_path;
use crate::config::models::{Chain, Keystore};
use crate::config::utils::{get_config, write_config};
use color_eyre::eyre;

pub fn process_set(
    chain_id: Option<u64>,
    rpc_url: Option<String>,
    local_keystore: Option<PathBuf>,
    aws_keystore: bool,
) -> eyre::Result<()> {
    let mut config = get_config()?;

    if let (Some(id), Some(url)) = (chain_id, &rpc_url) {
        config.chain = Some(Chain::Evm {
            id,
            rpc_url: url.clone(),
        });
    } else if chain_id.is_some() || rpc_url.is_some() {
        return Err(eyre::eyre!(
            "Both chain_id and rpc_url must be provided to set the chain"
        ));
    }

    if aws_keystore && local_keystore.is_some() {
        return Err(eyre::eyre!(
            "Cannot use both aws_keystore and local_keystore"
        ));
    }

    if aws_keystore {
        config.keystore = Keystore::Aws;
    } else if let Some(path) = local_keystore {
        config.keystore = Keystore::Local { path };
    }

    println!(
        "Updated config written to {:?}:\n{:#?}",
        get_config_path()?,
        config
    );

    write_config(config)?;

    Ok(())
}
