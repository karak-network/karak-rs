use aws_sdk_secretsmanager::primitives::Blob;
use thiserror::Error;

use crate::keypair::traits::Encryptable;

use super::traits::AsyncEncryptedKeystore;

#[derive(Debug, Error)]
pub enum AwsKeystoreError<Keypair: Encryptable + Send + Sync> {
    #[error("Encryption error: {0}")]
    EncryptionError(Keypair::EncryptionError),
    #[error("AWS Secrets Manager Put error: {0}")]
    AwsSecretsManagerPutError(
        #[from]
        aws_sdk_secretsmanager::error::SdkError<
            aws_sdk_secretsmanager::operation::put_secret_value::PutSecretValueError,
        >,
    ),
    #[error("AWS Secrets Manager Get error: {0}")]
    AwsSecretsManagerGetError(
        #[from]
        aws_sdk_secretsmanager::error::SdkError<
            aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValueError,
        >,
    ),
    #[error("AWS Secrets Manager Blob empty")]
    AwsSecretBlobEmpty,
}

pub struct AwsEncryptedKeystore {
    client: aws_sdk_secretsmanager::Client,
}

impl AwsEncryptedKeystore {
    pub fn new(config: &aws_config::SdkConfig) -> Self {
        Self {
            client: aws_sdk_secretsmanager::Client::new(config),
        }
    }
}

pub struct AwsKeystoreParams {
    secret_name: String,
}

impl<Keypair: Encryptable + Send + Sync + std::fmt::Debug>
    AsyncEncryptedKeystore<Keypair, AwsKeystoreParams> for AwsEncryptedKeystore
{
    type StorageError = AwsKeystoreError<Keypair>;

    async fn store(
        &self,
        keypair: &Keypair,
        passphrase: &str,
        params: &AwsKeystoreParams,
    ) -> Result<(), Self::StorageError> {
        let encrypted_keypair = keypair
            .encrypt(passphrase)
            .map_err(|err| AwsKeystoreError::EncryptionError(err))?;

        // TODO: Maybe handle some of the possible error scenarios here?
        // TODO: Also make sure this is idempotent and can work even if the secret does not exist yet
        self.client
            .put_secret_value()
            .secret_id(&params.secret_name)
            .set_secret_binary(Some(Blob::new(encrypted_keypair)))
            .send()
            .await?;

        Ok(())
    }

    async fn retrieve(
        &self,
        passphrase: &str,
        params: &AwsKeystoreParams,
    ) -> Result<Keypair, Self::StorageError> {
        let resp = self
            .client
            .get_secret_value()
            .secret_id(&params.secret_name)
            .send()
            .await?;

        let encrypted_keypair = match resp.secret_binary() {
            Some(blob) => blob.as_ref(),
            None => return Err(AwsKeystoreError::AwsSecretBlobEmpty),
        };

        Keypair::decrypt(encrypted_keypair, passphrase)
            .map_err(|err| AwsKeystoreError::EncryptionError(err))
    }
}
