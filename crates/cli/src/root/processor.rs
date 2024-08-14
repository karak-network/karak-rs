use color_eyre::eyre;

use crate::{
    bls, keypair,
    operator::{self},
};

use super::Root;

pub async fn process(command: Root) -> eyre::Result<()> {
    match command {
        Root::Keypair(keypair) => keypair::processor::process(keypair).await,
        Root::BLS(bls) => bls::processor::process(bls).await,
        Root::Operator(operator) => operator::processor::process(operator).await,
    }
}
