pub trait Signer<M: Sized> {
    fn sign_message(&self, message: M) -> Vec<u8>;
}
