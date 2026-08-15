use sqlx::sqlite::SqlitePoolOptions;

use crate::database::repositories::plugin_repository::PluginRepository;
use crate::error::AppError;
use crate::services::plugin_service::PluginService;

async fn service() -> PluginService {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("test database should connect");
    sqlx::raw_sql(include_str!("../../migrations/0001_initial.sql"))
        .execute(&pool)
        .await
        .expect("initial schema should apply");
    sqlx::raw_sql(include_str!("../../migrations/0012_plugin_registry.sql"))
        .execute(&pool)
        .await
        .expect("plugin registry schema should apply");
    let service = PluginService::new(PluginRepository::new(pool));
    service
        .sync_bundled_plugins()
        .await
        .expect("bundled plugins should synchronize");
    service
}

#[tokio::test]
async fn prepares_only_enabled_plugins_with_valid_payloads() {
    let service = service().await;
    let payload = serde_json::json!({
        "companies": [
            {"ticker": "AAA", "name": "Alpha", "metrics": {"revenue": 10}},
            {"ticker": "BBB", "name": "Beta", "metrics": {"revenue": 20}}
        ],
        "comparisonDimensions": ["revenue"]
    });

    let request = service
        .prepare_artifact("company-comparison", payload.clone())
        .await
        .expect("valid enabled plugin should prepare an artifact");
    assert_eq!(request.artifact_type.to_string(), "comparison_table");
    assert_eq!(request.payload, payload);

    service
        .set_enabled("company-comparison", false)
        .await
        .expect("plugin should be disabled");
    let error = service
        .prepare_artifact("company-comparison", payload)
        .await
        .expect_err("disabled plugin must not prepare an artifact");
    assert!(matches!(error, AppError::PermissionDenied(_)));
}
