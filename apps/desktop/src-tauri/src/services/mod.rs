// Service layer for Investment OS.
//
// Services own business logic and coordination between repositories.
// They handle:
// - Validation
// - Business rules
// - Cross-entity coordination
//
// Rules:
// - Tauri commands should stay thin
// - Services should not contain UI logic
// - Services should not expose database details

pub mod agent_service;
pub mod artifact_service;
pub mod settings_service;
pub mod system_service;
pub mod workspace_service;

#[cfg(test)]
mod agent_service_test;

#[cfg(test)]
mod workspace_service_test;
