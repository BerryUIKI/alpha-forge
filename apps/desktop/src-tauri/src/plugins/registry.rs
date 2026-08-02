use serde::{Deserialize, Serialize};

use crate::error::AppError;

const MAX_WINDOW_WIDTH: u32 = 1600;
const MAX_WINDOW_HEIGHT: u32 = 1200;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub entry: String,
    pub input_schema: String,
    pub permissions: Vec<PluginPermission>,
    pub window: PluginWindow,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission { Network }

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginWindow { pub width: u32, pub height: u32, pub resizable: bool }

impl PluginManifest {
    pub fn parse(json: &str) -> Result<Self, AppError> {
        let manifest: Self = serde_json::from_str(json).map_err(|_| AppError::Validation("Plugin manifest is invalid".to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), AppError> {
        if !is_plugin_id(&self.id) || self.name.trim().is_empty() || self.version.trim().is_empty() { return Err(AppError::Validation("Plugin id, name, and version are required".to_string())); }
        if !is_relative_file(&self.entry) || !is_relative_file(&self.input_schema) { return Err(AppError::Validation("Plugin entry and input schema must be safe relative file paths".to_string())); }
        if self.window.width == 0 || self.window.height == 0 || self.window.width > MAX_WINDOW_WIDTH || self.window.height > MAX_WINDOW_HEIGHT { return Err(AppError::Validation("Plugin window dimensions are outside the allowed range".to_string())); }
        Ok(())
    }
}

fn is_plugin_id(value: &str) -> bool { !value.is_empty() && value.len() <= 64 && value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') }
fn is_relative_file(value: &str) -> bool { !value.is_empty() && !value.starts_with('/') && !value.contains("..") && !value.contains('\\') }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn accepts_internal_manifest() { let manifest = PluginManifest::parse(include_str!("../../../../../plugins/company-comparison/manifest.json")).unwrap(); assert_eq!(manifest.id, "company-comparison"); }
    #[test] fn rejects_unsafe_entry() { let json = r#"{"id":"safe-plugin","name":"Safe","version":"1","entry":"../main.html","inputSchema":"schema.json","permissions":[],"window":{"width":400,"height":300,"resizable":true}}"#; assert!(PluginManifest::parse(json).is_err()); }
}
