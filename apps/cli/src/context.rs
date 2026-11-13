// Service wiring for the CLI.
//
// The CLI deliberately links the desktop Rust library and reuses the same
// repositories, services, migrations, credential adapter, and agent executor as
// the Tauri app. It never starts a Tauri runtime and never opens SQLite on its
// own path.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use alpha_forge_lib::database::connection::create_pool;
use alpha_forge_lib::database::migrations;
use alpha_forge_lib::database::repositories::agent_task_repository::AgentTaskRepository;
use alpha_forge_lib::database::repositories::artifact_repository::ArtifactRepository;
use alpha_forge_lib::database::repositories::workspace_repository::WorkspaceRepository;
use alpha_forge_lib::error::AppError;
use alpha_forge_lib::providers::ai::OpenAiResearchProvider;
use alpha_forge_lib::security::credentials::{CredentialStore, OsKeychainCredentialStore};
use alpha_forge_lib::services::agent_service::AgentService;
use alpha_forge_lib::services::artifact_service::ArtifactService;
use alpha_forge_lib::services::workspace_service::WorkspaceService;
use provider_core::ResearchProvider;
use sqlx::SqlitePool;

use crate::error::CliError;

/// Bundle identifier used by the desktop app.
///
/// Tauri resolves `app_data_dir()` to the platform data directory joined with
/// this identifier. The CLI mirrors that rule so both front ends address the
/// same database unless `--db` is supplied.
pub const BUNDLE_IDENTIFIER: &str = "com.berry.alphaforge";

/// Database file name created by the desktop bootstrap.
pub const DATABASE_FILE: &str = "alpha_forge.db";

/// Services shared by CLI commands.
pub struct Services {
    pub workspaces: Arc<WorkspaceService>,
    pub agent: Arc<AgentService>,
    pub artifacts: Arc<ArtifactService>,
    pub credentials: Arc<dyn CredentialStore + Send + Sync>,
    pub provider: Arc<dyn ResearchProvider>,
    pool: SqlitePool,
}

impl Services {
    /// Opens the desktop database with the production credential store and
    /// OpenAI provider.
    pub async fn open(db_path: Option<&Path>) -> Result<Self, CliError> {
        let credentials: Arc<dyn CredentialStore + Send + Sync> =
            Arc::new(OsKeychainCredentialStore);
        let provider: Arc<dyn ResearchProvider> =
            Arc::new(OpenAiResearchProvider::new().map_err(|_| {
                AppError::Internal("could not initialize the OpenAI research provider".to_string())
            })?);

        Self::open_with(db_path, credentials, provider).await
    }

    /// Opens the database with injected dependencies.
    ///
    /// Used by integration tests to substitute an in-memory credential store and
    /// a deterministic research provider.
    pub async fn open_with(
        db_path: Option<&Path>,
        credentials: Arc<dyn CredentialStore + Send + Sync>,
        provider: Arc<dyn ResearchProvider>,
    ) -> Result<Self, CliError> {
        let path = resolve_db_path(db_path)?;

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(CliError::DatabaseDirectory)?;
            }
        }

        let db_url = format!("sqlite:{}?mode=rwc", path.display());
        let pool = create_pool(&db_url).await?;
        migrations::run(&pool).await?;

        Ok(Self::from_pool(pool, credentials, provider))
    }

    /// Builds services from an existing pool.
    pub fn from_pool(
        pool: SqlitePool,
        credentials: Arc<dyn CredentialStore + Send + Sync>,
        provider: Arc<dyn ResearchProvider>,
    ) -> Self {
        let workspaces = WorkspaceService::new(WorkspaceRepository::new(pool.clone()));
        let agent = AgentService::new(AgentTaskRepository::new(pool.clone()));
        let artifacts = ArtifactService::new(ArtifactRepository::new(pool.clone()));

        Self {
            workspaces: Arc::new(workspaces),
            agent: Arc::new(agent),
            artifacts: Arc::new(artifacts),
            credentials,
            provider,
            pool,
        }
    }

    /// Returns a pool handle for callers that must construct a repository
    /// instance (for example the agent task executor).
    pub fn database_pool(&self) -> SqlitePool {
        self.pool.clone()
    }
}

/// Resolves the database path used by a command.
pub fn resolve_db_path(override_path: Option<&Path>) -> Result<PathBuf, CliError> {
    if let Some(path) = override_path {
        if path.as_os_str().is_empty() {
            return Err(AppError::Validation("Database path cannot be empty".to_string()).into());
        }
        return Ok(path.to_path_buf());
    }

    default_db_path()
}

/// Mirrors Tauri's `app_data_dir()` for the AlphaForge bundle.
pub fn default_db_path() -> Result<PathBuf, CliError> {
    let base = platform_data_dir().ok_or(CliError::DataDirectoryUnavailable)?;
    Ok(base.join(BUNDLE_IDENTIFIER).join(DATABASE_FILE))
}

fn platform_data_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|home| {
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
        })
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(|home| PathBuf::from(home).join(".local").join("share"))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_path_wins_and_is_preserved() {
        let path = Path::new("relative/test.db");
        assert_eq!(resolve_db_path(Some(path)).unwrap().as_path(), path);
    }

    #[test]
    fn empty_explicit_path_is_rejected() {
        assert!(matches!(
            resolve_db_path(Some(Path::new(""))),
            Err(CliError::App(AppError::Validation(_)))
        ));
    }

    #[test]
    fn default_path_ends_with_the_bundle_directory_and_database_file() {
        if platform_data_dir().is_none() {
            // The host does not expose the platform data directory in this
            // environment; the error path is covered by the explicit-path tests.
            return;
        }

        let path = default_db_path().unwrap();
        assert!(path.ends_with(Path::new(BUNDLE_IDENTIFIER).join(DATABASE_FILE)));
    }

    #[test]
    fn missing_platform_data_directory_is_reported() {
        // Only meaningful on hosts where the expected variable is absent.
        if platform_data_dir().is_some() {
            return;
        }
        assert!(matches!(
            default_db_path(),
            Err(CliError::DataDirectoryUnavailable)
        ));
    }
}
