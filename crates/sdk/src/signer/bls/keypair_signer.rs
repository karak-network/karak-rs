use std::ops::Neg;

use ark_bn254::{Bn254, Fq, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::{BigInt, Field, One, PrimeField};
use ark_serialize::{CanonicalSerialize, Compress, SerializationError};
use thiserror::Error;

use crate::{
    keypair::{
        bn254::{self, G2Pubkey},
        traits::Keypair,
    },
    signer::traits::Signer,
};

use super::signature::Signature;

pub struct KeypairSigner {
    keypair: bn254::Keypair,
}

impl From<bn254::Keypair> for KeypairSigner {
    fn from(value: bn254::Keypair) -> Self {
        Self { keypair: value }
    }
}

#[derive(Debug, Error)]
pub enum KeypairSignerError {
    #[error("Serialization error: {0}")]
    SerializationError(SerializationError),
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Decoding error: {0}")]
    DecodingError(#[from] bs58::decode::Error),
}

pub type KeypairSignerResult<T> = Result<T, KeypairSignerError>;

impl From<SerializationError> for KeypairSignerError {
    fn from(value: SerializationError) -> Self {
        Self::SerializationError(value)
    }
}

impl Signer<&[u8; 32]> for KeypairSigner {
    type Error = KeypairSignerError;

    /// Caller is responsible for ensuring `hash` is a 32-byte hash of some arbitrary sized message
    fn sign_message(&self, hash: &[u8; 32]) -> KeypairSignerResult<Vec<u8>> {
        let sk = self.keypair.secret_key();
        let hm = hash_to_g1_point(hash);
        // TODO: Check whether its better/worse to use the projective version of the point
        let sig = (hm * sk).into_affine();

        let mut sig_bytes = vec![];

        // TODO: Check whether this version is correct one to give to the EVM pre-compile (if not, fix)
        sig.serialize_with_mode(&mut sig_bytes, Compress::Yes)?;

        Ok(sig_bytes)
    }
}

pub fn verify_signature(
    pubkey: &G2Pubkey,
    sig: &Signature,
    message: &[u8; 32],
) -> KeypairSignerResult<()> {
    let gen_g2 = G2Affine::generator();
    let msg_point_g1 = hash_to_g1_point(message);

    let neg_sig = sig.0.neg();

    let p = [msg_point_g1, neg_sig];
    let q = [pubkey.0, gen_g2];

    // e(H(m), sk * G2) * e(-(sk * H(m)), G2) =? 1
    let multi_pairing = Bn254::multi_pairing(p, q);

    if !multi_pairing.0.is_one() {
        return Err(KeypairSignerError::InvalidSignature);
    }

    Ok(())
}

// Implements the hash-and-check algorithm
// see https://hackmd.io/@benjaminion/bls12-381#Hash-and-check
// Curve: y^2 = x^3 + 3
fn hash_to_g1_point(message: &[u8; 32]) -> G1Affine {
    // TODO: big-endian or litte-endian here?
    let mut x = Fq::from_be_bytes_mod_order(message);

    loop {
        let y2 = x.pow(BigInt::<4>::from(3_u32)) + Fq::from(3_u32);
        if let Some(y) = y2.sqrt() {
            return G1Affine::new(x, y);
        } else {
            x += Fq::one();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_keypair() -> bn254::Keypair {
        bn254::Keypair::generate()
    }

    const PRECOMPUTED_KEYPAIR: [u8; 32] = [
        198, 117, 151, 83, 137, 156, 117, 191, 115, 217, 13, 31, 182, 91, 76, 247, 59, 36, 217,
        199, 162, 183, 248, 204, 213, 190, 143, 127, 12, 30, 224, 32,
    ];
    // This signature is for message [42u8; 32]
    const PRECOMPUTED_SIGNATURE_FOR_KEYPAIR: &str = "4R27e1Aou8nsUsrMcb51WMS49BBzogFxo5fNygFzA9zk";

    #[test]
    fn test_precomputed_signature() {
        let keypair: bn254::Keypair = PRECOMPUTED_KEYPAIR.as_slice().try_into().unwrap();
        let signer = KeypairSigner::from(keypair.clone());
        let message = [42u8; 32];

        let expected_signature = bs58::decode(PRECOMPUTED_SIGNATURE_FOR_KEYPAIR)
            .into_vec()
            .unwrap();
        let actual_signature = signer.sign_message(&message).unwrap();
        assert_eq!(actual_signature, expected_signature);

        let expected_sig = Signature::try_from(expected_signature.as_slice()).unwrap();
        assert!(verify_signature(&keypair.public_key().g2, &expected_sig, &message).is_ok());
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message = [42u8; 32];

        let signature = signer.sign_message(&message).unwrap();
        let sig = Signature::try_from(signature.as_slice()).unwrap();

        assert!(verify_signature(&keypair.public_key().g2, &sig, &message).is_ok());
    }

    #[test]
    fn test_invalid_signature() {
        let keypair = generate_keypair();
        let other_keypair = generate_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message = [42u8; 32];

        let signature = signer.sign_message(&message).unwrap();

        let sig = Signature::try_from(signature.as_slice()).unwrap();
        assert!(verify_signature(&other_keypair.public_key().g2, &sig, &message).is_err());
    }

    #[test]
    fn test_combine_signatures() {
        let keypairs: Vec<_> = (0..3).map(|_| generate_keypair()).collect();
        let signers: Vec<_> = keypairs
            .iter()
            .map(|kp| KeypairSigner::from(kp.clone()))
            .collect();
        let message = [42u8; 32];

        let signatures: Vec<_> = signers
            .iter()
            .take(3)
            .map(|signer| signer.sign_message(&message).unwrap())
            .collect();

        let combined_sig = signatures
            .iter()
            .map(|sig| Signature::try_from(sig.as_slice()).unwrap())
            .sum();

        // Combine public keys
        let combined_pubkey = keypairs.iter().map(|kp| &kp.public_key().g2).sum();

        assert!(verify_signature(&combined_pubkey, &combined_sig, &message).is_ok());
    }

    #[test]
    fn test_combine_signatures_mismatch() {
        let keypairs: Vec<_> = (0..3).map(|_| generate_keypair()).collect();
        let signers: Vec<_> = keypairs
            .iter()
            .map(|kp| KeypairSigner::from(kp.clone()))
            .collect();
        let message = [42u8; 32];

        let signatures: Vec<_> = signers
            .iter()
            .take(2) // Only sign with the first two keypairs
            .map(|signer| signer.sign_message(&message).unwrap())
            .collect();

        let combined_sig = signatures
            .iter()
            .map(|sig| Signature::try_from(sig.as_slice()).unwrap())
            .sum();

        // Combine public keys (including the third unused one)
        let combined_pubkey = keypairs.iter().map(|kp| &kp.public_key().g2).sum();

        assert!(verify_signature(&combined_pubkey, &combined_sig, &message).is_err());
    }

    #[test]
    fn test_sign_different_messages() {
        let keypair = generate_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message1 = [1u8; 32];
        let message2 = [2u8; 32];

        let signature1 = signer.sign_message(&message1).unwrap();
        let signature2 = signer.sign_message(&message2).unwrap();

        let sig1 = Signature::try_from(signature1.as_slice()).unwrap();
        let sig2 = Signature::try_from(signature2.as_slice()).unwrap();

        assert!(verify_signature(&keypair.public_key().g2, &sig1, &message1).is_ok());
        assert!(verify_signature(&keypair.public_key().g2, &sig2, &message2).is_ok());
        assert!(verify_signature(&keypair.public_key().g2, &sig1, &message2).is_err());
        assert!(verify_signature(&keypair.public_key().g2, &sig2, &message1).is_err());
    }
}
