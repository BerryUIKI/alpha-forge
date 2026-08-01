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
pub mod settings_repository;
pub mod workspace_repository;

#[cfg(test)]
mod workspace_repository_test;
