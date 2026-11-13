// Command-line arguments and subcommands.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Top-level `alphaforge` invocation.
#[derive(Debug, Parser)]
#[command(
    name = "alphaforge",
    version,
    about = "Command-line interface for the AlphaForge research workspace",
    long_about = "Drive the local AlphaForge research workspace from a terminal.\n\n\
                  The CLI talks to the same SQLite database and Rust service layer as the \
                  desktop app. It is a research tool: it never places trades or makes \
                  investment decisions.\n\n\
                  Exit codes: 0 success, 1 error, 2 task failed, 3 task cancelled."
)]
pub struct Cli {
    /// Path to the SQLite database. Defaults to the desktop app's data directory.
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,

    /// Print machine-readable JSON instead of human-readable text.
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

/// Top-level command families.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage workspaces.
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommand,
    },
    /// Manage agent research tasks.
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },
    /// Manage the AI provider configuration.
    Provider {
        #[command(subcommand)]
        command: ProviderCommand,
    },
    /// Inspect stored artifacts.
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
}

/// Workspace commands.
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

/// Agent task commands.
#[derive(Debug, Subcommand)]
pub enum TaskCommand {
    /// List tasks in a workspace.
    List {
        /// Workspace ID.
        #[arg(long)]
        workspace: String,
    },
    /// Create a task in a workspace.
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
    /// Start a task and wait for it to finish.
    Start {
        /// Task ID.
        id: String,
        /// Cancel the task when it has not finished within this many seconds.
        #[arg(long)]
        timeout: Option<u64>,
    },
    /// Cancel a task that has not reached a terminal state.
    Cancel {
        /// Task ID.
        id: String,
    },
}

/// Provider commands.
#[derive(Debug, Subcommand)]
pub enum ProviderCommand {
    /// Show the provider configuration status.
    Status,
    /// Save the OpenAI API key in the OS keychain (prompted without echo).
    SetKey,
    /// Remove the stored OpenAI API key.
    ClearKey,
}

/// Artifact commands.
#[derive(Debug, Subcommand)]
pub enum ArtifactCommand {
    /// List stored artifacts.
    List {
        /// Only show artifacts in this workspace.
        #[arg(long)]
        workspace: Option<String>,
        /// Only show artifacts produced by this task.
        #[arg(long)]
        task: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{ArtifactCommand, Cli, Command, ProviderCommand, TaskCommand, WorkspaceCommand};

    #[test]
    fn parses_workspace_create() {
        let cli = Cli::parse_from(["alphaforge", "workspace", "create", "Alpha Research"]);

        match cli.command {
            Command::Workspace {
                command: WorkspaceCommand::Create { name },
            } => assert_eq!(name, "Alpha Research"),
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn parses_globals_and_task_start() {
        let cli = Cli::parse_from([
            "alphaforge",
            "--json",
            "--db",
            "test.db",
            "task",
            "start",
            "task-1",
            "--timeout",
            "30",
        ]);

        assert!(cli.json);
        assert_eq!(cli.db.as_deref(), Some(std::path::Path::new("test.db")));
        match cli.command {
            Command::Task {
                command: TaskCommand::Start { id, timeout },
            } => {
                assert_eq!(id, "task-1");
                assert_eq!(timeout, Some(30));
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn parses_provider_commands() {
        let status = Cli::parse_from(["alphaforge", "provider", "status"]);
        assert!(matches!(
            status.command,
            Command::Provider {
                command: ProviderCommand::Status
            }
        ));

        let set_key = Cli::parse_from(["alphaforge", "provider", "set-key"]);
        assert!(matches!(
            set_key.command,
            Command::Provider {
                command: ProviderCommand::SetKey
            }
        ));

        let clear_key = Cli::parse_from(["alphaforge", "provider", "clear-key"]);
        assert!(matches!(
            clear_key.command,
            Command::Provider {
                command: ProviderCommand::ClearKey
            }
        ));
    }

    #[test]
    fn parses_artifact_filters() {
        let cli = Cli::parse_from([
            "alphaforge",
            "artifact",
            "list",
            "--workspace",
            "ws-1",
            "--task",
            "task-1",
        ]);

        match cli.command {
            Command::Artifact {
                command: ArtifactCommand::List { workspace, task },
            } => {
                assert_eq!(workspace.as_deref(), Some("ws-1"));
                assert_eq!(task.as_deref(), Some("task-1"));
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn parses_task_create_and_cancel() {
        let create = Cli::parse_from([
            "alphaforge",
            "task",
            "create",
            "--workspace",
            "ws-1",
            "--title",
            "Assess demand",
            "--description",
            "Compare demand drivers.",
        ]);
        match create.command {
            Command::Task {
                command:
                    TaskCommand::Create {
                        workspace,
                        title,
                        description,
                    },
            } => {
                assert_eq!(workspace, "ws-1");
                assert_eq!(title, "Assess demand");
                assert_eq!(description.as_deref(), Some("Compare demand drivers."));
            }
            other => panic!("unexpected command: {other:?}"),
        }

        let cancel = Cli::parse_from(["alphaforge", "task", "cancel", "task-1"]);
        assert!(matches!(
            cancel.command,
            Command::Task {
                command: TaskCommand::Cancel { id }
            } if id == "task-1"
        ));
    }
}
