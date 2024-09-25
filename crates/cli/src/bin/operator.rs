use karak_cli::{components::Operator, Cli, Runner};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    Cli::new(Runner::Operator, "Karak Operator CLI", None)
        .with_configure()
        .with_component::<Operator>(None)
        .run()
        .await?;

    Ok(())
}
