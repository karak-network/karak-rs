pub mod aggregate;
pub mod sign;
pub mod verify;

use aggregate::{process_aggregate, AggregateParams};
use color_eyre::eyre;
use sign::process_sign;
use verify::process_verify;

use super::{Aggregate, BLS};
use crate::config::models::Profile;

// TODO: use profile where needed
pub async fn process(command: BLS, _: Profile) -> eyre::Result<()> {
    match command {
        BLS::Sign {
            keypair_location,
            keypair,
            message,
        } => process_sign(keypair_location, keypair, message).await,
        BLS::Verify {
            message: message_args,
            pubkey,
            signature,
        } => process_verify(message_args, pubkey, signature),
        BLS::Aggregate(aggregate) => match aggregate {
            Aggregate::Signatures { signatures } => {
                process_aggregate(AggregateParams::Signatures(signatures))
            }
            Aggregate::Pubkeys { pubkeys } => process_aggregate(AggregateParams::Pubkeys(pubkeys)),
        },
    }
}
