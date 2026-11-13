// CLI error type and exit-code contract.

/// The command failed before producing a result.
pub const EXIT_ERROR: u8 = 1;
/// A started task ended in the `failed` state.
pub const EXIT_TASK_FAILED: u8 = 2;
/// A started task ended in the `cancelled` state.
pub const EXIT_TASK_CANCELLED: u8 = 3;

/// Errors surfaced by the CLI.
///
/// Domain and service failures are wrapped in the desktop library's `AppError`
/// so the CLI keeps the same stable error contract as the application instead of
/// inventing a parallel error taxonomy.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(transparent)]
    App(#[from] alpha_forge_lib::error::AppError),

    #[error(
        "could not resolve the AlphaForge data directory; pass --db <path> to select a database"
    )]
    DataDirectoryUnavailable,

    #[error("could not prepare the database directory: {0}")]
    DatabaseDirectory(#[source] std::io::Error),

    #[error("could not read the API key from the terminal: {0}")]
    CredentialInput(#[source] std::io::Error),

    #[error("could not write command output: {0}")]
    Output(#[source] std::io::Error),
}
