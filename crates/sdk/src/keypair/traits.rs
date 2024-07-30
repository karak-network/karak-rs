use std::{error::Error, fmt::Display};

pub trait Keypair: Display {
    type SecretKey;
    type PublicKey;

    fn generate() -> Self;
    // TODO: Add from methods to "load" keys
    fn secret_key(&self) -> &Self::SecretKey;
    fn public_key(&self) -> &Self::PublicKey;
}

pub trait Encryptable: Sized {
    type EncryptionError: Error + Send + Sync + 'static;

    fn encrypt(&self, passphrase: &str) -> Result<Vec<u8>, Self::EncryptionError>;
    fn decrypt(encrypted_keypair: &[u8], passphrase: &str) -> Result<Self, Self::EncryptionError>;
}
