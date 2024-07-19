pub trait Signer<M: Sized> {
    type Error;

    fn sign_message(&self, message: M) -> Result<Vec<u8>, Self::Error>;
}
