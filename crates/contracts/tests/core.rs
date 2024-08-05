#[cfg(test)]
mod core_tests {
    use alloy::{
        providers::{layers::AnvilProvider, ProviderBuilder, RootProvider},
        transports::http::{Client, Http},
    };
    use eyre::Result;
    use karak_contracts::Core::{self, CoreInstance};
    use tokio::sync::OnceCell;

    type Hc = Http<Client>;
    type Ap = AnvilProvider<RootProvider<Hc>, Hc>;
    type AnvilCoreInstance = CoreInstance<Hc, Ap>;

    static PROVIDER: OnceCell<Ap> = OnceCell::const_new();
    static CORE: OnceCell<AnvilCoreInstance> = OnceCell::const_new();

    fn setup_anvil_provider() -> Ap {
        ProviderBuilder::new()
            .on_anvil_with_config(|anvil| anvil.args(["--code-size-limit", "1000000"]))
    }

    async fn deploy_core() -> Result<AnvilCoreInstance> {
        let provider = PROVIDER
            .get_or_init(|| async { setup_anvil_provider() })
            .await
            .clone();

        let core = Core::deploy(provider).await?;
        Ok(core)
    }

    #[tokio::test]
    async fn test_core_version() -> Result<()> {
        let core = CORE.get_or_try_init(deploy_core).await?;
        let version = core.VERSION().call().await?;

        assert_eq!(version._0, "2.0.0");
        Ok(())
    }
}
