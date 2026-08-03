// Thesis management Tauri commands — M5 Investment Knowledge System.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::thesis::{
    AddEvidenceInput, CreateThesisInput, EvidenceDirection, InvestmentThesis,
    ThesisConfidenceSnapshot, ThesisEvidence, UpdateConfidenceInput,
};

// Thesis CRUD commands

#[tauri::command]
pub async fn create_thesis(
    workspace_id: String,
    title: String,
    thesis: String,
    confidence: Option<i32>,
    state: State<'_, AppState>,
) -> Result<InvestmentThesis, AppError> {
    state
        .thesis_service
        .create_thesis(CreateThesisInput {
            workspace_id,
            title,
            thesis,
            confidence,
        })
        .await
}

#[tauri::command]
pub async fn get_thesis(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<InvestmentThesis>, AppError> {
    state.thesis_service.get_thesis(&id).await
}

#[tauri::command]
pub async fn list_theses(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<InvestmentThesis>, AppError> {
    state.thesis_service.list_theses(&workspace_id).await
}

#[tauri::command]
pub async fn activate_thesis(
    id: String,
    state: State<'_, AppState>,
) -> Result<InvestmentThesis, AppError> {
    state.thesis_service.activate_thesis(&id).await
}

#[tauri::command]
pub async fn start_thesis_validation(
    id: String,
    state: State<'_, AppState>,
) -> Result<InvestmentThesis, AppError> {
    state.thesis_service.start_validation(&id).await
}

#[tauri::command]
pub async fn complete_thesis_validation(
    id: String,
    outcome: String,
    validated: bool,
    state: State<'_, AppState>,
) -> Result<InvestmentThesis, AppError> {
    state
        .thesis_service
        .complete_validation(&id, outcome, validated)
        .await
}

#[tauri::command]
pub async fn update_thesis_confidence(
    thesis_id: String,
    confidence: i32,
    state: State<'_, AppState>,
) -> Result<InvestmentThesis, AppError> {
    state
        .thesis_service
        .update_confidence(UpdateConfidenceInput {
            thesis_id,
            confidence,
        })
        .await
}

#[tauri::command]
pub async fn close_thesis(
    id: String,
    state: State<'_, AppState>,
) -> Result<InvestmentThesis, AppError> {
    state.thesis_service.close_thesis(&id).await
}

#[tauri::command]
pub async fn delete_thesis(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.thesis_service.delete_thesis(&id).await
}

// Evidence commands

#[tauri::command]
pub async fn add_thesis_evidence(
    thesis_id: String,
    direction: String,
    evidence: String,
    source_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ThesisEvidence, AppError> {
    let dir = match direction.as_str() {
        "supporting" => EvidenceDirection::Supporting,
        "contradicting" => EvidenceDirection::Contradicting,
        _ => {
            return Err(AppError::Validation(
                "Evidence direction must be 'supporting' or 'contradicting'".to_string(),
            ));
        }
    };

    state
        .thesis_service
        .add_evidence(AddEvidenceInput {
            thesis_id,
            direction: dir,
            evidence,
            source_id,
        })
        .await
}

#[tauri::command]
pub async fn list_thesis_evidence(
    thesis_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisEvidence>, AppError> {
    state.thesis_service.list_evidence(&thesis_id).await
}

#[tauri::command]
pub async fn delete_thesis_evidence(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.thesis_service.delete_evidence(&id).await
}

#[tauri::command]
pub async fn list_thesis_confidence_history(
    thesis_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisConfidenceSnapshot>, AppError> {
    state
        .thesis_service
        .list_confidence_history(&thesis_id)
        .await
}
