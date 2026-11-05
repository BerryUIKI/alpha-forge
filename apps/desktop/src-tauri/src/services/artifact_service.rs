// Artifact service — persists validated artifacts produced by agent tasks.

use crate::artifacts::schema::ArtifactDraft;
use crate::database::repositories::artifact_repository::ArtifactRepository;
use crate::error::AppError;
use domain::artifact::Artifact;

/// Service for storing and listing artifacts.
#[derive(Clone)]
pub struct ArtifactService {
    repo: ArtifactRepository,
}

impl ArtifactService {
    pub fn new(repo: ArtifactRepository) -> Self {
        Self { repo }
    }

    /// Persists the artifacts produced by a completed task.
    pub async fn persist_task_artifacts(
        &self,
        workspace_id: &str,
        task_id: &str,
        drafts: &[ArtifactDraft],
    ) -> Result<Vec<Artifact>, AppError> {
        let mut artifacts = Vec::with_capacity(drafts.len());

        for draft in drafts {
            artifacts.push(
                self.repo
                    .create(
                        Some(task_id),
                        Some(workspace_id),
                        &draft.artifact_type,
                        &draft.title,
                        &draft.data,
                    )
                    .await?,
            );
        }

        Ok(artifacts)
    }

    /// Gets an artifact by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Artifact>, AppError> {
        self.repo.get(id).await
    }

    /// Lists artifacts, optionally filtered by workspace and task.
    pub async fn list(
        &self,
        workspace_id: Option<&str>,
        task_id: Option<&str>,
    ) -> Result<Vec<Artifact>, AppError> {
        self.repo.list(workspace_id, task_id).await
    }
}
