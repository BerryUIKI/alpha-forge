# Portfolio Feature — Roadmap

> Target feature decomposition for the Wealthfolio integration. This document is
> the single source of truth for the Portfolio module's planned capability,
> phase progression, and acceptance gates. See
> [`../PORTFOLIO_INTEGRATION_PLAN.md`](../PORTFOLIO_INTEGRATION_PLAN.md) for the
> approved integration decisions and [`API_SPEC.md`](API_SPEC.md) for the live
> API surface.
>
> **All documentation is in English.**

---

## Overview

The Portfolio module turns AlphaForge from a research workspace into a
research + decision tracking platform. It ports the full financial domain from
Wealthfolio (AGPL-3.0) — accounts, holdings, lots, valuation, performance,
allocation, snapshots, net worth, income, and market data — while resolving
Wealthfolio's technical debt (3,264 panic points, Diesel bake-in) instead of
copying it.

**Product guardrails (AGENTS.md §15, §17):**
- Tracking, analysis, and research alignment — never autonomous trading
- No execution of trades, no automated investment decisions

---

## Vision

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
                       ↑                        │
                       └────── Portfolio ───────┘
```

The Portfolio module closes the loop: a thesis is validated against real
holdings, valuations, and performance. Every financial number must be
traceable, auditable, and typed — money and quantity are `rust_decimal::Decimal`,
never `f64`.

---

## Timeline Overview

| Phase | Status | Key Deliverable |
|-------|--------|-----------------|
| Phase 1 | ✅ Complete | Financial schema on SQLx (migrations 0015–0021) + repositories |
| Phase 2 | ✅ Complete | Core financial services (holdings, lots, valuation, performance, allocation, snapshots, net worth) + 18 commands |
| Phase 2.5 | ✅ Done | Market-data crate (quotes, asset profiles), income service |
| Phase 3 | ✅ Complete | Frontend UI (dashboard, accounts, holdings, activities) |
| Phase 3.5 | ✅ Complete | Repository-level CRUD commands (platform, account, asset, quote seed) |
| Phase 4 | ✅ Complete | Thesis ↔ financial-asset linkage (PR #209) |
| Phase 5 | 🚧 In progress | Hardening and release-candidate work tracked by M11 |

Planning estimates are indicative; do not treat calendar weeks as a completion
claim.

---

## Phase 1 — Storage ✅

**Goal:** Financial schema on SQLx, single SQLite database.

### Deliverables
- [x] Domain models in `crates/domain/src/financial.rs` (17 structs, 17 enums,
      canonical values match migration CHECK constraints)
- [x] SQLx migrations 0015–0021: platforms, financial_accounts, assets,
      activities, lots, valuation, allocation_targets, snapshots
- [x] SQLx repositories: account, asset, activity, lot, valuation, snapshot,
      allocation_target, taxonomy (each with `_test.rs`)
- [x] `financial_support.rs` row-parsing helpers (typed errors, no panics)

### Acceptance
- [x] `cargo test` green
- [x] Migration runs from a clean DB
- [x] No `unwrap()`/`expect()` in new repo code

---

## Phase 2 — Core Financial Services ✅

**Goal:** Business logic services with typed `Result`, no panic paths.

### Deliverables
- [x] `HoldingsService` — aggregate positions from lots + quotes
- [x] `LotService` — FIFO disposal via `record_sell()`, open-lot inventory
- [x] `ValuationService` — daily account valuation, status classification
- [x] `PerformanceService` — XIRR (Newton's method) + time-weighted return
- [x] `AllocationService` — scope breakdown in basis points, constraint checks
- [x] `SnapshotService` — point-in-time holdings snapshots
- [x] `NetWorthService` — cross-account net worth, liabilities handling
- [x] 18 Tauri commands in `commands/financial.rs`
- [x] 41 tests across 7 service test files

### Acceptance
- [x] `cargo test` green
- [x] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [x] Every service returns typed `AppError` — no panic paths

---

## Phase 2.5 — Market Data + Income ✅ Done

**Goal:** Provider-agnostic market data + income aggregation, both fully
testable without the main application.

### Deliverables
- [x] New workspace crate `crates/market-data` (ported from Wealthfolio)
  - [x] Core models: `InstrumentId`, `Quote`, `AssetProfile`, `Coverage`,
        `SearchResult`, `DividendEvent`, `ProviderInstrument`, `SplitEvent`
  - [x] `MarketDataError` with `RetryClass` classification
  - [x] `MarketDataProvider` async trait + `ProviderCapabilities` + `RateLimit`
  - [x] `ProviderRegistry` — orchestration, circuit breaker, rate limiter,
        quote validation, fetch diagnostics
  - [x] `ResolverChain` — asset overrides → deterministic symbol rules
  - [x] `FixtureProvider` — deterministic synthetic data for tests
  - [x] `YahooProvider` — Yahoo Finance (equities, crypto, FX)
- [x] `IncomeService` — income aggregation (by month/type/asset/account/currency,
      YoY growth) on the existing `ActivityRepository`
- [x] Domain models for income summaries in `crates/domain/src/financial.rs`

### Acceptance
- [x] `cargo test` green (new crate tests + service tests)
- [x] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [x] No `unwrap()`/`expect()` outside tests
- [x] Docs updated (this roadmap, `API_SPEC.md`)

---

## Phase 3 — Frontend UI ✅ Complete

**Goal:** Replace the placeholder `PortfolioDashboard` with the real portfolio
workspace.

### Deliverables
- [x] Sidebar: permanent Portfolio entry (D6)
- [x] Account management and canonical financial CRUD dialogs
- [x] Holdings view: positions, cost basis, market value, and gains
- [x] Activity ledger: transactions, dividends, and fees
- [x] Lot tracking and FIFO disposal support
- [x] Valuation and performance charts
- [x] Allocation view and drift warnings
- [x] Snapshots and net-worth surfaces
- [x] Income aggregation services and UI integration

### Acceptance
- [x] `pnpm lint` and `pnpm typecheck` passed at integration
- [x] English and Simplified-Chinese catalogs extended with parity
- [x] Loading, empty, partial, offline, and error behavior covered on the integrated surfaces

---

## Phase 3.5 — Repository CRUD Commands ✅ Complete

**Goal:** Thin Tauri commands exposing repository-level CRUD so the main
application can seed data without frontend work.

### Deliverables
- [x] Platform CRUD
- [x] Financial-account CRUD and archive behavior
- [x] Asset and quote CRUD
- [x] Activity creation and list operations
- [x] Lot operations used by holdings and FIFO flows
- [x] Taxonomy and allocation-target CRUD
- [x] Market-data resolution and refresh commands

### Acceptance
- [x] Commands registered with `AppState` wiring and IPC parity coverage
- [x] Repository-backed command and service tests passed at integration

---

## Phase 4 — Thesis ↔ Financial-Asset Linkage ✅ Complete

**Goal:** Connect research to decisions (D5).

### Deliverables
- [x] Append-only migration `0023_theses_portfolio_asset_link.sql`
- [x] Optional financial-asset picker and link/unlink behavior in thesis UI
- [x] Portfolio review alignment uses authoritative asset linkage with symbol fallback
- [x] Linked asset identity is visible from thesis cards and details

### Acceptance
- [x] Create or select asset -> link thesis -> review shows alignment

---

## Phase 5 — Hardening & Release Candidate 🚧 In progress

### Deliverables

- [x] Generic and IBKR activity-statement file import (PR #213)
- [x] Market-data refresh and quote caching (PR #211)
- [x] Top-level financial architecture and data-model synchronization (PR #215)
- [x] Audit canonical and legacy Portfolio surfaces (M11-01, see [`LEGACY_SURFACE_AUDIT.md`](LEGACY_SURFACE_AUDIT.md))
- [x] Establish approved large-portfolio performance baseline and budgets (M11-03, see [`PERFORMANCE_BASELINE.md`](PERFORMANCE_BASELINE.md))
- [ ] Meet approved scale and resilience budgets with focused hardening (M11-04)
- [ ] Sweep remaining production panic paths and error typing (M11-05)
- [ ] Produce macOS Apple Silicon and Windows NSIS release-candidate packages (M11-06)
- [ ] Retain cross-platform packaged smoke and release-gate evidence (M11-07)

Live broker synchronization and FIRE/retirement planning are deferred. They require separate product, credential, privacy, security, and architecture approval and are not part of M11.

---

## Dependency Graph

```text
Phase 1 (storage)
    │
    ▼
Phase 2 (services) ────► Phase 2.5 (market data, income)
    │                              │
    ▼                              ▼
Phase 3 (frontend UI) ◄─────── consumer of market-data
    │
    ▼
Phase 3.5 (CRUD commands) — enables app-seeding for Phase 3
    │
    ▼
Phase 4 (thesis linkage) — after theses + holdings both exist
    │
    ▼
Phase 5 / M11 (hardening and release candidate)
```

Phases 1 through 4 are complete. Phase 5 is governed by the
[M11 execution plan](../milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md).
Historical phase dependencies remain useful for architecture context but are no
longer an active branch sequence.

---

## Definition of Done (every phase)

- [ ] `cargo test` green
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] No new `unwrap()`/`expect()` outside tests
- [ ] Docs updated before the code ships
- [ ] PR merged through a focused task branch into `dev`

---

## References

- [Integration Plan](../PORTFOLIO_INTEGRATION_PLAN.md) — approved decisions D1–D10
- [Legacy Surface Audit](LEGACY_SURFACE_AUDIT.md) — M11-01 authoritative surface inventory and disposition
- [Performance Baseline](PERFORMANCE_BASELINE.md) — M11-03 large-portfolio benchmarks and approved performance budgets
- [API Specification](API_SPEC.md) — live command surface
- [Domain Models](DOMAIN_MODELS.md) — enum/struct reference
- [Frontend Integration](FRONTEND_INTEGRATION.md) — flagship UI plan
- [M11 Execution Plan](../milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md) — active hardening and release queue
- [Wealthfolio Audit](../wealthfolio-audit/README.md) — 14 audit documents
