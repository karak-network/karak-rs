pub mod processor;

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

use crate::{bls::BLS, config::Config, keypair::Keypair, operator::OperatorArgs};

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Parser)]
#[command(version, about = "Karak CLI", long_about = None, styles = CLAP_STYLING)]
#[command(propagate_version = true)]
pub enum Root {
    /// Keypair management
    #[command(subcommand)]
    Keypair(Keypair),
    /// BLS operations
    #[command(subcommand)]
    BLS(BLS),

    /// Operator management
    #[command()]
    Operator(OperatorArgs),

    /// Config management
    #[command(subcommand)]
    Config(Config),
}
