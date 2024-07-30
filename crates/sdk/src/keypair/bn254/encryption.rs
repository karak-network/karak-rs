use super::{Keypair, KeypairError};
use crate::{
    encryption::{self, EncryptDataV3Error},
    keypair::traits::Encryptable,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeypairEncryptionError {
    #[error("Keypair error: {0}")]
    KeypairError(#[from] KeypairError),

    #[error("Encryption error: {0}")]
    EncryptionError(#[from] EncryptDataV3Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
}

impl Encryptable for Keypair {
    type EncryptionError = KeypairEncryptionError;

    fn encrypt(&self, passphrase: &str) -> Result<Vec<u8>, KeypairEncryptionError> {
        let serialized_keypair: Vec<u8> = self.try_into()?;
        // TODO: I kept the scrypt_log_n parameter to 15 here but this is orders of magnitude less secure than the geth keystore value of 18
        //       This is in the interest of testing speed for now but we should update it later once we finalize the params
        let encrypted_keypair =
            encryption::encrypt_data_v3(&serialized_keypair, passphrase.as_bytes(), 14, 1)?;

        Ok(bincode::serialize(&encrypted_keypair)?)
    }

    fn decrypt(encrypted_keypair: &[u8], passphrase: &str) -> Result<Self, KeypairEncryptionError> {
        let serialized_keypair = encryption::decrypt_data_v3(
            &bincode::deserialize(encrypted_keypair)?,
            passphrase.as_bytes(),
        )?;

        Ok(Keypair::try_from(serialized_keypair.as_slice())?)
    }
}
