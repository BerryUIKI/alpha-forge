// AlphaForge CLI — drive the local research workspace from a terminal.
//
// The library is intentionally thin: argument parsing, service wiring, output
// formatting, and command handlers. All persistence and execution flows through
// the desktop Rust library so the CLI cannot drift from the application.

pub mod cli;
pub mod commands;
pub mod context;
pub mod error;
pub mod events;
pub mod output;

use std::io::Write;
use std::process::ExitCode;

use crate::cli::{Cli, Command};
use crate::context::Services;
use crate::error::CliError;
use crate::output::OutputFormat;

/// Executes one parsed command and returns the process exit code.
pub async fn run(cli: Cli) -> Result<ExitCode, CliError> {
    let services = Services::open(cli.db.as_deref()).await?;
    let format = if cli.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    let mut stdout = std::io::stdout().lock();
    let exit_code = match cli.command {
        Command::Workspace { command } => {
            commands::workspace::run(&services, command, format, &mut stdout).await?
        }
        Command::Task { command } => {
            commands::task::run(&services, command, format, &mut stdout).await?
        }
        Command::Provider { command } => {
            commands::provider::run(&services, command, format, &mut stdout)?
        }
        Command::Artifact { command } => {
            commands::artifact::run(&services, command, format, &mut stdout).await?
        }
    };
    stdout.flush().map_err(CliError::Output)?;

    Ok(exit_code)
}
