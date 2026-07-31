// Shared application state.
// Scope: database pool handle, services, configuration, and runtime handles.
// Agent / artifact / plugin systems will be added in later phases.

use sqlx::SqlitePool;

use crate::database::repositories::settings_repository::SettingsRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::services::settings_service::SettingsService;
use crate::services::workspace_service::WorkspaceService;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub settings_service: SettingsService,
    pub workspace_service: WorkspaceService,
}

impl AppState {
    pub fn new(db_pool: SqlitePool) -> Self {
        // Create repositories
        let settings_repo = SettingsRepository::new(db_pool.clone());
        let workspace_repo = WorkspaceRepository::new(db_pool.clone());

        // Create services
        let settings_service = SettingsService::new(settings_repo);
        let workspace_service = WorkspaceService::new(workspace_repo);

        Self {
            db_pool,
            settings_service,
            workspace_service,
        }
    }
}