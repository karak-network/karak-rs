use karak_cli::{components::Dss, Cli, Runner};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    Cli::new(Runner::Dss, "Karak DSS CLI", None)
        .with_configure()
        .with_component::<Dss>(None)
        .run()
        .await?;

    Ok(())
}
