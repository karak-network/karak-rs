use std::str::FromStr;

use karak_sdk::{keypair::bn254::G2Pubkey, signer::bls::signature::Signature};

pub enum AggregateParams {
    Signatures(Vec<String>),
    Pubkeys(Vec<String>),
}

pub fn process_aggregate(params: AggregateParams) -> color_eyre::Result<()> {
    match params {
        AggregateParams::Signatures(signatures) => {
            let signatures: Vec<Signature> = signatures
                .iter()
                // TODO: Clean up this unwrap
                .map(|signature| Signature::from_str(signature).unwrap())
                .collect();

            let agg_signature: Signature = signatures.iter().sum();

            println!("Aggregated signature: {agg_signature}");

            Ok(())
        }
        AggregateParams::Pubkeys(pubkeys) => {
            let pubkeys: Vec<G2Pubkey> = pubkeys
                .iter()
                // TODO: Clean up this unwrap
                .map(|pubkey| G2Pubkey::from_str(pubkey).unwrap().into())
                .collect();

            let agg_key: G2Pubkey = pubkeys.iter().sum();

            println!("Aggregated key: {agg_key}");

            Ok(())
        }
    }
}
