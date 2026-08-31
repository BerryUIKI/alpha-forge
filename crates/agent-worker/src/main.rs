use clap::Parser;
use std::process::ExitCode;

mod fixture_runner;
use fixture_runner::{FixtureMode, FixtureWorker};

#[derive(Parser, Debug)]
#[command(
    name = "alphaforge-agent-worker",
    about = "Supervised subprocess worker runtime for AlphaForge Agent tasks",
    version
)]
struct Cli {
    /// Run worker in deterministic fixture simulation mode for tests
    #[arg(long, value_enum)]
    fixture_mode: Option<FixtureMode>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Some(mode) = cli.fixture_mode {
        let worker = FixtureWorker::new(mode);
        if let Err(e) = worker.run() {
            eprintln!("[alphaforge-agent-worker] Error: {}", e);
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }

    // Default interactive loop (or fallback)
    let worker = FixtureWorker::new(FixtureMode::Success);
    if let Err(e) = worker.run() {
        eprintln!("[alphaforge-agent-worker] Error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
