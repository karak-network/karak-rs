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
    let profile_str = root.profile.unwrap_or_default();
    let config_path = root.config_path.unwrap_or_default();

    match root.command {
        Some(Command::Config(config)) => {
            config::processor::process(config, profile_str, config_path).await
        }

        Some(Command::Configure) => {
            config::processor::process_configure(profile_str, config_path).await
        }

        _ => {
            let profile = pre_run(profile_str.clone(), config_path.clone())?;
            let profile_name = profile_str.as_str();

            match root.command {
                Some(Command::Keypair(keypair)) => {
                    keypair::processor::process(keypair, profile, profile_name, config_path.clone())
                        .await
                }

                #[cfg(feature = "bls")]
                Some(Command::BLS(bls)) => bls::processor::process(bls, profile).await,

                Some(Command::Operator(operator)) => {
                    operator::processor::process(*operator, profile, profile_name, config_path)
                        .await
                }

                Some(Command::Config(_)) => unreachable!(),

                Some(Command::Configure) => unreachable!(),

                None => Ok(()),
            }
        }
    }
}
