// AI provider configuration commands.

use std::process::ExitCode;

use anyhow::Result;

use crate::cli::ProviderCommand;
use crate::context::Services;
use crate::output::{self, OutputFormat};

pub fn run(
    services: &Services,
    command: ProviderCommand,
    format: OutputFormat,
) -> Result<ExitCode> {
    match command {
        ProviderCommand::Status => {
            let status = services.provider.status()?;
            match format {
                OutputFormat::Json => output::print_json(&status)?,
                OutputFormat::Text => println!(
                    "{}: {} (model: {})",
                    status.provider,
                    if status.configured {
                        "configured"
                    } else {
                        "not configured"
                    },
                    status.model
                ),
            }
        }
        ProviderCommand::SetKey => {
            let api_key = rpassword::prompt_password("OpenAI API key: ")?;
            services.provider.set_api_key(&api_key)?;
            println!("API key saved to the OS keychain.");
        }
        ProviderCommand::ClearKey => {
            services.provider.clear_api_key()?;
            println!("API key removed.");
        }
    }

    Ok(ExitCode::SUCCESS)
}
