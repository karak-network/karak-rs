pub mod get;
pub mod init;
pub mod set;

use color_eyre::eyre;
use get::process_get;
use init::process_init;
use set::process_set;

use super::Config;

pub async fn process(command: Config) -> eyre::Result<()> {
    match command {
        Config::Init { path, overwrite } => process_init(path, overwrite),
        Config::Get => process_get(),
        Config::Set {
            chain_id,
            rpc_url,
            local_keystore,
            aws_keystore,
        } => process_set(chain_id, rpc_url, local_keystore, aws_keystore),
    }
}
