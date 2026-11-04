// AlphaForge CLI — drive the local research workspace from a terminal.

mod cli;
mod commands;
mod context;
mod output;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Command};
use crate::output::OutputFormat;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli).await {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(1)
        }
    }
}

async fn run(cli: Cli) -> Result<ExitCode> {
    let services = context::Services::open(cli.db.as_deref()).await?;
    let format = if cli.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    match cli.command {
        Command::Workspace { command } => {
            commands::workspace::run(&services, command, format).await
        }
        Command::Task { command } => commands::task::run(&services, command, format).await,
    }
}
