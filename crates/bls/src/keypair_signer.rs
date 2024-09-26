use std::ops::Neg;

use ark_bn254::{Bn254, Fq, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::{BigInt, Field, One, PrimeField};
use signature::{Error as SignatureError, Signer, Verifier};

use karak_kms::keypair::{
    bn254::{self, algebra::g1::G1Point, G2Pubkey, PublicKey},
    traits::Keypair,
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

pub type KeypairSignerResult<T> = Result<T, SignatureError>;

impl Signer<Signature> for KeypairSigner {
    /// Caller is responsible for ensuring `hash` is a 32-byte hash of some arbitrary sized message
    fn try_sign(&self, bytes: &[u8]) -> KeypairSignerResult<Signature> {
        let sk = self.keypair.secret_key();
        let hm = hash_to_g1_point(bytes);
        // TODO: Check whether its better/worse to use the projective version of the point
        let sig = (hm * sk).into_affine();

        Ok(Signature::from(sig))
    }
}

pub struct Bn254Verifier(G2Pubkey);

impl From<&G2Pubkey> for Bn254Verifier {
    fn from(value: &G2Pubkey) -> Self {
        Self(value.to_owned())
    }
}
pub struct Bn254CombinedVerifier(PublicKey);

impl From<&PublicKey> for Bn254CombinedVerifier {
    fn from(value: &PublicKey) -> Self {
        Self(value.to_owned())
    }
}

impl Verifier<Signature> for Bn254Verifier {
    fn verify(&self, message: &[u8], sig: &Signature) -> KeypairSignerResult<()> {
        let gen_g2 = G2Affine::generator();
        let msg_point_g1 = hash_to_g1_point(message);

        let neg_sig = sig.0.neg();

        let p = [msg_point_g1, neg_sig];
        let q = [self.0 .0, gen_g2];

        // e(H(m), sk * G2) * e(-(sk * H(m)), G2) =? 1
        let multi_pairing = Bn254::multi_pairing(p, q);

        if !multi_pairing.0.is_one() {
            return Err(SignatureError::new());
        }

        Ok(())
    }
}

impl Verifier<Signature> for Bn254CombinedVerifier {
    fn verify(&self, message: &[u8], sig: &Signature) -> KeypairSignerResult<()> {
        let signature_plus_pubkey_g1 = sig + &self.0.g1;
        let hash_plus_generator_g1 =
            G1Point::from(hash_to_g1_point(message)) + G1Point::generator();

        let gen_g2 = G2Affine::generator();

        let neg_sig = signature_plus_pubkey_g1.0.neg();

        let p = [hash_plus_generator_g1.0, neg_sig];
        let q = [self.0.g2.0, gen_g2];

        // e((H(m)+G1), sk * G2) * e(-(sk * (H(m) + G1)), G2) =? 1
        let multi_pairing = Bn254::multi_pairing(p, q);

        if !multi_pairing.0.is_one() {
            return Err(SignatureError::new());
        }

        Ok(())
    }
}

// Implements the hash-and-check algorithm
// see https://hackmd.io/@benjaminion/bls12-381#Hash-and-check
// Curve: y^2 = x^3 + 3
pub fn hash_to_g1_point(message: &[u8]) -> G1Affine {
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
    use std::{str::FromStr, sync::OnceLock};

    use super::*;

    fn generate_keypair() -> bn254::Keypair {
        bn254::Keypair::generate()
    }

    static PRECOMPUTED_KEYPAIR: OnceLock<bn254::Keypair> = OnceLock::new();

    fn precomputed_keypair() -> &'static bn254::Keypair {
        PRECOMPUTED_KEYPAIR.get_or_init(|| {
            bn254::Keypair::from_bytes([
                198, 117, 151, 83, 137, 156, 117, 191, 115, 217, 13, 31, 182, 91, 76, 247, 59, 36,
                217, 199, 162, 183, 248, 204, 213, 190, 143, 127, 12, 30, 224, 32,
            ])
            .unwrap()
        })
    }
    // This signature is for message [42u8; 32]
    static PRECOMPUTED_SIGNATURE_FOR_KEYPAIR: OnceLock<Signature> = OnceLock::new();

    fn precomputed_signature_for_keypair() -> &'static Signature {
        PRECOMPUTED_SIGNATURE_FOR_KEYPAIR.get_or_init(|| {
            Signature::from_str("4R27e1Aou8nsUsrMcb51WMS49BBzogFxo5fNygFzA9zk").unwrap()
        })
    }

    #[test]
    fn test_precomputed_signature() {
        let keypair = precomputed_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message = [42u8; 32];
        let verifier = Bn254Verifier::from(&keypair.public_key().g2);
        let combined_verifier = Bn254CombinedVerifier::from(keypair.public_key());

        let expected_signature = precomputed_signature_for_keypair();
        let actual_signature = signer.try_sign(&message).unwrap();
        assert_eq!(actual_signature, *expected_signature);

        assert!(verifier.verify(&message, &expected_signature).is_ok());
        assert!(combined_verifier.verify(&message, &expected_signature).is_ok());
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message = [42u8; 32];
        let verifier = Bn254Verifier::from(&keypair.public_key().g2);
        let combined_verifier = Bn254CombinedVerifier::from(keypair.public_key());

        let signature = signer.try_sign(&message).unwrap();

        assert!(verifier.verify(&message, &signature).is_ok());
        assert!(combined_verifier.verify(&message, &signature).is_ok());
    }

    #[test]
    fn test_invalid_signature() {
        let keypair = generate_keypair();
        let other_keypair = generate_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message = [42u8; 32];
        let verifier = Bn254Verifier::from(&other_keypair.public_key().g2);
        let combined_verifier = Bn254CombinedVerifier::from(other_keypair.public_key());

        let signature = signer.try_sign(&message).unwrap();

        assert!(matches!(
            verifier.verify(&message, &signature),
            Err(SignatureError)
        ));
        assert!(matches!(
            combined_verifier.verify(&message, &signature),
            Err(SignatureError)
        ))
    }

    #[test]
    fn test_aggregated_signatures() {
        let keypairs: Vec<_> = (0..3).map(|_| generate_keypair()).collect();
        let signers: Vec<_> = keypairs
            .iter()
            .map(|kp| KeypairSigner::from(kp.clone()))
            .collect();
        let message = [42u8; 32];

        let signatures: Vec<_> = signers
            .iter()
            .take(3)
            .map(|signer| signer.try_sign(&message).unwrap())
            .collect();

        let aggregated_sig = signatures.iter().sum();

        // Combine public keys
        let aggregated_g2 = keypairs.iter().map(|kp| &kp.public_key().g2).sum();
        let aggregated_g1 = keypairs.iter().map(|kp| &kp.public_key().g1).sum();
        let aggregated_keypair = PublicKey{g1: aggregated_g1, g2: aggregated_g2};

        let verifier = Bn254Verifier::from(&aggregated_g2);
        let combined_verifier = Bn254CombinedVerifier::from(&aggregated_keypair);

        assert!(verifier.verify(&message, &aggregated_sig).is_ok());
        assert!(combined_verifier.verify(&message, &aggregated_sig).is_ok());
    }

    #[test]
    fn test_aggregated_signatures_mismatch() {
        let keypairs: Vec<_> = (0..3).map(|_| generate_keypair()).collect();
        let signers: Vec<_> = keypairs
            .iter()
            .map(|kp| KeypairSigner::from(kp.clone()))
            .collect();
        let message = [42u8; 32];

        let signatures: Vec<_> = signers
            .iter()
            .take(2) // Only sign with the first two keypairs
            .map(|signer| signer.try_sign(&message).unwrap())
            .collect();

        let aggregated_sig = signatures.iter().sum();

        // Combine public keys (including the third unused one)
        let aggregated_g2 = keypairs.iter().map(|kp| &kp.public_key().g2).sum();
        let aggregated_g1 = keypairs.iter().map(|kp| &kp.public_key().g1).sum();
        let aggregated_keypair = PublicKey{g1: aggregated_g1, g2: aggregated_g2};

        let verifier = Bn254Verifier::from(&aggregated_g2);
        let combined_verifier = Bn254CombinedVerifier::from(&aggregated_keypair);

        assert!(matches!(
            verifier.verify(&message, &aggregated_sig),
            Err(SignatureError)
        ));
        assert!(matches!(combined_verifier.verify(&message, &aggregated_sig),
        Err(SignatureError)
    ))
    }

    #[test]
    fn test_sign_different_messages() {
        let keypair = generate_keypair();
        let signer = KeypairSigner::from(keypair.clone());
        let message1 = [1u8; 32];
        let message2 = [2u8; 32];

        let signature1 = signer.try_sign(&message1).unwrap();
        let signature2 = signer.try_sign(&message2).unwrap();

        let verifier = Bn254Verifier::from(&signer.keypair.public_key().g2);
        let combined_verifier = Bn254CombinedVerifier::from(signer.keypair.public_key());

        assert!(verifier.verify(&message1, &signature1).is_ok());
        assert!(combined_verifier.verify(&message1, &signature1).is_ok());
        assert!(verifier.verify(&message2, &signature2).is_ok());
        assert!(combined_verifier.verify(&message2, &signature2).is_ok());
        assert!(matches!(
            verifier.verify(&message2, &signature1),
            Err(SignatureError)
        ));
        assert!(matches!(
            combined_verifier.verify(&message2, &signature1),
            Err(SignatureError)
        ));
        assert!(matches!(
            verifier.verify(&message1, &signature2),
            Err(SignatureError)
        ));
        assert!(matches!(
            combined_verifier.verify(&message1, &signature2),
            Err(SignatureError)
        ));
    }
}
