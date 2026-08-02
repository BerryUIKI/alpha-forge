// Thesis service — handles investment thesis business logic.

use crate::database::repositories::thesis_repository::ThesisRepository;
use crate::error::AppError;
use domain::thesis::{
    AddEvidenceInput, CreateThesisInput, InvestmentThesis, ThesisConfidenceSnapshot,
    ThesisEvidence, ThesisStatus,
    UpdateConfidenceInput,
};

pub struct ThesisService {
    repo: ThesisRepository,
}

impl ThesisService {
    pub fn new(repo: ThesisRepository) -> Self {
        Self { repo }
    }

    /// Create a new investment thesis.
    pub async fn create_thesis(
        &self,
        input: CreateThesisInput,
    ) -> Result<InvestmentThesis, AppError> {
        // Validate input
        if input.title.trim().is_empty() {
            return Err(AppError::Validation("Thesis title cannot be empty".to_string()));
        }
        if input.thesis.trim().is_empty() {
            return Err(AppError::Validation("Thesis content cannot be empty".to_string()));
        }

        // Validate confidence if provided
        if let Some(confidence) = input.confidence {
            if !(0..=100).contains(&confidence) {
                return Err(AppError::Validation(
                    "Confidence must be between 0 and 100".to_string(),
                ));
            }
        }

        self.repo.create_thesis(input).await
    }

    /// Get a thesis by ID.
    pub async fn get_thesis(&self, id: &str) -> Result<Option<InvestmentThesis>, AppError> {
        self.repo.get_thesis(id).await
    }

    /// List all theses for a workspace.
    pub async fn list_theses(&self, workspace_id: &str) -> Result<Vec<InvestmentThesis>, AppError> {
        self.repo.list_by_workspace(workspace_id).await
    }

    /// Activate a thesis for active tracking.
    pub async fn activate_thesis(&self, id: &str) -> Result<InvestmentThesis, AppError> {
        let thesis = self
            .repo
            .get_thesis(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Thesis '{}' not found", id)))?;

        if thesis.status != ThesisStatus::Draft {
            return Err(AppError::Validation(
                "Only draft theses can be activated".to_string(),
            ));
        }

        self.repo.update_status(id, ThesisStatus::Active).await?;
        self.repo.get_thesis(id).await?.ok_or_else(|| {
            AppError::Internal("Thesis disappeared after update".to_string())
        })
    }

    /// Start validation of a thesis.
    pub async fn start_validation(&self, id: &str) -> Result<InvestmentThesis, AppError> {
        let thesis = self
            .repo
            .get_thesis(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Thesis '{}' not found", id)))?;

        if thesis.status != ThesisStatus::Active {
            return Err(AppError::Validation(
                "Only active theses can be validated".to_string(),
            ));
        }

        self.repo.update_status(id, ThesisStatus::Validating).await?;
        self.repo.get_thesis(id).await?.ok_or_else(|| {
            AppError::Internal("Thesis disappeared after update".to_string())
        })
    }

    /// Complete validation with an outcome.
    pub async fn complete_validation(
        &self,
        id: &str,
        outcome: String,
        validated: bool,
    ) -> Result<InvestmentThesis, AppError> {
        let thesis = self
            .repo
            .get_thesis(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Thesis '{}' not found", id)))?;

        if thesis.status != ThesisStatus::Validating {
            return Err(AppError::Validation(
                "Only theses under validation can be completed".to_string(),
            ));
        }

        let status = if validated {
            ThesisStatus::Validated
        } else {
            ThesisStatus::Closed
        };

        self.repo.record_outcome(id, outcome, status).await?;
        self.repo.get_thesis(id).await?.ok_or_else(|| {
            AppError::Internal("Thesis disappeared after update".to_string())
        })
    }

    /// Update thesis confidence.
    pub async fn update_confidence(
        &self,
        input: UpdateConfidenceInput,
    ) -> Result<InvestmentThesis, AppError> {
        let thesis = self
            .repo
            .get_thesis(&input.thesis_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Thesis '{}' not found", input.thesis_id))
            })?;

        if thesis.status == ThesisStatus::Closed {
            return Err(AppError::Validation(
                "Cannot update confidence of a closed thesis".to_string(),
            ));
        }

        self.repo.update_confidence(input.clone()).await?;
        self.repo.get_thesis(&input.thesis_id).await?.ok_or_else(|| {
            AppError::Internal("Thesis disappeared after update".to_string())
        })
    }

    /// Close a thesis.
    pub async fn close_thesis(&self, id: &str) -> Result<InvestmentThesis, AppError> {
        let thesis = self
            .repo
            .get_thesis(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Thesis '{}' not found", id)))?;

        if thesis.status == ThesisStatus::Closed {
            return Err(AppError::Validation("Thesis is already closed".to_string()));
        }

        self.repo.update_status(id, ThesisStatus::Closed).await?;
        self.repo.get_thesis(id).await?.ok_or_else(|| {
            AppError::Internal("Thesis disappeared after update".to_string())
        })
    }

    /// Delete a thesis.
    pub async fn delete_thesis(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete_thesis(id).await
    }

    /// Add evidence to a thesis.
    pub async fn add_evidence(
        &self,
        input: AddEvidenceInput,
    ) -> Result<ThesisEvidence, AppError> {
        let thesis = self
            .repo
            .get_thesis(&input.thesis_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Thesis '{}' not found", input.thesis_id))
            })?;

        if thesis.status == ThesisStatus::Closed {
            return Err(AppError::Validation(
                "Cannot add evidence to a closed thesis".to_string(),
            ));
        }

        if input.evidence.trim().is_empty() {
            return Err(AppError::Validation("Evidence cannot be empty".to_string()));
        }

        self.repo.add_evidence(input).await
    }

    /// List all evidence for a thesis.
    pub async fn list_evidence(&self, thesis_id: &str) -> Result<Vec<ThesisEvidence>, AppError> {
        self.repo.list_evidence(thesis_id).await
    }

    /// Remove evidence from a thesis.
    pub async fn delete_evidence(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete_evidence(id).await
    }

    /// Retrieve immutable confidence review history for a thesis.
    pub async fn list_confidence_history(
        &self,
        thesis_id: &str,
    ) -> Result<Vec<ThesisConfidenceSnapshot>, AppError> {
        self.repo.list_confidence_history(thesis_id).await
    }
}
