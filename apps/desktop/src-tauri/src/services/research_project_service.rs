// Research project service — handles research project business logic.

use crate::database::repositories::research_project_repository::ResearchProjectRepository;
use crate::error::AppError;
use domain::research::{CreateProjectInput, ProjectStatus, ResearchProject};

pub struct ResearchProjectService {
    repo: ResearchProjectRepository,
}

impl ResearchProjectService {
    pub fn new(repo: ResearchProjectRepository) -> Self {
        Self { repo }
    }

    pub async fn create_project(&self, input: CreateProjectInput) -> Result<ResearchProject, AppError> {
        if input.title.trim().is_empty() {
            return Err(AppError::Validation("Project title cannot be empty".to_string()));
        }
        self.repo.create(input).await
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<ResearchProject>, AppError> {
        self.repo.get(id).await
    }

    pub async fn list_projects(&self, workspace_id: &str) -> Result<Vec<ResearchProject>, AppError> {
        self.repo.list_by_workspace(workspace_id).await
    }

    pub async fn archive_project(&self, id: &str) -> Result<ResearchProject, AppError> {
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Project '{}' not found", id)))?;
        self.repo.update_status(id, ProjectStatus::Archived).await?;
        self.repo.get(id).await?.ok_or_else(|| AppError::Internal("Project disappeared".to_string()))
    }

    pub async fn complete_project(&self, id: &str) -> Result<ResearchProject, AppError> {
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Project '{}' not found", id)))?;
        self.repo.update_status(id, ProjectStatus::Completed).await?;
        self.repo.get(id).await?.ok_or_else(|| AppError::Internal("Project disappeared".to_string()))
    }

    pub async fn delete_project(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
