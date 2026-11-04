// Human-readable and JSON output.

use anyhow::Result;
use serde::Serialize;

use domain::task::{AgentTask, AgentTaskEvent, TaskEventType};
use domain::workspace::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub fn print_workspaces(workspaces: &[Workspace], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(&workspaces),
        OutputFormat::Text => {
            println!("{}", render_workspaces(workspaces));
            Ok(())
        }
    }
}

pub fn print_tasks(tasks: &[AgentTask], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(&tasks),
        OutputFormat::Text => {
            println!("{}", render_tasks(tasks));
            Ok(())
        }
    }
}

pub fn print_task(task: &AgentTask, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(task),
        OutputFormat::Text => {
            println!("{}", render_task(task));
            Ok(())
        }
    }
}

pub fn print_events(events: &[AgentTaskEvent], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(&events),
        OutputFormat::Text => {
            println!("{}", render_events(events));
            Ok(())
        }
    }
}

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

pub fn render_tasks(tasks: &[AgentTask]) -> String {
    if tasks.is_empty() {
        return "No tasks.".to_string();
    }

    let mut output = format!("{:<36}  {:<12}  TITLE\n", "ID", "STATUS");
    for task in tasks {
        output.push_str(&format!(
            "{:<36}  {:<12}  {}\n",
            task.id,
            task.status.to_string(),
            truncate(&task.title, 60)
        ));
    }

    output.trim_end().to_string()
}

pub fn render_task(task: &AgentTask) -> String {
    let mut output = format!(
        "ID:          {}\nWorkspace:   {}\nTitle:       {}\nStatus:      {}\nCreated:     {}\nUpdated:     {}\n",
        task.id,
        task.workspace_id,
        task.title,
        task.status,
        task.created_at.to_rfc3339(),
        task.updated_at.to_rfc3339()
    );

    if let Some(description) = &task.description {
        output.push_str(&format!("Description: {description}\n"));
    }
    if let Some(error) = &task.error {
        output.push_str(&format!("Error:       {error}\n"));
    }
    if let Some(output_text) = &task.output {
        output.push_str(&format!("Output:\n{output_text}\n"));
    }

    output.trim_end().to_string()
}

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
            event_name(&event.event_type),
            detail
        ));
    }

    output.trim_end().to_string()
}

fn event_name(event_type: &TaskEventType) -> &'static str {
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

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let mut truncated: String = value.chars().take(max_chars.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use domain::task::TaskStatus;

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
            output: None,
            error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_render_workspaces() {
        let rendered = render_workspaces(&[workspace()]);

        assert!(rendered.contains("workspace-1"));
        assert!(rendered.contains("Alpha Research"));
    }

    #[test]
    fn test_render_empty_collections() {
        assert_eq!(render_workspaces(&[]), "No workspaces.");
        assert_eq!(render_tasks(&[]), "No tasks.");
        assert_eq!(render_events(&[]), "No events.");
    }

    #[test]
    fn test_render_task_includes_error_and_output() {
        let mut task = task(TaskStatus::Failed);
        task.error = Some("provider unavailable".to_string());

        let rendered = render_task(&task);

        assert!(rendered.contains("task-1"));
        assert!(rendered.contains("failed"));
        assert!(rendered.contains("provider unavailable"));
    }

    #[test]
    fn test_render_events_includes_progress_payload() {
        let events = vec![AgentTaskEvent {
            id: "event-1".to_string(),
            task_id: "task-1".to_string(),
            event_type: TaskEventType::TaskProgress,
            payload: Some("Analyzing request...".to_string()),
            created_at: Utc::now(),
        }];

        let rendered = render_events(&events);

        assert!(rendered.contains("task_progress"));
        assert!(rendered.contains("Analyzing request..."));
    }

    #[test]
    fn test_truncate_keeps_short_values() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("abcdef", 4), "abc…");
    }

    #[test]
    fn test_task_serializes_to_json() {
        let json =
            serde_json::to_string(&task(TaskStatus::Completed)).expect("task should serialize");

        assert!(json.contains("\"id\":\"task-1\""));
        assert!(json.contains("\"status\":\"completed\""));
    }
}
