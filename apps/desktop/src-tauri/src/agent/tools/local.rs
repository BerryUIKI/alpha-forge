// Read-only tools over local workspace data.

use serde::Deserialize;
use serde_json::{json, Value};

use crate::agent::tools::AgentTool;
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use crate::text::truncate_with_notice;
use domain::task::AgentTask;

const MAX_LISTED_TASKS: usize = 20;
const MAX_TOOL_OUTPUT_CHARS: usize = 4000;

/// Lists recent tasks in a workspace.
#[derive(Clone)]
pub struct ListTasksTool {
    repo: AgentTaskRepository,
}

impl ListTasksTool {
    pub fn new(repo: AgentTaskRepository) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Deserialize)]
struct ListTasksArgs {
    workspace_id: String,
    status: Option<String>,
}

#[async_trait::async_trait]
impl AgentTool for ListTasksTool {
    fn name(&self) -> &'static str {
        "list_tasks"
    }

    fn description(&self) -> &'static str {
        "List recent agent tasks in a workspace, optionally filtered by status."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "workspace_id": {
                    "type": "string",
                    "description": "Workspace ID"
                },
                "status": {
                    "type": "string",
                    "enum": [
                        "created", "queued", "running", "waiting_for_input",
                        "completed", "failed", "cancelled"
                    ],
                    "description": "Optional status filter"
                }
            },
            "required": ["workspace_id"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
        let args: ListTasksArgs = serde_json::from_value(arguments).map_err(|error| {
            AppError::Validation(format!("invalid list_tasks arguments: {error}"))
        })?;

        let tasks = self.repo.list_by_workspace(&args.workspace_id).await?;
        let items = tasks
            .into_iter()
            .filter(|task| {
                args.status
                    .as_deref()
                    .is_none_or(|status| task.status.as_str() == status)
            })
            .take(MAX_LISTED_TASKS)
            .map(|task| task_summary(&task))
            .collect::<Vec<_>>();

        Ok(json!({ "tasks": items }))
    }
}

/// Searches task titles and descriptions in a workspace.
#[derive(Clone)]
pub struct SearchTasksTool {
    repo: AgentTaskRepository,
}

impl SearchTasksTool {
    pub fn new(repo: AgentTaskRepository) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Deserialize)]
struct SearchTasksArgs {
    workspace_id: String,
    query: String,
}

#[async_trait::async_trait]
impl AgentTool for SearchTasksTool {
    fn name(&self) -> &'static str {
        "search_tasks"
    }

    fn description(&self) -> &'static str {
        "Search agent task titles and descriptions in a workspace by substring."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "workspace_id": {
                    "type": "string",
                    "description": "Workspace ID"
                },
                "query": {
                    "type": "string",
                    "description": "Text to search for in task titles and descriptions"
                }
            },
            "required": ["workspace_id", "query"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
        let args: SearchTasksArgs = serde_json::from_value(arguments).map_err(|error| {
            AppError::Validation(format!("invalid search_tasks arguments: {error}"))
        })?;

        if args.query.trim().is_empty() {
            return Err(AppError::Validation(
                "search query cannot be empty".to_string(),
            ));
        }

        let tasks = self
            .repo
            .search(
                &args.workspace_id,
                args.query.trim(),
                MAX_LISTED_TASKS as i64,
            )
            .await?;
        let items = tasks.iter().map(task_summary).collect::<Vec<_>>();

        Ok(json!({ "tasks": items }))
    }
}

/// Returns the stored output or error of a task.
#[derive(Clone)]
pub struct GetTaskOutputTool {
    repo: AgentTaskRepository,
}

impl GetTaskOutputTool {
    pub fn new(repo: AgentTaskRepository) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Deserialize)]
struct GetTaskOutputArgs {
    task_id: String,
}

#[async_trait::async_trait]
impl AgentTool for GetTaskOutputTool {
    fn name(&self) -> &'static str {
        "get_task_output"
    }

    fn description(&self) -> &'static str {
        "Read the stored output or error of a previous agent task."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "task_id": {
                    "type": "string",
                    "description": "Task ID"
                }
            },
            "required": ["task_id"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
        let args: GetTaskOutputArgs = serde_json::from_value(arguments).map_err(|error| {
            AppError::Validation(format!("invalid get_task_output arguments: {error}"))
        })?;

        let task = self
            .repo
            .get(&args.task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", args.task_id)))?;

        Ok(json!({
            "id": task.id,
            "title": task.title,
            "status": task.status.as_str(),
            "output": task.output.as_deref().map(|output| truncate_with_notice(output, MAX_TOOL_OUTPUT_CHARS)),
            "error": task.error,
        }))
    }
}

fn task_summary(task: &AgentTask) -> Value {
    json!({
        "id": task.id,
        "title": task.title,
        "status": task.status.as_str(),
        "created_at": task.created_at.to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::task::{CreateAgentTaskInput, TaskStatus};

    async fn setup_repo() -> AgentTaskRepository {
        AgentTaskRepository::new(crate::test_support::test_pool_with_workspace().await)
    }

    async fn create_task(repo: &AgentTaskRepository, title: &str) -> domain::task::AgentTask {
        repo.create(CreateAgentTaskInput {
            workspace_id: "w1".to_string(),
            title: title.to_string(),
            description: None,
        })
        .await
        .expect("Failed to create task")
    }

    #[tokio::test]
    async fn test_list_tasks_returns_task_summaries() {
        let repo = setup_repo().await;
        create_task(&repo, "Analyze Tesla").await;
        create_task(&repo, "Analyze Nvidia").await;
        let tool = ListTasksTool::new(repo);

        let result = tool
            .execute(json!({"workspace_id": "w1"}))
            .await
            .expect("tool should execute");

        assert_eq!(result["tasks"].as_array().map(Vec::len), Some(2));
        assert!(result["tasks"][0]["id"].is_string());
        assert_eq!(result["tasks"][0]["status"], "created");
    }

    #[tokio::test]
    async fn test_list_tasks_filters_by_status() {
        let repo = setup_repo().await;
        let created = create_task(&repo, "Created task").await;
        let completed = create_task(&repo, "Completed task").await;
        repo.complete(&completed.id, Some(r#"{"summary":"done"}"#))
            .await
            .expect("Failed to complete task");
        let tool = ListTasksTool::new(repo);

        let result = tool
            .execute(json!({"workspace_id": "w1", "status": "completed"}))
            .await
            .expect("tool should execute");

        let tasks = result["tasks"].as_array().expect("tasks array");
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0]["id"], completed.id);
        assert_ne!(tasks[0]["id"], created.id);
    }

    #[tokio::test]
    async fn test_search_tasks_matches_title() {
        let repo = setup_repo().await;
        create_task(&repo, "Analyze Tesla").await;
        create_task(&repo, "Analyze Nvidia").await;
        let tool = SearchTasksTool::new(repo);

        let result = tool
            .execute(json!({"workspace_id": "w1", "query": "Tesla"}))
            .await
            .expect("tool should execute");

        let tasks = result["tasks"].as_array().expect("tasks array");
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0]["title"], "Analyze Tesla");
    }

    #[tokio::test]
    async fn test_get_task_output_returns_stored_output() {
        let repo = setup_repo().await;
        let task = create_task(&repo, "Completed task").await;
        repo.complete(&task.id, Some(r#"{"summary":"done"}"#))
            .await
            .expect("Failed to complete task");
        let tool = GetTaskOutputTool::new(repo);

        let result = tool
            .execute(json!({"task_id": task.id}))
            .await
            .expect("tool should execute");

        assert_eq!(result["status"], "completed");
        assert_eq!(result["output"], r#"{"summary":"done"}"#);
        assert!(result["error"].is_null());
    }

    #[tokio::test]
    async fn test_get_task_output_rejects_missing_task() {
        let tool = GetTaskOutputTool::new(setup_repo().await);

        let result = tool.execute(json!({"task_id": "missing"})).await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[test]
    fn test_status_matches_domain_display() {
        assert_eq!(TaskStatus::WaitingForInput.to_string(), "waiting_for_input");
    }
}
