// Workspace commands.

use std::io::Write;
use std::process::ExitCode;

use domain::workspace::CreateWorkspaceInput;

use crate::cli::WorkspaceCommand;
use crate::context::Services;
use crate::error::CliError;
use crate::output::{self, OutputFormat};

pub async fn run(
    services: &Services,
    command: WorkspaceCommand,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<ExitCode, CliError> {
    match command {
        WorkspaceCommand::List => {
            let workspaces = services.workspaces.list().await?;
            match format {
                OutputFormat::Json => output::print_json(out, &workspaces)?,
                OutputFormat::Text => {
                    writeln!(out, "{}", output::render_workspaces(&workspaces))
                        .map_err(CliError::Output)?;
                }
            }
        }
        WorkspaceCommand::Create { name } => {
            let workspace = services
                .workspaces
                .create(CreateWorkspaceInput { name })
                .await?;

            match format {
                OutputFormat::Json => output::print_json(out, &workspace)?,
                OutputFormat::Text => {
                    writeln!(
                        out,
                        "Created workspace {} ({})",
                        workspace.name, workspace.id
                    )
                    .map_err(CliError::Output)?;
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
