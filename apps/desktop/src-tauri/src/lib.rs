pub mod agent;
pub mod app;
pub mod artifacts;
pub mod commands;
pub mod config;
pub mod database;
pub mod documents;
pub mod error;
pub mod menu;
pub mod plugins;
pub mod providers;
pub mod security;
pub mod services;
pub mod telemetry;
pub mod windows;

use app::state::AppState;
use tauri::Manager;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    telemetry::init();

    info!("Investment OS starting");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();

            // Setup native menu bar
            if let Err(error) = menu::setup_menu(&handle) {
                tracing::error!("Failed to setup menu: {}", error);
            }

            tauri::async_runtime::spawn(async move {
                match app::bootstrap::init_database(&handle).await {
                    Ok(pool) => match AppState::new(pool, handle.clone()) {
                        Ok(state) => {
                            if let Err(error) = state.plugin_service.sync_bundled_plugins().await {
                                tracing::error!(
                                    error_code = error.code(),
                                    "bundled plugin synchronization failed"
                                );
                            }
                            handle.manage(state);
                            info!("app state initialized");
                        }
                        Err(error) => {
                            tracing::error!(
                                error_code = error.code(),
                                "app state initialization failed"
                            );
                        }
                    },
                    Err(error) => {
                        tracing::error!(
                            error_code = error.code(),
                            "database initialization failed"
                        );
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // System commands
            commands::system::get_system_info,
            commands::system::get_config_dir,
            commands::system::get_data_dir,
            commands::system::check_database_health,
            commands::system::export_local_backup,
            commands::system::check_for_update,
            // Settings commands
            commands::settings::health_check,
            commands::settings::get_app_info,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::delete_setting,
            commands::settings::list_settings,
            // Workspace commands
            commands::workspace::create_workspace,
            commands::workspace::list_workspaces,
            commands::workspace::get_workspace,
            commands::workspace::update_workspace,
            commands::workspace::delete_workspace,
            // Agent commands
            commands::agent::create_agent_task,
            commands::agent::get_agent_task,
            commands::agent::list_agent_tasks,
            commands::agent::get_task_events,
            commands::agent::queue_agent_task,
            commands::agent::start_agent_task,
            commands::agent::cancel_agent_task,
            // Research commands
            commands::research::create_research_project,
            commands::research::get_research_project,
            commands::research::list_research_projects,
            commands::research::archive_research_project,
            commands::research::complete_research_project,
            commands::research::delete_research_project,
            commands::research::create_research_document,
            commands::research::get_research_document,
            commands::research::list_research_documents,
            commands::research::delete_research_document,
            commands::research::import_research_pdf,
            commands::research::import_research_web_page,
            commands::research::search_research_document,
            commands::research::semantic_search_research_document,
            commands::research::create_research_source,
            commands::research::list_research_sources,
            commands::research::create_research_note,
            commands::research::list_research_notes,
            commands::research::delete_research_note,
            commands::research::create_research_report,
            commands::research::get_research_report,
            commands::research::list_research_reports,
            commands::research::delete_research_report,
            // Thesis commands
            commands::thesis::create_thesis,
            commands::thesis::get_thesis,
            commands::thesis::list_theses,
            commands::thesis::activate_thesis,
            commands::thesis::start_thesis_validation,
            commands::thesis::complete_thesis_validation,
            commands::thesis::update_thesis_confidence,
            commands::thesis::close_thesis,
            commands::thesis::delete_thesis,
            commands::thesis::add_thesis_evidence,
            commands::thesis::list_thesis_evidence,
            commands::thesis::delete_thesis_evidence,
            commands::thesis::list_thesis_confidence_history,
            // Knowledge graph commands
            commands::knowledge_graph::create_knowledge_entity,
            commands::knowledge_graph::list_knowledge_entities,
            commands::knowledge_graph::create_knowledge_relationship,
            commands::knowledge_graph::list_knowledge_relationships,
            commands::knowledge_graph::link_thesis_knowledge_entity,
            commands::knowledge_graph::list_thesis_knowledge_links,
            // Journal commands
            commands::journal::list_journal_entries,
            // Portfolio commands
            commands::portfolio::list_portfolio_accounts,
            commands::portfolio::create_portfolio_account,
            commands::portfolio::create_portfolio_position,
            commands::portfolio::list_portfolio_positions,
            commands::portfolio::import_portfolio_transactions_csv,
            commands::portfolio::list_portfolio_transactions,
            commands::portfolio::get_portfolio_allocation,
            commands::portfolio::get_portfolio_concentration_risks,
            commands::portfolio::link_portfolio_theme,
            commands::portfolio::get_portfolio_theme_exposure,
            commands::portfolio::get_portfolio_thesis_alignment,
            commands::portfolio::generate_portfolio_review,
            // Plugin commands
            commands::plugins::list_plugins,
            commands::plugins::set_plugin_enabled,
            commands::plugins::create_plugin_artifact,
            // Artifacts commands
            commands::artifacts::create_artifact,
            commands::artifacts::get_artifact,
            commands::artifacts::list_artifacts,
            commands::artifacts::list_task_artifacts,
            commands::artifacts::start_artifact_generation,
            commands::artifacts::complete_artifact_generation,
            commands::artifacts::fail_artifact_generation,
            commands::artifacts::start_viewing_artifact,
            commands::artifacts::close_artifact,
            commands::artifacts::delete_artifact,
            commands::artifacts::list_open_artifacts,
        ])
        .run(tauri::generate_context!())
        .expect("failed to launch Investment OS");
}
