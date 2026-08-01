// Workspace service — handles workspace business logic.

use uuid::Uuid;

use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::error::AppError;
use domain::workspace::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};

/// Service for managing workspaces.
pub struct WorkspaceService {
    repo: WorkspaceRepository,
}

impl WorkspaceService {
    /// Creates a new workspace service.
    pub fn new(repo: WorkspaceRepository) -> Self {
        Self { repo }
    }

    /// Creates a new workspace.
    pub async fn create(&self, input: CreateWorkspaceInput) -> Result<Workspace, AppError> {
        // Validate workspace name
        let name = input.name.trim();
        if name.is_empty() {
            return Err(AppError::Validation(
                "Workspace name cannot be empty".to_string(),
            ));
        }

        if name.len() > 200 {
            return Err(AppError::Validation(
                "Workspace name cannot exceed 200 characters".to_string(),
            ));
        }

        // Generate UUID
        let id = Uuid::new_v4().to_string();

        // Create workspace via repository
        self.repo.create(&id, name).await
    }

    /// Lists all workspaces.
    pub async fn list(&self) -> Result<Vec<Workspace>, AppError> {
        self.repo.list().await
    }

    /// Gets a workspace by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        if id.trim().is_empty() {
            return Err(AppError::Validation(
                "Workspace ID cannot be empty".to_string(),
            ));
        }

        self.repo.get(id).await
    }

    /// Updates a workspace.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateWorkspaceInput,
    ) -> Result<Workspace, AppError> {
        // Validate ID
        if id.trim().is_empty() {
            return Err(AppError::Validation(
                "Workspace ID cannot be empty".to_string(),
            ));
        }

        // Validate name if provided
        let name = input
            .name
            .as_ref()
            .map(|n| n.trim())
            .filter(|n| !n.is_empty());

        let name = match name {
            Some(n) if n.len() > 200 => {
                return Err(AppError::Validation(
                    "Workspace name cannot exceed 200 characters".to_string(),
                ));
            }
            Some(n) => n.to_string(),
            None => {
                return Err(AppError::Validation(
                    "Workspace name must be provided".to_string(),
                ));
            }
        };

        self.repo.update(id, &name).await
    }

    /// Deletes a workspace.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        if id.trim().is_empty() {
            return Err(AppError::Validation(
                "Workspace ID cannot be empty".to_string(),
            ));
        }

        self.repo.delete(id).await
    }
}
