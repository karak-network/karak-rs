pub mod init;

use color_eyre::eyre;
use init::process_init;

use super::Config;

pub async fn process(command: Config) -> eyre::Result<()> {
    match command {
        Config::Init { path, overwrite } => process_init(path, overwrite).await,
    }
}
