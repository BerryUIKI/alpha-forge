// AI provider configuration commands.
//
// The CLI never prints, echoes, or returns a stored secret. `provider status`
// reports only whether a key exists, and `provider set-key` reads the key with
// terminal echo disabled before handing it to the shared keychain adapter.

use std::io::Write;
use std::process::ExitCode;

use alpha_forge_lib::error::AppError;
use alpha_forge_lib::providers::ai::DEFAULT_OPENAI_MODEL;
use alpha_forge_lib::security::credentials::{
    delete_openai_api_key, has_openai_api_key, save_openai_api_key,
};
use serde::Serialize;

use crate::cli::ProviderCommand;
use crate::context::Services;
use crate::error::CliError;
use crate::output::{self, OutputFormat};

const PROVIDER_NAME: &str = "openai";
const API_KEY_PROMPT: &str = "OpenAI API key: ";

/// Provider configuration summary exposed as text or JSON.
#[derive(Debug, Serialize)]
pub struct ProviderStatus {
    pub provider: &'static str,
    pub model: &'static str,
    pub configured: bool,
}

pub fn run(
    services: &Services,
    command: ProviderCommand,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<ExitCode, CliError> {
    match command {
        ProviderCommand::Status => {
            print_status(services, format, out)?;
        }
        ProviderCommand::SetKey => {
            let api_key =
                rpassword::prompt_password(API_KEY_PROMPT).map_err(CliError::CredentialInput)?;
            let api_key = api_key.trim();

            if api_key.is_empty() {
                return Err(
                    AppError::Validation("OpenAI API key cannot be empty".to_string()).into(),
                );
            }

            if let Err(error) = save_openai_api_key(&*services.credentials, api_key) {
                return Err(error.into());
            }

            match format {
                OutputFormat::Json => print_status(services, format, out)?,
                OutputFormat::Text => {
                    writeln!(out, "API key saved to the OS keychain.").map_err(CliError::Output)?;
                }
            }
        }
        ProviderCommand::ClearKey => {
            delete_openai_api_key(&*services.credentials)?;

            match format {
                OutputFormat::Json => print_status(services, format, out)?,
                OutputFormat::Text => {
                    writeln!(out, "API key removed.").map_err(CliError::Output)?;
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn print_status(
    services: &Services,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    let status = current_status(services)?;

    match format {
        OutputFormat::Json => output::print_json(out, &status)?,
        OutputFormat::Text => {
            writeln!(out, "{}", render_status(&status)).map_err(CliError::Output)?;
        }
    }

    Ok(())
}

fn current_status(services: &Services) -> Result<ProviderStatus, CliError> {
    let configured = has_openai_api_key(&*services.credentials)?;
    Ok(ProviderStatus {
        provider: PROVIDER_NAME,
        model: DEFAULT_OPENAI_MODEL,
        configured,
    })
}

/// Renders the provider status without exposing the credential itself.
pub fn render_status(status: &ProviderStatus) -> String {
    format!(
        "{}: {}\nmodel: {}",
        status.provider,
        if status.configured {
            "configured"
        } else {
            "not configured"
        },
        status.model
    )
}

#[cfg(test)]
mod tests {
    use super::{render_status, ProviderStatus};

    #[test]
    fn renders_configured_and_missing_states() {
        let configured = ProviderStatus {
            provider: "openai",
            model: "test-model",
            configured: true,
        };
        let missing = ProviderStatus {
            provider: "openai",
            model: "test-model",
            configured: false,
        };

        assert_eq!(
            render_status(&configured),
            "openai: configured\nmodel: test-model"
        );
        assert_eq!(
            render_status(&missing),
            "openai: not configured\nmodel: test-model"
        );
    }
}
