// Research domain models for M4 Research Workspace.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Research project - container for research work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProject {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Active,
    Archived,
    Completed,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::Archived => write!(f, "archived"),
            ProjectStatus::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectInput {
    pub workspace_id: String,
    pub title: String,
    pub description: Option<String>,
}

/// Research document - PDFs, web pages, notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchDocument {
    pub id: String,
    pub project_id: String,
    pub document_type: DocumentType,
    pub title: String,
    pub content: Option<String>,
    pub source_url: Option<String>,
    pub file_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    Pdf,
    WebPage,
    Note,
    Report,
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::Pdf => write!(f, "pdf"),
            DocumentType::WebPage => write!(f, "web_page"),
            DocumentType::Note => write!(f, "note"),
            DocumentType::Report => write!(f, "report"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentInput {
    pub project_id: String,
    pub document_type: DocumentType,
    pub title: String,
    pub content: Option<String>,
    pub source_url: Option<String>,
    pub file_path: Option<String>,
}

/// Research source - external references with provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub id: String,
    pub document_id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub retrieved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Research note - user annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchNote {
    pub id: String,
    pub document_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteInput {
    pub document_id: String,
    pub content: String,
}

/// Research report - generated outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub report_type: ReportType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    Analysis,
    Summary,
    Thesis,
    Recommendation,
}

impl std::fmt::Display for ReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportType::Analysis => write!(f, "analysis"),
            ReportType::Summary => write!(f, "summary"),
            ReportType::Thesis => write!(f, "thesis"),
            ReportType::Recommendation => write!(f, "recommendation"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReportInput {
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub report_type: ReportType,
}
