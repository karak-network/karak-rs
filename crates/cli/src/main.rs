use clap::Parser;
use karak_cli::root::{processor, Root};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Root::parse();

    processor::process(cli).await?;

    Ok(())
}
