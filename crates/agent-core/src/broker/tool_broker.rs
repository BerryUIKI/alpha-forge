use crate::error::{SupervisorError, SupervisorResult};
use agent_protocol::messages::{RunScope, ToolRequest, ToolResponse};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

const MAX_TOOL_RESPONSE_BYTES: usize = 256 * 1024;

/// Trait implemented by host-side tool handlers.
#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn execute(&self, scope: &RunScope, params: &Value) -> Result<Value, String>;
}

/// Description and permission metadata for a host tool.
pub struct RegisteredTool {
    pub name: String,
    pub description: String,
    pub is_read_only: bool,
    pub handler: Arc<dyn ToolHandler>,
}

/// Host-owned tool broker providing safe, scoped, read-only tools to worker subprocesses.
pub struct ToolBroker {
    tools: HashMap<String, RegisteredTool>,
}

impl ToolBroker {
    pub fn new() -> Self {
        let mut broker = Self {
            tools: HashMap::new(),
        };
        broker.register_default_tools();
        broker
    }

    pub fn register_tool(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        is_read_only: bool,
        handler: Arc<dyn ToolHandler>,
    ) {
        let name_str = name.into();
        self.tools.insert(
            name_str.clone(),
            RegisteredTool {
                name: name_str,
                description: description.into(),
                is_read_only,
                handler,
            },
        );
    }

    fn register_default_tools(&mut self) {
        // Safe read-only mock chunk search handler
        struct ChunkSearchHandler;
        #[async_trait]
        impl ToolHandler for ChunkSearchHandler {
            async fn execute(&self, scope: &RunScope, params: &Value) -> Result<Value, String> {
                let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
                Ok(json!({
                    "workspaceId": scope.workspace_id,
                    "query": query,
                    "chunks": [
                        {
                            "id": "chunk-001",
                            "title": "Quarterly Performance",
                            "text": "Gross margins expanded by 240 basis points year over year."
                        }
                    ]
                }))
            }
        }

        self.register_tool(
            "research.search_chunks",
            "Searches ingested research chunks within the active workspace",
            true,
            Arc::new(ChunkSearchHandler),
        );
    }

    /// Handles a brokered tool request with scope isolation and output size bounding.
    pub async fn handle_request(
        &self,
        scope: &RunScope,
        req: &ToolRequest,
    ) -> SupervisorResult<ToolResponse> {
        // Prohibit dangerous patterns
        if req.tool_name.contains("sql")
            || req.tool_name.contains("shell")
            || req.tool_name.contains("exec")
            || req.tool_name.contains("trade")
            || req.tool_name.contains("credential")
        {
            warn!(tool = %req.tool_name, "Rejected attempt to call dangerous tool");
            return Ok(ToolResponse {
                request_id: req.request_id.clone(),
                result: json!({
                    "error": format!("Tool '{}' is strictly prohibited by security policy", req.tool_name),
                    "success": false
                }),
            });
        }

        let tool = match self.tools.get(&req.tool_name) {
            Some(t) => t,
            None => {
                warn!(tool = %req.tool_name, "Worker requested unknown tool");
                return Ok(ToolResponse {
                    request_id: req.request_id.clone(),
                    result: json!({
                        "error": format!("Unknown or unapproved tool '{}'", req.tool_name),
                        "success": false
                    }),
                });
            }
        };

        debug!(
            request_id = %req.request_id,
            tool = %req.tool_name,
            workspace = %scope.workspace_id,
            "Dispatching brokered tool execution"
        );

        match tool.handler.execute(scope, &req.parameters).await {
            Ok(result) => {
                let serialized = serde_json::to_string(&result).map_err(|e| {
                    SupervisorError::Internal(format!("Failed to serialize tool result: {}", e))
                })?;

                if serialized.len() > MAX_TOOL_RESPONSE_BYTES {
                    return Ok(ToolResponse {
                        request_id: req.request_id.clone(),
                        result: json!({
                            "error": format!("Tool output exceeded limit ({} > {} bytes)", serialized.len(), MAX_TOOL_RESPONSE_BYTES),
                            "success": false
                        }),
                    });
                }

                Ok(ToolResponse {
                    request_id: req.request_id.clone(),
                    result,
                })
            }
            Err(err) => Ok(ToolResponse {
                request_id: req.request_id.clone(),
                result: json!({
                    "error": err,
                    "success": false
                }),
            }),
        }
    }
}

impl Default for ToolBroker {
    fn default() -> Self {
        Self::new()
    }
}
