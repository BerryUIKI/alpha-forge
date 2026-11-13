// Human-readable and JSON rendering.

use std::io::Write;

use domain::artifact::Artifact;
use domain::task::{AgentTask, AgentTaskEvent, TaskEventType};
use domain::workspace::Workspace;
use serde::Serialize;

use crate::error::CliError;

/// Output format selected by `--json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn is_json(self) -> bool {
        self == OutputFormat::Json
    }
}

/// Prints a value as pretty-printed JSON with a trailing newline.
pub fn print_json<T: Serialize + ?Sized>(out: &mut dyn Write, value: &T) -> Result<(), CliError> {
    let encoded = serde_json::to_string_pretty(value).map_err(|_| {
        CliError::App(alpha_forge_lib::error::AppError::Internal(
            "could not serialize command output".to_string(),
        ))
    })?;
    writeln!(out, "{encoded}").map_err(CliError::Output)
}

/// Renders a workspace table.
pub fn render_workspaces(workspaces: &[Workspace]) -> String {
    if workspaces.is_empty() {
        return "No workspaces.".to_string();
    }

    let mut output = format!("{:<36}  {:<40}  CREATED\n", "ID", "NAME");
    for workspace in workspaces {
        output.push_str(&format!(
            "{:<36}  {:<40}  {}\n",
            workspace.id,
            truncate(&workspace.name, 40),
            workspace.created_at.to_rfc3339()
        ));
    }

    output.trim_end().to_string()
}

/// Renders a task table.
pub fn render_tasks(tasks: &[AgentTask]) -> String {
    if tasks.is_empty() {
        return "No tasks.".to_string();
    }

    let mut output = format!("{:<36}  {:<17}  TITLE\n", "ID", "STATUS");
    for task in tasks {
        output.push_str(&format!(
            "{:<36}  {:<17}  {}\n",
            task.id,
            task.status,
            truncate(&task.title, 60)
        ));
    }

    output.trim_end().to_string()
}

/// Renders a single task's metadata.
pub fn render_task(task: &AgentTask) -> String {
    let mut output = format!(
        "ID:          {}\nWorkspace:   {}\nTitle:       {}\nStatus:      {}\nCreated:     {}\nUpdated:     {}",
        task.id,
        task.workspace_id,
        task.title,
        task.status,
        task.created_at.to_rfc3339(),
        task.updated_at.to_rfc3339()
    );

    if let Some(description) = &task.description {
        output.push_str(&format!("\nDescription: {description}"));
    }

    output
}

/// Renders a task together with the persisted execution outcome.
///
/// Completion output and failure messages live in the task event timeline
/// rather than on the task row.
pub fn render_task_result(task: &AgentTask, events: &[AgentTaskEvent]) -> String {
    let mut output = render_task(task);

    if let Some(event) = events.iter().rev().find(|event| {
        matches!(
            event.event_type,
            TaskEventType::TaskCompleted | TaskEventType::TaskFailed
        )
    }) {
        match event.event_type {
            TaskEventType::TaskCompleted => {
                if let Some(payload) = &event.payload {
                    output.push_str("\n\nOutput:\n");
                    output.push_str(&pretty_payload(payload));
                }
            }
            TaskEventType::TaskFailed => {
                if let Some(payload) = &event.payload {
                    output.push_str(&format!("\n\nError: {payload}"));
                }
            }
            _ => {}
        }
    }

    output
}

/// Renders a task event timeline.
pub fn render_events(events: &[AgentTaskEvent]) -> String {
    if events.is_empty() {
        return "No events.".to_string();
    }

    let mut output = String::new();
    for event in events {
        let detail = match event.event_type {
            TaskEventType::TaskProgress | TaskEventType::TaskFailed => {
                event.payload.as_deref().unwrap_or_default()
            }
            _ => "",
        };
        output.push_str(&format!(
            "{}  {:<24}  {}\n",
            event.created_at.to_rfc3339(),
            event_type_name(&event.event_type),
            detail
        ));
    }

    output.trim_end().to_string()
}

/// Renders an artifact table.
pub fn render_artifacts(artifacts: &[Artifact]) -> String {
    if artifacts.is_empty() {
        return "No artifacts.".to_string();
    }

    let mut output = format!(
        "{:<36}  {:<20}  {:<12}  {:<36}  CREATED\n",
        "ID", "TYPE", "STATUS", "WORKSPACE"
    );
    for artifact in artifacts {
        output.push_str(&format!(
            "{:<36}  {:<20}  {:<12}  {:<36}  {}\n",
            artifact.id,
            truncate(&artifact.artifact_type.to_string(), 20),
            artifact.status,
            artifact.workspace_id,
            artifact.created_at.to_rfc3339()
        ));
    }

    output.trim_end().to_string()
}

/// Shortens a value to `max` characters, appending an ellipsis when truncated.
pub fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_string();
    }

    let mut truncated: String = value.chars().take(max.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
}

fn pretty_payload(payload: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| payload.to_string()),
        Err(_) => payload.to_string(),
    }
}

fn event_type_name(event_type: &TaskEventType) -> &'static str {
    match event_type {
        TaskEventType::TaskCreated => "task_created",
        TaskEventType::TaskQueued => "task_queued",
        TaskEventType::TaskStarted => "task_started",
        TaskEventType::TaskProgress => "task_progress",
        TaskEventType::TaskWaitingForInput => "task_waiting_for_input",
        TaskEventType::TaskCompleted => "task_completed",
        TaskEventType::TaskFailed => "task_failed",
        TaskEventType::TaskCancelled => "task_cancelled",
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use domain::artifact::{ArtifactStatus, ArtifactType};
    use domain::task::TaskStatus;
    use domain::workspace::Workspace;

    use super::*;

    fn workspace() -> Workspace {
        Workspace {
            id: "workspace-1".to_string(),
            name: "Alpha Research".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn task(status: TaskStatus) -> AgentTask {
        AgentTask {
            id: "task-1".to_string(),
            workspace_id: "workspace-1".to_string(),
            title: "Analyze Tesla".to_string(),
            description: Some("Q4 earnings".to_string()),
            status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn event(event_type: TaskEventType, payload: Option<&str>) -> AgentTaskEvent {
        AgentTaskEvent {
            id: "event-1".to_string(),
            task_id: "task-1".to_string(),
            event_type,
            payload: payload.map(str::to_string),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn renders_workspaces_and_empty_states() {
        let rendered = render_workspaces(&[workspace()]);
        assert!(rendered.contains("workspace-1"));
        assert!(rendered.contains("Alpha Research"));

        assert_eq!(render_workspaces(&[]), "No workspaces.");
        assert_eq!(render_tasks(&[]), "No tasks.");
        assert_eq!(render_events(&[]), "No events.");
        assert_eq!(render_artifacts(&[]), "No artifacts.");
    }

    #[test]
    fn renders_task_metadata_and_description() {
        let rendered = render_task(&task(TaskStatus::Running));

        assert!(rendered.contains("task-1"));
        assert!(rendered.contains("running"));
        assert!(rendered.contains("Q4 earnings"));
    }

    #[test]
    fn renders_completion_output_from_the_event_timeline() {
        let events = vec![event(
            TaskEventType::TaskCompleted,
            Some(r#"{"summary":"Demand is stable","confidence":60}"#),
        )];

        let rendered = render_task_result(&task(TaskStatus::Completed), &events);

        assert!(rendered.contains("Demand is stable"));
        assert!(rendered.contains("\"confidence\": 60"));
    }

    #[test]
    fn renders_failure_message_from_the_event_timeline() {
        let events = vec![event(
            TaskEventType::TaskFailed,
            Some("The research provider request failed."),
        )];

        let rendered = render_task_result(&task(TaskStatus::Failed), &events);

        assert!(rendered.contains("Error: The research provider request failed."));
    }

    #[test]
    fn renders_artifact_rows() {
        let artifacts = vec![Artifact {
            id: "artifact-1".to_string(),
            task_id: Some("task-1".to_string()),
            workspace_id: "workspace-1".to_string(),
            artifact_type: ArtifactType::ComparisonTable,
            status: ArtifactStatus::Completed,
            input: serde_json::json!({}),
            output: None,
            error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }];

        let rendered = render_artifacts(&artifacts);

        assert!(rendered.contains("artifact-1"));
        assert!(rendered.contains("comparison_table"));
        assert!(rendered.contains("completed"));
    }

    #[test]
    fn truncates_long_values_with_an_ellipsis() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("abcdef", 4), "abc…");
        assert_eq!(truncate("日本語", 2), "日…");
    }

    #[test]
    fn prints_json_records() {
        let mut buffer = Vec::new();
        print_json(&mut buffer, &workspace()).expect("json output should succeed");

        let printed = String::from_utf8(buffer).expect("output should be utf-8");
        assert!(printed.contains("\"id\": \"workspace-1\""));
        assert!(printed.ends_with('\n'));
    }
}
