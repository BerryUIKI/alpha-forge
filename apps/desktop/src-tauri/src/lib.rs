// `async_trait` adds `#[must_use]` to boxed futures; newer Clippy versions
// flag that as redundant. Allow it once for every async trait in the crate.
#![allow(unknown_lints, clippy::double_must_use)]

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
pub mod text;
pub mod windows;

#[cfg(test)]
mod test_support;

use app::state::AppState;
use tauri::Manager;
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
                        match state.agent_service.recover_interrupted_tasks().await {
                            Ok(0) => {}
                            Ok(count) => info!(count, "recovered interrupted agent tasks"),
                            Err(e) => {
                                tracing::error!(?e, "failed to recover interrupted agent tasks")
                            }
                        }
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
            // Settings commands
            commands::settings::health_check,
            commands::settings::get_app_info,
            commands::settings::get_setting,
            commands::settings::set_setting,
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
            commands::agent::retry_agent_task,
            // Provider commands
            commands::provider::get_provider_status,
            commands::provider::set_provider_api_key,
            commands::provider::clear_provider_api_key,
            // Research commands
            commands::research::list_research_documents,
            commands::research::create_research_document,
            commands::research::get_research_document,
            commands::research::update_research_document,
            commands::research::delete_research_document,
            commands::research::purge_research_document,
            // Journal commands
            commands::journal::list_theses,
            // Portfolio commands
            commands::portfolio::list_portfolio_accounts,
            // Artifacts commands
            commands::artifacts::list_artifacts,
            commands::artifacts::open_artifact_window,
        ])
        .run(tauri::generate_context!())
        .expect("failed to launch Investment OS");
}
