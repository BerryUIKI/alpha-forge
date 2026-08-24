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
pub mod goose_service;
pub mod knowledge_graph_service;
pub mod option_service;
pub mod plugin_service;
pub mod portfolio_option_service;
pub mod portfolio_service;
pub mod proposal_service;
pub mod research_document_service;
pub mod research_note_service;
pub mod research_project_service;
pub mod research_report_service;
pub mod research_source_service;
pub mod settings_service;
pub mod strategy_service;
pub mod system_service;
pub mod thesis_service;
pub mod workspace_service;

// Financial services (Phase 2 — Wealthfolio port)
pub mod allocation_service;
pub mod holdings_service;
pub mod income_service;
pub mod lot_service;
pub mod net_worth_service;
pub mod performance_service;
pub mod snapshot_service;
pub mod valuation_service;

#[cfg(test)]
mod agent_service_test;

#[cfg(test)]
mod workspace_service_test;

#[cfg(test)]
mod plugin_service_test;

#[cfg(test)]
mod artifact_service_test;

#[cfg(test)]
mod holdings_service_test;

#[cfg(test)]
mod lot_service_test;

#[cfg(test)]
mod valuation_service_test;

#[cfg(test)]
mod performance_service_test;

#[cfg(test)]
mod allocation_service_test;

#[cfg(test)]
mod snapshot_service_test;

#[cfg(test)]
mod net_worth_service_test;

#[cfg(test)]
mod income_service_test;

#[cfg(test)]
mod option_service_test;

#[cfg(test)]
mod strategy_service_test;
