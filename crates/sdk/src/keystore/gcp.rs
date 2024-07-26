use thiserror::Error;

use crate::keypair::traits::Encryptable;

use super::traits::EncryptedKeystore;

#[derive(Debug, Error)]
pub enum GcpKeystoreError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

pub struct GcpEncrypedKeystore {
    // TODO
}

impl GcpEncrypedKeystore {
    // TODO
    pub fn new() -> Self {
        Self {}
    }
}

impl<Keypair: Encryptable> EncryptedKeystore<Keypair> for GcpEncrypedKeystore {
    type StorageError = GcpKeystoreError;

    fn store(&self, _keypair: &Keypair, _passphrase: &str) -> Result<(), Self::StorageError> {
        todo!()
    }

    fn retrieve(&self, _passphrase: &str) -> Result<Keypair, Self::StorageError> {
        todo!()
    }
}
