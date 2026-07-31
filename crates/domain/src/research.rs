// Research domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchDocument {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub id: String,
    pub document_id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub retrieved_at: Option<DateTime<Utc>>,
}
