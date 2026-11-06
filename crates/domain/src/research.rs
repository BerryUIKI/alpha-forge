// Research domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchDocument {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub content: Option<String>,
    pub kind: ResearchDocumentKind,
    pub format: ResearchDocumentFormat,
    pub summary: Option<String>,
    pub archived_at: Option<DateTime<Utc>>,
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

/// Where a research document came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResearchDocumentKind {
    #[default]
    Note,
    Web,
    Pdf,
    Agent,
}

impl ResearchDocumentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResearchDocumentKind::Note => "note",
            ResearchDocumentKind::Web => "web",
            ResearchDocumentKind::Pdf => "pdf",
            ResearchDocumentKind::Agent => "agent",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "web" => ResearchDocumentKind::Web,
            "pdf" => ResearchDocumentKind::Pdf,
            "agent" => ResearchDocumentKind::Agent,
            _ => ResearchDocumentKind::Note,
        }
    }
}

/// How a document body should be rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResearchDocumentFormat {
    #[default]
    Markdown,
    Text,
}

impl ResearchDocumentFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResearchDocumentFormat::Markdown => "markdown",
            ResearchDocumentFormat::Text => "text",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "text" => ResearchDocumentFormat::Text,
            _ => ResearchDocumentFormat::Markdown,
        }
    }
}

/// Input for creating a research document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResearchDocumentInput {
    pub workspace_id: String,
    pub title: String,
    pub content: Option<String>,
    #[serde(default)]
    pub kind: Option<ResearchDocumentKind>,
    #[serde(default)]
    pub format: Option<ResearchDocumentFormat>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// Input for updating a research document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResearchDocumentInput {
    pub title: String,
    pub content: Option<String>,
}
