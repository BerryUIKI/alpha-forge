use crate::database::repositories::research_source_repository::ResearchSourceRepository;
use crate::error::AppError;
use crate::security::url_policy::normalize_research_url;
use domain::research::ResearchSource;

pub struct ResearchSourceService {
    repo: ResearchSourceRepository,
}

impl ResearchSourceService {
    pub fn new(repo: ResearchSourceRepository) -> Self {
        Self { repo }
    }

    pub async fn create_source(
        &self,
        document_id: String,
        url: Option<String>,
        title: Option<String>,
    ) -> Result<ResearchSource, AppError> {
        let url = url.ok_or_else(|| AppError::Validation("Source URL is required".to_string()))?;
        let normalized_url = normalize_research_url(&url)?;
        if title
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(AppError::Validation(
                "Source title cannot be empty when provided".to_string(),
            ));
        }
        self.repo
            .create(
                document_id,
                Some(normalized_url),
                title.map(|value| value.trim().to_string()),
            )
            .await
    }

    pub async fn list_sources(&self, document_id: &str) -> Result<Vec<ResearchSource>, AppError> {
        self.repo.list_by_document(document_id).await
    }
}
