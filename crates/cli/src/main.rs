pub mod bls;
pub mod keypair;
pub mod root;
pub mod shared;

use clap::Parser;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = crate::root::Root::parse();

    crate::root::processor::process(cli).await?;

    Ok(())
}
