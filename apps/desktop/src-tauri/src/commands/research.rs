// Research project Tauri commands — M4 Research Workspace.

use tauri::State;
use crate::app::state::AppState;
use crate::documents::pdf_import::select_and_extract_pdf;
use crate::documents::web_import::fetch_web_page;
use crate::error::AppError;
use domain::research::{CreateDocumentInput, CreateNoteInput, CreateProjectInput, CreateReportInput, DocumentType, ReportType, ResearchDocument, ResearchNote, ResearchProject, ResearchReport, ResearchSearchMatch, ResearchSource};

// Project commands
#[tauri::command]
pub async fn create_research_project(workspace_id: String, title: String, description: Option<String>, state: State<'_, AppState>) -> Result<ResearchProject, AppError> {
    state.research_project_service.create_project(CreateProjectInput { workspace_id, title, description }).await
}

#[tauri::command]
pub async fn get_research_project(id: String, state: State<'_, AppState>) -> Result<Option<ResearchProject>, AppError> {
    state.research_project_service.get_project(&id).await
}

#[tauri::command]
pub async fn list_research_projects(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchProject>, AppError> {
    state.research_project_service.list_projects(&workspace_id).await
}

#[tauri::command]
pub async fn archive_research_project(id: String, state: State<'_, AppState>) -> Result<ResearchProject, AppError> {
    state.research_project_service.archive_project(&id).await
}

#[tauri::command]
pub async fn complete_research_project(id: String, state: State<'_, AppState>) -> Result<ResearchProject, AppError> {
    state.research_project_service.complete_project(&id).await
}

#[tauri::command]
pub async fn delete_research_project(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_project_service.delete_project(&id).await
}

// Document commands
#[tauri::command]
pub async fn create_research_document(project_id: String, document_type: String, title: String, content: Option<String>, source_url: Option<String>, file_path: Option<String>, state: State<'_, AppState>) -> Result<ResearchDocument, AppError> {
    let doc_type = match document_type.as_str() { "pdf" => DocumentType::Pdf, "web_page" => DocumentType::WebPage, "note" => DocumentType::Note, "report" => DocumentType::Report, _ => DocumentType::Note };
    state.research_document_service.create_document(CreateDocumentInput { project_id, document_type: doc_type, title, content, source_url, file_path }).await
}

#[tauri::command]
pub async fn get_research_document(id: String, state: State<'_, AppState>) -> Result<Option<ResearchDocument>, AppError> {
    state.research_document_service.get_document(&id).await
}

#[tauri::command]
pub async fn list_research_documents(project_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchDocument>, AppError> {
    state.research_document_service.list_documents(&project_id).await
}

#[tauri::command]
pub async fn delete_research_document(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_document_service.delete_document(&id).await
}

#[tauri::command]
pub async fn import_research_pdf(project_id: String, state: State<'_, AppState>) -> Result<Option<ResearchDocument>, AppError> {
    let Some(pdf) = select_and_extract_pdf().await? else { return Ok(None); };
    state.research_document_service.create_document(CreateDocumentInput {
        project_id,
        document_type: DocumentType::Pdf,
        title: pdf.title,
        content: Some(pdf.content),
        source_url: None,
        file_path: None,
    }).await.map(Some)
}

#[tauri::command]
pub async fn import_research_web_page(project_id: String, url: String, state: State<'_, AppState>) -> Result<ResearchDocument, AppError> {
    let page = fetch_web_page(&url).await?;
    let document = state.research_document_service.create_document(CreateDocumentInput { project_id, document_type: DocumentType::WebPage, title: page.title.clone(), content: Some(page.content), source_url: Some(page.url.clone()), file_path: None }).await?;
    state.research_source_service.create_source(document.id.clone(), Some(page.url), Some(page.title)).await?;
    Ok(document)
}

#[tauri::command]
pub async fn search_research_document(id: String, query: String, state: State<'_, AppState>) -> Result<Vec<ResearchSearchMatch>, AppError> {
    state.research_document_service.search_document(&id, &query).await
}

#[tauri::command]
pub async fn create_research_source(document_id: String, url: Option<String>, title: Option<String>, state: State<'_, AppState>) -> Result<ResearchSource, AppError> {
    state.research_source_service.create_source(document_id, url, title).await
}

#[tauri::command]
pub async fn list_research_sources(document_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchSource>, AppError> {
    state.research_source_service.list_sources(&document_id).await
}

#[tauri::command]
pub async fn create_research_note(document_id: String, content: String, state: State<'_, AppState>) -> Result<ResearchNote, AppError> {
    state.research_note_service.create_note(CreateNoteInput { document_id, content }).await
}

#[tauri::command]
pub async fn list_research_notes(document_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchNote>, AppError> {
    state.research_note_service.list_notes(&document_id).await
}

#[tauri::command]
pub async fn delete_research_note(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_note_service.delete_note(&id).await
}

// Report commands
#[tauri::command]
pub async fn create_research_report(project_id: String, title: String, content: String, report_type: String, state: State<'_, AppState>) -> Result<ResearchReport, AppError> {
    let r_type = match report_type.as_str() { "analysis" => ReportType::Analysis, "summary" => ReportType::Summary, "thesis" => ReportType::Thesis, "recommendation" => ReportType::Recommendation, _ => ReportType::Analysis };
    state.research_report_service.create_report(CreateReportInput { project_id, title, content, report_type: r_type }).await
}

#[tauri::command]
pub async fn get_research_report(id: String, state: State<'_, AppState>) -> Result<Option<ResearchReport>, AppError> {
    state.research_report_service.get_report(&id).await
}

#[tauri::command]
pub async fn list_research_reports(project_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchReport>, AppError> {
    state.research_report_service.list_reports(&project_id).await
}

#[tauri::command]
pub async fn delete_research_report(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_report_service.delete_report(&id).await
}
