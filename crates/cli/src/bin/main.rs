use karak_cli::{
    components::{Dss, Operator},
    Cli, Runner,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    Cli::new(Runner::Karak, "Karak CLI", None)
        .with_configure()
        .with_component::<Dss>(Some("dss"))
        .with_component::<Operator>(Some("operator"))
        .run()
        .await?;

    Ok(())
}
