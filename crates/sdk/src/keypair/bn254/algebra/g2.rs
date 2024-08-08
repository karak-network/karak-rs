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
pub struct G2Point(pub G2Affine);

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
