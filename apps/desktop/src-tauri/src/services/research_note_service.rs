use crate::database::repositories::research_note_repository::ResearchNoteRepository;
use crate::error::AppError;
use domain::research::{CreateNoteInput, ResearchNote};

pub struct ResearchNoteService {
    repo: ResearchNoteRepository,
}

impl ResearchNoteService {
    pub fn new(repo: ResearchNoteRepository) -> Self {
        Self { repo }
    }

    pub async fn create_note(&self, input: CreateNoteInput) -> Result<ResearchNote, AppError> {
        if input.content.trim().is_empty() {
            return Err(AppError::Validation(
                "Note content cannot be empty".to_string(),
            ));
        }
        self.repo.create(input).await
    }

    pub async fn list_notes(&self, document_id: &str) -> Result<Vec<ResearchNote>, AppError> {
        self.repo.list_by_document(document_id).await
    }

    pub async fn delete_note(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
