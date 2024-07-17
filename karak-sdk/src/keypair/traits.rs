use std::fmt::Display;

pub trait Keypair: Display {
    type SecretKey;
    type PublicKey;

    fn generate() -> Self;
    // TODO: Add from methods to "load" keys
    fn secret_key(&self) -> &Self::SecretKey;
    fn public_key(&self) -> &Self::PublicKey;
}
