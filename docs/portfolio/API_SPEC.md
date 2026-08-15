# Portfolio Module — API Specification

> **Status:** Phase 1 complete (storage layer). Phase 2 ✅ Done (services)
> **Target branch:** `feature/portfolio-integration`
> **Audience:** Main dev developer integrating portfolio into the main application.
> **All documentation is in English.**

---

## 1. Overview

The Portfolio module provides financial account tracking, activity ledger, tax lot
management, portfolio valuation, performance calculation, and allocation target
management. It replaces the existing placeholder `portfolio_accounts` / `positions`
/ `transactions` tables with a canonical financial schema ported from the
Wealthfolio codebase (AGPL-3.0).

### 1.1 Architecture

```
Frontend (React)
    │
    ├── desktopApi.invoke('command_name', params)
    │
    ▼
Tauri Command (Rust, commands/financial.rs)  ← Phase 2 ✅
    │
    ▼
Service Layer (7 services in services/)  ← Phase 2 ✅
    │  ├── holdings_service.rs
    │  ├── lot_service.rs
    │  ├── valuation_service.rs
    │  ├── performance_service.rs
    │  ├── allocation_service.rs
    │  ├── snapshot_service.rs
    │  └── net_worth_service.rs
    │
    ▼
Repository Layer (database/repositories/*_repository.rs)  ← Phase 1 ✅
    │
    ▼
SQLx + SQLite (migrations/0015-0021)  ← Phase 1 ✅
```

### 1.2 Phase Map

| Phase | Status | What | Docs |
|-------|--------|------|------|
| Phase 1 | ✅ Done | Domain models, migrations, repositories | `crates/domain/src/financial.rs`, `migrations/0015-0021`, repo files |
| Phase 2 | ✅ Done | Financial business logic services (holdings, lots, valuation, performance, allocation, snapshots, net worth) + 18 Tauri commands in `commands/financial.rs` | `docs/portfolio/API_SPEC.md` (section 9) |
| Phase 2.5 | 🔜 Next | Tauri commands for repository-level CRUD (create_platform, create_financial_account, create_asset, upsert_quote, create_activity, create_lot, create_taxonomy, etc.) | TBD |
| Phase 3 | ⏳ Pending | Frontend UI (pages, components, dashboard) | `docs/portfolio/FRONTEND_INTEGRATION.md` |
| Phase 4 | ⏳ Pending | Thesis ↔ holding linkage | `docs/portfolio/THESIS_LINKAGE.md` |
| Phase 5 | ⏳ Pending | Polish, broker sync, market data | TBD |

---

## 2. Required Tauri Commands

The following commands must be registered in `apps/desktop/src-tauri/src/commands/mod.rs`
and wired into `AppState`. The main dev developer should add these to the Tauri
`invoke_handler()` registration.

### 2.1 Platform Management

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_platform` | `CreatePlatformInput` | `Platform` | Create a brokerage/platform entry |
| `list_platforms` | — | `Vec<Platform>` | List all platforms |
| `get_platform` | `id: String` | `Option<Platform>` | Get platform by ID |

### 2.2 Account Management

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_financial_account` | `CreateAccountInput` | `FinancialAccount` | Create a financial account |
| `list_financial_accounts` | `workspace_id: String` | `Vec<FinancialAccount>` | List accounts in workspace |
| `get_financial_account` | `id: String` | `Option<FinancialAccount>` | Get account by ID |
| `archive_financial_account` | `id: String` | `()` | Soft-delete (archive) an account |

### 2.3 Asset Management

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_asset` | `CreateAssetInput` | `Asset` | Register a tradeable asset |
| `get_asset` | `id: String` | `Option<Asset>` | Get asset by ID |
| `find_asset_by_instrument_key` | `key: String` | `Option<Asset>` | Find by instrument key |
| `list_active_assets` | — | `Vec<Asset>` | List all active assets |
| `upsert_quote` | `UpsertQuoteInput` | `Quote` | Add or update a quote for a day |
| `get_quote_for_day` | `asset_id, date` | `Option<Quote>` | Get quote for a specific day |
| `list_quotes_for_asset` | `asset_id: String` | `Vec<Quote>` | Price history for an asset |

### 2.4 Activity Ledger

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_activity` | `CreateActivityInput` | `Activity` | Record a trade/cash movement |
| `get_activity` | `id: String` | `Option<Activity>` | Get activity by ID |
| `list_activities_by_account` | `account_id: String` | `Vec<Activity>` | All activities for an account |
| `list_activities_by_asset` | `asset_id: String` | `Vec<Activity>` | All activities for an asset |
| `create_import_run` | `CreateImportRunInput` | `ImportRun` | Start a batch import session |
| `list_import_runs` | `account_id: String` | `Vec<ImportRun>` | Import history for an account |

### 2.5 Tax Lot Management

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_lot` | `CreateLotInput` | `Lot` | Create a tax lot (from buy activity) |
| `get_lot` | `id: String` | `Option<Lot>` | Get lot by ID |
| `list_open_lots_by_account_asset` | `account_id, asset_id` | `Vec<Lot>` | Open lots for a position |
| `list_open_lots_by_account` | `account_id: String` | `Vec<Lot>` | All open lots in an account |
| `create_lot_disposal` | `CreateLotDisposalInput` | `LotDisposal` | Dispose lots (sell/expire) |
| `list_disposals_by_account` | `account_id: String` | `Vec<LotDisposal>` | Disposal history |

### 2.6 Snapshots & Valuation

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_snapshot` | `CreateSnapshotInput` | `HoldingSnapshot` | Record a point-in-time snapshot |
| `get_snapshot` | `id: String` | `Option<HoldingSnapshot>` | Get snapshot by ID |
| `list_snapshots_by_account` | `account_id: String` | `Vec<HoldingSnapshot>` | Snapshot history |
| `delete_snapshot` | `id: String` | `()` | Remove a snapshot |
| `upsert_valuation` | `UpsertValuationInput` | `DailyAccountValuation` | Record daily valuation |
| `get_valuation` | `account_id, date` | `Option<DailyAccountValuation>` | Get valuation for a date |
| `list_valuations_by_account` | `account_id, start_date, end_date` | `Vec<DailyAccountValuation>` | Valuation time series |
| `delete_valuation_for_date` | `account_id, date` | `()` | Remove a specific valuation |

### 2.7 Taxonomies & Allocation

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_taxonomy` | `CreateTaxonomyInput` | `Taxonomy` | Create a taxonomy (e.g. "Sector") |
| `get_taxonomy` | `id: String` | `Option<Taxonomy>` | Get taxonomy by ID |
| `list_taxonomies` | — | `Vec<Taxonomy>` | List all taxonomies |
| `create_taxonomy_category` | `CreateTaxonomyCategoryInput` | `TaxonomyCategory` | Add a category to a taxonomy |
| `list_taxonomy_categories` | `taxonomy_id: String` | `Vec<TaxonomyCategory>` | Categories in a taxonomy |
| `assign_asset_to_taxonomy_category` | `AssetTaxonomyAssignmentInput` | `AssetTaxonomyAssignment` | Tag an asset with a category |
| `list_assignments_for_asset` | `asset_id: String` | `Vec<AssetTaxonomyAssignment>` | Categories for an asset |
| `list_assignments_by_taxonomy` | `taxonomy_id: String` | `Vec<AssetTaxonomyAssignment>` | All assignments in a taxonomy |
| `remove_taxonomy_assignment` | `id: String` | `()` | Remove an asset-category link |
| `create_allocation_target` | `CreateAllocationTargetInput` | `AllocationTarget` | Create an allocation target |
| `get_allocation_target` | `id: String` | `Option<AllocationTarget>` | Get target by ID |
| `list_allocation_targets` | `include_archived: bool` | `Vec<AllocationTarget>` | List all targets |
| `archive_allocation_target` | `id: String` | `()` | Soft-delete a target |
| `add_allocation_weight` | `AllocationTargetWeightInput` | `AllocationTargetWeight` | Add weight row to a target |
| `list_allocation_weights` | `target_id: String` | `Vec<AllocationTargetWeight>` | Weights for a target |
| `add_allocation_constraint` | `AllocationTargetConstraintInput` | `AllocationTargetConstraint` | Add constraint to a target |
| `list_allocation_constraints` | `target_id: String` | `Vec<AllocationTargetConstraint>` | Constraints for a target |

---

## 3. TypeScript Type Definitions

The main dev developer must create `apps/desktop/src/types/financial.ts` with
TypeScript types matching the Rust domain models. These types use `snake_case`
field names as Tauri converts them automatically.

### 3.1 Enums

```typescript
// Domain enums (all string-based, serialized as snake_case)
type AccountType = "securities" | "cash" | "credit_card" | "cryptocurrency";
type TrackingMode = "not_set" | "transactions" | "holdings";
type AssetKind = "investment" | "property" | "vehicle" | "collectible"
               | "precious_metal" | "private_equity" | "liability" | "other" | "fx";
type QuoteMode = "market" | "manual";
type InstrumentType = "equity" | "crypto" | "fx" | "option" | "metal";
type ActivityType = "buy" | "sell" | "split" | "dividend" | "interest"
                  | "deposit" | "withdrawal" | "transfer_in" | "transfer_out"
                  | "fee" | "tax" | "credit" | "adjustment" | "cash_journal";
type ActivityStatus = "posted" | "pending" | "canceled";
type ValuationStatus = "initial" | "estimated" | "calculated" | "reviewed" | "final";
type BasisStatus = "filled" | "settled" | "partial";
type HoldingSnapshotSource = "manual" | "system" | "import";
type CostBasisMethod = "fifo" | "lifo" | "average_cost" | "specific_id";
type ScopeType = "account" | "portfolio" | "workspace";
type ConstraintSubjectType = "asset" | "category" | "taxonomy" | "account";
type ConstraintAction = "min" | "max" | "exact" | "exclude";
type ConstraintEffect = "hard" | "soft" | "warning";
```

### 3.2 Domain Structs

See `docs/portfolio/DOMAIN_MODELS.md` for the complete type definitions.

### 3.3 Desktop API Client

The main dev should create `apps/desktop/src/lib/desktop-api/financial.ts` following
the existing pattern in `portfolio.ts`:

```typescript
import { invoke } from "@tauri-apps/api/core";

// Export all types from the types file
export * from "@/types/financial";

// Platform
export function createPlatform(input: CreatePlatformInput): Promise<Platform> {
  return invoke("create_platform", { input });
}

// ... one function per Tauri command
```

---

## 4. Error Codes

All portfolio commands return `AppError` with the following error codes:

| Code | Description | Recoverable |
|------|-------------|-------------|
| `INTERNAL` | Database error, unexpected state | false |
| `NOT_FOUND` | Entity not found (by ID) | true |
| `VALIDATION` | Input validation failed | true |
| `PERMISSION_DENIED` | Cross-workspace access | false |
| `TIMEOUT` | Long-running operation timed out | true |

---

## 5. AppState Registration

The main dev developer must add the following to `AppState`:

### 5.1 New Repositories

```rust
// In database/repositories/mod.rs — already added in PR #91
pub mod account_repository;
pub mod activity_repository;
pub mod asset_repository;
pub mod lot_repository;
pub mod snapshot_repository;
pub mod valuation_repository;
pub mod taxonomy_repository;
pub mod allocation_target_repository;
pub mod financial_support;  // Row-parsing helpers
```

### 5.2 New Services (Phase 2 — completed)

```rust
// In services/mod.rs — added in Phase 2
pub mod holdings_service;
pub mod lot_service;
pub mod valuation_service;
pub mod performance_service;  // XIRR, time-weighted return
pub mod allocation_service;
pub mod snapshot_service;
pub mod net_worth_service;
```

### 5.3 AppState Wiring

```rust
// In app/state.rs
use crate::database::repositories::account_repository::{
    AccountRepository, PlatformRepository,
};
use crate::database::repositories::activity_repository::ActivityRepository;
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::lot_repository::LotRepository;
use crate::database::repositories::snapshot_repository::SnapshotRepository;
use crate::database::repositories::valuation_repository::ValuationRepository;
use crate::database::repositories::taxonomy_repository::TaxonomyRepository;
use crate::database::repositories::allocation_target_repository::AllocationTargetRepository;

use crate::services::holdings_service::HoldingsService;
use crate::services::lot_service::LotService;
use crate::services::valuation_service::ValuationService;
use crate::services::performance_service::PerformanceService;
use crate::services::allocation_service::AllocationService;
use crate::services::snapshot_service::SnapshotService;
use crate::services::net_worth_service::NetWorthService;

pub struct AppState {
    // ... existing fields ...
    
    // Financial repositories (Phase 1)
    pub platform_repo: PlatformRepository,
    pub account_repo: AccountRepository,
    pub asset_repo: AssetRepository,
    pub activity_repo: ActivityRepository,
    pub lot_repo: LotRepository,
    pub snapshot_repo: SnapshotRepository,
    pub valuation_repo: ValuationRepository,
    pub taxonomy_repo: TaxonomyRepository,
    pub allocation_target_repo: AllocationTargetRepository,

    // Financial services (Phase 2)
    pub holdings_service: Arc<HoldingsService>,
    pub lot_service: LotService,
    pub valuation_service: ValuationService,
    pub performance_service: PerformanceService,
    pub allocation_service: AllocationService,
    pub snapshot_service: SnapshotService,
    pub net_worth_service: NetWorthService,
}
```

> **Note:** The repository-level CRUD commands (create_platform, create_financial_account,
> create_asset, upsert_quote, create_activity, create_lot, create_taxonomy, etc.) are
> available as Rust APIs through the repository layer but are **not yet exposed as
> Tauri commands**. Only the Phase 2 service-level commands (section 9) are currently
> wired as Tauri commands. The repository-level commands will be wired in a future phase
> (Phase 2.5).

---

## 6. Data Storage Conventions

All financial data follows these rules:

| Concept | Storage Format | Example |
|---------|---------------|---------|
| Money / quantity | TEXT decimal (rust_decimal) | `"123.45"` |
| Dates | TEXT YYYY-MM-DD | `"2026-08-13"` |
| Timestamps | TEXT RFC 3339 UTC | `"2026-08-13T12:00:00.000Z"` |
| Primary keys | TEXT UUID v4 | `"a1b2c3d4-..."` |
| JSON metadata | TEXT (json_valid CHECK) | `{"key":"value"}` |

**Never use REAL or INTEGER for monetary values.** The TEXT decimal format
preserves precision through round-trips. See `financial_support.rs` for
row-parsing helpers (`parse_decimal`, `parse_timestamp`, `parse_date`, etc.).

---

## 7. Migration Ordering

Migrations are append-only. The financial migrations occupy slots 0015–0021:

| # | Name | Content |
|---|------|---------|
| 0015 | `financial_platforms_accounts` | `platforms` + `financial_accounts` tables |
| 0016 | `financial_assets_quotes` | `assets` + `quotes` tables |
| 0017 | `financial_activities` | `activities` + `import_runs` tables |
| 0018 | `financial_lots` | `lots` + `lot_disposals` tables |
| 0019 | `financial_snapshots_valuation` | `holding_snapshots` + `snapshot_positions` + `daily_account_valuation` |
| 0020 | `financial_taxonomies_allocation` | `taxonomies`, `taxonomy_categories`, `asset_taxonomy_assignments`, `allocation_targets`, `allocation_target_weights`, `allocation_target_constraints` |
| 0021 | `financial_valuation_unique` | Unique index on `daily_account_valuation(account_id, valuation_date)` |

Future financial migrations must start at 0022.

---

## 8. Dependency Graph

```
Domain Models (crates/domain/src/financial.rs)
    └── used by all repositories
Migrations (0015-0021)
    └── create tables used by repositories
Repositories (database/repositories/*_repository.rs)
    ├── account_repository ──── PlatformRepository, AccountRepository
    ├── asset_repository ────── AssetRepository, QuoteRepository (embedded)
    ├── activity_repository ─── ActivityRepository, ImportRunRepository (embedded)
    ├── lot_repository ──────── LotRepository, LotDisposalRepository (embedded)
    ├── snapshot_repository ─── SnapshotRepository, SnapshotPositionRepository (embedded)
    ├── valuation_repository ── ValuationRepository
    ├── taxonomy_repository ─── TaxonomyRepository, CategoryRepository, AssignmentRepository (embedded)
    └── allocation_target_repo ─ AllocationTargetRepository, WeightRepository, ConstraintRepository (embedded)
Services (Phase 2) ──→ Repositories
    ├── holdings_service ──→ lot_repo, valuation_repo, activity_repo, quote_repo (via asset_repo)
    ├── lot_service ───────→ lot_repo, activity_repo
    ├── valuation_service ──→ valuation_repo, holdings_service
    ├── performance_service ─→ valuation_repo, activity_repo
    ├── allocation_service ──→ holdings_service, taxonomy_repo, allocation_target_repo
    ├── snapshot_service ────→ snapshot_repo, holdings_service
    └── net_worth_service ───→ holdings_service, account_repo
Commands (commands/financial.rs) ──→ Services
```

---

## 9. Phase 2 Tauri Commands

The following 18 commands are defined in `apps/desktop/src-tauri/src/commands/financial.rs`
and wired into `AppState`. They are registered in `commands/mod.rs` as `pub mod financial;`
and added to the Tauri `invoke_handler()`.

### 9.1 Holdings

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `get_holdings` | `account_id: String, as_of_date: String` (YYYY-MM-DD) | `HoldingsSummary` | Get current holdings for a single account |
| `get_all_holdings` | `as_of_date: String` (YYYY-MM-DD) | `Vec<HoldingsSummary>` | Get holdings for all non-archived accounts |

### 9.2 Lots

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `record_sell` | `account_id: String, asset_id: String, activity_id: String` | `FifoReductionResult` | Record a sell activity against the FIFO lot inventory |
| `get_open_lots` | `account_id: String, asset_id: String` | `Vec<Lot>` | Get open lots for an account + asset combination |
| `get_open_lots_for_account` | `account_id: String` | `Vec<Lot>` | Get all open lots for an account |

### 9.3 Valuation

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `calculate_valuation_day` | `account_id: String, date: String` (YYYY-MM-DD) | `DailyAccountValuation` | Calculate and persist one day's valuation for an account |
| `get_valuation` | `account_id: String, date: String` (YYYY-MM-DD) | `Option<DailyAccountValuation>` | Get a single valuation row |
| `get_valuation_series` | `account_id: String` | `Vec<DailyAccountValuation>` | Get the full valuation series for an account |
| `calculate_all_valuations` | `date: String` (YYYY-MM-DD) | `Vec<DailyAccountValuation>` | Calculate and persist valuations for all active accounts on a date |

### 9.4 Performance

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `compute_performance_summary` | `account_id: String, start_date: String, end_date: String` (YYYY-MM-DD) | `PerformanceSummary` | Compute performance summary (XIRR, TWR) for an account |
| `get_performance_time_series` | `account_id: String` | `Vec<PerformancePoint>` | Get the performance time-series for an account |

### 9.5 Allocation

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `get_allocation` | `scope_type: String, scope_id: Option<String>, as_of_date: String` (YYYY-MM-DD) | `AllocationBreakdown` | Compute allocation breakdown for a scope |
| `check_allocation_constraints` | `scope_type: String, scope_id: Option<String>, as_of_date: String` (YYYY-MM-DD) | `Vec<String>` | Check constraints that apply to a scope |

### 9.6 Snapshots

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `create_snapshot` | `account_id: String, snapshot_date: String` (YYYY-MM-DD), `label: Option<String>` | `HoldingSnapshot` | Create a snapshot from the current holdings of an account |
| `get_snapshot` | `id: String` | `Option<HoldingSnapshot>` | Get a snapshot by ID |
| `list_snapshots` | `account_id: String` | `Vec<HoldingSnapshot>` | List snapshots for an account |
| `delete_snapshot` | `id: String` | `()` | Delete a snapshot |

### 9.7 Net Worth

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `compute_net_worth` | `as_of_date: String` (YYYY-MM-DD), `base_currency: Option<String>` (defaults to "USD") | `NetWorthSnapshot` | Compute net worth as of a given date |

---

## 10. Testing

- Repository tests use `setup_test_db()` from `test_support.rs` — full migration
  chain on `:memory:` with FK enforcement.
- 8 test files with 40+ tests covering CRUD, pagination, FK violations,
  idempotency, and ON CONFLICT behavior.
- Run with: `cargo test -- financial_repository` or `cargo test -- account_repository`