use serde::Serialize;
use tauri::State;

use crate::app::state::AppState;
use crate::database::repositories::plugin_repository::InstalledPlugin;
use crate::error::AppError;
use crate::plugins::registry::PluginManifest;
use domain::artifact::{Artifact, CreateArtifactInput};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginStatus {
    pub manifest: PluginManifest,
    pub enabled: bool,
}

impl From<InstalledPlugin> for PluginStatus {
    fn from(plugin: InstalledPlugin) -> Self {
        Self {
            manifest: plugin.manifest,
            enabled: plugin.enabled,
        }
    }
}

#[tauri::command]
pub async fn list_plugins(state: State<'_, AppState>) -> Result<Vec<PluginStatus>, AppError> {
    state
        .plugin_service
        .list_plugins()
        .await
        .map(|plugins| plugins.into_iter().map(PluginStatus::from).collect())
}

#[tauri::command]
pub async fn set_plugin_enabled(
    plugin_id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.plugin_service.set_enabled(&plugin_id, enabled).await
}

#[tauri::command]
pub async fn create_plugin_artifact(
    workspace_id: String,
    plugin_id: String,
    input: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    let request = state.plugin_service.prepare_artifact(&plugin_id, input).await?;
    let artifact = state.artifact_service.create_artifact(CreateArtifactInput {
        workspace_id,
        task_id: None,
        artifact_type: request.artifact_type,
        input: request.payload.clone(),
    }).await?;
    state.artifact_service.start_generation(&artifact.id).await?;
    state.artifact_service.complete_generation(&artifact.id, request.payload).await
}
