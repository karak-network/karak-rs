use std::str::FromStr;

use karak_sdk::{
    keypair::bn254::{G2Pubkey, PublicKeyError},
    signer::bls::signature::{Signature, SignatureError},
};

pub enum AggregateParams {
    Signatures(Vec<String>),
    Pubkeys(Vec<String>),
}

pub fn process_aggregate(params: AggregateParams) -> color_eyre::Result<()> {
    match params {
        AggregateParams::Signatures(signatures) => {
            let signatures: Vec<Signature> = signatures
                .iter()
                .map(|signature| Signature::from_str(signature))
                .collect::<Result<Vec<Signature>, SignatureError>>()?;

            let agg_signature: Signature = signatures.iter().sum();

            println!("Aggregated signature: {agg_signature}");

            Ok(())
        }
        AggregateParams::Pubkeys(pubkeys) => {
            let pubkeys: Vec<G2Pubkey> = pubkeys
                .iter()
                .map(|pubkey| G2Pubkey::from_str(pubkey))
                .collect::<Result<Vec<G2Pubkey>, PublicKeyError>>()?;

            let agg_key: G2Pubkey = pubkeys.iter().sum();

            println!("Aggregated key: {agg_key}");

            Ok(())
        }
    }
}
