// Workspace commands.

use std::process::ExitCode;

use anyhow::Result;
use domain::workspace::CreateWorkspaceInput;

use crate::cli::WorkspaceCommand;
use crate::context::Services;
use crate::output::{self, OutputFormat};

pub async fn run(
    services: &Services,
    command: WorkspaceCommand,
    format: OutputFormat,
) -> Result<ExitCode> {
    match command {
        WorkspaceCommand::List => {
            let workspaces = services.workspaces.list().await?;
            output::print_workspaces(&workspaces, format)?;
        }
        WorkspaceCommand::Create { name } => {
            let workspace = services
                .workspaces
                .create(CreateWorkspaceInput { name })
                .await?;

            match format {
                OutputFormat::Json => output::print_json(&workspace)?,
                OutputFormat::Text => {
                    println!("Created workspace {} ({})", workspace.name, workspace.id);
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
