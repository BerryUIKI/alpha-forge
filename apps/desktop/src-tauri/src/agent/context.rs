// Agent context assembly.
//
// Context is rebuilt for every task from the local workspace: the workspace
// name and summaries of recently completed tasks. Research documents will be
// added once the research data model is implemented.

use serde::Serialize;

use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::error::AppError;
use domain::task::{AgentTask, TaskStatus};

const MAX_RECENT_TASKS: usize = 5;
const MAX_RESEARCH_DOCUMENTS: usize = 5;
const MAX_SUMMARY_CHARS: usize = 400;

/// Workspace context handed to the agent for one task.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct TaskContext {
    pub workspace_id: String,
    pub workspace_name: Option<String>,
    pub recent_tasks: Vec<ContextTask>,
    pub research_documents: Vec<ContextDocument>,
}

/// A completed task summarized for the prompt.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ContextTask {
    pub title: String,
    pub status: TaskStatus,
    pub summary: Option<String>,
}

/// A research document summarized for the prompt.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ContextDocument {
    pub title: String,
    pub summary: String,
}

impl TaskContext {
    /// Renders the context for the user prompt; returns `None` when empty.
    pub fn render(&self) -> Option<String> {
        let mut lines = Vec::new();

        if !self.workspace_id.trim().is_empty() {
            lines.push(format!("Workspace ID: {}", self.workspace_id));
        }

        if let Some(name) = &self.workspace_name {
            lines.push(format!("Workspace: {name}"));
        }

        if !self.research_documents.is_empty() {
            lines.push("Research documents in this workspace:".to_string());
            for document in &self.research_documents {
                if document.summary.is_empty() {
                    lines.push(format!("- {}", document.title));
                } else {
                    lines.push(format!("- {} — {}", document.title, document.summary));
                }
            }
        }

        if !self.recent_tasks.is_empty() {
            lines.push("Recent completed tasks in this workspace:".to_string());
            for task in &self.recent_tasks {
                match &task.summary {
                    Some(summary) => lines.push(format!("- {} — {}", task.title, summary)),
                    None => lines.push(format!("- {}", task.title)),
                }
            }
        }

        if lines.is_empty() {
            None
        } else {
            Some(lines.join("\n"))
        }
    }
}

/// Builds the context for a task.
#[allow(unknown_lints, clippy::double_must_use)]
#[async_trait::async_trait]
pub trait TaskContextProvider: Send + Sync {
    async fn build(&self, task: &AgentTask) -> Result<TaskContext, AppError>;
}

/// Context provider backed by the workspace, task, and research repositories.
#[derive(Clone)]
pub struct WorkspaceContextProvider {
    workspaces: WorkspaceRepository,
    tasks: AgentTaskRepository,
    research: ResearchDocumentRepository,
}

impl WorkspaceContextProvider {
    pub fn new(
        workspaces: WorkspaceRepository,
        tasks: AgentTaskRepository,
        research: ResearchDocumentRepository,
    ) -> Self {
        Self {
            workspaces,
            tasks,
            research,
        }
    }
}

#[async_trait::async_trait]
impl TaskContextProvider for WorkspaceContextProvider {
    async fn build(&self, task: &AgentTask) -> Result<TaskContext, AppError> {
        let workspace_name = self
            .workspaces
            .get(&task.workspace_id)
            .await?
            .map(|workspace| workspace.name);

        let research_documents = self
            .research
            .list_by_workspace(&task.workspace_id)
            .await?
            .into_iter()
            .take(MAX_RESEARCH_DOCUMENTS)
            .map(|document| ContextDocument {
                title: document.title,
                summary: summarize_document(document.content.as_deref()),
            })
            .collect();

        let recent_tasks = self
            .tasks
            .list_by_workspace(&task.workspace_id)
            .await?
            .into_iter()
            .filter(|candidate| {
                candidate.id != task.id && candidate.status == TaskStatus::Completed
            })
            .take(MAX_RECENT_TASKS)
            .map(|candidate| ContextTask {
                title: candidate.title,
                status: candidate.status,
                summary: candidate
                    .output
                    .as_deref()
                    .map(|output| summarize_output(output, MAX_SUMMARY_CHARS)),
            })
            .collect();

        Ok(TaskContext {
            workspace_id: task.workspace_id.clone(),
            workspace_name,
            recent_tasks,
            research_documents,
        })
    }
}

/// Context provider that always returns an empty context.
#[derive(Debug, Clone, Default)]
pub struct EmptyContextProvider;

#[async_trait::async_trait]
impl TaskContextProvider for EmptyContextProvider {
    async fn build(&self, task: &AgentTask) -> Result<TaskContext, AppError> {
        Ok(TaskContext {
            workspace_id: task.workspace_id.clone(),
            workspace_name: None,
            recent_tasks: Vec::new(),
            research_documents: Vec::new(),
        })
    }
}

/// Prefers the structured `summary` field, falling back to the raw output.
fn summarize_output(output: &str, max_chars: usize) -> String {
    let summary = serde_json::from_str::<serde_json::Value>(output)
        .ok()
        .and_then(|value| {
            value
                .get("summary")
                .and_then(|summary| summary.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| output.to_string());

    truncate(&summary, max_chars)
}

fn summarize_document(content: Option<&str>) -> String {
    content
        .map(str::trim)
        .filter(|content| !content.is_empty())
        .map(|content| truncate(content, MAX_SUMMARY_CHARS))
        .unwrap_or_default()
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let mut truncated: String = value.chars().take(max_chars.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
}
