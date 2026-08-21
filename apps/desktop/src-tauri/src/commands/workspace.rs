// Workspace Tauri commands — Phase 1.5 / S2 IPC Normalization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::workspace::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};

/// Command boundary DTO for Workspace.
/// Serialized in camelCase for frontend IPC consistency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDto {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Workspace> for WorkspaceDto {
    fn from(w: Workspace) -> Self {
        Self {
            id: w.id,
            name: w.name,
            created_at: w.created_at,
            updated_at: w.updated_at,
        }
    }
}

#[tauri::command]
pub async fn create_workspace(
    name: String,
    state: State<'_, AppState>,
) -> Result<WorkspaceDto, AppError> {
    let input = CreateWorkspaceInput { name };
    let workspace = state.workspace_service.create(input).await?;
    Ok(workspace.into())
}

#[tauri::command]
pub async fn list_workspaces(state: State<'_, AppState>) -> Result<Vec<WorkspaceDto>, AppError> {
    let workspaces = state.workspace_service.list().await?;
    Ok(workspaces.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn get_workspace(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<WorkspaceDto>, AppError> {
    let workspace = state.workspace_service.get(&id).await?;
    Ok(workspace.map(Into::into))
}

#[tauri::command]
pub async fn update_workspace(
    id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<WorkspaceDto, AppError> {
    let input = UpdateWorkspaceInput { name: Some(name) };
    let workspace = state.workspace_service.update(&id, input).await?;
    Ok(workspace.into())
}

#[tauri::command]
pub async fn delete_workspace(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.workspace_service.delete(&id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn workspace_dto_serializes_to_camel_case_json() {
        let fixed_time = Utc.with_ymd_and_hms(2026, 8, 20, 12, 0, 0).unwrap();
        let dto = WorkspaceDto {
            id: "ws-123".to_string(),
            name: "Alpha Fund".to_string(),
            created_at: fixed_time,
            updated_at: fixed_time,
        };

        let json = serde_json::to_string(&dto).expect("serialization failed");
        assert!(json.contains("\"createdAt\":"));
        assert!(json.contains("\"updatedAt\":"));
        assert!(!json.contains("\"created_at\":"));
        assert!(!json.contains("\"updated_at\":"));

        // Roundtrip test
        let deserialized: WorkspaceDto =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(deserialized, dto);
    }
}
