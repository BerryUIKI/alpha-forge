// Shared application state.
// Scope: database pool handle, services, configuration, and runtime handles.
// Agent / artifact / plugin systems will be added in later phases.

use std::sync::Arc;

use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::agent::executor::{ExecutorConfig, TaskExecutor};
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::database::repositories::greeks_repository::GreeksRepository;
use crate::database::repositories::option_chain_repository::OptionChainRepository;
use crate::database::repositories::option_contract_repository::OptionContractRepository;
use crate::database::repositories::settings_repository::SettingsRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::services::agent_service::AgentService;
use crate::services::option_service::OptionService;
use crate::services::settings_service::SettingsService;
use crate::services::system_service::SystemService;
use crate::services::workspace_service::WorkspaceService;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub settings_service: SettingsService,
    pub workspace_service: WorkspaceService,
    pub agent_service: AgentService,
    pub system_service: SystemService,
    pub task_executor: Arc<TaskExecutor>,
    pub option_service: OptionService,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, app_handle: AppHandle) -> Self {
        // Create repositories
        let settings_repo = SettingsRepository::new(db_pool.clone());
        let workspace_repo = WorkspaceRepository::new(db_pool.clone());
        let agent_task_repo = AgentTaskRepository::new(db_pool.clone());
        let agent_task_repo_for_executor = AgentTaskRepository::new(db_pool.clone());

        // Option repositories
        let option_chain_repo = Arc::new(OptionChainRepository::new(db_pool.clone()));
        let option_contract_repo = Arc::new(OptionContractRepository::new(db_pool.clone()));
        let greeks_repo = Arc::new(GreeksRepository::new(db_pool.clone()));

        // Create services
        let settings_service = SettingsService::new(settings_repo);
        let workspace_service = WorkspaceService::new(workspace_repo);
        let agent_service = AgentService::new(agent_task_repo);
        let system_service = SystemService::new(app_handle.clone());

        // Create option service
        let option_service =
            OptionService::new(option_chain_repo, option_contract_repo, greeks_repo);

        // Create task executor
        let executor_config = ExecutorConfig::default();
        let task_executor = Arc::new(TaskExecutor::new(
            agent_task_repo_for_executor,
            app_handle.clone(),
            executor_config,
        ));

        Self {
            db_pool,
            settings_service,
            workspace_service,
            agent_service,
            system_service,
            task_executor,
            option_service,
        }
    }
}
