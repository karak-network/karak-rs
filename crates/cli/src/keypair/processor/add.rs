use color_eyre::eyre;

use crate::config::{
    add_keystore_to_profile,
    models::{Keystore, Profile},
};
use crate::keypair::{processor::prompt, AwsKeypairConfig, KeypairArgs, LocalKeypairConfig};

pub async fn process_add(
    keypair_args: Option<KeypairArgs>,
    aws_config: Option<AwsKeypairConfig>,
    local_config: Option<LocalKeypairConfig>,
    profile: Profile,
    profile_name: &str,
    config_path: String,
) -> eyre::Result<()> {
    let keypair_args = prompt::prompt_keypair_args(keypair_args)?;
    let KeypairArgs {
        keystore,
        curve,
        keystore_name,
    } = keypair_args;

    // values will be set by prompt
    let keystore = keystore.unwrap();
    let curve = curve.unwrap();
    let keystore_name = keystore_name.unwrap();

    match keystore {
        Keystore::Aws { .. } => {
            let aws_config = prompt::prompt_aws_config(aws_config).await?;
            let AwsKeypairConfig {
                secret_name,
                profile: aws_profile,
            } = aws_config;

            // values will be set by prompt, unwrap safe
            let secret_name = secret_name.unwrap();
            let aws_profile = aws_profile.unwrap();

            add_keystore_to_profile(
                profile_name.to_string(),
                profile,
                curve,
                Keystore::Aws {
                    secret: secret_name,
                    profile: aws_profile,
                },
                &keystore_name,
                config_path,
            )?;
        }
        Keystore::Local { .. } => {
            let local_config = prompt::prompt_local_config(local_config).await?;
            let LocalKeypairConfig { keystore_path } = local_config;

            // values will be set by prompt, unwrap safe
            let keystore_path = keystore_path.unwrap();

            add_keystore_to_profile(
                profile_name.to_string(),
                profile,
                curve,
                Keystore::Local {
                    path: keystore_path,
                },
                &keystore_name,
                config_path,
            )?;
        }
    }
    Ok(())
}
