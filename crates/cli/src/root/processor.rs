use color_eyre::eyre;

use crate::{
    config, keypair,
    operator::{self},
};

#[cfg(feature = "bls")]
use crate::bls;

use super::Root;

pub async fn process(command: Root) -> eyre::Result<()> {
    match command {
        Root::Keypair(keypair) => keypair::processor::process(keypair).await,
        #[cfg(feature = "bls")]
        Root::BLS(bls) => bls::processor::process(bls).await,
        Root::Operator(operator) => operator::processor::process(operator).await,
        Root::Config(config) => config::processor::process(config).await,
    }
}
