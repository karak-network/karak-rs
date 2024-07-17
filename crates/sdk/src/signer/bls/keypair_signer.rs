use std::ops::Neg;

use ark_bn254::{Bn254, Fq, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::{BigInt, Field, One, PrimeField};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};

use crate::{
    keypair::{
        bn254::{self, G2Pubkey},
        traits::Keypair,
    },
    signer::traits::Signer,
};

pub struct KeypairSigner {
    keypair: bn254::Keypair,
}

impl From<bn254::Keypair> for KeypairSigner {
    fn from(value: bn254::Keypair) -> Self {
        Self { keypair: value }
    }
}

impl Signer<&[u8; 32]> for KeypairSigner {
    /// Caller is responsible for ensuring `hash` is a 32-byte hash of some arbitrary sized message
    fn sign_message(&self, hash: &[u8; 32]) -> Vec<u8> {
        let sk = self.keypair.secret_key();
        let hm = hash_to_g1_point(hash);
        // TODO: Check whether its better/worse to use the projective version of the point
        let sig = (hm * sk).into_affine();

        let mut sig_bytes = vec![];

        // TODO: Check whether this version is correct one to give to the EVM pre-compile (if not, fix)
        sig.serialize_with_mode(&mut sig_bytes, Compress::Yes)
            .unwrap();

        sig_bytes
    }
}

pub fn verify_signature(
    pubkey: G2Pubkey,
    sig: &str,
    message: &[u8; 32],
    // TODO: Change error type to more appropriate one
) -> Result<(), SerializationError> {
    let gen_g2 = G2Affine::generator();
    let msg_point_g1 = hash_to_g1_point(message);

    let sig_bytes = bs58::decode(sig)
        .into_vec()
        .map_err(|_| SerializationError::InvalidData)?;

    let sig_g1 =
        G1Affine::deserialize_with_mode(sig_bytes.as_slice(), Compress::Yes, Validate::Yes)?;

    let neg_sig_g1 = sig_g1.neg();

    let p = [msg_point_g1, neg_sig_g1];
    let q = [*pubkey, gen_g2];

    // e(H(m), sk * G2) * e(-(sk * H(m)), G2) =? 1
    let multi_pairing = Bn254::multi_pairing(p, q);

    if !multi_pairing.0.is_one() {
        return Err(SerializationError::InvalidData);
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
