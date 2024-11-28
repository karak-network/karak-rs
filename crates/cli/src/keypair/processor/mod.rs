pub mod add;
pub mod generate;
pub mod list;
pub mod prompt;
pub mod pubkey;

use add::process_add;
use color_eyre::eyre;
use generate::process_generate;
use list::process_list;
use pubkey::process_pubkey;

use super::Keypair;
use crate::config::models::Profile;

pub async fn process(
    command: Keypair,
    profile: Profile,
    profile_name: &str,
    config_path: String,
) -> eyre::Result<()> {
    match command {
        Keypair::Generate {
            keypair: keypair_args,
            passphrase,
        } => process_generate(keypair_args, passphrase, profile, profile_name, config_path).await,
        Keypair::Pubkey {
            keystore_name,
            passphrase,
            curve,
        } => process_pubkey(profile, keystore_name, passphrase, curve).await,
        Keypair::List { curve } => process_list(profile, curve).await,
        Keypair::Add {
            keypair_args,
            aws_config,
            local_config,
        } => {
            process_add(
                keypair_args,
                aws_config,
                local_config,
                profile,
                profile_name,
                config_path,
            )
            .await
        }
    }
}
