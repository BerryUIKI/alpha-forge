//! Goose Service for shadow-mode research analysis
//!
//! Provides the integration layer between:
//! - GooseAdapter (process lifecycle)
//! - McpBridge (read-only AlphaForge context)
//! - Agent task system (progress, persistence)
//!
//! Flow:
//! 1. User selects "Goose shadow analysis"
//! 2. Service creates task, sets MCP scope
//! 3. GooseAdapter executes recipe
//! 4. Progress events emitted to UI
//! 5. Result persisted with provenance
//! 6. User reviews in Artifact (no auto-persist)

use std::sync::Arc;

use tauri::AppHandle;
use tracing::info;

use crate::agent::events;
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use crate::goose::{
    GooseAdapter, GooseConfig, McpBridge, Recipe,
    adapter::RunId,
    config::ExecutionBudget,
    output::StructuredResponse,
};
use crate::services::thesis_service::ThesisService;
use crate::services::workspace_service::WorkspaceService;

/// Goose service configuration
#[derive(Debug, Clone)]
pub struct GooseServiceConfig {
    /// Goose binary configuration
    pub goose_config: GooseConfig,

    /// Default execution budget
    pub default_budget: ExecutionBudget,

    /// Enable shadow mode (read-only)
    pub shadow_mode_enabled: bool,
}

impl Default for GooseServiceConfig {
    fn default() -> Self {
        Self {
            goose_config: GooseConfig::default(),
            default_budget: ExecutionBudget::default(),
            shadow_mode_enabled: true,
        }
    }
}

/// Input for starting a Goose shadow analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StartShadowAnalysisInput {
    /// Workspace to analyze
    pub workspace_id: String,

    /// Optional thesis to focus on
    pub thesis_id: Option<String>,

    /// Optional research project to focus on
    pub research_project_id: Option<String>,

    /// Custom instructions for Goose
    pub instructions: Option<String>,
}

/// Result of a shadow analysis run
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShadowAnalysisResult {
    /// Run ID
    pub run_id: String,

    /// Workspace analyzed
    pub workspace_id: String,

    /// Structured response from Goose
    pub response: StructuredResponse,

    /// Execution duration in milliseconds
    pub duration_ms: u64,

    /// Provider used
    pub provider: Option<String>,

    /// Model used
    pub model: Option<String>,
}

/// Goose Service
pub struct GooseService {
    /// Goose adapter for process management
    adapter: Arc<GooseAdapter>,

    /// MCP bridge for AlphaForge context
    mcp_bridge: Arc<McpBridge>,

    /// Task repository for persistence
    #[allow(dead_code)]
    task_repo: AgentTaskRepository,

    /// App handle for event emission
    app_handle: AppHandle,

    /// Configuration
    config: GooseServiceConfig,
}

impl GooseService {
    /// Create a new Goose service
    pub fn new(
        workspace_service: Arc<WorkspaceService>,
        thesis_service: Arc<ThesisService>,
        task_repo: AgentTaskRepository,
        app_handle: AppHandle,
        config: GooseServiceConfig,
    ) -> Self {
        let adapter = Arc::new(GooseAdapter::new(config.goose_config.clone()));
        let mcp_bridge = Arc::new(McpBridge::new(workspace_service, thesis_service));

        Self {
            adapter,
            mcp_bridge,
            task_repo,
            app_handle,
            config,
        }
    }

    /// Start a shadow analysis run
    pub async fn start_shadow_analysis(
        &self,
        input: StartShadowAnalysisInput,
    ) -> Result<ShadowAnalysisResult, AppError> {
        // Check if shadow mode is enabled
        if !self.config.shadow_mode_enabled {
            return Err(AppError::Validation(
                "Shadow mode is not enabled".to_string(),
            ));
        }

        let run_id = RunId::new();

        info!(
            run_id = ?run_id,
            workspace_id = %input.workspace_id,
            "Starting shadow analysis"
        );

        // Emit start event
        events::emit_progress(&self.app_handle, &run_id.to_string(), "Starting Goose shadow analysis");

        // Set MCP scope
        self.mcp_bridge
            .set_scope(crate::goose::mcp::AuthorizedScope {
                workspace_id: input.workspace_id.clone(),
                task_id: run_id.to_string(),
                user_id: None,
            })
            .await;

        // Create recipe
        let mut recipe = Recipe::shadow_analysis(&input.workspace_id);

        // Add custom instructions if provided
        if let Some(instructions) = &input.instructions {
            recipe.prompt = Some(format!(
                "{}\n\nAdditional instructions: {}",
                recipe.prompt.as_ref().unwrap_or(&String::new()),
                instructions
            ));
        }

        // Add thesis focus if provided
        if let Some(thesis_id) = &input.thesis_id {
            recipe.prompt = Some(format!(
                "{}\n\nFocus on thesis ID: {}",
                recipe.prompt.as_ref().unwrap_or(&String::new()),
                thesis_id
            ));
        }

        // Execute Goose
        events::emit_progress(&self.app_handle, &run_id.to_string(), "Executing Goose recipe");

        let result = self
            .adapter
            .execute(run_id, &recipe, &self.config.default_budget)
            .await
            .map_err(|e| AppError::Internal(format!("Goose execution failed: {}", e)))?;

        // Clear MCP scope
        self.mcp_bridge.clear_scope().await;

        // Emit completion event
        events::emit_completion(&self.app_handle, &run_id.to_string(), Some("Shadow analysis completed"));

        info!(
            run_id = ?run_id,
            duration_ms = result.duration.as_millis(),
            "Shadow analysis completed"
        );

        // Return result
        Ok(ShadowAnalysisResult {
            run_id: run_id.to_string(),
            workspace_id: input.workspace_id,
            response: result.response.clone(),
            duration_ms: result.duration.as_millis() as u64,
            provider: result.response.provider.clone(),
            model: result.response.model.clone(),
        })
    }

    /// Cancel a running analysis
    pub async fn cancel_analysis(&self, run_id: &str) -> Result<(), AppError> {
        let run_id_parsed = RunId(run_id.parse().map_err(|_| {
            AppError::Validation("Invalid run ID".to_string())
        })?);

        self.adapter.cancel(run_id_parsed).await.map_err(|e| {
            AppError::Internal(format!("Failed to cancel analysis: {}", e))
        })?;

        events::emit_cancellation(&self.app_handle, run_id);

        info!(run_id = %run_id, "Shadow analysis cancelled");
        Ok(())
    }

    /// Shutdown the service
    pub async fn shutdown(&self) {
        self.adapter.shutdown().await;
        self.mcp_bridge.clear_scope().await;
        info!("Goose service shutdown complete");
    }

    /// Check if the service is healthy
    pub async fn health_check(&self) -> Result<GooseHealthStatus, AppError> {
        let binary_ok = self.adapter.verify_binary().await.is_ok();
        let shadow_mode = self.config.shadow_mode_enabled;

        Ok(GooseHealthStatus {
            binary_available: binary_ok,
            shadow_mode_enabled: shadow_mode,
            max_concurrent: self.config.goose_config.max_concurrent,
        })
    }
}

/// Health status of the Goose service
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GooseHealthStatus {
    pub binary_available: bool,
    pub shadow_mode_enabled: bool,
    pub max_concurrent: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_reasonable() {
        let config = GooseServiceConfig::default();
        assert!(config.shadow_mode_enabled);
        assert!(config.default_budget.max_turns > 0);
    }
}