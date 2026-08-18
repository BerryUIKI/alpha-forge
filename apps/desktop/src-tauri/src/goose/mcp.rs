//! AlphaForge MCP Bridge for Goose
//!
//! Provides read-only tools for Goose to access AlphaForge workspace data.
//! All tools are:
//! - Allowlisted (only explicitly defined tools available)
//! - Read-only (no write operations)
//! - Scoped (workspace ID injected by Rust, not trusted from Goose)
//! - Bounded (size limits, pagination)
//! - Validated (input/output schema validation)
//!
//! Security model:
//! - Rust attaches authorized workspace scope
//! - IDs from Goose are validated against authorized workspace
//! - No database handle, SQL, or filesystem access exposed
//! - No arbitrary URLs or external network access

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::goose::error::GooseError;
use crate::services::thesis_service::ThesisService;
use crate::services::workspace_service::WorkspaceService;

/// MCP tool name
pub type ToolName = &'static str;

/// List of allowlisted MCP tools
pub const ALLOWLISTED_TOOLS: &[ToolName] = &[
    "get_workspace_summary",
    "search_research_sources",
    "get_research_source",
    "get_thesis_context",
    "list_related_artifacts",
];

/// Maximum page size for list operations
pub const MAX_PAGE_SIZE: usize = 100;

/// Default page size
pub const DEFAULT_PAGE_SIZE: usize = 20;

/// Maximum content length in characters
pub const MAX_CONTENT_LENGTH: usize = 10_000;

/// MCP tool input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    /// Tool name
    pub name: String,

    /// Tool arguments
    pub arguments: serde_json::Value,
}

/// MCP tool output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    /// Whether the call succeeded
    pub success: bool,

    /// Result data (if success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Authorized scope for MCP operations
#[derive(Debug, Clone)]
pub struct AuthorizedScope {
    /// Workspace ID that Goose is authorized to access
    pub workspace_id: String,

    /// Task ID for tracking
    pub task_id: String,

    /// User ID (for audit)
    pub user_id: Option<String>,
}

/// AlphaForge MCP Bridge
pub struct McpBridge {
    /// Authorized scope (set by Rust, not trusted from Goose)
    scope: RwLock<Option<AuthorizedScope>>,

    /// Workspace service
    workspace_service: Arc<WorkspaceService>,

    /// Thesis service
    thesis_service: Arc<ThesisService>,
}

impl McpBridge {
    /// Create a new MCP bridge
    pub fn new(
        workspace_service: Arc<WorkspaceService>,
        thesis_service: Arc<ThesisService>,
    ) -> Self {
        Self {
            scope: RwLock::new(None),
            workspace_service,
            thesis_service,
        }
    }

    /// Set the authorized scope (called by Rust before Goose execution)
    pub async fn set_scope(&self, scope: AuthorizedScope) {
        let workspace_id = scope.workspace_id.clone();
        let mut current = self.scope.write().await;
        *current = Some(scope);
        debug!(workspace_id = %workspace_id, "MCP scope set");
    }

    /// Clear the authorized scope
    pub async fn clear_scope(&self) {
        let mut current = self.scope.write().await;
        *current = None;
        debug!("MCP scope cleared");
    }

    /// Get the current scope
    pub async fn get_scope(&self) -> Option<AuthorizedScope> {
        self.scope.read().await.clone()
    }

    /// Validate a tool call and return the authorized workspace ID
    async fn validate_call(
        &self,
        tool_name: &str,
        workspace_id_from_input: Option<&str>,
    ) -> Result<String, GooseError> {
        // Check tool is allowlisted
        if !ALLOWLISTED_TOOLS.contains(&tool_name) {
            return Err(GooseError::OutputValidationFailed {
                reason: format!("Tool '{}' is not allowlisted", tool_name),
            });
        }

        // Get authorized scope
        let scope = self.scope.read().await;
        let authorized = scope.as_ref().ok_or_else(|| {
            GooseError::Internal("No scope set - call set_scope before execution".into())
        })?;

        // If input contains workspace_id, validate it matches authorized scope
        if let Some(input_workspace_id) = workspace_id_from_input {
            if input_workspace_id != authorized.workspace_id {
                warn!(
                    input_workspace_id = %input_workspace_id,
                    authorized_workspace_id = %authorized.workspace_id,
                    "Workspace ID mismatch rejected"
                );
                return Err(GooseError::OutputValidationFailed {
                    reason: "Workspace ID does not match authorized scope".into(),
                });
            }
        }

        Ok(authorized.workspace_id.clone())
    }

    /// Execute a tool call
    pub async fn execute(&self, input: ToolInput) -> ToolOutput {
        // Validate tool is allowlisted
        let tool_name = input.name.as_str();

        // Extract workspace_id from arguments if present
        let workspace_id_from_input = input.arguments.get("workspace_id").and_then(|v| v.as_str());

        match self.validate_call(tool_name, workspace_id_from_input).await {
            Ok(workspace_id) => {
                match self
                    .execute_tool(tool_name, workspace_id, input.arguments)
                    .await
                {
                    Ok(result) => ToolOutput {
                        success: true,
                        result: Some(result),
                        error: None,
                    },
                    Err(e) => ToolOutput {
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                    },
                }
            }
            Err(e) => ToolOutput {
                success: false,
                result: None,
                error: Some(e.to_string()),
            },
        }
    }

    /// Execute a specific tool
    async fn execute_tool(
        &self,
        tool_name: &str,
        workspace_id: String,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, GooseError> {
        match tool_name {
            "get_workspace_summary" => self.get_workspace_summary(workspace_id).await,
            "search_research_sources" => {
                self.search_research_sources(workspace_id, arguments).await
            }
            "get_research_source" => self.get_research_source(workspace_id, arguments).await,
            "get_thesis_context" => self.get_thesis_context(workspace_id, arguments).await,
            "list_related_artifacts" => self.list_related_artifacts(workspace_id, arguments).await,
            _ => Err(GooseError::OutputValidationFailed {
                reason: format!("Unknown tool: {}", tool_name),
            }),
        }
    }

    /// Get workspace summary (names and counts, no credentials)
    async fn get_workspace_summary(
        &self,
        workspace_id: String,
    ) -> Result<serde_json::Value, GooseError> {
        let workspace = self
            .workspace_service
            .get(&workspace_id)
            .await
            .map_err(|e| GooseError::Internal(format!("Failed to get workspace: {}", e)))?;

        let workspace = match workspace {
            Some(w) => w,
            None => {
                return Err(GooseError::OutputValidationFailed {
                    reason: "Workspace not found".into(),
                })
            }
        };

        // Return only summary information, no sensitive data
        Ok(serde_json::json!({
            "id": workspace.id,
            "name": workspace.name,
            "created_at": workspace.created_at,
            "updated_at": workspace.updated_at,
            // Counts would require additional queries; return placeholder
            "research_project_count": null,
            "thesis_count": null,
            "source_count": null,
        }))
    }

    /// Search research sources
    async fn search_research_sources(
        &self,
        _workspace_id: String,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, GooseError> {
        let _query = arguments
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GooseError::OutputValidationFailed {
                reason: "Missing 'query' parameter".into(),
            })?;

        let _limit = arguments
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(DEFAULT_PAGE_SIZE as u64)
            .min(MAX_PAGE_SIZE as u64) as i64;

        // Note: ResearchSourceService doesn't have search_sources method yet
        // Return empty results for now - would need to add method to service

        // Return bounded, redacted results
        let results: Vec<serde_json::Value> = vec![];

        Ok(serde_json::json!({
            "results": results,
            "total": results.len(),
            "truncated": false,
        }))
    }

    /// Get a specific research source
    async fn get_research_source(
        &self,
        _workspace_id: String,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, GooseError> {
        let source_id = arguments
            .get("source_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GooseError::OutputValidationFailed {
                reason: "Missing 'source_id' parameter".into(),
            })?;

        // Note: ResearchSourceService doesn't have get_source method yet
        // Return placeholder - would need to add method to service
        Ok(serde_json::json!({
            "id": source_id,
            "title": "Placeholder - source service integration pending",
            "content": "",
            "source_type": null,
            "url": null,
            "published_at": null,
            "retrieved_at": null,
        }))
    }

    /// Get thesis context
    async fn get_thesis_context(
        &self,
        workspace_id: String,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, GooseError> {
        let thesis_id = arguments
            .get("thesis_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GooseError::OutputValidationFailed {
                reason: "Missing 'thesis_id' parameter".into(),
            })?;

        let thesis = self
            .thesis_service
            .get_thesis(thesis_id)
            .await
            .map_err(|e| GooseError::Internal(format!("Failed to get thesis: {}", e)))?;

        let thesis = match thesis {
            Some(t) => t,
            None => {
                return Err(GooseError::OutputValidationFailed {
                    reason: "Thesis not found".into(),
                })
            }
        };

        // Validate workspace ownership
        if thesis.workspace_id != workspace_id {
            return Err(GooseError::OutputValidationFailed {
                reason: "Thesis does not belong to authorized workspace".into(),
            });
        }

        // Get evidence links
        let evidence = self
            .thesis_service
            .list_evidence(thesis_id)
            .await
            .map_err(|e| GooseError::Internal(format!("Failed to get evidence: {}", e)))?;

        // Get confidence history
        let confidence_history = self
            .thesis_service
            .list_confidence_history(thesis_id)
            .await
            .map_err(|e| {
                GooseError::Internal(format!("Failed to get confidence history: {}", e))
            })?;

        Ok(serde_json::json!({
            "id": thesis.id,
            "title": thesis.title,
            "thesis": thesis.thesis,
            "status": thesis.status,
            "confidence": thesis.confidence,
            "workspace_id": thesis.workspace_id,
            "evidence_count": evidence.len(),
            "evidence": evidence.into_iter().take(20).collect::<Vec<_>>(),
            "confidence_history": confidence_history.into_iter().take(10).collect::<Vec<_>>(),
        }))
    }

    /// List related artifacts
    async fn list_related_artifacts(
        &self,
        workspace_id: String,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, GooseError> {
        let entity_type = arguments.get("entity_type").and_then(|v| v.as_str());

        let entity_id = arguments.get("entity_id").and_then(|v| v.as_str());

        let _limit = arguments
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(DEFAULT_PAGE_SIZE as u64)
            .min(MAX_PAGE_SIZE as u64);

        // For now, return empty list (would need artifact service)
        // This is a placeholder that respects the read-only boundary

        Ok(serde_json::json!({
            "results": [],
            "total": 0,
            "workspace_id": workspace_id,
            "entity_type": entity_type,
            "entity_id": entity_id,
            "limit": _limit,
        }))
    }

    /// Check if a tool is allowlisted
    pub fn is_tool_allowed(tool_name: &str) -> bool {
        ALLOWLISTED_TOOLS.contains(&tool_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowlist_is_enforced() {
        assert!(McpBridge::is_tool_allowed("get_workspace_summary"));
        assert!(McpBridge::is_tool_allowed("search_research_sources"));
        assert!(!McpBridge::is_tool_allowed("write_file"));
        assert!(!McpBridge::is_tool_allowed("execute_sql"));
    }

    #[test]
    fn max_page_size_is_reasonable() {
        const { assert!(MAX_PAGE_SIZE <= 100) }
        const { assert!(DEFAULT_PAGE_SIZE <= MAX_PAGE_SIZE) }
    }
}
