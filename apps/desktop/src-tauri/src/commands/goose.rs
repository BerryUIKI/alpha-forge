//! Tauri commands for Goose integration and human-approved proposals (M10)

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use crate::goose::config::ProviderPolicy;
use crate::services::goose_service::{
    GooseHealthStatus, ShadowAnalysisResult, StartShadowAnalysisInput,
};
use domain::proposal::{CreateProposalInput, Proposal, ProposalStatus};

/// Start a Goose shadow analysis
#[tauri::command]
pub async fn start_goose_shadow_analysis(
    input: StartShadowAnalysisInput,
    state: State<'_, AppState>,
) -> Result<ShadowAnalysisResult, AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.start_shadow_analysis(input).await
}

/// Cancel a running Goose analysis
#[tauri::command]
pub async fn cancel_goose_analysis(
    run_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.cancel_analysis(&run_id).await
}

/// Check Goose service health
#[tauri::command]
pub async fn check_goose_health(state: State<'_, AppState>) -> Result<GooseHealthStatus, AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.health_check().await
}

/// Get Goose runtime diagnostics (M10-G6)
#[tauri::command]
pub async fn get_goose_diagnostics(
    state: State<'_, AppState>,
) -> Result<crate::services::goose_service::GooseDiagnostics, AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.get_diagnostics().await
}

/// Get Goose provider policy (M10-G5)
#[tauri::command]
pub async fn get_goose_provider_policy(
    _state: State<'_, AppState>,
) -> Result<ProviderPolicy, AppError> {
    Ok(ProviderPolicy::default())
}

/// Create a proposal (M10-G4)
#[tauri::command]
pub async fn create_goose_proposal(
    input: CreateProposalInput,
    state: State<'_, AppState>,
) -> Result<Proposal, AppError> {
    state.proposal_service.create_proposal(input).await
}

/// List proposals for a workspace (M10-G4)
#[tauri::command]
pub async fn list_goose_proposals(
    workspace_id: String,
    status: Option<ProposalStatus>,
    state: State<'_, AppState>,
) -> Result<Vec<Proposal>, AppError> {
    state
        .proposal_service
        .list_proposals(&workspace_id, status)
        .await
}

/// Get a proposal by ID (M10-G4)
#[tauri::command]
pub async fn get_goose_proposal(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Proposal>, AppError> {
    state.proposal_service.get_proposal(&id).await
}

/// Accept a proposal and persist to domain service (M10-G4)
#[tauri::command]
pub async fn accept_goose_proposal(
    id: String,
    state: State<'_, AppState>,
) -> Result<Proposal, AppError> {
    state
        .proposal_service
        .accept_proposal(&id, &state.thesis_service, &state.research_note_service)
        .await
}

/// Reject a proposal (M10-G4)
#[tauri::command]
pub async fn reject_goose_proposal(
    id: String,
    state: State<'_, AppState>,
) -> Result<Proposal, AppError> {
    state.proposal_service.reject_proposal(&id).await
}
