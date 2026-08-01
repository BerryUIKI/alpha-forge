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
pub mod settings_repository;
pub mod workspace_repository;

// Option platform repositories
pub mod greeks_repository;
pub mod option_chain_repository;
pub mod option_contract_repository;
pub mod option_position_repository;
pub mod option_strategy_repository;

#[cfg(test)]
mod agent_task_repository_test;

#[cfg(test)]
mod artifact_repository_test;

#[cfg(test)]
mod workspace_repository_test;
