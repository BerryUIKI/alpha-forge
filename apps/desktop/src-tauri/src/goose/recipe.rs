//! Recipe loader and validator for Goose execution
//!
//! Recipes are YAML configurations that define:
//! - Prompt/instructions for Goose
//! - Extensions (MCP servers) to load
//! - Parameters for dynamic values
//! - Structured response schema
//! - Limits and settings

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::goose::error::GooseError;

/// Goose recipe definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Recipe {
    /// Recipe format version
    pub version: Option<String>,

    /// Short title describing the recipe
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Static instructions (alternative to prompt)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Dynamic prompt (required for headless mode)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// MCP extensions to load
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<Extension>,

    /// Dynamic parameters
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, Parameter>,

    /// Structured response schema
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseSchema>,

    /// Execution settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,

    /// Retry configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryConfig>,
}

/// MCP extension configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Extension {
    /// Extension type (stdio, sse, etc.)
    #[serde(rename = "type")]
    pub extension_type: String,

    /// Extension name
    pub name: String,

    /// Command to run
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmd: Option<String>,

    /// Command arguments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// Environment variables for secrets
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_keys: Vec<String>,

    /// Timeout in milliseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

/// Parameter definition for dynamic values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter type (string, number, file, etc.)
    #[serde(rename = "type")]
    pub param_type: String,

    /// Whether the parameter is required
    #[serde(default)]
    pub required: bool,

    /// Default value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Structured response schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseSchema {
    /// JSON Schema definition
    pub json_schema: JsonSchema,
}

/// JSON Schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    /// Schema type
    #[serde(rename = "type")]
    pub schema_type: String,

    /// Object properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, serde_json::Value>,

    /// Required properties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
}

/// Execution settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Maximum turns
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<u32>,

    /// Model to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Provider to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    #[serde(default)]
    pub max_retries: u32,

    /// Delay between retries in milliseconds
    #[serde(default)]
    pub retry_delay_ms: u64,
}

impl Recipe {
    /// Load a recipe from a YAML file
    pub async fn from_file(path: &Path) -> Result<Self, GooseError> {
        if !path.exists() {
            return Err(GooseError::RecipeNotFound { path: path.to_path_buf() });
        }

        let contents = tokio::fs::read_to_string(path)
            .await
            .map_err(GooseError::Io)?;

        let recipe: Recipe = serde_yaml::from_str(&contents).map_err(|e| {
            GooseError::RecipeValidationFailed {
                reason: format!("YAML parse error: {}", e),
            }
        })?;

        Ok(recipe)
    }

    /// Write recipe to a YAML file
    pub async fn write_to_file(&self, path: &Path) -> Result<(), GooseError> {
        let contents = serde_yaml::to_string(self).map_err(|e| GooseError::Internal(format!(
            "Failed to serialize recipe: {}",
            e
        )))?;

        tokio::fs::write(path, contents)
            .await
            .map_err(GooseError::Io)?;

        Ok(())
    }

    /// Validate the recipe for security and correctness
    pub fn validate(&self) -> Result<(), GooseError> {
        // Must have either instructions or prompt
        if self.instructions.is_none() && self.prompt.is_none() {
            return Err(GooseError::RecipeValidationFailed {
                reason: "Recipe must have either instructions or prompt".into(),
            });
        }

        // Prompt is required for headless mode
        if self.prompt.is_none() {
            return Err(GooseError::RecipeValidationFailed {
                reason: "Prompt is required for headless execution".into(),
            });
        }

        // Validate extensions
        for ext in &self.extensions {
            // Only allowlisted extensions permitted
            if !Self::is_extension_allowed(&ext.name) {
                return Err(GooseError::RecipeValidationFailed {
                    reason: format!("Extension '{}' is not allowlisted", ext.name),
                });
            }

            // No shell commands in extensions
            if ext.cmd.as_ref().map(|c| c.contains("sh") || c.contains("bash") || c.contains("cmd")).unwrap_or(false) {
                return Err(GooseError::RecipeValidationFailed {
                    reason: format!("Extension '{}' contains shell command", ext.name),
                });
            }
        }

        // Must have structured response schema
        if self.response.is_none() {
            return Err(GooseError::RecipeValidationFailed {
                reason: "Structured response schema is required".into(),
            });
        }

        Ok(())
    }

    /// Check if an extension is allowlisted
    fn is_extension_allowed(name: &str) -> bool {
        // Only AlphaForge MCP bridge is allowed initially
        matches!(name, "alphaforge" | "alphaforge-mcp" | "alphaforge-bridge")
    }

    /// Calculate checksum of recipe content
    pub fn checksum(&self) -> String {
        let contents = serde_yaml::to_string(self).unwrap_or_default();
        format!("{:x}", Sha256::digest(contents.as_bytes()))
    }

    /// Create a shadow analysis recipe
    pub fn shadow_analysis(workspace_id: &str) -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            "summary".to_string(),
            serde_json::json!({"type": "string", "description": "Brief summary of findings"}),
        );
        properties.insert(
            "claims".to_string(),
            serde_json::json!({
                "type": "array",
                "items": {"type": "object", "properties": {"claim": {"type": "string"}, "confidence": {"type": "number"}}}
            }),
        );
        properties.insert(
            "evidence".to_string(),
            serde_json::json!({
                "type": "array",
                "items": {"type": "object", "properties": {"claim_id": {"type": "string"}, "source_id": {"type": "string"}, "excerpt": {"type": "string"}}}
            }),
        );
        properties.insert(
            "risks".to_string(),
            serde_json::json!({
                "type": "array",
                "items": {"type": "object", "properties": {"risk": {"type": "string"}, "severity": {"type": "string"}}}
            }),
        );
        properties.insert(
            "sourceIds".to_string(),
            serde_json::json!({"type": "array", "items": {"type": "string"}}),
        );
        properties.insert(
            "confidence".to_string(),
            serde_json::json!({"type": "number", "description": "Overall confidence 0-100"}),
        );

        Recipe {
            version: Some("1.0".into()),
            title: "Shadow Analysis".into(),
            description: "Read-only analysis of workspace research data".into(),
            prompt: Some(format!(
                "Analyze the research data in workspace {} and provide structured findings. \
                 Do not make any changes. Only read and synthesize information.",
                workspace_id
            )),
            instructions: None,
            extensions: vec![Extension {
                extension_type: "stdio".into(),
                name: "alphaforge-bridge".into(),
                cmd: None, // Will be set by Rust
                args: vec![],
                env_keys: vec![],
                timeout: Some(300000),
            }],
            parameters: HashMap::new(),
            response: Some(ResponseSchema {
                json_schema: JsonSchema {
                    schema_type: "object".into(),
                    properties,
                    required: vec![
                        "summary".into(),
                        "claims".into(),
                        "evidence".into(),
                        "risks".into(),
                        "sourceIds".into(),
                        "confidence".into(),
                    ],
                },
            }),
            settings: Some(Settings {
                max_turns: Some(30),
                model: None,
                provider: None,
            }),
            retry: Some(RetryConfig {
                max_retries: 2,
                retry_delay_ms: 1000,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_analysis_recipe_is_valid() {
        let recipe = Recipe::shadow_analysis("test-workspace");
        assert!(recipe.validate().is_ok());
    }

    #[test]
    fn recipe_without_prompt_is_invalid() {
        let recipe = Recipe {
            title: "Test".into(),
            description: "Test".into(),
            prompt: None,
            instructions: Some("Test instructions".into()),
            response: None,
            ..Default::default()
        };
        assert!(recipe.validate().is_err());
    }

    #[test]
    fn disallowed_extension_is_rejected() {
        let recipe = Recipe {
            title: "Test".into(),
            description: "Test".into(),
            prompt: Some("Test".into()),
            extensions: vec![Extension {
                extension_type: "stdio".into(),
                name: "filesystem".into(), // Not allowlisted
                cmd: Some("fs-mcp".into()),
                ..Default::default()
            }],
            response: Some(ResponseSchema {
                json_schema: JsonSchema {
                    schema_type: "object".into(),
                    properties: HashMap::new(),
                    required: vec![],
                },
            }),
            ..Default::default()
        };
        assert!(recipe.validate().is_err());
    }
}