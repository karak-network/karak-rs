use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Sub},
    str::FromStr,
};

use ark_bn254::{G2Affine, G2Projective};
use ark_ec::CurveGroup;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use super::AlgebraError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct G2Point(pub(crate) G2Affine);

impl G2Point {
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, AlgebraError> {
        Ok(G2Point::deserialize_compressed(bytes.as_ref())?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AlgebraError> {
        let mut bytes = Vec::new();
        self.serialize_compressed(&mut bytes)?;
        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for G2Point {
    type Error = AlgebraError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(G2Point::deserialize_compressed(bytes)?)
    }
}

impl Add for &G2Point {
    type Output = G2Point;

    fn add(self, rhs: Self) -> Self::Output {
        G2Point((self.0 + rhs.0).into())
    }
}

impl Add for G2Point {
    type Output = G2Point;

    fn add(self, rhs: Self) -> Self::Output {
        G2Point((self.0 + rhs.0).into())
    }
}

impl Sum<G2Point> for G2Point {
    fn sum<I: Iterator<Item = G2Point>>(iter: I) -> G2Point {
        G2Point(
            iter.fold(G2Projective::default(), |acc, g2| acc + g2.0)
                .into_affine(),
        )
    }
}

impl<'a> Sum<&'a G2Point> for G2Point {
    fn sum<I: Iterator<Item = &'a G2Point>>(iter: I) -> Self {
        G2Point(
            iter.fold(G2Projective::default(), |acc, g2| acc + g2.0)
                .into_affine(),
        )
    }
}

impl Sub for &G2Point {
    type Output = G2Point;

    fn sub(self, rhs: Self) -> Self::Output {
        G2Point((self.0 - rhs.0).into())
    }
}

impl Sub for G2Point {
    type Output = G2Point;

    fn sub(self, rhs: Self) -> Self::Output {
        G2Point((self.0 - rhs.0).into())
    }
}

impl FromStr for G2Point {
    type Err = AlgebraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        G2Point::from_bytes(bs58::decode(s).into_vec()?)
    }
}

impl Display for G2Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.to_bytes().map_err(|_| std::fmt::Error)?;
        write!(f, "{}", bs58::encode(bytes).into_string())?;

        Ok(())
    }
}

impl From<G2Affine> for G2Point {
    fn from(g2: G2Affine) -> Self {
        G2Point(g2)
    }
}

// Bindings for conversions to SolType
mod sol {
    use alloy::{primitives::U256, sol_types};
    use alloy_sol_types::{abi::token::FixedSeqToken, SolType};
    use ark_bn254::{Fq, Fq2, G2Affine};
    use ark_ff::{BigInt, PrimeField};

    use super::G2Point;
    #[doc(hidden)]
    type UnderlyingSolTuple<'a> = (
        sol_types::sol_data::FixedArray<sol_types::sol_data::Uint<256>, 2>,
        sol_types::sol_data::FixedArray<sol_types::sol_data::Uint<256>, 2>,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = ([U256; 2usize], [U256; 2usize]);
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
    impl From<G2Point> for UnderlyingRustTuple<'_> {
        fn from(g2: G2Point) -> Self {
            let x0 = U256::from_limbs(g2.0.x.c0.into_bigint().0);
            let x1 = U256::from_limbs(g2.0.x.c1.into_bigint().0);
            let y0 = U256::from_limbs(g2.0.y.c0.into_bigint().0);
            let y1 = U256::from_limbs(g2.0.y.c1.into_bigint().0);
            ([x0, x1], [y0, y1])
        }
    }
    #[doc(hidden)]
    impl<'a> From<&'a G2Point> for UnderlyingRustTuple<'a> {
        fn from(g2: &'a G2Point) -> Self {
            let x0 = U256::from_limbs(g2.0.x.c0.into_bigint().0);
            let x1 = U256::from_limbs(g2.0.x.c1.into_bigint().0);
            let y0 = U256::from_limbs(g2.0.y.c0.into_bigint().0);
            let y1 = U256::from_limbs(g2.0.y.c1.into_bigint().0);
            ([x0, x1], [y0, y1])
        }
    }

    #[doc(hidden)]
    impl From<UnderlyingRustTuple<'_>> for G2Point {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            let [x0, x1] = tuple.0;
            let x = Fq2::new(
                Fq::from(BigInt(x0.into_limbs())),
                Fq::from(BigInt(x1.into_limbs())),
            );
            let [y0, y1] = tuple.1;
            let y = Fq2::new(
                Fq::from(BigInt(y0.into_limbs())),
                Fq::from(BigInt(y1.into_limbs())),
            );
            G2Point(G2Affine::new(x, y))
        }
    }

    impl alloy_sol_types::SolValue for G2Point {
        type SolType = Self;
    }

    impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            let ([x0, x1], [y0, y1]) = UnderlyingRustTuple::from(self);

            // IMPORTANT: The order of the elements in the tuple must match the order of the fields in the struct
            (
                FixedSeqToken([
                    sol_types::sol_data::Uint::<256>::tokenize(&x1),
                    sol_types::sol_data::Uint::<256>::tokenize(&x0),
                ]),
                FixedSeqToken([
                    sol_types::sol_data::Uint::<256>::tokenize(&y1),
                    sol_types::sol_data::Uint::<256>::tokenize(&y0),
                ]),
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

    impl alloy_sol_types::SolType for G2Point {
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
            let ([x0, x1], [y0, y1]) =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);

            // IMPORTANT: The order of the elements in the tuple must match the order of the fields in the struct
            <Self as From<UnderlyingRustTuple<'_>>>::from(([x1, x0], [y1, y0]))
        }
    }

    impl alloy_sol_types::SolStruct for G2Point {
        const NAME: &'static str = "G2Point";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed("G2Point(uint256[2] X,uint256[2] Y)")
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
            let ([x0, x1], [y0, y1]) = UnderlyingRustTuple::from(self);
            [
                <sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(&x1)
                    .0,
                <sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(&x0)
                    .0,
                <sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(&y1)
                    .0,
                <sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(&y0)
                    .0,
            ]
            .concat()
        }
    }

    impl alloy_sol_types::EventTopic for G2Point {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            let ([x0, x1], [y0, y1]) = UnderlyingRustTuple::from(rust);
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(&x1)
            + <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(&x0)
            + <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(&y1)
            + <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(&y0)
        }
        #[inline]
        fn encode_topic_preimage(
            rust: &Self::RustType,
            out: &mut alloy_sol_types::private::Vec<u8>,
        ) {
            let ([x0, x1], [y0, y1]) = UnderlyingRustTuple::from(rust);
            out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &x1, out,
            );
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &x0, out,
            );
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &y1, out,
            );
            <sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &y0, out,
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
    fn test_abi_encode_decode() {
        let x_1 = U256::from_str_radix(
            "1fb19bb476f6b9e44e2a32234da8212f61cd63919354bc06aef31e3cfaff3ebc",
            16,
        )
        .unwrap();
        let x_0 = U256::from_str_radix(
            "22606845ff186793914e03e21df544c34ffe2f2f3504de8a79d9159eca2d98d9",
            16,
        )
        .unwrap();
        let y_1 = U256::from_str_radix(
            "2bd368e28381e8eccb5fa81fc26cf3f048eea9abfdd85d7ed3ab3698d63e4f90",
            16,
        )
        .unwrap();
        let y_0 = U256::from_str_radix(
            "2fe02e47887507adf0ff1743cbac6ba291e66f59be6bd763950bb16041a0a85e",
            16,
        )
        .unwrap();

        let g2 = G2Point::from(([x_0, x_1], [y_0, y_1]));
        let encoded = g2.abi_encode();
        assert_eq!(encoded, ([x_1, x_0], [y_1, y_0]).abi_encode());
    }

    #[test]
    fn test_abi_decode() {
        let encoded = hex::decode(
            "1fb19bb476f6b9e44e2a32234da8212f61cd63919354bc06aef31e3cfaff3ebc\
            22606845ff186793914e03e21df544c34ffe2f2f3504de8a79d9159eca2d98d9\
            2bd368e28381e8eccb5fa81fc26cf3f048eea9abfdd85d7ed3ab3698d63e4f90\
            2fe02e47887507adf0ff1743cbac6ba291e66f59be6bd763950bb16041a0a85e",
        )
        .unwrap();
        let g1 = G2Point::abi_decode(&encoded, true).unwrap();
        assert_eq!(
            g1.to_string(),
            "5MKnC67sfNsXsMfMxrQaU42RETSQvZV66R5xntcGgSL6VdFCD3Ef4PawpJGK8igYVstf4vgfVqb5cCfByEJ7FaxN"
        );
    }
}
