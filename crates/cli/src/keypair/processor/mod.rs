pub mod generate;
pub mod pubkey;

use color_eyre::eyre;
use generate::process_generate;
use pubkey::process_pubkey;

use super::Keypair;

pub async fn process(command: Keypair) -> eyre::Result<()> {
    match command {
        Keypair::Generate {
            keypair: keypair_args,
            curve,
        } => process_generate(keypair_args, curve).await,
        Keypair::Pubkey {
            keypair: keypair_args,
            keypair_location: keypair_location_args,
            curve,
        } => process_pubkey(keypair_args, keypair_location_args, curve).await,
    }
}
