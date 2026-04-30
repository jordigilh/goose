use goose::acp::server::{AcpProviderFactory, GooseAcpAgent};
use goose::agents::GoosePlatform;
use goose::config::base::CONFIG_YAML_NAME;
use goose::config::{Config, GooseMode};
use goose::providers::base::Provider;
use std::sync::Arc;

#[tokio::test]
async fn acp_agent_rejects_different_config_dir_after_global_initialization() {
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();
    let data = tempfile::tempdir().unwrap();

    let first_config = first.path().join(CONFIG_YAML_NAME);
    let second_config = second.path().join(CONFIG_YAML_NAME);

    let config = Config::init_global(first.path().to_path_buf()).unwrap();
    assert_eq!(config.path(), first_config.display().to_string());

    let provider_factory: AcpProviderFactory = Arc::new(|_, _, _| {
        Box::pin(async { Err::<Arc<dyn Provider>, _>(anyhow::anyhow!("provider not used")) })
    });

    let err = match GooseAcpAgent::new(
        provider_factory,
        Vec::new(),
        data.path().to_path_buf(),
        second.path().to_path_buf(),
        GooseMode::Auto,
        true,
        GoosePlatform::GooseCli,
    )
    .await
    {
        Ok(_) => panic!("expected config directory mismatch"),
        Err(err) => err,
    };

    let message = err.to_string();
    assert!(
        message.contains("Global config already initialized with writable path"),
        "{message}"
    );
    assert!(
        message.contains(&first_config.display().to_string()),
        "{message}"
    );
    assert!(
        message.contains(&second_config.display().to_string()),
        "{message}"
    );
}
