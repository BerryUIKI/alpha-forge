// Artifact window management.
//
// Each artifact opens in its own WebView with a minimal capability set
// (`capabilities/artifact-window.json`, `core:default` only). The window loads
// a dedicated entry point that renders the payload with predefined components;
// no privileged application commands are imported by that entry.

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::AppError;
use domain::artifact::Artifact;

pub const ARTIFACT_WINDOW_PREFIX: &str = "artifact-";

/// Builds the initialization script that hands the artifact payload to the window.
///
/// The payload is embedded as a JSON string literal and parsed with `JSON.parse`,
/// so quotes, newlines, or script-like content in the artifact cannot break out
/// of the assignment.
pub fn artifact_init_script(artifact: &Artifact) -> Result<String, AppError> {
    let data: serde_json::Value = serde_json::from_str(&artifact.data)
        .map_err(|_| AppError::Internal("stored artifact payload is not valid JSON".to_string()))?;

    let payload = serde_json::json!({
        "id": artifact.id,
        "title": artifact.title,
        "artifact_type": artifact.artifact_type,
        "data": data,
        "created_at": artifact.created_at.to_rfc3339(),
    });

    let encoded = serde_json::to_string(&payload)
        .map_err(|error| AppError::Internal(format!("failed to encode artifact: {error}")))?;
    let literal = serde_json::to_string(&encoded)
        .map_err(|error| AppError::Internal(format!("failed to quote artifact: {error}")))?;

    Ok(format!(
        "window.__ALPHAFORGE_ARTIFACT__ = JSON.parse({literal});"
    ))
}

/// Opens an artifact window, or focuses it when it is already open.
pub fn open_artifact_window(app: &AppHandle, artifact: &Artifact) -> Result<(), AppError> {
    let label = format!("{ARTIFACT_WINDOW_PREFIX}{}", artifact.id);

    if let Some(window) = app.get_webview_window(&label) {
        window.show().map_err(|error| {
            AppError::Internal(format!("failed to show artifact window: {error}"))
        })?;
        window.set_focus().map_err(|error| {
            AppError::Internal(format!("failed to focus artifact window: {error}"))
        })?;
        return Ok(());
    }

    let script = artifact_init_script(artifact)?;

    WebviewWindowBuilder::new(app, label, WebviewUrl::App("artifact.html".into()))
        .title(format!("Artifact — {}", artifact.title))
        .inner_size(1000.0, 720.0)
        .min_inner_size(640.0, 480.0)
        .resizable(true)
        .initialization_script(&script)
        .build()
        .map_err(|error| AppError::Internal(format!("failed to open artifact window: {error}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use domain::artifact::ArtifactStatus;

    fn artifact(title: &str, data: &str) -> Artifact {
        let now = Utc::now();
        Artifact {
            id: "artifact-1".to_string(),
            task_id: Some("task-1".to_string()),
            workspace_id: Some("ws-1".to_string()),
            artifact_type: "timeline".to_string(),
            title: title.to_string(),
            data: data.to_string(),
            status: ArtifactStatus::Completed,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn test_init_script_escapes_quotes_and_newlines() {
        let script = artifact_init_script(&artifact(
            "A \"quoted\" title\nwith newline",
            r#"{"events":[{"date":"2026-01-01","title":"Start"}]}"#,
        ))
        .expect("script should build");

        let literal = script
            .strip_prefix("window.__ALPHAFORGE_ARTIFACT__ = JSON.parse(")
            .and_then(|rest| rest.strip_suffix(");"))
            .expect("script should have the expected shape");

        assert!(!literal.contains('\n'));

        let encoded: String =
            serde_json::from_str(literal).expect("literal should be a JSON string");
        let payload: serde_json::Value =
            serde_json::from_str(&encoded).expect("payload should be JSON");

        assert_eq!(payload["title"], "A \"quoted\" title\nwith newline");
        assert_eq!(payload["artifact_type"], "timeline");
        assert_eq!(payload["data"]["events"][0]["title"], "Start");
    }

    #[test]
    fn test_init_script_rejects_invalid_payload() {
        let result = artifact_init_script(&artifact("Title", "not json"));

        assert!(matches!(result, Err(AppError::Internal(_))));
    }
}
