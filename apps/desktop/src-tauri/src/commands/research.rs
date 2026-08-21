// Research project Tauri commands — M4 Research Workspace.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::documents::pdf_import::select_and_extract_pdf;
use crate::documents::web_import::fetch_web_page;
use crate::error::AppError;
use domain::research::{
    CreateDocumentInput, CreateNoteInput, CreateProjectInput, CreateReportInput, DocumentType,
    ProjectStatus, ReportType, ResearchDocument, ResearchNote, ResearchProject, ResearchReport,
    ResearchSearchMatch, ResearchSource,
};

/// DTO for ResearchProject with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchProjectDto {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ResearchProject> for ResearchProjectDto {
    fn from(project: ResearchProject) -> Self {
        Self {
            id: project.id,
            workspace_id: project.workspace_id,
            title: project.title,
            description: project.description,
            status: project.status,
            created_at: project.created_at.to_rfc3339(),
            updated_at: project.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for ResearchDocument with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchDocumentDto {
    pub id: String,
    pub project_id: String,
    pub document_type: DocumentType,
    pub title: String,
    pub content: Option<String>,
    pub source_url: Option<String>,
    pub file_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ResearchDocument> for ResearchDocumentDto {
    fn from(doc: ResearchDocument) -> Self {
        Self {
            id: doc.id,
            project_id: doc.project_id,
            document_type: doc.document_type,
            title: doc.title,
            content: doc.content,
            source_url: doc.source_url,
            file_path: doc.file_path,
            created_at: doc.created_at.to_rfc3339(),
            updated_at: doc.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for ResearchSource with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSourceDto {
    pub id: String,
    pub document_id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub retrieved_at: Option<String>,
    pub created_at: String,
}

impl From<ResearchSource> for ResearchSourceDto {
    fn from(source: ResearchSource) -> Self {
        Self {
            id: source.id,
            document_id: source.document_id,
            url: source.url,
            title: source.title,
            retrieved_at: source.retrieved_at.map(|dt| dt.to_rfc3339()),
            created_at: source.created_at.to_rfc3339(),
        }
    }
}

/// DTO for ResearchNote with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchNoteDto {
    pub id: String,
    pub document_id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ResearchNote> for ResearchNoteDto {
    fn from(note: ResearchNote) -> Self {
        Self {
            id: note.id,
            document_id: note.document_id,
            content: note.content,
            created_at: note.created_at.to_rfc3339(),
            updated_at: note.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for ResearchReport with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchReportDto {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub report_type: ReportType,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ResearchReport> for ResearchReportDto {
    fn from(report: ResearchReport) -> Self {
        Self {
            id: report.id,
            project_id: report.project_id,
            title: report.title,
            content: report.content,
            report_type: report.report_type,
            created_at: report.created_at.to_rfc3339(),
            updated_at: report.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for ResearchSearchMatch with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSearchMatchDto {
    pub ordinal: usize,
    pub content: String,
    pub score: usize,
}

impl From<ResearchSearchMatch> for ResearchSearchMatchDto {
    fn from(m: ResearchSearchMatch) -> Self {
        Self {
            ordinal: m.ordinal,
            content: m.content,
            score: m.score,
        }
    }
}

// Project commands
#[tauri::command]
pub async fn create_research_project(
    workspace_id: String,
    title: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<ResearchProjectDto, AppError> {
    state
        .research_project_service
        .create_project(CreateProjectInput {
            workspace_id,
            title,
            description,
        })
        .await
        .map(ResearchProjectDto::from)
}

#[tauri::command]
pub async fn get_research_project(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<ResearchProjectDto>, AppError> {
    state
        .research_project_service
        .get_project(&id)
        .await
        .map(|opt| opt.map(ResearchProjectDto::from))
}

#[tauri::command]
pub async fn list_research_projects(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchProjectDto>, AppError> {
    state
        .research_project_service
        .list_projects(&workspace_id)
        .await
        .map(|projects| projects.into_iter().map(ResearchProjectDto::from).collect())
}

#[tauri::command]
pub async fn archive_research_project(
    id: String,
    state: State<'_, AppState>,
) -> Result<ResearchProjectDto, AppError> {
    state
        .research_project_service
        .archive_project(&id)
        .await
        .map(ResearchProjectDto::from)
}

#[tauri::command]
pub async fn complete_research_project(
    id: String,
    state: State<'_, AppState>,
) -> Result<ResearchProjectDto, AppError> {
    state
        .research_project_service
        .complete_project(&id)
        .await
        .map(ResearchProjectDto::from)
}

#[tauri::command]
pub async fn delete_research_project(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.research_project_service.delete_project(&id).await
}

// Document commands
#[tauri::command]
pub async fn create_research_document(
    project_id: String,
    document_type: String,
    title: String,
    content: Option<String>,
    source_url: Option<String>,
    file_path: Option<String>,
    state: State<'_, AppState>,
) -> Result<ResearchDocumentDto, AppError> {
    let doc_type = match document_type.as_str() {
        "pdf" => DocumentType::Pdf,
        "web_page" => DocumentType::WebPage,
        "note" => DocumentType::Note,
        "report" => DocumentType::Report,
        _ => DocumentType::Note,
    };
    state
        .research_document_service
        .create_document(CreateDocumentInput {
            project_id,
            document_type: doc_type,
            title,
            content,
            source_url,
            file_path,
        })
        .await
        .map(ResearchDocumentDto::from)
}

#[tauri::command]
pub async fn get_research_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<ResearchDocumentDto>, AppError> {
    state
        .research_document_service
        .get_document(&id)
        .await
        .map(|opt| opt.map(ResearchDocumentDto::from))
}

#[tauri::command]
pub async fn list_research_documents(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchDocumentDto>, AppError> {
    state
        .research_document_service
        .list_documents(&project_id)
        .await
        .map(|docs| docs.into_iter().map(ResearchDocumentDto::from).collect())
}

#[tauri::command]
pub async fn delete_research_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.research_document_service.delete_document(&id).await
}

#[tauri::command]
pub async fn import_research_pdf(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Option<ResearchDocumentDto>, AppError> {
    let Some(pdf) = select_and_extract_pdf().await? else {
        return Ok(None);
    };
    state
        .research_document_service
        .create_document(CreateDocumentInput {
            project_id,
            document_type: DocumentType::Pdf,
            title: pdf.title,
            content: Some(pdf.content),
            source_url: None,
            file_path: None,
        })
        .await
        .map(|doc| Some(ResearchDocumentDto::from(doc)))
}

#[tauri::command]
pub async fn import_research_web_page(
    project_id: String,
    url: String,
    state: State<'_, AppState>,
) -> Result<ResearchDocumentDto, AppError> {
    let page = fetch_web_page(&url).await?;
    let document = state
        .research_document_service
        .create_document(CreateDocumentInput {
            project_id,
            document_type: DocumentType::WebPage,
            title: page.title.clone(),
            content: Some(page.content),
            source_url: Some(page.url.clone()),
            file_path: None,
        })
        .await?;
    state
        .research_source_service
        .create_source(document.id.clone(), Some(page.url), Some(page.title))
        .await?;
    Ok(ResearchDocumentDto::from(document))
}

#[tauri::command]
pub async fn search_research_document(
    id: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchSearchMatchDto>, AppError> {
    state
        .research_document_service
        .search_document(&id, &query)
        .await
        .map(|matches| {
            matches
                .into_iter()
                .map(ResearchSearchMatchDto::from)
                .collect()
        })
}

#[tauri::command]
pub async fn semantic_search_research_document(
    id: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchSearchMatchDto>, AppError> {
    state
        .research_document_service
        .semantic_search_document(&id, &query)
        .await
        .map(|matches| {
            matches
                .into_iter()
                .map(ResearchSearchMatchDto::from)
                .collect()
        })
}

#[tauri::command]
pub async fn create_research_source(
    document_id: String,
    url: Option<String>,
    title: Option<String>,
    state: State<'_, AppState>,
) -> Result<ResearchSourceDto, AppError> {
    state
        .research_source_service
        .create_source(document_id, url, title)
        .await
        .map(ResearchSourceDto::from)
}

#[tauri::command]
pub async fn list_research_sources(
    document_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchSourceDto>, AppError> {
    state
        .research_source_service
        .list_sources(&document_id)
        .await
        .map(|sources| sources.into_iter().map(ResearchSourceDto::from).collect())
}

#[tauri::command]
pub async fn create_research_note(
    document_id: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<ResearchNoteDto, AppError> {
    state
        .research_note_service
        .create_note(CreateNoteInput {
            document_id,
            content,
        })
        .await
        .map(ResearchNoteDto::from)
}

#[tauri::command]
pub async fn list_research_notes(
    document_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchNoteDto>, AppError> {
    state
        .research_note_service
        .list_notes(&document_id)
        .await
        .map(|notes| notes.into_iter().map(ResearchNoteDto::from).collect())
}

#[tauri::command]
pub async fn delete_research_note(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_note_service.delete_note(&id).await
}

// Report commands
#[tauri::command]
pub async fn create_research_report(
    project_id: String,
    title: String,
    content: String,
    report_type: String,
    state: State<'_, AppState>,
) -> Result<ResearchReportDto, AppError> {
    let r_type = match report_type.as_str() {
        "analysis" => ReportType::Analysis,
        "summary" => ReportType::Summary,
        "thesis" => ReportType::Thesis,
        "recommendation" => ReportType::Recommendation,
        _ => ReportType::Analysis,
    };
    state
        .research_report_service
        .create_report(CreateReportInput {
            project_id,
            title,
            content,
            report_type: r_type,
        })
        .await
        .map(ResearchReportDto::from)
}

#[tauri::command]
pub async fn get_research_report(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<ResearchReportDto>, AppError> {
    state
        .research_report_service
        .get_report(&id)
        .await
        .map(|opt| opt.map(ResearchReportDto::from))
}

#[tauri::command]
pub async fn list_research_reports(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchReportDto>, AppError> {
    state
        .research_report_service
        .list_reports(&project_id)
        .await
        .map(|reports| reports.into_iter().map(ResearchReportDto::from).collect())
}

#[tauri::command]
pub async fn delete_research_report(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.research_report_service.delete_report(&id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_research_dtos_camel_case_serialization() {
        let now = Utc::now();
        let project = ResearchProject {
            id: "proj-1".to_string(),
            workspace_id: "ws-1".to_string(),
            title: "Semiconductors".to_string(),
            description: Some("Overview".to_string()),
            status: ProjectStatus::Active,
            created_at: now,
            updated_at: now,
        };
        let project_dto = ResearchProjectDto::from(project);
        let project_json = serde_json::to_string(&project_dto).expect("project serialization");
        assert!(project_json.contains("\"workspaceId\":\"ws-1\""));
        assert!(project_json.contains("\"createdAt\":"));
        assert!(!project_json.contains("\"workspace_id\":"));

        let document = ResearchDocument {
            id: "doc-1".to_string(),
            project_id: "proj-1".to_string(),
            document_type: DocumentType::Pdf,
            title: "Report.pdf".to_string(),
            content: Some("Text".to_string()),
            source_url: Some("https://example.com".to_string()),
            file_path: Some("/tmp/report.pdf".to_string()),
            created_at: now,
            updated_at: now,
        };
        let document_dto = ResearchDocumentDto::from(document);
        let document_json = serde_json::to_string(&document_dto).expect("doc serialization");
        assert!(document_json.contains("\"projectId\":\"proj-1\""));
        assert!(document_json.contains("\"documentType\":\"pdf\""));
        assert!(document_json.contains("\"sourceUrl\":\"https://example.com\""));
        assert!(document_json.contains("\"filePath\":\"/tmp/report.pdf\""));
        assert!(!document_json.contains("\"project_id\":"));

        let source = ResearchSource {
            id: "src-1".to_string(),
            document_id: "doc-1".to_string(),
            url: Some("https://example.com".to_string()),
            title: Some("Example".to_string()),
            retrieved_at: Some(now),
            created_at: now,
        };
        let source_dto = ResearchSourceDto::from(source);
        let source_json = serde_json::to_string(&source_dto).expect("source serialization");
        assert!(source_json.contains("\"documentId\":\"doc-1\""));
        assert!(source_json.contains("\"retrievedAt\":"));
        assert!(!source_json.contains("\"document_id\":"));

        let note = ResearchNote {
            id: "note-1".to_string(),
            document_id: "doc-1".to_string(),
            content: "Note text".to_string(),
            created_at: now,
            updated_at: now,
        };
        let note_dto = ResearchNoteDto::from(note);
        let note_json = serde_json::to_string(&note_dto).expect("note serialization");
        assert!(note_json.contains("\"documentId\":\"doc-1\""));
        assert!(!note_json.contains("\"document_id\":"));

        let report = ResearchReport {
            id: "rep-1".to_string(),
            project_id: "proj-1".to_string(),
            title: "Analysis".to_string(),
            content: "Content".to_string(),
            report_type: ReportType::Analysis,
            created_at: now,
            updated_at: now,
        };
        let report_dto = ResearchReportDto::from(report);
        let report_json = serde_json::to_string(&report_dto).expect("report serialization");
        assert!(report_json.contains("\"projectId\":\"proj-1\""));
        assert!(report_json.contains("\"reportType\":\"analysis\""));
        assert!(!report_json.contains("\"project_id\":"));
        assert!(!report_json.contains("\"report_type\":"));
    }
}
