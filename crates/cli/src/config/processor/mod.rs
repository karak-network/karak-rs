pub mod get;
pub mod init;

use color_eyre::eyre;
use get::process_get;
use init::process_init;

use super::Config;

pub async fn process(command: Config) -> eyre::Result<()> {
    match command {
        Config::Init { path, overwrite } => process_init(path, overwrite).await,
        Config::Get => process_get(),
    }
}
