// Research report service — handles report business logic.

use crate::database::repositories::research_report_repository::ResearchReportRepository;
use crate::error::AppError;
use domain::research::{CreateReportInput, ResearchReport};

pub struct ResearchReportService {
    repo: ResearchReportRepository,
}

impl ResearchReportService {
    pub fn new(repo: ResearchReportRepository) -> Self {
        Self { repo }
    }

    pub async fn create_report(
        &self,
        input: CreateReportInput,
    ) -> Result<ResearchReport, AppError> {
        if input.title.trim().is_empty() {
            return Err(AppError::Validation(
                "Report title cannot be empty".to_string(),
            ));
        }
        if input.content.trim().is_empty() {
            return Err(AppError::Validation(
                "Report content cannot be empty".to_string(),
            ));
        }
        self.repo.create(input).await
    }

    pub async fn get_report(&self, id: &str) -> Result<Option<ResearchReport>, AppError> {
        self.repo.get(id).await
    }
    pub async fn list_reports(&self, project_id: &str) -> Result<Vec<ResearchReport>, AppError> {
        self.repo.list_by_project(project_id).await
    }
    pub async fn delete_report(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
