pub mod generate;
pub mod prompt;
pub mod pubkey;

use color_eyre::eyre;
use generate::process_generate;
use pubkey::process_pubkey;

use super::Keypair;
use crate::config::models::Profile;

pub async fn process(command: Keypair, profile: Profile) -> eyre::Result<()> {
    match command {
        Keypair::Generate {
            keypair: keypair_args,
            curve,
        } => process_generate(keypair_args, curve, profile.key_generation_folder).await,
        Keypair::Pubkey {
            keypair: keypair_args,
            keypair_location: keypair_location_args,
            curve,
        } => process_pubkey(keypair_args, keypair_location_args, curve).await,
    }
}
