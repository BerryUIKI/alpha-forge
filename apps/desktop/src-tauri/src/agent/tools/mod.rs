// Agent tool registry and built-in tools.
//
// Tool definitions follow the JSON Schema shape used by OpenAI function
// calling and MCP: a name, a description, and an object `parameters` schema.
// Tool execution errors are returned to the model as `{"error": ...}` by the
// runtime so it can correct course, mirroring MCP's `isError` tool results.

mod calculator;
mod local;
mod research;

pub use calculator::CalculatorTool;
pub use local::{GetTaskOutputTool, ListTasksTool, SearchTasksTool};
pub use research::{GetResearchDocumentTool, SearchResearchTool};

use std::sync::Arc;

use serde_json::{json, Value};

use crate::error::AppError;
use crate::providers::ai::ToolSpec;

/// A capability the agent can invoke during a task.
#[allow(unknown_lints, clippy::double_must_use)]
#[async_trait::async_trait]
pub trait AgentTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    /// JSON Schema for the tool arguments (an object schema).
    fn parameters(&self) -> Value;
    async fn execute(&self, arguments: Value) -> Result<Value, AppError>;
}

/// Registry of tools available to the agent.
#[derive(Clone, Default)]
pub struct ToolRegistry {
    tools: Arc<Vec<Arc<dyn AgentTool>>>,
}

impl ToolRegistry {
    pub fn new(tools: Vec<Arc<dyn AgentTool>>) -> Self {
        Self {
            tools: Arc::new(tools),
        }
    }

    /// Tool definitions sent to the provider.
    pub fn definitions(&self) -> Vec<ToolSpec> {
        self.tools
            .iter()
            .map(|tool| ToolSpec {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                parameters: tool.parameters(),
            })
            .collect()
    }

    /// Executes a tool using the raw JSON arguments from the model.
    pub async fn execute(&self, name: &str, arguments: &str) -> Result<Value, AppError> {
        let tool = self
            .tools
            .iter()
            .find(|tool| tool.name() == name)
            .ok_or_else(|| AppError::Validation(format!("Unknown tool '{}'", name)))?;

        let arguments = if arguments.trim().is_empty() {
            json!({})
        } else {
            serde_json::from_str(arguments).map_err(|error| {
                AppError::Validation(format!("Invalid arguments for tool '{}': {}", name, error))
            })?
        };

        tool.execute(arguments).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct EchoTool;

    #[async_trait::async_trait]
    impl AgentTool for EchoTool {
        fn name(&self) -> &'static str {
            "echo"
        }

        fn description(&self) -> &'static str {
            "Echo the arguments."
        }

        fn parameters(&self) -> Value {
            json!({"type": "object", "properties": {}})
        }

        async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
            Ok(json!({ "echo": arguments }))
        }
    }

    #[test]
    fn test_definitions_include_registered_tools() {
        let registry = ToolRegistry::new(vec![Arc::new(EchoTool)]);

        let definitions = registry.definitions();

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].name, "echo");
        assert_eq!(definitions[0].parameters["type"], "object");
    }

    #[tokio::test]
    async fn test_execute_routes_to_tool() {
        let registry = ToolRegistry::new(vec![Arc::new(EchoTool)]);

        let result = registry
            .execute("echo", r#"{"value":42}"#)
            .await
            .expect("tool should execute");

        assert_eq!(result["echo"]["value"], 42);
    }

    #[tokio::test]
    async fn test_execute_rejects_unknown_tool() {
        let registry = ToolRegistry::new(vec![Arc::new(EchoTool)]);

        let result = registry.execute("missing", "{}").await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[tokio::test]
    async fn test_execute_rejects_invalid_arguments() {
        let registry = ToolRegistry::new(vec![Arc::new(EchoTool)]);

        let result = registry.execute("echo", "not json").await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }
}
