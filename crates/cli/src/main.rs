use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};

use crate::root::{processor, Root};

#[cfg(feature = "bls")]
pub mod bls;
pub mod config;
pub mod constants;
pub mod keypair;
mod model;
pub mod operator;
pub mod prompter;
pub mod root;
pub mod shared;
mod types;
pub mod util;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Root::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = Root::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else if cli.command.is_none() {
        let mut cmd = Root::command();
        cmd.print_help().expect("Failed to print help");
    }

    processor::process(cli).await?;

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
