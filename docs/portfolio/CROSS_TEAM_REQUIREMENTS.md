# Portfolio Module — Cross-Team Requirements

> **Status:** Phase 1 complete. Requirements for main dev branch coordination.
> **Audience:** Main dev developer who will merge `feature/portfolio-integration` into `dev`/`main`.
> **All documentation is in English.**

---

## 1. Summary

The Portfolio module (Phase 1) is complete on `feature/portfolio-integration` and
is ready for the main dev developer to review and merge. The following items
require coordination with the main dev branch.

---

## 2. Database Migration Coordination

### 2.1 Migration Slot Reservation

The following migration slots are occupied on `feature/portfolio-integration`:

| Slot | Name | Purpose |
|------|------|---------|
| 0015 | `financial_platforms_accounts` | Platforms + financial accounts |
| 0016 | `financial_assets_quotes` | Assets + quotes |
| 0017 | `financial_activities` | Activities + import runs |
| 0018 | `financial_lots` | Tax lots + lot disposals |
| 0019 | `financial_snapshots_valuation` | Holding snapshots + daily valuation |
| 0020 | `financial_taxonomies_allocation` | Taxonomies, categories, allocation targets |
| 0021 | `financial_valuation_unique` | Unique index on valuation table |

**Note:** These slots may conflict with migrations already on `dev`/`main`. If
`dev` has already added migrations 0015–0021 for other purposes, the financial
migrations will need to be renumbered to occupy the next available slots.

### 2.2 Migration Runner

`migrations.rs` has been modified to add `apply_financial_migrations()` at the
end of the migration chain. This function must be preserved when merging.

### 2.3 No Existing Table Changes

All financial migrations are **pure additive DDL** — they create new tables only.
No existing research, option, or workspace tables are modified. The only exception
is `_migrations` (the migration tracking table), which is updated by the runner.

---

## 3. Tauri Command Registration

### 3.1 New Commands

The following command modules need to be registered in the Tauri `invoke_handler()`:

```rust
// In apps/desktop/src-tauri/src/lib.rs or main.rs
.invoke_handler(tauri::generate_handler![
    // ... existing commands ...
    
    // Financial commands (Phase 1 — to be created in Phase 2)
    commands::portfolio::create_platform,
    commands::portfolio::list_platforms,
    commands::portfolio::create_financial_account,
    commands::portfolio::list_financial_accounts,
    commands::portfolio::create_asset,
    commands::portfolio::upsert_quote,
    commands::portfolio::create_activity,
    commands::portfolio::list_activities_by_account,
    // ... see docs/portfolio/API_SPEC.md for full list (40+ commands)
])
```

**Note:** The `commands/portfolio.rs` file currently contains the old placeholder
commands (14 functions using the old `PortfolioAccount`/`Position`/`PortfolioTransaction`
models). These will coexist with the new financial commands during Phase 2/3, then
be retired once the new UI is fully operational.

### 3.2 Command Naming Convention

All new financial commands follow the naming pattern:
- `create_*` / `list_*` / `get_*` / `update_*` / `delete_*` / `archive_*`
- `upsert_*` for upsert operations
- All use `snake_case` Tauri command names

---

## 4. AppState Wiring

### 4.1 New Repositories to Wire

The following repositories need to be instantiated in `AppState::new()`:

```rust
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

// In AppState::new():
let platform_repo = PlatformRepository::new(db_pool.clone());
let account_repo = AccountRepository::new(db_pool.clone());
let asset_repo = AssetRepository::new(db_pool.clone());
let activity_repo = ActivityRepository::new(db_pool.clone());
let lot_repo = LotRepository::new(db_pool.clone());
let snapshot_repo = SnapshotRepository::new(db_pool.clone());
let valuation_repo = ValuationRepository::new(db_pool.clone());
let taxonomy_repo = TaxonomyRepository::new(db_pool.clone());
let allocation_target_repo = AllocationTargetRepository::new(db_pool.clone());
```

### 4.2 New State Fields

```rust
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
}
```

---

## 5. TypeScript Type Definitions

### 5.1 New Type File

The main dev developer should create `apps/desktop/src/types/financial.ts` with
TypeScript types matching the 17 domain structs and 17 enums. See
`docs/portfolio/DOMAIN_MODELS.md` for the complete type reference.

### 5.2 New Desktop API Client

The main dev developer should create `apps/desktop/src/lib/desktop-api/financial.ts`
with one function per Tauri command (40+ functions). See `docs/portfolio/API_SPEC.md`
for the full command list.

### 5.3 Existing File Compatibility

The existing `src/lib/desktop-api/portfolio.ts` and `src/types/option.ts` files
are **not affected** by the portfolio changes. They will continue to work during
the transition period.

---

## 6. GUI Slot Reservation

### 6.1 Sidebar Entry

A permanent **Portfolio (📊)** entry is needed in the left sidebar, bottom section
(user area). Reference: `src/components/layout/LeftSidebar/UserOperations.tsx`.

The existing `LeftSidebar.tsx` has a `UserOperations` section at the bottom with
settings, theme toggle, and user profile. The Portfolio entry should be placed
there, above the user profile section.

### 6.2 Route

A route at `/portfolio` is needed, pointing to the existing
`src/pages/portfolio/PortfolioPage.tsx` (which currently renders a placeholder
`PortfolioDashboard` component).

### 6.3 No New Dependencies

The portfolio UI uses only:
- Existing shadcn/ui components from `packages/ui/`
- `recharts` for charts (already in the repo)
- `@tanstack/react-table` for tables (already in the repo)
- `@tanstack/react-query` for state management (already in the repo)

**No new npm packages are required.**

---

## 7. Cargo Dependency Changes

### 7.1 New Workspace Dependency

Already added in PR #89:

```toml
# Cargo.toml (workspace)
rust_decimal = { version = "1", features = ["serde"] }
```

### 7.2 Crate Dependency

Already added:

```toml
# crates/domain/Cargo.toml
rust_decimal.workspace = true
```

```toml
# apps/desktop/src-tauri/Cargo.toml
# (rust_decimal available via workspace — no direct dep needed)
```

---

## 8. Merge Conflict Risk Areas

When merging `feature/portfolio-integration` into `dev`, the following files are
most likely to have conflicts:

| File | Risk | Reason |
|------|------|--------|
| `apps/desktop/src-tauri/src/database/migrations.rs` | High | Both branches add migration functions |
| `apps/desktop/src-tauri/src/database/repositories/mod.rs` | Medium | New module declarations added |
| `apps/desktop/src-tauri/src/app/state.rs` | Medium | New repository fields added |
| `apps/desktop/src-tauri/src/commands/mod.rs` | Medium | New command modules may be added |
| `Cargo.lock` | Medium | New rust_decimal dependency |
| `docs/DATA_MODEL.md` | Low | Pure documentation, easy to merge |

---

## 9. Phase 2 Requirements (Coming Soon)

Phase 2 will add the following services, which will need Tauri commands and
AppState wiring:

- `HoldingsService` — current holdings, aggregated by account/asset
- `LotService` — FIFO/LIFO cost-basis tracking, realized gains
- `ValuationService` — daily portfolio valuation, time-series
- `PerformanceService` — XIRR, time-weighted return
- `AllocationService` — allocation vs targets, constraint checking
- `SnapshotService` — snapshot CRUD, position management

These will be built on top of the Phase 1 repositories.

---

## 10. Placeholder Retirement Plan

The existing placeholder models will be retired in phases:

| Phase | What | When |
|-------|------|------|
| Phase 3 | Replace `PortfolioDashboard` UI with real financial dashboard | Frontend phase |
| Phase 3 | Replace `portfolio_accounts` / `positions` / `transactions` tables with financial tables | After UI migration complete |
| Phase 5 | Remove old `portfolio_repository.rs` and `portfolio_service.rs` | After all consumers migrated |
| Phase 5 | Remove old `domain::portfolio` module | After all consumers migrated |

**Do not delete the old code until the new code is fully operational.** Both
systems can coexist — the new financial tables are completely independent of
the old placeholder tables.