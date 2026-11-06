// Tests for the artifact repository.

#[cfg(test)]
mod tests {
    use super::super::artifact_repository::ArtifactRepository;
    use domain::artifact::ArtifactStatus;

    async fn setup_repo() -> ArtifactRepository {
        let pool = crate::test_support::test_pool_with_workspace().await;

        sqlx::query(
            "INSERT INTO agent_tasks (id, workspace_id, title, status, created_at, updated_at)
             VALUES ('t1', 'w1', 'Research', 'completed', datetime('now'), datetime('now'))",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert task");

        ArtifactRepository::new(pool)
    }

    #[tokio::test]
    async fn test_create_and_list_artifact() {
        let repo = setup_repo().await;

        let created = repo
            .create(
                Some("t1"),
                Some("w1"),
                "comparison-table",
                "Tesla vs Nvidia",
                r#"{"columns":[],"rows":[]}"#,
            )
            .await
            .expect("artifact should be created");

        assert_eq!(created.artifact_type, "comparison-table");
        assert_eq!(created.title, "Tesla vs Nvidia");
        assert_eq!(created.status, ArtifactStatus::Completed);

        let artifacts = repo
            .list(Some("w1"), Some("t1"))
            .await
            .expect("artifacts should list");

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].id, created.id);
        assert_eq!(artifacts[0].workspace_id.as_deref(), Some("w1"));
        assert_eq!(artifacts[0].task_id.as_deref(), Some("t1"));
        assert_eq!(artifacts[0].data, r#"{"columns":[],"rows":[]}"#);
    }

    #[tokio::test]
    async fn test_get_artifact() {
        let repo = setup_repo().await;

        let created = repo
            .create(Some("t1"), Some("w1"), "timeline", "Timeline", "{}")
            .await
            .expect("artifact should be created");

        let fetched = repo
            .get(&created.id)
            .await
            .expect("artifact should load")
            .expect("artifact should exist");
        assert_eq!(fetched.title, "Timeline");
        assert_eq!(fetched.artifact_type, "timeline");

        assert!(repo
            .get("missing")
            .await
            .expect("lookup should succeed")
            .is_none());
    }

    #[tokio::test]
    async fn test_list_returns_all_when_filters_are_none() {
        let repo = setup_repo().await;

        repo.create(None, Some("w1"), "timeline", "Timeline", "{}")
            .await
            .expect("artifact should be created");
        repo.create(None, None, "timeline", "Global", "{}")
            .await
            .expect("artifact should be created");

        let artifacts = repo.list(None, None).await.expect("artifacts should list");

        assert_eq!(artifacts.len(), 2);
    }

    #[tokio::test]
    async fn test_list_filters_by_workspace() {
        let repo = setup_repo().await;

        repo.create(None, Some("w1"), "timeline", "Workspace artifact", "{}")
            .await
            .expect("artifact should be created");
        repo.create(None, None, "timeline", "Global artifact", "{}")
            .await
            .expect("artifact should be created");

        let artifacts = repo
            .list(Some("w1"), None)
            .await
            .expect("artifacts should list");

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].title, "Workspace artifact");
    }

    #[tokio::test]
    async fn test_unknown_status_returns_internal_error() {
        let repo = setup_repo().await;

        sqlx::query(
            "INSERT INTO artifacts (id, artifact_type, title, input, status)
             VALUES ('a1', 'timeline', 'Timeline', '{}', 'mystery')",
        )
        .execute(repo.pool())
        .await
        .expect("Failed to insert artifact");

        let result = repo.get("a1").await;

        assert!(matches!(result, Err(crate::error::AppError::Internal(_))));
    }
}
