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
use crate::database::repositories::greeks_repository::GreeksRepository;
use crate::database::repositories::knowledge_graph_repository::KnowledgeGraphRepository;
use crate::database::repositories::option_chain_repository::OptionChainRepository;
use crate::database::repositories::option_contract_repository::OptionContractRepository;
use crate::database::repositories::option_position_repository::OptionPositionRepository;
use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
use crate::database::repositories::plugin_repository::PluginRepository;
use crate::database::repositories::portfolio_repository::PortfolioRepository;
use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::database::repositories::research_note_repository::ResearchNoteRepository;
use crate::database::repositories::research_project_repository::ResearchProjectRepository;
use crate::database::repositories::research_report_repository::ResearchReportRepository;
use crate::database::repositories::research_source_repository::ResearchSourceRepository;
use crate::database::repositories::settings_repository::SettingsRepository;
use crate::database::repositories::strategy_leg_repository::StrategyLegRepository;
use crate::database::repositories::thesis_repository::ThesisRepository;
use crate::database::repositories::workspace_repository::WorkspaceRepository;

// Financial repositories (Phase 2 — Wealthfolio port)
use crate::database::repositories::account_repository::{AccountRepository, PlatformRepository};
use crate::database::repositories::activity_repository::{ActivityRepository, ImportRunRepository};
use crate::database::repositories::allocation_target_repository::AllocationTargetRepository;
use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
use crate::database::repositories::snapshot_repository::SnapshotRepository;
use crate::database::repositories::taxonomy_repository::TaxonomyRepository;
use crate::database::repositories::valuation_repository::ValuationRepository;

use crate::error::AppError;
use crate::providers::ai::OpenAiResearchProvider;
use crate::services::agent_service::AgentService;
use crate::services::allocation_service::AllocationService;
use crate::services::artifact_service::ArtifactService;
use crate::services::goose_service::GooseService;
use crate::services::holdings_service::HoldingsService;
use crate::services::knowledge_graph_service::KnowledgeGraphService;
use crate::services::lot_service::LotService;
use crate::services::net_worth_service::NetWorthService;
use crate::services::option_service::OptionService;
use crate::services::performance_service::PerformanceService;
use crate::services::plugin_service::PluginService;
use crate::services::portfolio_option_service::PortfolioOptionService;
use crate::services::portfolio_service::PortfolioService;
use crate::services::research_document_service::ResearchDocumentService;
use crate::services::research_note_service::ResearchNoteService;
use crate::services::research_project_service::ResearchProjectService;
use crate::services::research_report_service::ResearchReportService;
use crate::services::research_source_service::ResearchSourceService;
use crate::services::settings_service::SettingsService;
use crate::services::snapshot_service::SnapshotService;
use crate::services::strategy_service::StrategyService;
use crate::services::system_service::SystemService;
use crate::services::thesis_service::ThesisService;
use crate::services::valuation_service::ValuationService;
use crate::services::workspace_service::WorkspaceService;
use provider_core::ResearchProvider;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub settings_service: SettingsService,
    pub workspace_service: WorkspaceService,
    pub agent_service: AgentService,
    pub artifact_service: ArtifactService,
    pub research_project_service: ResearchProjectService,
    pub research_document_service: ResearchDocumentService,
    pub research_note_service: ResearchNoteService,
    pub research_report_service: ResearchReportService,
    pub research_source_service: ResearchSourceService,
    pub thesis_service: ThesisService,
    pub knowledge_graph_service: KnowledgeGraphService,
    pub option_service: OptionService,
    pub strategy_service: StrategyService,
    pub portfolio_option_service: PortfolioOptionService,
    pub portfolio_service: PortfolioService,
    pub plugin_service: PluginService,
    pub system_service: SystemService,
    // Financial repositories (Phase 3.5 — CRUD commands)
    pub platform_repo: Arc<PlatformRepository>,
    pub account_repo: Arc<AccountRepository>,
    pub asset_repo: Arc<AssetRepository>,
    pub quote_repo: Arc<QuoteRepository>,
    pub activity_repo: Arc<ActivityRepository>,
    pub import_run_repo: Arc<ImportRunRepository>,
    pub lot_repo: Arc<LotRepository>,
    pub disposal_repo: Arc<LotDisposalRepository>,
    pub valuation_repo: Arc<ValuationRepository>,
    pub taxonomy_repo: Arc<TaxonomyRepository>,
    pub target_repo: Arc<AllocationTargetRepository>,
    pub snapshot_repo: Arc<SnapshotRepository>,
    // Financial services (Phase 2 — Wealthfolio port)
    pub holdings_service: Arc<HoldingsService>,
    pub lot_service: LotService,
    pub valuation_service: ValuationService,
    pub performance_service: PerformanceService,
    pub allocation_service: AllocationService,
    pub snapshot_service: SnapshotService,
    pub net_worth_service: NetWorthService,
    pub task_executor: Arc<TaskExecutor>,
    pub artifact_manager: Arc<ArtifactManager>,
    /// Goose service for shadow-mode analysis (optional, initialized when M10 is enabled)
    pub goose_service: Option<Arc<GooseService>>,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, app_handle: AppHandle) -> Result<Self, AppError> {
        // Create repositories
        let settings_repo = SettingsRepository::new(db_pool.clone());
        let workspace_repo = WorkspaceRepository::new(db_pool.clone());
        let agent_task_repo = AgentTaskRepository::new(db_pool.clone());
        let agent_task_repo_for_executor = AgentTaskRepository::new(db_pool.clone());
        let artifact_repo = ArtifactRepository::new(db_pool.clone());
        let research_project_repo = ResearchProjectRepository::new(db_pool.clone());
        let research_document_repo = ResearchDocumentRepository::new(db_pool.clone());
        let research_note_repo = ResearchNoteRepository::new(db_pool.clone());
        let research_report_repo = ResearchReportRepository::new(db_pool.clone());
        let research_source_repo = ResearchSourceRepository::new(db_pool.clone());
        let thesis_repo = ThesisRepository::new(db_pool.clone());
        let thesis_repo_for_knowledge_graph = ThesisRepository::new(db_pool.clone());
        let knowledge_graph_repo = KnowledgeGraphRepository::new(db_pool.clone());
        let option_chain_repo = OptionChainRepository::new(db_pool.clone());
        let option_contract_repo = OptionContractRepository::new(db_pool.clone());
        let option_contract_repo_for_strategy = OptionContractRepository::new(db_pool.clone());
        let greeks_repo = GreeksRepository::new(db_pool.clone());
        let option_strategy_repo = OptionStrategyRepository::new(db_pool.clone());
        let option_position_repo = OptionPositionRepository::new(db_pool.clone());
        let strategy_leg_repo = StrategyLegRepository::new(db_pool.clone());
        let portfolio_repo_for_option = PortfolioRepository::new(db_pool.clone());
        let portfolio_repo = PortfolioRepository::new(db_pool.clone());
        let plugin_repo = PluginRepository::new(db_pool.clone());

        // Create services
        let settings_service = SettingsService::new(settings_repo);
        let workspace_service = WorkspaceService::new(workspace_repo);
        let agent_service = AgentService::new(agent_task_repo);
        let artifact_service = ArtifactService::new(artifact_repo);
        let research_project_service = ResearchProjectService::new(research_project_repo);
        let research_document_service = ResearchDocumentService::new(research_document_repo);
        let research_note_service = ResearchNoteService::new(research_note_repo);
        let research_report_service = ResearchReportService::new(research_report_repo);
        let research_source_service = ResearchSourceService::new(research_source_repo);
        let thesis_service = ThesisService::new(thesis_repo);
        let knowledge_graph_service =
            KnowledgeGraphService::new(knowledge_graph_repo, thesis_repo_for_knowledge_graph);

        // Create Arc wrappers for shared repositories
        let option_strategy_repo_arc = Arc::new(option_strategy_repo);

        let option_service = OptionService::new(
            Arc::new(option_chain_repo),
            Arc::new(option_contract_repo),
            Arc::new(greeks_repo),
            option_strategy_repo_arc.clone(),
        );
        let strategy_service = StrategyService::new(
            option_strategy_repo_arc,
            Arc::new(strategy_leg_repo),
            Arc::new(option_contract_repo_for_strategy),
        );
        let portfolio_option_service =
            PortfolioOptionService::new(option_position_repo, portfolio_repo_for_option);
        let portfolio_service = PortfolioService::new(portfolio_repo);
        let plugin_service = PluginService::new(plugin_repo);
        let system_service = SystemService::new(app_handle.clone(), db_pool.clone());

        // Financial repositories (Phase 2 — Wealthfolio port)
        let platform_repo = Arc::new(PlatformRepository::new(db_pool.clone()));
        let account_repo = Arc::new(AccountRepository::new(db_pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(db_pool.clone()));
        let quote_repo = Arc::new(QuoteRepository::new(db_pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(db_pool.clone()));
        let disposal_repo = Arc::new(LotDisposalRepository::new(db_pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(db_pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(db_pool.clone()));
        let valuation_repo = Arc::new(ValuationRepository::new(db_pool.clone()));
        let taxonomy_repo = Arc::new(TaxonomyRepository::new(db_pool.clone()));
        let target_repo = Arc::new(AllocationTargetRepository::new(db_pool.clone()));
        let snapshot_repo = Arc::new(SnapshotRepository::new(db_pool.clone()));

        // Financial services (Phase 2 — Wealthfolio port)
        let holdings_service = Arc::new(HoldingsService::new(
            account_repo.clone(),
            asset_repo.clone(),
            quote_repo.clone(),
            lot_repo.clone(),
            disposal_repo.clone(),
        ));
        let lot_service = LotService::new(
            lot_repo.clone(),
            disposal_repo.clone(),
            activity_repo.clone(),
        );
        let valuation_service = ValuationService::new(
            valuation_repo.clone(),
            account_repo.clone(),
            holdings_service.clone(),
        );
        let performance_service =
            PerformanceService::new(valuation_repo.clone(), account_repo.clone());
        let allocation_service = AllocationService::new(
            taxonomy_repo.clone(),
            target_repo.clone(),
            account_repo.clone(),
            holdings_service.clone(),
        );
        let snapshot_service = SnapshotService::new(
            snapshot_repo.clone(),
            account_repo.clone(),
            holdings_service.clone(),
        );
        let net_worth_service =
            NetWorthService::new(account_repo.clone(), holdings_service.clone());

        // Create task executor
        let executor_config = ExecutorConfig::default();
        let provider: Arc<dyn ResearchProvider> =
            Arc::new(OpenAiResearchProvider::new().map_err(|_| {
                AppError::Internal("could not initialize the OpenAI research provider".to_string())
            })?);
        let task_executor = Arc::new(TaskExecutor::new(
            agent_task_repo_for_executor,
            app_handle.clone(),
            executor_config,
            provider,
        ));

        // Create artifact manager
        let artifact_manager = Arc::new(ArtifactManager::new(app_handle.clone()));

        Ok(Self {
            db_pool,
            settings_service,
            workspace_service,
            agent_service,
            artifact_service,
            research_project_service,
            research_document_service,
            research_note_service,
            research_report_service,
            research_source_service,
            thesis_service,
            knowledge_graph_service,
            option_service,
            strategy_service,
            portfolio_option_service,
            portfolio_service,
            plugin_service,
            system_service,
            // Financial repositories (Phase 3.5 — CRUD commands)
            platform_repo,
            account_repo,
            asset_repo,
            quote_repo,
            activity_repo,
            import_run_repo,
            lot_repo,
            disposal_repo,
            valuation_repo,
            taxonomy_repo,
            target_repo,
            snapshot_repo,
            // Financial services (Phase 2 — Wealthfolio port)
            holdings_service,
            lot_service,
            valuation_service,
            performance_service,
            allocation_service,
            snapshot_service,
            net_worth_service,
            task_executor,
            artifact_manager,
            // Goose service initialized as None (enabled when M10 is activated)
            goose_service: None,
        })
    }
}
