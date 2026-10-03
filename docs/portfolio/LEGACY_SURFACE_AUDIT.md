# Portfolio Legacy and Canonical Surface Audit (M11-01)

> **Task Reference:** Milestone M11-01 (`Audit the canonical and legacy Portfolio surfaces`)  
> **Status:** Completed  
> **Author:** AlphaForge Engineering Team  
> **Date:** October 3, 2026  
> **Language:** English (Normative)

---

## 1. Executive Summary

This document establishes the authoritative, evidence-backed surface inventory of both the **canonical financial path** and the **legacy placeholder portfolio path** in AlphaForge.

Prior to the Wealthfolio financial capability integration (Phases 1–4), AlphaForge contained placeholder portfolio structures consisting of 12 Tauri commands, minimal database tables (`portfolio_accounts`, `positions`, `transactions`, `portfolio_theme_links`), a lightweight repository, a service, domain models in `crates/domain/src/portfolio.rs`, and a frontend hook `usePortfolio.ts`.

With the completion of Phases 1 through 4 (migrations 0015–0021, 0023; 10 SQLx repositories; 10 financial services; 64 canonical Tauri commands; `useFinancialData.ts`; `usePortfolioPerformance.ts`; and complete UI integration across Portfolio and Today views), the canonical financial path is fully operational.

**Key Audit Findings:**
1. **Zero UI Consumers of Legacy Surface:** Static analysis confirms **0** active React components, pages, or widgets consume `desktopApi.portfolio` or `usePortfolio.ts`. All active UI surfaces (`PortfolioPage`, `TodayPage`, `HoldingsTab`, `ActivityTab`, `PerformanceTab`, `AllocationTab`, `AccountsTab`, `NetWorthTab`, `IncomeTab`, `SnapshotsTab`, and associated dialogs) consume canonical `desktopApi.financial` and `useFinancialData.ts` (or `usePortfolioPerformance.ts`).
2. **Legacy IPC Footprint:** Exactly 12 legacy Tauri commands are currently registered in `apps/desktop/src-tauri/src/lib.rs` and allowlisted in `apps/desktop/src-tauri/permissions/artifacts.toml`.
3. **Decoupled Option Service:** `PortfolioOptionService` (`apps/desktop/src-tauri/src/services/portfolio_option_service.rs`) contains an unused `portfolio_repo: PortfolioRepository` field marked `#[allow(dead_code)]`. It performs no operations against legacy tables and can be cleanly decoupled in M11-02.
4. **Database Safety & Migration Immutability:** Applied migrations `0001` through `0023` are append-only and strictly immutable per AGENTS.md §6. Legacy tables (`portfolio_accounts`, `positions`, `transactions`, `portfolio_theme_links`) remain inert in the SQLite database file, preserving binary database compatibility without running destructive schema changes.
5. **Issue #194–#198 Verification:** Code audit of commit `33d611a` (PR #199) verifies that issues #194 through #198 are fully resolved with concrete in-tree implementations.

---

## 2. Inventory: Legacy Portfolio Surface

### 2.1 Legacy Tauri Commands (12 Commands)

The legacy Tauri commands are defined in `apps/desktop/src-tauri/src/commands/portfolio.rs`, registered in `apps/desktop/src-tauri/src/lib.rs`, and allowlisted in `apps/desktop/src-tauri/permissions/artifacts.toml`:

| # | Legacy Tauri Command | Rust Service Method | Repository Call | Canonical Replacement |
|---|----------------------|---------------------|-----------------|-----------------------|
| 1 | `create_portfolio_account` | `PortfolioService::create_account` | `PortfolioRepository::create_account` | `create_financial_account` (`financial_crud.rs`) |
| 2 | `list_portfolio_accounts` | `PortfolioService::list_accounts` | `PortfolioRepository::list_accounts` | `list_financial_accounts`, `list_all_financial_accounts` (`financial_crud.rs`) |
| 3 | `create_portfolio_position` | `PortfolioService::create_position` | `PortfolioRepository::create_position` | `create_activity` (Buy) or `create_lot` (`financial_crud.rs`) |
| 4 | `list_portfolio_positions` | `PortfolioService::list_positions` | `PortfolioRepository::list_positions` | `get_holdings`, `get_all_holdings` (`financial.rs`) |
| 5 | `import_portfolio_transactions_csv` | `PortfolioService::import_transactions_csv` | `PortfolioRepository::create_transaction` | `import_activities_csv` (`financial_crud.rs`) |
| 6 | `list_portfolio_transactions` | `PortfolioService::list_transactions` | `PortfolioRepository::list_transactions` | `list_activities_by_account`, `list_activities_by_asset` (`financial_crud.rs`) |
| 7 | `get_portfolio_allocation` | `PortfolioService::allocation_by_workspace` | Aggregates positions | `get_allocation` (`financial.rs`) |
| 8 | `get_portfolio_concentration_risks` | `PortfolioService::concentration_risks` | Aggregates positions | `check_allocation_constraints` (`financial.rs`) |
| 9 | `link_portfolio_theme` | `PortfolioService::link_theme` | `PortfolioRepository::link_theme` | `assign_asset_to_taxonomy_category` (`financial_crud.rs`) & `link_thesis_asset` (`thesis.rs`) |
| 10 | `get_portfolio_theme_exposure` | `PortfolioService::theme_exposure` | Aggregates theme links | `get_allocation` with taxonomy categories (`financial.rs`) |
| 11 | `get_portfolio_thesis_alignment` | `PortfolioService::thesis_alignment` | Queries theses | Canonical thesis asset link (`theses.portfolio_asset_id` -> `financial_assets.id`) |
| 12 | `generate_portfolio_review` | `PortfolioService::review` | Combines risks + alignment | Canonical thesis review backed by `HoldingsService` |

### 2.2 Legacy Rust Backend Components

| File Path | Component | Responsibility | Current Dependents |
|-----------|-----------|----------------|--------------------|
| `apps/desktop/src-tauri/src/commands/portfolio.rs` | 12 commands + 8 DTOs | Tauri IPC boundary for placeholder portfolio | `src/lib.rs` (`generate_handler!`), `artifacts.toml` |
| `apps/desktop/src-tauri/src/services/portfolio_service.rs` | `PortfolioService` | Business logic for placeholder positions and accounts | `AppState` (`app/state.rs`), `commands/portfolio.rs` |
| `apps/desktop/src-tauri/src/database/repositories/portfolio_repository.rs` | `PortfolioRepository` | SQLx persistence for tables `portfolio_accounts`, `positions`, `transactions`, `portfolio_theme_links` | `PortfolioService`, `PortfolioOptionService` (`dead_code`), `AppState` |
| `crates/domain/src/portfolio.rs` | 14 domain structs and enums | Data contracts (`PortfolioAccount`, `Position`, `PortfolioTransaction`, etc.) | `portfolio_repository.rs`, `portfolio_service.rs`, `commands/portfolio.rs` |
| `crates/domain/src/lib.rs` | `pub mod portfolio;` | Crate export | External crates (`apps/desktop/src-tauri`) |

### 2.3 Legacy Frontend Components

| File Path | Symbols / Exports | Responsibility | Consumers |
|-----------|-------------------|----------------|-----------|
| `apps/desktop/src/lib/desktop-api/portfolio.ts` | 8 Zod schemas, 8 types, 2 param interfaces, 12 invoke wrappers | Client-side IPC wrapper for legacy commands | `src/lib/desktop-api/index.ts`, `usePortfolio.ts` |
| `apps/desktop/src/lib/desktop-api/index.ts` | `portfolio: portfolioApi` | Exports legacy namespace on `desktopApi` | `usePortfolio.ts`, `usePortfolio.test.tsx` |
| `apps/desktop/src/features/portfolio/hooks/usePortfolio.ts` | `usePortfolioAccounts`, `usePortfolioPositions`, `usePortfolioTransactions`, `usePortfolioAllocation`, `usePortfolioConcentrationRisks`, `usePortfolioThemeExposure`, `usePortfolioThesisAlignment`, `useCreatePortfolioAccount`, `useCreatePortfolioPosition`, `useImportPortfolioTransactionsCsv`, `useLinkPortfolioTheme`, `usePortfolioReview` | React Query hooks wrapping legacy API | `usePortfolio.test.tsx` only (**0 UI components**) |
| `apps/desktop/src/features/portfolio/hooks/usePortfolio.test.tsx` | 13 unit tests | Tests for `usePortfolio.ts` | Test runner |

---

## 3. Inventory: Canonical Financial Surface

The canonical financial surface was ported from Wealthfolio and adapted for AlphaForge:

### 3.1 Persistence & Migrations (SQLx)
- **Migrations:**
  - `0015_financial_platforms_accounts.sql`: `platforms`, `accounts`
  - `0016_financial_assets_quotes.sql`: `assets`, `quotes`
  - `0017_financial_activities.sql`: `activities`, `import_runs`
  - `0018_financial_lots.sql`: `lots`, `lot_disposals`
  - `0019_financial_snapshots_valuation.sql`: `holding_snapshots`, `daily_account_valuation`
  - `0020_financial_taxonomies_allocation.sql`: `taxonomies`, `taxonomy_categories`, `asset_taxonomy_assignments`, `allocation_targets`, `allocation_target_weights`, `allocation_target_constraints`
  - `0021_financial_valuation_unique.sql`: Unique constraint on `daily_account_valuation(account_id, date)`
  - `0023_theses_portfolio_asset_link.sql`: `theses.portfolio_asset_id REFERENCES assets(id)`
- **Repositories (`apps/desktop/src-tauri/src/database/repositories/`):**
  - `PlatformRepository`, `AccountRepository`
  - `AssetRepository` (with FX quote support)
  - `ActivityRepository`, `ImportRunRepository`
  - `LotRepository`
  - `ValuationRepository`
  - `SnapshotRepository`
  - `TaxonomyRepository`, `AllocationTargetRepository`

### 3.2 Domain Layer (`crates/domain/src/financial.rs`)
- 17 structs, 17 enums, all monetary and quantity fields using `rust_decimal::Decimal`.
- Complete models for Accounts, Assets, Activities, Lots, Valuation, Performance, Allocation, Taxonomies, and Snapshots.

### 3.3 Core Services (`apps/desktop/src-tauri/src/services/`)
- `HoldingsService`: Aggregates holdings from lots + quotes with multi-currency FX conversion.
- `LotService`: FIFO lot inventory tracking and sell disposals.
- `ValuationService`: Multi-currency daily valuation calculations with FX conversion.
- `PerformanceService`: XIRR (Newton's method) and Time-Weighted Return (TWR) calculations.
- `AllocationService`: Basis-points asset allocation breakdown and target constraint evaluations.
- `SnapshotService`: Point-in-time holding snapshots.
- `NetWorthService`: Multi-account aggregated net worth.
- `ActivityService`: Atomic, idempotent transaction ingestion and FIFO lot synchronization.
- `ActivityImportService`: Resilient CSV importing (generic and IBKR formats) with audit logging.
- `IncomeService`: Income aggregation by month, type, asset, account, and currency.

### 3.4 Canonical Tauri Commands (64 Commands)
- **`commands/financial.rs` (18 Service Commands):**
  - Holdings: `get_holdings`, `get_all_holdings`
  - Lots: `record_sell`, `get_open_lots`, `get_open_lots_for_account`
  - Valuation: `calculate_valuation_day`, `get_valuation`, `get_valuation_series`, `calculate_all_valuations`
  - Performance: `compute_performance_summary`, `get_performance_time_series`
  - Allocation: `get_allocation`, `check_allocation_constraints`
  - Snapshots: `create_snapshot`, `get_snapshot`, `list_snapshots`, `delete_snapshot`
  - Net Worth: `compute_net_worth`
- **`commands/financial_crud.rs` (46 CRUD & Utility Commands):**
  - Platforms: `create_platform`, `list_platforms`, `get_platform`
  - Accounts: `create_financial_account`, `list_financial_accounts`, `list_all_financial_accounts`, `get_financial_account`, `archive_financial_account`
  - Assets: `create_asset`, `get_asset`, `find_asset_by_instrument_key`, `list_active_assets`
  - Quotes: `upsert_quote`, `get_quote_for_day`, `list_quotes_for_asset`, `refresh_asset_quote`, `refresh_all_active_quotes`
  - Activities: `create_activity`, `get_activity`, `list_activities_by_account`, `list_activities_by_asset`
  - Import Runs: `create_import_run`, `list_import_runs`, `import_activities_csv`
  - Lots: `create_lot`, `get_lot`
  - Valuation: `upsert_valuation`, `list_valuations_by_account`, `delete_valuation_for_date`
  - Taxonomies: `create_taxonomy`, `get_taxonomy`, `list_taxonomies`, `create_taxonomy_category`, `list_taxonomy_categories`, `assign_asset_to_taxonomy_category`, `list_assignments_for_asset`, `list_assignments_by_taxonomy`, `remove_taxonomy_assignment`
  - Allocation Targets: `create_allocation_target`, `get_allocation_target`, `list_allocation_targets`, `archive_allocation_target`, `add_allocation_weight`, `list_allocation_weights`, `add_allocation_constraint`, `list_allocation_constraints`

### 3.5 Canonical Frontend Consumer Layer
- `desktopApi.financial` in `apps/desktop/src/lib/desktop-api/financial.ts` (64 command wrappers).
- `useFinancialData.ts` in `apps/desktop/src/features/portfolio/hooks/` (React Query hooks for accounts, holdings, activities, valuations, performance, allocation, snapshots, platforms, and CRUD).
- `usePortfolioPerformance.ts` in `apps/desktop/src/pages/today/hooks/` (Workspace-level performance chart aggregation calling `listAllFinancialAccounts` and `getPerformanceTimeSeries`).
- **UI Surfaces:**
  - `PortfolioPage.tsx` (`apps/desktop/src/pages/portfolio/PortfolioPage.tsx`)
  - `TodayPage.tsx` (`apps/desktop/src/pages/today/TodayPage.tsx`)
  - Tabs: `HoldingsTab.tsx`, `ActivityTab.tsx`, `PerformanceTab.tsx`, `AllocationTab.tsx`, `AccountsTab.tsx`, `NetWorthTab.tsx`, `IncomeTab.tsx`, `SnapshotsTab.tsx`
  - Dialogs: `AddAccountDialog.tsx`, `AddAssetDialog.tsx`, `AddActivityDialog.tsx`, `ImportActivitiesDialog.tsx`, `ManageTaxonomiesDialog.tsx`, `CreateAllocationTargetDialog.tsx`
  - Research / Thesis Linkage: `ThesisCard.tsx`, `ThesisDetail.tsx`, `FinancialAssetPicker.tsx`

---

## 4. Disposition Table for Milestone M11-02

Every legacy element identified in the audit has been assigned a single, unambiguous disposition:

| Item Identifier | Layer | Disposition | Target Milestone | Rationale / Removal Action |
|-----------------|-------|-------------|------------------|----------------------------|
| 12 commands in `commands/portfolio.rs` | Rust Tauri Commands | **Remove** | M11-02 | Zero active UI consumers. Remove module and unregister from `lib.rs`. |
| `commands/portfolio.rs` | File | **Remove** | M11-02 | File completely superseded by `commands/financial.rs` and `commands/financial_crud.rs`. |
| 12 command entries in `artifacts.toml` | Tauri Permissions | **Remove** | M11-02 | Remove legacy commands from `main-commands` allowlist. |
| `PortfolioService` in `services/portfolio_service.rs` | Rust Service | **Remove** | M11-02 | Replace with canonical services; remove file and module export in `services/mod.rs`. |
| `PortfolioRepository` in `repositories/portfolio_repository.rs` | Rust Repository | **Remove** | M11-02 | No longer used. Remove file and module export in `repositories/mod.rs`. |
| `portfolio_repo` in `PortfolioOptionService` | Rust Service | **Migrate Consumer** | M11-02 | Remove dead field `portfolio_repo` from `PortfolioOptionService::new`. |
| `portfolio_service` & `portfolio_repo` in `AppState` | Rust AppState | **Remove** | M11-02 | Remove field and initialization in `apps/desktop/src-tauri/src/app/state.rs`. |
| `crates/domain/src/portfolio.rs` | Rust Domain Crate | **Remove** | M11-02 | Placeholder models superseded by `crates/domain/src/financial.rs`. Remove file. |
| `pub mod portfolio;` in `crates/domain/src/lib.rs` | Rust Domain Crate | **Remove** | M11-02 | Remove module declaration. |
| `apps/desktop/src/lib/desktop-api/portfolio.ts` | TypeScript Desktop API | **Remove** | M11-02 | Superseded by `desktop-api/financial.ts`. |
| `desktopApi.portfolio` in `desktop-api/index.ts` | TypeScript Desktop API | **Remove** | M11-02 | Remove namespace export from `desktopApi`. |
| `usePortfolio.ts` in `features/portfolio/hooks/` | React Hooks | **Remove** | M11-02 | Zero UI consumers. All components use `useFinancialData.ts`. |
| `usePortfolio.test.tsx` in `features/portfolio/hooks/` | React Tests | **Remove** | M11-02 | Obsolete unit test suite for removed hook. |
| Migrations `0001`, `0010`, `0011` tables | SQLite Database | **Retain (Inert)** | Indefinite | Migrations are immutable. Tables remain inert; no destructive DDL; DB compatibility preserved. |

---

## 5. Persistence and Database Compatibility Evaluation

### 5.1 Migration Immutability
In accordance with AGENTS.md §6 and the repository's Git & DB safety rules:
- Migrations `0001` through `0023` are append-only.
- No existing migration file may be modified, renumbered, or deleted.
- No destructive `DROP TABLE` migration will be introduced in M11-02. Dropping tables from existing user databases introduces unnecessary migration failure modes for zero runtime gain.

### 5.2 Legacy Table State in Existing Databases
Existing user databases contain the legacy tables created by migrations `0001`, `0010`, and `0011`:
- `portfolio_accounts`
- `positions`
- `transactions`
- `portfolio_theme_links`

Once M11-02 retires the runtime code, the application will no longer execute any queries (`SELECT`, `INSERT`, `UPDATE`, `DELETE`) against these tables. The tables remain entirely benign and inert. Any existing user database will continue to open and run migrations seamlessly without schema divergence.

### 5.3 Canonical Thesis Linkage
The thesis linkage established in Phase 4 uses migration `0023_theses_portfolio_asset_link.sql`, adding:
```sql
ALTER TABLE theses ADD COLUMN portfolio_asset_id TEXT REFERENCES assets(id) ON DELETE SET NULL;
```
This foreign key strictly references `assets(id)` (the canonical financial assets table created in migration `0016`), not any legacy table. Thus, research-to-portfolio linking is 100% canonical and completely decoupled from legacy schema.

---

## 6. Audit and Closure Evidence: Issues #194 Through #198

In PR #199 (commit `33d611ab3c64c0ccd954b90b8c1fee5bab5cdbfe`), five critical audit issues (#194 through #198) were addressed across the codebase. Static verification confirms that all changes are present on `dev`:

### 6.1 Issue #194 — Eliminate mutex lock unwrap panics in agent-core
- **Location:** `crates/agent-core/src/diagnostics.rs` and `crates/agent-core/src/manager.rs`
- **Verification Evidence:**
  - `diagnostics.rs`: Mutex lock acquisitions use `match self.events.lock()` handling poisoned locks by acquiring `poisoned.into_inner()` or returning structured errors.
  - `manager.rs`: Replaced `.lock().unwrap()` with `.lock().map_err(|e| AgentError::Internal(...))` and poison-recovery logic across task lifecycle methods.
- **Status:** **Verified Closed**.

### 6.2 Issue #195 — Eliminate mutex lock expect panics in market-data
- **Location:** `crates/market-data/src/registry/circuit_breaker.rs` and `crates/market-data/src/registry/rate_limiter.rs`
- **Verification Evidence:**
  - `circuit_breaker.rs`: Replaced `.lock().expect(...)` with poisoned-lock recovery `.unwrap_or_else(|poisoned| poisoned.into_inner())`.
  - `rate_limiter.rs`: Replaced `.lock().expect(...)` with poisoned-lock recovery.
- **Status:** **Verified Closed**.

### 6.3 Issue #196 — Resolve ESLint warnings across workspace
- **Location:** `eslint.config.js`, `apps/desktop/src/components/layout/MainLayout.tsx`, `packages/ui/src/components/ui/use-toast.ts`, and test files.
- **Verification Evidence:**
  - Running `pnpm lint` yields 0 warnings and 0 errors across all workspace packages.
- **Status:** **Verified Closed**.

### 6.4 Issue #197 — Replace unsafe `as any` casts with type narrowing
- **Location:** `apps/desktop/src/lib/errors/index.ts`, `AddAssetDialog.tsx`, and test files.
- **Verification Evidence:**
  - Replaced arbitrary type assertions with custom type guards (`isAppError`, `isTauriError`, `isAxiosError`) and unknown-type narrowing.
- **Status:** **Verified Closed**.

### 6.5 Issue #198 — Implement FX rate lookup for holdings valuation & Goose token extraction
- **Location:** `apps/desktop/src-tauri/src/services/holdings_service.rs`, `valuation_service.rs`, `asset_repository.rs`, and `apps/desktop/src-tauri/src/goose/adapter.rs`
- **Verification Evidence:**
  - `AssetRepository::find_fx_rate`: Queries direct currency pair quotes (`USD/EUR`) and inverse quotes (`EUR/USD`) from the quotes table.
  - `HoldingsService`: Converts non-base currency assets to account base currency using historical or latest FX quotes with 1.0 parity fallback for same currency.
  - `ValuationService`: Applies multi-currency valuation aggregation across accounts into base currency.
  - `goose/adapter.rs`: Implemented regex and JSON metric extraction for input tokens, output tokens, and total token usage from Goose stdout/stderr logs.
- **Status:** **Verified Closed**.

---

## 7. Phase Alignment Across Documentation

The status of the Portfolio integration phases has been reconciled across all authoritative documentation:

| Phase | Description | Reconciled Status | Authoritative Reference |
|-------|-------------|-------------------|-------------------------|
| Phase 1 | Financial Schema (SQLx migrations 0015–0021) & Repositories | ✅ Complete | `docs/portfolio/ROADMAP.md` §Phase 1 |
| Phase 2 | Core Financial Services (Holdings, Lots, Valuation, Performance, Allocation, Snapshots, Net Worth) | ✅ Complete | `docs/portfolio/ROADMAP.md` §Phase 2 |
| Phase 2.5 | Market Data Crate (`crates/market-data`) & Income Service | ✅ Complete | `docs/portfolio/ROADMAP.md` §Phase 2.5 |
| Phase 3 | Frontend UI (Portfolio Workspace, Dashboard, Holdings, Activities, Lots, Valuation, Performance) | ✅ Complete | `docs/portfolio/ROADMAP.md` §Phase 3 |
| Phase 3.5 | Repository CRUD Commands (Platforms, Accounts, Assets, Quotes, Taxonomies, Allocation Targets) | ✅ Complete | `docs/portfolio/ROADMAP.md` §Phase 3.5 |
| Phase 4 | Research Thesis ↔ Financial Asset Linkage (Migration 0023, D5, PR #209) | ✅ Complete | `docs/portfolio/ROADMAP.md` §Phase 4 |
| Phase 5 / M11 | Hardening, Legacy Retirement, Scale Benchmarks & Release Packaging | 🚧 Active Queue | `docs/milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md` |

---

## 8. M11-02 Execution & Retirement Record

In Milestone M11-02 (`Retire superseded Portfolio runtime paths`), all approved legacy items were retired:

1. **Rust Tauri Backend Cleanup (Completed):**
   - Removed `apps/desktop/src-tauri/src/commands/portfolio.rs`.
   - Removed `pub mod portfolio;` from `apps/desktop/src-tauri/src/commands/mod.rs`.
   - Removed `apps/desktop/src-tauri/src/services/portfolio_service.rs`.
   - Removed `pub mod portfolio_service;` from `apps/desktop/src-tauri/src/services/mod.rs`.
   - Removed `apps/desktop/src-tauri/src/database/repositories/portfolio_repository.rs`.
   - Removed `pub mod portfolio_repository;` from `apps/desktop/src-tauri/src/database/repositories/mod.rs`.
   - Decoupled `PortfolioOptionService::new` in `apps/desktop/src-tauri/src/services/portfolio_option_service.rs` by eliminating the dead `portfolio_repo` parameter and field.
   - Removed `portfolio_service` and `portfolio_repo` from `AppState` in `apps/desktop/src-tauri/src/app/state.rs`.
   - Unregistered all 12 legacy commands from `generate_handler!` in `apps/desktop/src-tauri/src/lib.rs`.
   - Removed all 12 legacy command entries from `apps/desktop/src-tauri/permissions/artifacts.toml`.

2. **Domain Crate Cleanup (Completed):**
   - Removed `crates/domain/src/portfolio.rs`.
   - Removed `pub mod portfolio;` from `crates/domain/src/lib.rs`.

3. **Frontend Desktop API and Hooks Cleanup (Completed):**
   - Removed `apps/desktop/src/lib/desktop-api/portfolio.ts`.
   - Removed `portfolio` export from `apps/desktop/src/lib/desktop-api/index.ts`.
   - Removed `apps/desktop/src/features/portfolio/hooks/usePortfolio.ts`.
   - Removed `apps/desktop/src/features/portfolio/hooks/usePortfolio.test.tsx`.

4. **IPC Registration Parity (Verified):**
   - IPC parity script `node scripts/check-ipc-registration.mjs` verifies exactly 176 registered Rust handlers and 176 frontend invocations with 0 uninvoked and 0 missing commands.
   - All tests pass: `pnpm typecheck`, `pnpm lint`, `pnpm test`, `cargo fmt --check`, `cargo clippy`, and `cargo test --workspace`.
