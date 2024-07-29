use async_trait::async_trait;

use crate::keypair::traits::Encryptable;
use std::error::Error;

pub trait EncryptedKeystore<Keypair: Encryptable> {
    type StorageError: Error;

    fn store(&self, keypair: &Keypair, passphrase: &str) -> Result<(), Self::StorageError>;
    fn retrieve(&self, passphrase: &str) -> Result<Keypair, Self::StorageError>;
}

// TODO: We should add the other params to the sync trait as well (this would require a small refactor of the local keystore)
#[async_trait]
pub trait AsyncEncryptedKeystore<Keypair: Encryptable + Send + Sync, OtherParams: Send + Sync> {
    type StorageError: Error + Send + Sync;

    async fn store(
        &self,
        keypair: &Keypair,
        passphrase: &str,
        other_params: &OtherParams,
    ) -> Result<(), Self::StorageError>;
    async fn retrieve(
        &self,
        passphrase: &str,
        other_params: &OtherParams,
    ) -> Result<Keypair, Self::StorageError>;
}
