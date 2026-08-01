// Artifact service — handles artifact business logic.

use crate::database::repositories::artifact_repository::ArtifactRepository;
use crate::error::AppError;
use domain::artifact::{Artifact, ArtifactStatus, CreateArtifactInput};

/// Service for managing artifacts.
pub struct ArtifactService {
    repo: ArtifactRepository,
}

impl ArtifactService {
    /// Creates a new artifact service.
    pub fn new(repo: ArtifactRepository) -> Self {
        Self { repo }
    }

    /// Creates a new artifact.
    pub async fn create_artifact(&self, input: CreateArtifactInput) -> Result<Artifact, AppError> {
        // Validate input
        if input.workspace_id.trim().is_empty() {
            return Err(AppError::Validation(
                "Workspace ID cannot be empty".to_string(),
            ));
        }

        // Create artifact via repository
        let artifact = self.repo.create(input).await?;

        Ok(artifact)
    }

    /// Gets an artifact by ID.
    pub async fn get_artifact(&self, id: &str) -> Result<Option<Artifact>, AppError> {
        if id.trim().is_empty() {
            return Err(AppError::Validation(
                "Artifact ID cannot be empty".to_string(),
            ));
        }

        self.repo.get(id).await
    }

    /// Lists all artifacts for a workspace.
    pub async fn list_artifacts(&self, workspace_id: &str) -> Result<Vec<Artifact>, AppError> {
        if workspace_id.trim().is_empty() {
            return Err(AppError::Validation(
                "Workspace ID cannot be empty".to_string(),
            ));
        }

        self.repo.list_by_workspace(workspace_id).await
    }

    /// Lists all artifacts for a task.
    pub async fn list_task_artifacts(&self, task_id: &str) -> Result<Vec<Artifact>, AppError> {
        if task_id.trim().is_empty() {
            return Err(AppError::Validation(
                "Task ID cannot be empty".to_string(),
            ));
        }

        self.repo.list_by_task(task_id).await
    }

    /// Starts artifact generation.
    pub async fn start_generation(&self, id: &str) -> Result<Artifact, AppError> {
        let artifact = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

        // Validate state transition
        if artifact.status != ArtifactStatus::Pending {
            return Err(AppError::Validation(format!(
                "Cannot start generation in '{}' state",
                artifact.status
            )));
        }

        // Update status
        self.repo.update_status(id, ArtifactStatus::Generating).await?;

        // Fetch updated artifact
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Artifact disappeared after update".to_string()))
    }

    /// Completes artifact generation.
    pub async fn complete_generation(
        &self,
        id: &str,
        output: serde_json::Value,
    ) -> Result<Artifact, AppError> {
        let artifact = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

        // Validate state transition
        if artifact.status != ArtifactStatus::Generating {
            return Err(AppError::Validation(format!(
                "Cannot complete generation in '{}' state",
                artifact.status
            )));
        }

        // Update output and status
        self.repo.update_output(id, output).await?;

        // Fetch updated artifact
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Artifact disappeared after update".to_string()))
    }

    /// Marks artifact as failed.
    pub async fn fail_generation(&self, id: &str, error: &str) -> Result<Artifact, AppError> {
        let artifact = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

        // Can fail from generating or pending state
        if artifact.status != ArtifactStatus::Generating
            && artifact.status != ArtifactStatus::Pending
        {
            return Err(AppError::Validation(format!(
                "Cannot fail generation in '{}' state",
                artifact.status
            )));
        }

        // Set error
        self.repo.set_error(id, error).await?;

        // Fetch updated artifact
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Artifact disappeared after update".to_string()))
    }

    /// Marks artifact as viewing.
    pub async fn start_viewing(&self, id: &str) -> Result<Artifact, AppError> {
        let artifact = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

        // Validate state transition
        if artifact.status != ArtifactStatus::Completed {
            return Err(AppError::Validation(format!(
                "Cannot view artifact in '{}' state",
                artifact.status
            )));
        }

        // Update status
        self.repo.update_status(id, ArtifactStatus::Viewing).await?;

        // Fetch updated artifact
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Artifact disappeared after update".to_string()))
    }

    /// Closes artifact.
    pub async fn close_artifact(&self, id: &str) -> Result<Artifact, AppError> {
        let artifact = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

        // Can close from viewing or completed state
        if artifact.status != ArtifactStatus::Viewing
            && artifact.status != ArtifactStatus::Completed
        {
            return Err(AppError::Validation(format!(
                "Cannot close artifact in '{}' state",
                artifact.status
            )));
        }

        // Update status
        self.repo.update_status(id, ArtifactStatus::Closed).await?;

        // Fetch updated artifact
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Artifact disappeared after update".to_string()))
    }

    /// Deletes an artifact.
    pub async fn delete_artifact(&self, id: &str) -> Result<(), AppError> {
        if id.trim().is_empty() {
            return Err(AppError::Validation(
                "Artifact ID cannot be empty".to_string(),
            ));
        }

        // Verify artifact exists
        let artifact = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

        // Only allow deletion from closed state
        if artifact.status != ArtifactStatus::Closed {
            return Err(AppError::Validation(
                "Can only delete artifacts in 'closed' state".to_string(),
            ));
        }

        self.repo.delete(id).await
    }
}
