// Artifact runtime manager — manages artifact window lifecycle.

use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::RwLock;

use crate::error::AppError;

const MIN_ARTIFACT_WINDOW_WIDTH: f64 = 320.0;
const MAX_ARTIFACT_WINDOW_WIDTH: f64 = 1600.0;
const MIN_ARTIFACT_WINDOW_HEIGHT: f64 = 240.0;
const MAX_ARTIFACT_WINDOW_HEIGHT: f64 = 1200.0;

/// Unique identifier for an artifact window.
pub type WindowId = String;

/// Configuration for artifact window creation.
#[derive(Debug, Clone)]
pub struct ArtifactWindowConfig {
    pub artifact_id: String,
    pub artifact_type: String,
    pub title: String,
    pub width: f64,
    pub height: f64,
}

impl Default for ArtifactWindowConfig {
    fn default() -> Self {
        Self {
            artifact_id: String::new(),
            artifact_type: String::new(),
            title: "Artifact".to_string(),
            width: 800.0,
            height: 600.0,
        }
    }
}

/// Tracks active artifact windows.
pub struct ArtifactManager {
    app_handle: AppHandle,
    /// Map from artifact ID to window label.
    active_windows: Arc<RwLock<HashMap<String, String>>>,
}

impl ArtifactManager {
    /// Creates a new artifact manager.
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            active_windows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Opens an artifact in a new window.
    pub async fn open_artifact(&self, config: ArtifactWindowConfig) -> Result<String, AppError> {
        validate_artifact_window_config(&config)?;
        let window_label = format!("artifact-{}", config.artifact_id);

        // Check if window already exists
        {
            let windows = self.active_windows.read().await;
            if windows.contains_key(&config.artifact_id) {
                return Err(AppError::Validation(format!(
                    "Artifact '{}' is already open",
                    config.artifact_id
                )));
            }
        }

        // Build artifact URL
        // For now, we'll use a simple HTML template
        // In production, this would route to a React component
        let artifact_url = format!(
            "/artifact/{}/{}",
            config.artifact_id, config.artifact_type
        );

        // Create window
        let artifact_url = artifact_url
            .parse()
            .map_err(|_| AppError::Validation("Artifact route is invalid".to_string()))?;
        let _window = WebviewWindowBuilder::new(
            &self.app_handle,
            &window_label,
            WebviewUrl::App(artifact_url)
        )
        .title(&config.title)
        .inner_size(config.width, config.height)
        .resizable(true)
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to create artifact window: {}", e)))?;

        // Track window
        {
            let mut windows = self.active_windows.write().await;
            windows.insert(config.artifact_id.clone(), window_label.clone());
        }

        Ok(window_label)
    }

    /// Closes an artifact window.
    pub async fn close_artifact(&self, artifact_id: &str) -> Result<(), AppError> {
        let window_label = {
            let windows = self.active_windows.read().await;
            windows.get(artifact_id).cloned()
        };

        if let Some(label) = window_label {
            // Get window and close it
            if let Some(window) = self.app_handle.get_webview_window(&label) {
                window.close().map_err(|e| {
                    AppError::Internal(format!("Failed to close artifact window: {}", e))
                })?;
            }

            // Remove from tracking
            {
                let mut windows = self.active_windows.write().await;
                windows.remove(artifact_id);
            }
        }

        Ok(())
    }

    /// Checks if an artifact window is open.
    pub async fn is_artifact_open(&self, artifact_id: &str) -> bool {
        let windows = self.active_windows.read().await;
        windows.contains_key(artifact_id)
    }

    /// Lists all open artifact windows.
    pub async fn list_open_artifacts(&self) -> Vec<String> {
        let windows = self.active_windows.read().await;
        windows.keys().cloned().collect()
    }

    /// Emits an event to an artifact window.
    pub async fn emit_to_artifact(
        &self,
        artifact_id: &str,
        event: &str,
        payload: serde_json::Value,
    ) -> Result<(), AppError> {
        let window_label = {
            let windows = self.active_windows.read().await;
            windows.get(artifact_id).cloned()
        };

        if let Some(label) = window_label {
            if let Some(window) = self.app_handle.get_webview_window(&label) {
                window.emit(event, payload).map_err(|e| {
                    AppError::Internal(format!("Failed to emit to artifact window: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Updates artifact data in an open window.
    pub async fn update_artifact_data(
        &self,
        artifact_id: &str,
        data: serde_json::Value,
    ) -> Result<(), AppError> {
        self.emit_to_artifact(artifact_id, "artifact:update", data)
            .await
    }

    /// Changes theme for an artifact window.
    pub async fn set_artifact_theme(
        &self,
        artifact_id: &str,
        theme: &str,
    ) -> Result<(), AppError> {
        self.emit_to_artifact(
            artifact_id,
            "artifact:theme",
            serde_json::json!({ "theme": theme }),
        )
        .await
    }
}

fn validate_artifact_window_config(config: &ArtifactWindowConfig) -> Result<(), AppError> {
    uuid::Uuid::parse_str(&config.artifact_id)
        .map_err(|_| AppError::Validation("Artifact id must be a UUID".to_string()))?;
    if !is_route_segment(&config.artifact_type) {
        return Err(AppError::Validation(
            "Artifact type must be a safe route segment".to_string(),
        ));
    }
    if !config.width.is_finite()
        || !config.height.is_finite()
        || !(MIN_ARTIFACT_WINDOW_WIDTH..=MAX_ARTIFACT_WINDOW_WIDTH).contains(&config.width)
        || !(MIN_ARTIFACT_WINDOW_HEIGHT..=MAX_ARTIFACT_WINDOW_HEIGHT).contains(&config.height)
    {
        return Err(AppError::Validation(
            "Artifact window dimensions are outside the allowed range".to_string(),
        ));
    }
    Ok(())
}

fn is_route_segment(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_window_config_default() {
        let config = ArtifactWindowConfig::default();
        assert_eq!(config.title, "Artifact");
        assert_eq!(config.width, 800.0);
        assert_eq!(config.height, 600.0);
    }

    #[test]
    fn test_artifact_window_config_custom() {
        let config = ArtifactWindowConfig {
            artifact_id: "test-123".to_string(),
            artifact_type: "comparison_table".to_string(),
            title: "Test Artifact".to_string(),
            width: 1024.0,
            height: 768.0,
        };
        assert_eq!(config.artifact_id, "test-123");
        assert_eq!(config.artifact_type, "comparison_table");
    }

    #[test]
    fn validates_safe_artifact_window_configuration() {
        let config = ArtifactWindowConfig {
            artifact_id: "2a707687-3fc5-4b02-81ba-043830213244".to_string(),
            artifact_type: "comparison_table".to_string(),
            title: "Comparison".to_string(),
            width: 1024.0,
            height: 768.0,
        };
        assert!(validate_artifact_window_config(&config).is_ok());
    }

    #[test]
    fn rejects_unsafe_artifact_routes_and_window_sizes() {
        let unsafe_route = ArtifactWindowConfig {
            artifact_id: "2a707687-3fc5-4b02-81ba-043830213244".to_string(),
            artifact_type: "../settings".to_string(),
            title: "Unsafe".to_string(),
            width: 1024.0,
            height: 768.0,
        };
        assert!(validate_artifact_window_config(&unsafe_route).is_err());

        let invalid_size = ArtifactWindowConfig {
            artifact_type: "timeline".to_string(),
            width: 1.0,
            ..unsafe_route
        };
        assert!(validate_artifact_window_config(&invalid_size).is_err());
    }
}
