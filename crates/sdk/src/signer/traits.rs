pub trait Signer<M: Sized> {
    type Error;
    type Signature;

    fn sign_message(&self, message: M) -> Result<Self::Signature, Self::Error>;
}
