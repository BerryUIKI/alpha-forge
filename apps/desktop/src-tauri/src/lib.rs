pub mod agent;
pub mod app;
pub mod artifacts;
pub mod commands;
pub mod config;
pub mod database;
pub mod documents;
pub mod error;
pub mod plugins;
pub mod providers;
pub mod security;
pub mod services;
pub mod telemetry;
pub mod windows;

use tauri::Manager;
use app::state::AppState;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    telemetry::init();

    info!("Investment OS starting");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                match app::bootstrap::init_database(&handle).await {
                    Ok(pool) => {
                        let state = AppState::new(pool, handle.clone());
                        handle.manage(state);
                        info!("app state initialized");
                    }
                    Err(e) => {
                        tracing::error!(?e, "database initialization failed");
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
            // Journal commands
            commands::journal::list_theses,
            // Portfolio commands
            commands::portfolio::list_portfolio_accounts,
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
