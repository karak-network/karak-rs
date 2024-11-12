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
        Some(Command::Config(config)) => {
            config::processor::process(config, root.profile.unwrap(), root.config_path.unwrap())
                .await
        }

        Some(Command::Configure) => {
            config::processor::process_configure(root.profile.unwrap(), root.config_path.unwrap())
                .await
        }

        _ => {
            let profile = pre_run(root.profile.unwrap(), root.config_path.unwrap())?;

            match root.command {
                Some(Command::Keypair(keypair)) => {
                    keypair::processor::process(keypair, profile).await
                }

                #[cfg(feature = "bls")]
                Some(Command::BLS(bls)) => bls::processor::process(bls, profile).await,

                Some(Command::Operator(operator)) => {
                    operator::processor::process(operator, profile).await
                }

                Some(Command::Config(_)) => unreachable!(),

                Some(Command::Configure) => unreachable!(),

                None => Ok(()),
            }
        }
    }
}
