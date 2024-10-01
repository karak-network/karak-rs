use std::str::FromStr;

use color_eyre::eyre;
use karak_kms::keypair::bn254::{Bn254Error, G2Pubkey,bls::signature::Signature};

pub enum AggregateParams {
    Signatures(Vec<String>),
    Pubkeys(Vec<String>),
}

pub fn process_aggregate(params: AggregateParams) -> eyre::Result<()> {
    match params {
        AggregateParams::Signatures(signatures) => {
            let signatures: Vec<Signature> = signatures
                .iter()
                .map(|signature| Signature::from_str(signature))
                .collect::<Result<Vec<Signature>, Bn254Error>>()?;

            let agg_signature: Signature = signatures.iter().sum();

            println!("Aggregated signature: {agg_signature}");

            Ok(())
        }
        AggregateParams::Pubkeys(pubkeys) => {
            let pubkeys: Vec<G2Pubkey> = pubkeys
                .iter()
                .map(|pubkey| G2Pubkey::from_str(pubkey))
                .collect::<Result<Vec<G2Pubkey>, Bn254Error>>()?;

            let agg_key: G2Pubkey = pubkeys.iter().sum();

            println!("Aggregated key: {agg_key}");

            Ok(())
        }
    }
}
