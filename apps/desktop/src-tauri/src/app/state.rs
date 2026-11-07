// Shared application state.
// Scope: database pool handle, services, configuration, and runtime handles.

use std::sync::Arc;

use sqlx::SqlitePool;

use crate::agent::context::WorkspaceContextProvider;
use crate::agent::events::TauriEventPublisher;
use crate::agent::openai_runtime::OpenAiRuntime;
use crate::agent::tools::{
    CalculatorTool, GetResearchDocumentTool, GetTaskOutputTool, ListTasksTool, SearchResearchTool,
    SearchTasksTool, ToolRegistry,
};
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::database::repositories::artifact_repository::ArtifactRepository;
use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::database::repositories::settings_repository::SettingsRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::providers::ai::{AiClient, OpenAiClient, DEFAULT_OPENAI_MODEL};
use crate::security::credentials::{CredentialStore, KeyringCredentialStore};
use crate::services::agent_service::AgentService;
use crate::services::artifact_service::ArtifactService;
use crate::services::provider_service::ProviderService;
use crate::services::research_service::ResearchService;
use crate::services::settings_service::SettingsService;
use crate::services::workspace_service::WorkspaceService;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub settings_service: SettingsService,
    pub workspace_service: WorkspaceService,
    pub provider_service: ProviderService,
    pub artifact_service: ArtifactService,
    pub research_service: ResearchService,
    pub agent_service: AgentService,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, app_handle: tauri::AppHandle) -> Self {
        // Create repositories
        let settings_repo = SettingsRepository::new(db_pool.clone());
        let workspace_repo = WorkspaceRepository::new(db_pool.clone());
        let agent_task_repo = AgentTaskRepository::new(db_pool.clone());
        let artifact_repo = ArtifactRepository::new(db_pool.clone());
        let research_repo = ResearchDocumentRepository::new(db_pool.clone());

        // Provider credentials live in the OS keychain, never in the database.
        let credentials: Arc<dyn CredentialStore> = Arc::new(KeyringCredentialStore::new(
            app_handle.config().identifier.clone(),
        ));
        let model = DEFAULT_OPENAI_MODEL.to_string();
        let ai_client: Arc<dyn AiClient> =
            Arc::new(OpenAiClient::new(Arc::clone(&credentials), model.clone()));

        // Context and tools read from the same repositories as the services.
        let context = Arc::new(WorkspaceContextProvider::new(
            workspace_repo.clone(),
            agent_task_repo.clone(),
            research_repo.clone(),
        ));
        let tools = ToolRegistry::new(vec![
            Arc::new(ListTasksTool::new(agent_task_repo.clone())),
            Arc::new(SearchTasksTool::new(agent_task_repo.clone())),
            Arc::new(GetTaskOutputTool::new(agent_task_repo.clone())),
            Arc::new(SearchResearchTool::new(research_repo.clone())),
            Arc::new(GetResearchDocumentTool::new(research_repo.clone())),
            Arc::new(CalculatorTool::new()),
        ]);

        // Create services
        let settings_service = SettingsService::new(settings_repo);
        let workspace_service = WorkspaceService::new(workspace_repo);
        let provider_service = ProviderService::new(Arc::clone(&credentials), model);
        let artifact_service = ArtifactService::new(artifact_repo);
        let research_service = ResearchService::new(research_repo.clone());
        let agent_service = AgentService::new(
            agent_task_repo,
            Arc::new(OpenAiRuntime::new(ai_client, context, tools)),
            Arc::new(TauriEventPublisher::new(app_handle)),
            artifact_service.clone(),
        );

        Self {
            db_pool,
            settings_service,
            workspace_service,
            provider_service,
            artifact_service,
            research_service,
            agent_service,
        }
    }
}
