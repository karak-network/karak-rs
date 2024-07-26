use thiserror::Error;

use crate::keypair::traits::Encryptable;

use super::traits::EncryptedKeystore;

#[derive(Debug, Error)]
pub enum AwsKeystoreError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

pub struct AwsEncryptedKeystore {
    // TODO
}

impl AwsEncryptedKeystore {
    // TODO
    pub fn new() -> Self {
        Self {}
    }
}

impl<Keypair: Encryptable> EncryptedKeystore<Keypair> for AwsEncryptedKeystore {
    type StorageError = AwsKeystoreError;

    fn store(&self, _keypair: &Keypair, _passphrase: &str) -> Result<(), Self::StorageError> {
        todo!()
    }

    fn retrieve(&self, _passphrase: &str) -> Result<Keypair, Self::StorageError> {
        todo!()
    }
}
