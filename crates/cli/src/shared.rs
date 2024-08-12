use clap::ValueEnum;

#[derive(Clone, ValueEnum, Debug)]
pub enum Curve {
    /// BN254 (also known as alt_bn128) is the curve used in Ethereum for BLS aggregation
    Bn254,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Scheme {
    /// Boneh–Lynn–Shacham (BLS) signature scheme using BN254
    Bls,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Encoding {
    Utf8,
    Hex,
    Base64,
    Base64URL,
    Base58,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Keystore {
    Local,
    Aws,
}
