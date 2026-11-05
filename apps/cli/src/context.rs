// Service wiring for the CLI.
//
// The CLI talks to the same SQLite database and service layer as the desktop
// app. It never creates a Tauri runtime; background task events are discarded.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use sqlx::SqlitePool;

use domain::task::AgentTaskEvent;
use investment_os_lib::agent::context::WorkspaceContextProvider;
use investment_os_lib::agent::events::EventPublisher;
use investment_os_lib::agent::openai_runtime::OpenAiRuntime;
use investment_os_lib::agent::tools::{
    CalculatorTool, GetTaskOutputTool, ListTasksTool, SearchTasksTool, ToolRegistry,
};
use investment_os_lib::database::connection::create_pool;
use investment_os_lib::database::migrations;
use investment_os_lib::database::repositories::agent_task_repository::AgentTaskRepository;
use investment_os_lib::database::repositories::artifact_repository::ArtifactRepository;
use investment_os_lib::database::repositories::workspace_repository::WorkspaceRepository;
use investment_os_lib::error::AppError;
use investment_os_lib::providers::ai::{AiClient, OpenAiClient, DEFAULT_OPENAI_MODEL};
use investment_os_lib::security::credentials::{CredentialStore, KeyringCredentialStore};
use investment_os_lib::services::agent_service::AgentService;
use investment_os_lib::services::artifact_service::ArtifactService;
use investment_os_lib::services::provider_service::ProviderService;
use investment_os_lib::services::workspace_service::WorkspaceService;

/// Bundle identifier used by the desktop app; also the keychain service name.
pub const APP_IDENTIFIER: &str = "com.berry.investmentos";

const DB_FILE: &str = "investment_os.db";

/// Services shared by CLI commands.
pub struct Services {
    pub workspaces: WorkspaceService,
    pub agent: Arc<AgentService>,
    pub provider: ProviderService,
    pub artifacts: ArtifactService,
}

impl Services {
    /// Opens the database and builds the service layer.
    pub async fn open(db_path: Option<&Path>) -> Result<Self> {
        let path = match db_path {
            Some(path) => path.to_path_buf(),
            None => default_db_path()?,
        };

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let db_url = format!("sqlite:{}?mode=rwc", path.display());
        let pool = create_pool(&db_url).await?;
        migrations::run(&pool).await?;

        Ok(Self::from_pool(pool))
    }

    /// Builds services from an existing pool.
    pub fn from_pool(pool: SqlitePool) -> Self {
        let credentials: Arc<dyn CredentialStore> =
            Arc::new(KeyringCredentialStore::new(APP_IDENTIFIER));
        let model = DEFAULT_OPENAI_MODEL.to_string();
        let client: Arc<dyn AiClient> =
            Arc::new(OpenAiClient::new(Arc::clone(&credentials), model.clone()));
        let provider = ProviderService::new(credentials, model);

        let tasks = AgentTaskRepository::new(pool.clone());
        let artifact_service = ArtifactService::new(ArtifactRepository::new(pool.clone()));
        let workspaces = WorkspaceRepository::new(pool);

        let context = Arc::new(WorkspaceContextProvider::new(
            workspaces.clone(),
            tasks.clone(),
        ));
        let tools = ToolRegistry::new(vec![
            Arc::new(ListTasksTool::new(tasks.clone())),
            Arc::new(SearchTasksTool::new(tasks.clone())),
            Arc::new(GetTaskOutputTool::new(tasks.clone())),
            Arc::new(CalculatorTool::new()),
        ]);

        let agent = AgentService::new(
            tasks,
            Arc::new(OpenAiRuntime::new(client, context, tools)),
            Arc::new(NoopPublisher),
            artifact_service.clone(),
        );

        Self {
            workspaces: WorkspaceService::new(workspaces),
            agent: Arc::new(agent),
            provider,
            artifacts: artifact_service,
        }
    }
}

/// Event publisher that discards events; the CLI prints progress directly.
struct NoopPublisher;

impl EventPublisher for NoopPublisher {
    fn publish(&self, _event: &AgentTaskEvent) -> Result<(), AppError> {
        Ok(())
    }
}

/// Mirrors Tauri's app data directory for the desktop bundle.
pub fn default_db_path() -> Result<PathBuf> {
    let base = if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .ok_or_else(|| anyhow!("APPDATA is not set"))?
    } else if cfg!(target_os = "macos") {
        home_dir()?.join("Library").join("Application Support")
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or(home_dir()?.join(".local").join("share"))
    };

    Ok(base.join(APP_IDENTIFIER).join(DB_FILE))
}

fn home_dir() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("HOME is not set"))
}
