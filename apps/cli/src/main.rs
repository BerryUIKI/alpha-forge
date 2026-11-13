// The CLI keeps the console subsystem so stdin prompts and stdout/stderr work
// normally on every platform.

use std::process::ExitCode;

use alphaforge_cli::cli::Cli;
use alphaforge_cli::error::EXIT_ERROR;
use clap::Parser;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match alphaforge_cli::run(cli).await {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(EXIT_ERROR)
        }
    }
}
