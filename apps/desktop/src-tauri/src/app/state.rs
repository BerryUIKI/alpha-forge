// Shared application state.
// Scope: database pool handle, services, configuration, and runtime handles.
// Agent / artifact / plugin systems will be added in later phases.

use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::database::repositories::settings_repository::SettingsRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::services::agent_service::AgentService;
use crate::services::settings_service::SettingsService;
use crate::services::system_service::SystemService;
use crate::services::workspace_service::WorkspaceService;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub settings_service: SettingsService,
    pub workspace_service: WorkspaceService,
    pub agent_service: AgentService,
    pub system_service: SystemService,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, app_handle: AppHandle) -> Self {
        // Create repositories
        let settings_repo = SettingsRepository::new(db_pool.clone());
        let workspace_repo = WorkspaceRepository::new(db_pool.clone());
        let agent_task_repo = AgentTaskRepository::new(db_pool.clone());

        // Create services
        let settings_service = SettingsService::new(settings_repo);
        let workspace_service = WorkspaceService::new(workspace_repo);
        let agent_service = AgentService::new(agent_task_repo);
        let system_service = SystemService::new(app_handle);

        Self {
            db_pool,
            settings_service,
            workspace_service,
            agent_service,
            system_service,
        }
    }
}
