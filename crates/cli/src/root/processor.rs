use color_eyre::eyre;

use crate::{
    config::{self, processor::pre_run},
    keypair,
    operator::{self},
};

#[cfg(feature = "bls")]
use crate::bls;

use super::{Command, Root};

pub async fn process(root: Root) -> eyre::Result<()> {
    match root.command {
        Command::Config(config) => {
            config::processor::process(config, root.profile.unwrap(), root.config_path.unwrap())
                .await
        }

        _ => {
            let profile = pre_run(root.profile.unwrap(), root.config_path.unwrap())?;

            match root.command {
                Command::Keypair(keypair) => keypair::processor::process(keypair, profile).await,

                #[cfg(feature = "bls")]
                Command::BLS(bls) => bls::processor::process(bls, profile).await,

                Command::Operator(operator) => {
                    operator::processor::process(operator, profile).await
                }

                Command::Config(_) => unreachable!(), // We've already handled this case
            }
        }
    }
}
