// Repository layer for Investment OS.
//
// Repositories own all database queries and handle:
// - Converting between database rows and domain models
// - Mapping SQL errors to application errors
// - Encapsulating persistence logic
//
// Rules:
// - Commands must not contain SQL
// - Services must call repositories
// - Database models should remain separate from domain models

pub mod agent_task_repository;
pub mod artifact_repository;
pub mod knowledge_graph_repository;
pub mod plugin_repository;
pub mod portfolio_repository;
pub mod research_project_repository;
pub mod settings_repository;
pub mod thesis_repository;
pub mod workspace_repository;

// Option platform repositories
pub mod greeks_repository;
pub mod option_chain_repository;
pub mod option_contract_repository;
pub mod option_position_repository;
pub mod option_strategy_repository;
pub mod strategy_leg_repository;

#[cfg(test)]
mod agent_task_repository_test;

#[cfg(test)]
mod artifact_repository_test;

#[cfg(test)]
mod option_chain_repository_test;

#[cfg(test)]
mod option_strategy_repository_test;

#[cfg(test)]
mod thesis_repository_test;

pub mod research_document_repository;
pub mod research_note_repository;
pub mod research_report_repository;
pub mod research_source_repository;
#[cfg(test)]
mod workspace_repository_test;
