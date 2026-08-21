// Settings Tauri commands — Phase 1.5 / S2 IPC Normalization.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;

/// App information DTO returned to frontend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub identifier: String,
}

/// Key-value setting entry DTO returned to frontend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SettingItemDto {
    pub key: String,
    pub value: String,
}

#[tauri::command]
pub fn health_check() -> Result<String, AppError> {
    Ok("ok".to_string())
}

#[tauri::command]
pub fn get_app_info(app_handle: tauri::AppHandle) -> Result<AppInfo, AppError> {
    let config = app_handle.config();

    Ok(AppInfo {
        name: "Investment OS".to_string(),
        version: config.version.clone().unwrap_or_default(),
        identifier: config.identifier.clone(),
    })
}

#[tauri::command]
pub async fn get_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, AppError> {
    state.settings_service.get(&key).await
}

#[tauri::command]
pub async fn set_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.settings_service.set(&key, &value).await
}

#[tauri::command]
pub async fn delete_setting(key: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.settings_service.delete(&key).await
}

#[tauri::command]
pub async fn list_settings(state: State<'_, AppState>) -> Result<Vec<SettingItemDto>, AppError> {
    let items = state.settings_service.list().await?;
    Ok(items
        .into_iter()
        .map(|(key, value)| SettingItemDto { key, value })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_serializes_to_camel_case_json() {
        let info = AppInfo {
            name: "Investment OS".to_string(),
            version: "0.1.0".to_string(),
            identifier: "com.alphaforge.app".to_string(),
        };

        let json = serde_json::to_string(&info).expect("serialization failed");
        assert!(json.contains("\"appName\":") || json.contains("\"name\":"));
        assert!(json.contains("\"version\":"));
        assert!(json.contains("\"identifier\":"));

        let deserialized: AppInfo = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(deserialized, info);
    }

    #[test]
    fn setting_item_dto_serializes_to_camel_case_json() {
        let item = SettingItemDto {
            key: "app.theme".to_string(),
            value: "dark".to_string(),
        };

        let json = serde_json::to_string(&item).expect("serialization failed");
        assert_eq!(json, r#"{"key":"app.theme","value":"dark"}"#);

        let deserialized: SettingItemDto = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(deserialized, item);
    }
}
