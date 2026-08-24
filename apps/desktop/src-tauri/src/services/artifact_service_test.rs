use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::database::repositories::artifact_repository::ArtifactRepository;
use crate::error::AppError;
use crate::services::artifact_service::ArtifactService;
use domain::artifact::{ArtifactStatus, ArtifactType, CreateArtifactInput};

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("Failed to create test database");

    sqlx::query(include_str!("../../migrations/0001_initial.sql"))
        .execute(&pool)
        .await
        .expect("Failed to run initial migration");

    sqlx::query(include_str!(
        "../../migrations/0002_agent_tasks_enhancement.sql"
    ))
    .execute(&pool)
    .await
    .expect("Failed to run agent tasks migration");

    sqlx::query(include_str!(
        "../../migrations/0003_fix_status_constraint.sql"
    ))
    .execute(&pool)
    .await
    .expect("Failed to run status constraint fix");

    sqlx::query(include_str!("../../migrations/0004_enhance_artifacts.sql"))
        .execute(&pool)
        .await
        .expect("Failed to run artifacts enhancement migration");

    pool
}

async fn create_test_workspace(pool: &SqlitePool) -> String {
    let workspace_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?, 'Test Workspace', datetime('now'), datetime('now'))"
    )
    .bind(&workspace_id)
    .execute(pool)
    .await
    .expect("Failed to create test workspace");
    workspace_id
}

#[tokio::test]
async fn test_artifact_service_lifecycle() {
    let pool = setup_test_db().await;
    let workspace_id = create_test_workspace(&pool).await;
    let service = ArtifactService::new(ArtifactRepository::new(pool));

    // 1. Validation on empty workspace_id
    let invalid_input = CreateArtifactInput {
        workspace_id: "".to_string(),
        task_id: None,
        artifact_type: ArtifactType::ComparisonTable,
        input: serde_json::json!({"test": "data"}),
    };
    let err = service.create_artifact(invalid_input).await.unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));

    // 2. Create artifact
    let input = CreateArtifactInput {
        workspace_id: workspace_id.clone(),
        task_id: None,
        artifact_type: ArtifactType::ComparisonTable,
        input: serde_json::json!({"companies": ["AAPL", "MSFT"]}),
    };
    let artifact = service.create_artifact(input).await.unwrap();
    assert_eq!(artifact.status, ArtifactStatus::Pending);
    assert_eq!(artifact.artifact_type, ArtifactType::ComparisonTable);

    // 3. Start generation -> Generating
    let generating = service.start_generation(&artifact.id).await.unwrap();
    assert_eq!(generating.status, ArtifactStatus::Generating);

    // 4. Invalid start from Generating state
    let err = service.start_generation(&artifact.id).await.unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));

    // 5. Complete generation -> Completed
    let output = serde_json::json!({"metrics": {"AAPL": 100, "MSFT": 120}});
    let completed = service
        .complete_generation(&artifact.id, output.clone())
        .await
        .unwrap();
    assert_eq!(completed.status, ArtifactStatus::Completed);
    assert_eq!(completed.output, Some(output));

    // 6. Invalid complete from Completed state
    let err = service
        .complete_generation(&artifact.id, serde_json::json!({}))
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));

    // 7. Viewing lifecycle
    let viewing = service.start_viewing(&artifact.id).await.unwrap();
    assert_eq!(viewing.status, ArtifactStatus::Viewing);
    let closed = service.close_artifact(&artifact.id).await.unwrap();
    assert_eq!(closed.status, ArtifactStatus::Closed);

    // 8. List by workspace
    let list = service.list_artifacts(&workspace_id).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, artifact.id);
}

#[tokio::test]
async fn test_artifact_service_failure_flow() {
    let pool = setup_test_db().await;
    let workspace_id = create_test_workspace(&pool).await;
    let service = ArtifactService::new(ArtifactRepository::new(pool));

    let input = CreateArtifactInput {
        workspace_id: workspace_id.clone(),
        task_id: None,
        artifact_type: ArtifactType::ValuationModel,
        input: serde_json::json!({}),
    };
    let artifact = service.create_artifact(input).await.unwrap();

    let failed = service
        .fail_generation(&artifact.id, "Generation timeout")
        .await
        .unwrap();
    assert_eq!(failed.status, ArtifactStatus::Failed);
    assert_eq!(failed.error, Some("Generation timeout".to_string()));
}
