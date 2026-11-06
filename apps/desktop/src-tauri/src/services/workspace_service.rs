// Workspace service — handles workspace business logic.

use uuid::Uuid;

use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::error::AppError;
use crate::text::required_text;
use domain::workspace::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};

const MAX_ID_CHARS: usize = 100;
const MAX_NAME_CHARS: usize = 200;

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
        let name = required_text(&input.name, "Workspace name", MAX_NAME_CHARS)?;
        let id = Uuid::new_v4().to_string();

        self.repo.create(&id, &name).await
    }

    /// Lists all workspaces.
    pub async fn list(&self) -> Result<Vec<Workspace>, AppError> {
        self.repo.list().await
    }

    /// Gets a workspace by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        let id = required_text(id, "Workspace ID", MAX_ID_CHARS)?;

        self.repo.get(&id).await
    }

    /// Updates a workspace.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateWorkspaceInput,
    ) -> Result<Workspace, AppError> {
        let id = required_text(id, "Workspace ID", MAX_ID_CHARS)?;
        let name = required_text(&input.name, "Workspace name", MAX_NAME_CHARS)?;

        self.repo.update(&id, &name).await
    }

    /// Deletes a workspace.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let id = required_text(id, "Workspace ID", MAX_ID_CHARS)?;

        self.repo.delete(&id).await
    }
}
