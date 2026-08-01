use crate::database::repositories::research_source_repository::ResearchSourceRepository;
use crate::error::AppError;
use domain::research::ResearchSource;

pub struct ResearchSourceService {
    repo: ResearchSourceRepository,
}

impl ResearchSourceService {
    pub fn new(repo: ResearchSourceRepository) -> Self { Self { repo } }

    pub async fn create_source(&self, document_id: String, url: Option<String>, title: Option<String>) -> Result<ResearchSource, AppError> {
        if let Some(value) = &url {
            let normalized = value.trim();
            if !(normalized.starts_with("https://") || normalized.starts_with("http://")) {
                return Err(AppError::Validation("Source URL must use HTTP or HTTPS".to_string()));
            }
        }
        if title.as_deref().is_some_and(|value| value.trim().is_empty()) {
            return Err(AppError::Validation("Source title cannot be empty when provided".to_string()));
        }
        self.repo.create(document_id, url.map(|value| value.trim().to_string()), title.map(|value| value.trim().to_string())).await
    }

    pub async fn list_sources(&self, document_id: &str) -> Result<Vec<ResearchSource>, AppError> {
        self.repo.list_by_document(document_id).await
    }
}
