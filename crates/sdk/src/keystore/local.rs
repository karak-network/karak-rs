use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use crate::keypair::traits::Encryptable;

use super::traits::EncryptedKeystore;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocalKeystoreError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Decoding error: {0}")]
    DecodingError(#[from] bs58::decode::Error),
}

pub struct LocalEncryptedKeystore {
    file_path: PathBuf,
}

impl LocalEncryptedKeystore {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

impl<Keypair: Encryptable> EncryptedKeystore<Keypair> for LocalEncryptedKeystore {
    type StorageError = LocalKeystoreError;

    fn store(&self, keypair: &Keypair, passphrase: &str) -> Result<(), LocalKeystoreError> {
        let encrypted_keypair = keypair
            .encrypt(passphrase)
            // TODO: Handle this error better. There has to be a more idiomatic way. cc @johanan
            .map_err(|err| LocalKeystoreError::EncryptionError(err.to_string()))?;

        let mut file = File::create(&self.file_path)?;

        file.write_all(bs58::encode(&encrypted_keypair).into_vec().as_slice())?;

        Ok(())
    }

    fn retrieve(&self, passphrase: &str) -> Result<Keypair, LocalKeystoreError> {
        let mut file = File::open(&self.file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;

        let encrypted_keypair = bs58::decode(buf).into_vec()?;

        Keypair::decrypt(&encrypted_keypair, passphrase)
            // TODO: Handle this error better. There has to be a more idiomatic way. cc @johanan
            .map_err(|err| LocalKeystoreError::EncryptionError(err.to_string()))
    }
}
