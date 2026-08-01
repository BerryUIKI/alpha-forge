// Shared application state.
// Scope: database pool handle, services, configuration, and runtime handles.
// Agent / artifact / plugin systems will be added in later phases.

use std::sync::Arc;

use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::agent::executor::{ExecutorConfig, TaskExecutor};
use crate::artifacts::manager::ArtifactManager;
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::database::repositories::artifact_repository::ArtifactRepository;
use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::database::repositories::research_project_repository::ResearchProjectRepository;
use crate::database::repositories::research_report_repository::ResearchReportRepository;
use crate::database::repositories::settings_repository::SettingsRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;
use crate::services::agent_service::AgentService;
use crate::services::artifact_service::ArtifactService;
use crate::services::research_document_service::ResearchDocumentService;
use crate::services::research_project_service::ResearchProjectService;
use crate::services::research_report_service::ResearchReportService;
use crate::services::settings_service::SettingsService;
use crate::services::system_service::SystemService;
use crate::services::workspace_service::WorkspaceService;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub settings_service: SettingsService,
    pub workspace_service: WorkspaceService,
    pub agent_service: AgentService,
    pub artifact_service: ArtifactService,
    pub research_project_service: ResearchProjectService,
    pub research_document_service: ResearchDocumentService,
    pub research_report_service: ResearchReportService,
    pub system_service: SystemService,
    pub task_executor: Arc<TaskExecutor>,
    pub artifact_manager: Arc<ArtifactManager>,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, app_handle: AppHandle) -> Self {
        // Create repositories
        let settings_repo = SettingsRepository::new(db_pool.clone());
        let workspace_repo = WorkspaceRepository::new(db_pool.clone());
        let agent_task_repo = AgentTaskRepository::new(db_pool.clone());
        let agent_task_repo_for_executor = AgentTaskRepository::new(db_pool.clone());
        let artifact_repo = ArtifactRepository::new(db_pool.clone());
        let research_project_repo = ResearchProjectRepository::new(db_pool.clone());
        let research_document_repo = ResearchDocumentRepository::new(db_pool.clone());
        let research_report_repo = ResearchReportRepository::new(db_pool.clone());

        // Create services
        let settings_service = SettingsService::new(settings_repo);
        let workspace_service = WorkspaceService::new(workspace_repo);
        let agent_service = AgentService::new(agent_task_repo);
        let artifact_service = ArtifactService::new(artifact_repo);
        let research_project_service = ResearchProjectService::new(research_project_repo);
        let research_document_service = ResearchDocumentService::new(research_document_repo);
        let research_report_service = ResearchReportService::new(research_report_repo);
        let system_service = SystemService::new(app_handle.clone());

        // Create task executor
        let executor_config = ExecutorConfig::default();
        let task_executor = Arc::new(TaskExecutor::new(agent_task_repo_for_executor, app_handle.clone(), executor_config));

        // Create artifact manager
        let artifact_manager = Arc::new(ArtifactManager::new(app_handle.clone()));

        Self {
            db_pool,
            settings_service,
            workspace_service,
            agent_service,
            artifact_service,
            research_project_service,
            research_document_service,
            research_report_service,
            system_service,
            task_executor,
            artifact_manager,
        }
    }
}