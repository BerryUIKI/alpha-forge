// Artifact inspection commands.

use std::io::Write;
use std::process::ExitCode;

use domain::artifact::Artifact;

use crate::cli::ArtifactCommand;
use crate::context::Services;
use crate::error::CliError;
use crate::output::{self, OutputFormat};

pub async fn run(
    services: &Services,
    command: ArtifactCommand,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<ExitCode, CliError> {
    match command {
        ArtifactCommand::List { workspace, task } => {
            let artifacts = list_artifacts(services, workspace.as_deref(), task.as_deref()).await?;

            match format {
                OutputFormat::Json => output::print_json(out, &artifacts)?,
                OutputFormat::Text => {
                    writeln!(out, "{}", output::render_artifacts(&artifacts))
                        .map_err(CliError::Output)?;
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

async fn list_artifacts(
    services: &Services,
    workspace: Option<&str>,
    task: Option<&str>,
) -> Result<Vec<Artifact>, CliError> {
    let artifacts = match (workspace, task) {
        (Some(workspace_id), None) => services.artifacts.list_artifacts(workspace_id).await?,
        (None, Some(task_id)) => services.artifacts.list_task_artifacts(task_id).await?,
        (Some(workspace_id), Some(task_id)) => {
            let mut artifacts = services.artifacts.list_task_artifacts(task_id).await?;
            artifacts.retain(|artifact| artifact.workspace_id == workspace_id);
            artifacts
        }
        (None, None) => services.artifacts.list_all_artifacts().await?,
    };

    Ok(artifacts)
}
