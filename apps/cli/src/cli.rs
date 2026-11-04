// Command-line arguments.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "alphaforge",
    about = "Command-line interface for AlphaForge (Investment OS)",
    long_about = "Drive the local AlphaForge research workspace from a terminal.\n\n\
                  Exit codes: 0 success, 1 error, 2 task failed, 3 task cancelled."
)]
pub struct Cli {
    /// Path to the SQLite database. Defaults to the desktop app's data directory.
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,

    /// Print machine-readable JSON.
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage workspaces.
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommand,
    },
    /// Manage agent tasks.
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum WorkspaceCommand {
    /// List workspaces.
    List,
    /// Create a workspace.
    Create {
        /// Workspace name.
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
    /// List tasks in a workspace.
    List {
        /// Workspace ID.
        #[arg(long)]
        workspace: String,
    },
    /// Create a task.
    Create {
        /// Workspace ID.
        #[arg(long)]
        workspace: String,
        /// Task title.
        #[arg(long)]
        title: String,
        /// Optional task description.
        #[arg(long)]
        description: Option<String>,
    },
    /// Show a task and optionally its events.
    Show {
        /// Task ID.
        id: String,
        /// Include the task event timeline.
        #[arg(long)]
        events: bool,
    },
    /// Queue, start, and wait for a task.
    Run {
        /// Task ID.
        id: String,
        /// Cancel the task if it does not finish within this many seconds.
        #[arg(long)]
        timeout: Option<u64>,
    },
    /// Cancel a task and wait for it to stop.
    Cancel {
        /// Task ID.
        id: String,
    },
    /// Retry a failed or cancelled task and wait for it to finish.
    Retry {
        /// Task ID.
        id: String,
    },
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command, TaskCommand, WorkspaceCommand};

    #[test]
    fn test_parse_workspace_create() {
        let cli = Cli::parse_from(["alphaforge", "workspace", "create", "Alpha Research"]);

        match cli.command {
            Command::Workspace {
                command: WorkspaceCommand::Create { name },
            } => assert_eq!(name, "Alpha Research"),
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn test_parse_task_run_with_globals() {
        let cli = Cli::parse_from([
            "alphaforge",
            "--json",
            "--db",
            "test.db",
            "task",
            "run",
            "task-1",
            "--timeout",
            "30",
        ]);

        assert!(cli.json);
        assert_eq!(cli.db.as_deref(), Some(std::path::Path::new("test.db")));
        match cli.command {
            Command::Task {
                command: TaskCommand::Run { id, timeout },
            } => {
                assert_eq!(id, "task-1");
                assert_eq!(timeout, Some(30));
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }
}
