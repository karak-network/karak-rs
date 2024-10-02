use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Neg, Sub},
    str::FromStr,
};

use ark_bn254::{G1Affine, G1Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Serialize};

use crate::keypair::bn254::Bn254Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize, Hash)]
pub struct G1Point(pub G1Affine);

impl G1Point {
    pub fn generator() -> Self {
        G1Point(G1Affine::generator())
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, Bn254Error> {
        Ok(G1Point::deserialize_compressed(bytes.as_ref())?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Bn254Error> {
        let mut bytes = Vec::new();
        self.serialize_compressed(&mut bytes)?;
        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for G1Point {
    type Error = Bn254Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(G1Point::deserialize_compressed(bytes)?)
    }
}

impl Add for &G1Point {
    type Output = G1Point;

    fn add(self, rhs: Self) -> Self::Output {
        G1Point((self.0 + rhs.0).into())
    }
}

impl Add for G1Point {
    type Output = G1Point;

    fn add(self, rhs: Self) -> Self::Output {
        G1Point((self.0 + rhs.0).into())
    }
}

impl Sum<G1Point> for G1Point {
    fn sum<I: Iterator<Item = G1Point>>(iter: I) -> G1Point {
        G1Point(
            iter.fold(G1Projective::default(), |acc, g1| acc + g1.0)
                .into_affine(),
        )
    }
}

impl<'a> Sum<&'a G1Point> for G1Point {
    fn sum<I: Iterator<Item = &'a G1Point>>(iter: I) -> Self {
        G1Point(
            iter.fold(G1Projective::default(), |acc, g1| acc + g1.0)
                .into_affine(),
        )
    }
}

impl Sub for &G1Point {
    type Output = G1Point;

    fn sub(self, rhs: Self) -> Self::Output {
        G1Point((self.0 - rhs.0).into())
    }
}

impl Sub for G1Point {
    type Output = G1Point;

    fn sub(self, rhs: Self) -> Self::Output {
        G1Point((self.0 - rhs.0).into())
    }
}

impl Neg for G1Point {
    type Output = G1Point;

    fn neg(self) -> Self::Output {
        G1Point(self.0.neg())
    }
}

impl FromStr for G1Point {
    type Err = Bn254Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        G1Point::from_bytes(bs58::decode(s).into_vec()?)
    }
}

impl Display for G1Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.to_bytes().map_err(|_| std::fmt::Error)?;
        write!(f, "{}", bs58::encode(bytes).into_string())?;

        Ok(())
    }
}

impl From<G1Affine> for G1Point {
    fn from(g1: G1Affine) -> Self {
        G1Point(g1)
    }
}

impl Serialize for G1Point {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for G1Point {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        G1Point::from_str(&s).map_err(serde::de::Error::custom)
    }
}

// Bindings for conversions to SolType
mod sol {
    use alloy::{primitives::U256, sol_types};
    use alloy_sol_types::SolType;
    use ark_bn254::{Fq, G1Affine};
    use ark_ff::{BigInt, PrimeField};

    use super::G1Point;
    #[doc(hidden)]
    type UnderlyingSolTuple<'a> = (
        sol_types::sol_data::Uint<256>,
        sol_types::sol_data::Uint<256>,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = (U256, U256);
    #[cfg(test)]
    #[allow(dead_code, unreachable_patterns)]
    fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
        match _t {
            alloy_sol_types::private::AssertTypeEq::<
                <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
            >(_) => {}
        }
    }

    #[doc(hidden)]
    impl From<G1Point> for UnderlyingRustTuple<'_> {
        fn from(g1: G1Point) -> Self {
            let x = U256::from_limbs(g1.0.x.into_bigint().0);
            let y = U256::from_limbs(g1.0.y.into_bigint().0);
            (x, y)
        }
    }
    #[doc(hidden)]
    impl<'a> From<&'a G1Point> for UnderlyingRustTuple<'a> {
        fn from(g1: &'a G1Point) -> Self {
            let x = U256::from_limbs(g1.0.x.into_bigint().0);
            let y = U256::from_limbs(g1.0.y.into_bigint().0);
            (x, y)
        }
    }

    #[doc(hidden)]
    impl From<UnderlyingRustTuple<'_>> for G1Point {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            let x = Fq::from(BigInt(tuple.0.into_limbs()));
            let y = Fq::from(BigInt(tuple.1.into_limbs()));
            G1Point(G1Affine::new(x, y))
        }
    }

    impl alloy_sol_types::SolValue for G1Point {
        type SolType = Self;
    }

    impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            let tuple = UnderlyingRustTuple::from(self);
            (
                sol_types::sol_data::Uint::<256>::tokenize(&tuple.0),
                sol_types::sol_data::Uint::<256>::tokenize(&tuple.1),
            )
        }
        #[inline]
        fn stv_abi_encoded_size(&self) -> usize {
            if let Some(size) = Self::ENCODED_SIZE {
                return size;
            }
            let tuple = UnderlyingRustTuple::from(self);
            UnderlyingSolTuple::abi_encoded_size(&tuple)
        }
        #[inline]
        fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
            <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
        }
        #[inline]
        fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            let tuple = UnderlyingRustTuple::from(self);
            UnderlyingSolTuple::abi_encode_packed_to(&tuple, out)
        }
        #[inline]
        fn stv_abi_packed_encoded_size(&self) -> usize {
            if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                return size;
            }
            let tuple = UnderlyingRustTuple::from(self);
            <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
        }
    }

    impl alloy_sol_types::SolType for G1Point {
        type RustType = Self;
        type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
        const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
        const ENCODED_SIZE: Option<usize> =
            <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
        const PACKED_ENCODED_SIZE: Option<usize> =
            <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
        #[inline]
        fn valid_token(token: &Self::Token<'_>) -> bool {
            <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
        }
        #[inline]
        fn detokenize(token: Self::Token<'_>) -> Self::RustType {
            let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
            <Self as From<UnderlyingRustTuple<'_>>>::from(tuple)
        }
    }

    impl alloy_sol_types::SolStruct for G1Point {
        const NAME: &'static str = "G1Point";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
        }
        #[inline]
        fn eip712_components(
        ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
            alloy_sol_types::private::Vec::new()
        }
        #[inline]
        fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
            <Self as alloy_sol_types::SolStruct>::eip712_root_type()
        }
        #[inline]
        fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
            let tuple = UnderlyingRustTuple::from(self);
            [
                <sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
                    &tuple.0,
                )
                .0,
                <sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
                    &tuple.1,
                )
                .0,
            ]
            .concat()
        }
    }

    impl alloy_sol_types::EventTopic for G1Point {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            let rust = UnderlyingRustTuple::from(rust);
            <sol_types::sol_data::Uint<256>as alloy_sol_types::EventTopic> ::topic_preimage_length(&rust.0)+ <sol_types::sol_data::Uint<256>as alloy_sol_types::EventTopic> ::topic_preimage_length(&rust.1)
        }
        #[inline]
        fn encode_topic_preimage(
            rust: &Self::RustType,
            out: &mut alloy_sol_types::private::Vec<u8>,
        ) {
            let tuple = UnderlyingRustTuple::from(rust);
            out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &tuple.0, out,
            );
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &tuple.1, out,
            );
        }
        #[inline]
        fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
            let mut out = alloy_sol_types::private::Vec::new();
            <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
            alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::U256;
    use alloy_sol_types::SolValue;

    use super::*;

    #[test]
    fn test_abi_encode() {
        let x = U256::from_str_radix(
            "2cf44499d5d27bb186308b7af7af02ac5bc9eeb6a3d147c186b21fb1b76e18da",
            16,
        )
        .unwrap();
        let y = U256::from_str_radix(
            "2c0f001f52110ccfe69108924926e45f0b0c868df0e7bde1fe16d3242dc715f6",
            16,
        )
        .unwrap();
        let g1 = G1Point::from((x, y));
        let encoded = g1.abi_encode();
        let expected = (x, y).abi_encode();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_abi_decode() {
        let encoded = hex::decode(
            "2cf44499d5d27bb186308b7af7af02ac5bc9eeb6a3d147c186b21fb1b76e18da2c0f001f52110ccfe69108924926e45f0b0c868df0e7bde1fe16d3242dc715f6")
                .unwrap();
        let g1 = G1Point::abi_decode(&encoded, true).unwrap();
        assert_eq!(
            g1.to_string(),
            "FgMVD1tGpx5z4EW6b8kCjNcH8ydXuviEKUoW4fJecrdy"
        );
    }

    #[test]
    fn test_serde() {
        let g1 = G1Point::generator();
        let serialized = serde_json::to_string(&g1).unwrap();
        let deserialized = serde_json::from_str::<G1Point>(&serialized).unwrap();
        assert_eq!(g1, deserialized);
    }
}
