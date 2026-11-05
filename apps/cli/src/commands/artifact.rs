// Artifact inspection commands.

use std::process::ExitCode;

use anyhow::Result;

use crate::cli::ArtifactCommand;
use crate::context::Services;
use crate::output::{self, OutputFormat};

pub async fn run(
    services: &Services,
    command: ArtifactCommand,
    format: OutputFormat,
) -> Result<ExitCode> {
    match command {
        ArtifactCommand::List { workspace, task } => {
            let artifacts = services
                .artifacts
                .list(workspace.as_deref(), task.as_deref())
                .await?;

            match format {
                OutputFormat::Json => output::print_json(&artifacts)?,
                OutputFormat::Text => {
                    if artifacts.is_empty() {
                        println!("No artifacts.");
                    } else {
                        println!("{:<36}  {:<18}  TITLE", "ID", "TYPE");
                        for artifact in artifacts {
                            println!(
                                "{:<36}  {:<18}  {}",
                                artifact.id, artifact.artifact_type, artifact.title
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
