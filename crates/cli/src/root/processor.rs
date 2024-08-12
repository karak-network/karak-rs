use crate::{bls, keypair};

use super::Root;

pub async fn process(command: Root) -> color_eyre::Result<()> {
    match command {
        Root::Keypair(keypair) => keypair::processor::process(keypair).await,
        Root::BLS(bls) => bls::processor::process(bls).await,
    }
}
