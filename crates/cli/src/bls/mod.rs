use clap::{Args, Parser, Subcommand};
pub mod processor;

use crate::{
    keypair::{KeypairArgs, KeypairLocationArgs},
    shared::Encoding,
};

#[derive(Parser)]
#[command(version, about = "Karak BLS utility CLI", long_about = None)]
#[command(propagate_version = true)]
pub struct KeypairRoot {
    #[command(subcommand)]
    subcommand: BLS,
}

#[derive(Subcommand, Debug)]
pub enum BLS {
    /// Sign with keypair
    Sign {
        #[command(flatten)]
        keypair_location: KeypairLocationArgs,
        #[command(flatten)]
        keypair: KeypairArgs,
        #[command(flatten)]
        message: MessageArgs,
    },
    /// Verify G1 signature
    Verify {
        #[command(flatten)]
        message: MessageArgs,

        #[arg(short = 'k', long)]
        pubkey: String,

        #[arg(short, long)]
        signature: String,
    },
    /// Aggregate BLS signatures or pubkeys
    #[command(subcommand)]
    Aggregate(Aggregate),
}

#[derive(Subcommand, Debug)]
pub enum Aggregate {
    /// Aggregate BLS signatures
    Signatures {
        #[arg(short, long)]
        signatures: Vec<String>,
    },
    /// Aggregate BLS G2 pubkeys
    Pubkeys {
        #[arg(short, long)]
        pubkeys: Vec<String>,
    },
}

#[derive(Args, Debug)]
pub struct MessageArgs {
    #[arg(short, long)]
    pub message: String,

    #[arg(short = 'e', long, value_enum)]
    pub message_encoding: Encoding,
}
