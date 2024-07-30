use crate::keypair::traits::Encryptable;
use std::error::Error;

pub trait EncryptedKeystore<Keypair: Encryptable> {
    type StorageError: Error;

    fn store(&self, keypair: &Keypair, passphrase: &str) -> Result<(), Self::StorageError>;
    fn retrieve(&self, passphrase: &str) -> Result<Keypair, Self::StorageError>;
}
