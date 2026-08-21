// Artifacts Tauri commands — M3 Artifact Intelligence System.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::artifacts::manager::ArtifactWindowConfig;
use crate::error::AppError;
use domain::artifact::{Artifact, ArtifactStatus, ArtifactType, CreateArtifactInput};

/// DTO for Artifact with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactDto {
    pub id: String,
    pub workspace_id: String,
    pub task_id: Option<String>,
    pub artifact_type: String,
    pub status: ArtifactStatus,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Artifact> for ArtifactDto {
    fn from(artifact: Artifact) -> Self {
        Self {
            id: artifact.id,
            workspace_id: artifact.workspace_id,
            task_id: artifact.task_id,
            artifact_type: artifact.artifact_type.to_string(),
            status: artifact.status,
            input: artifact.input,
            output: artifact.output,
            error: artifact.error,
            created_at: artifact.created_at.to_rfc3339(),
            updated_at: artifact.updated_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub async fn create_artifact(
    workspace_id: String,
    task_id: Option<String>,
    artifact_type: String,
    input: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<ArtifactDto, AppError> {
    // Parse artifact type
    let artifact_type = match artifact_type.as_str() {
        "comparison_table" => ArtifactType::ComparisonTable,
        "timeline" => ArtifactType::Timeline,
        "industry_map" => ArtifactType::IndustryMap,
        "valuation_model" => ArtifactType::ValuationModel,
        "risk_dashboard" => ArtifactType::RiskDashboard,
        "earnings_analysis" => ArtifactType::EarningsAnalysis,
        "macro_dashboard" => ArtifactType::MacroDashboard,
        other => ArtifactType::Custom(other.to_string()),
    };

    let input = CreateArtifactInput {
        workspace_id,
        task_id,
        artifact_type,
        input,
    };

    state
        .artifact_service
        .create_artifact(input)
        .await
        .map(ArtifactDto::from)
}

#[tauri::command]
pub async fn get_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<ArtifactDto>, AppError> {
    state
        .artifact_service
        .get_artifact(&id)
        .await
        .map(|opt| opt.map(ArtifactDto::from))
}

#[tauri::command]
pub async fn list_artifacts(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactDto>, AppError> {
    state
        .artifact_service
        .list_artifacts(&workspace_id)
        .await
        .map(|artifacts| artifacts.into_iter().map(ArtifactDto::from).collect())
}

#[tauri::command]
pub async fn list_task_artifacts(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactDto>, AppError> {
    state
        .artifact_service
        .list_task_artifacts(&task_id)
        .await
        .map(|artifacts| artifacts.into_iter().map(ArtifactDto::from).collect())
}

#[tauri::command]
pub async fn start_artifact_generation(
    id: String,
    state: State<'_, AppState>,
) -> Result<ArtifactDto, AppError> {
    state
        .artifact_service
        .start_generation(&id)
        .await
        .map(ArtifactDto::from)
}

#[tauri::command]
pub async fn complete_artifact_generation(
    id: String,
    output: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<ArtifactDto, AppError> {
    state
        .artifact_service
        .complete_generation(&id, output)
        .await
        .map(ArtifactDto::from)
}

#[tauri::command]
pub async fn fail_artifact_generation(
    id: String,
    error: String,
    state: State<'_, AppState>,
) -> Result<ArtifactDto, AppError> {
    state
        .artifact_service
        .fail_generation(&id, &error)
        .await
        .map(ArtifactDto::from)
}

#[tauri::command]
pub async fn start_viewing_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<ArtifactDto, AppError> {
    // Get artifact first
    let artifact = state
        .artifact_service
        .get_artifact(&id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

    // Open artifact window
    let window_config = ArtifactWindowConfig {
        artifact_id: id.clone(),
        artifact_type: artifact.artifact_type.to_string(),
        title: format!("Artifact: {}", artifact.artifact_type),
        width: 1024.0,
        height: 768.0,
    };

    state.artifact_manager.open_artifact(window_config).await?;

    // Update status to viewing
    state
        .artifact_service
        .start_viewing(&id)
        .await
        .map(ArtifactDto::from)
}

#[tauri::command]
pub async fn close_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<ArtifactDto, AppError> {
    // Close artifact window
    state.artifact_manager.close_artifact(&id).await?;

    // Update status
    state
        .artifact_service
        .close_artifact(&id)
        .await
        .map(ArtifactDto::from)
}

#[tauri::command]
pub async fn delete_artifact(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.artifact_service.delete_artifact(&id).await
}

#[tauri::command]
pub async fn list_open_artifacts(state: State<'_, AppState>) -> Result<Vec<String>, AppError> {
    Ok(state.artifact_manager.list_open_artifacts().await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_artifact_dto_camel_case_serialization() {
        let now = Utc::now();
        let artifact = Artifact {
            id: "art-1".to_string(),
            task_id: Some("task-1".to_string()),
            workspace_id: "workspace-1".to_string(),
            artifact_type: ArtifactType::ComparisonTable,
            status: ArtifactStatus::Completed,
            input: serde_json::json!({"query": "tech stocks"}),
            output: Some(serde_json::json!({"companies": ["AAPL", "MSFT"]})),
            error: None,
            created_at: now,
            updated_at: now,
        };

        let dto = ArtifactDto::from(artifact);
        let json = serde_json::to_string(&dto).expect("serialization failed");

        assert!(json.contains("\"workspaceId\":"));
        assert!(json.contains("\"taskId\":"));
        assert!(json.contains("\"artifactType\":\"comparison_table\""));
        assert!(json.contains("\"createdAt\":"));
        assert!(json.contains("\"updatedAt\":"));
        assert!(!json.contains("\"workspace_id\":"));
        assert!(!json.contains("\"task_id\":"));
        assert!(!json.contains("\"artifact_type\":"));
        assert!(!json.contains("\"created_at\":"));
        assert!(!json.contains("\"updated_at\":"));

        let deserialized: ArtifactDto =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(deserialized, dto);
    }
}
