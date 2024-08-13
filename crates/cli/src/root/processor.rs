use color_eyre::eyre;

use crate::{bls, keypair};

use super::Root;

pub async fn process(command: Root) -> eyre::Result<()> {
    match command {
        Root::Keypair(keypair) => keypair::processor::process(keypair).await,
        Root::BLS(bls) => bls::processor::process(bls).await,
        Root::Operator => todo!(),
    }
}
