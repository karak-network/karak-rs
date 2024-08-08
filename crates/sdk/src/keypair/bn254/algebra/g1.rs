use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Sub},
    str::FromStr,
};

use ark_bn254::{G1Affine, G1Projective};
use ark_ec::CurveGroup;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use super::AlgebraError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct G1Point(pub G1Affine);

impl G1Point {
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, AlgebraError> {
        Ok(G1Point::deserialize_compressed(bytes.as_ref())?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AlgebraError> {
        let mut bytes = Vec::new();
        self.serialize_compressed(&mut bytes)?;
        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for G1Point {
    type Error = AlgebraError;

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

impl FromStr for G1Point {
    type Err = AlgebraError;

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
