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
| Phase 3 | 📋 Planned | Frontend UI (dashboard, accounts, holdings, activities) |
| Phase 3.5 | 📋 Planned | Repository-level CRUD commands (platform, account, asset, quote seed) |
| Phase 4 | 📋 Planned | Thesis ↔ holding linkage |
| Phase 5 | 📋 Planned | Polish, broker sync, import formats, FIRE calculator |

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

## Phase 3 — Frontend UI 📋 Planned

**Goal:** Replace the placeholder `PortfolioDashboard` with the real portfolio
workspace.

### Deliverables
- [ ] Sidebar: permanent **Portfolio (📊)** entry (D6)
- [ ] Account management: create/edit/archive accounts, platforms
- [ ] Holdings view: positions, cost basis, market value, gains
- [ ] Activity ledger: transactions, dividends, fees
- [ ] Lot tracking: open lots, FIFO disposal history
- [ ] Valuation & performance charts (time-weighted, XIRR)
- [ ] Allocation view: actual vs target, drift warnings
- [ ] Snapshots & net worth timeline
- [ ] Income view: dividends/interest aggregation, YoY growth

### Acceptance
- [ ] `pnpm lint` + `pnpm typecheck` green
- [ ] i18n catalogs extended (existing Portfolio zh/en)
- [ ] Empty / partial / error states for every async surface

---

## Phase 3.5 — Repository CRUD Commands 📋 Planned

**Goal:** Thin Tauri commands exposing repository-level CRUD so the main
application can seed data without frontend work.

### Deliverables
- [ ] `create_platform`, `list_platforms`, `get_platform`
- [ ] `create_financial_account`, `list_financial_accounts`, `archive_financial_account`
- [ ] `create_asset`, `upsert_quote`
- [ ] `create_activity`
- [ ] `create_lot` (manual lot seeding)
- [ ] `create_taxonomy` (categories for allocation)
- [ ] Market-data commands: `search_symbols`, `get_quote`, `get_asset_profile`

### Acceptance
- [ ] All commands registered in `lib.rs` with `AppState` wiring
- [ ] Command tests (repository-backed) green

---

## Phase 4 — Thesis ↔ Holding Linkage 📋 Planned

**Goal:** Connect research to decisions (D5).

### Deliverables
- [ ] Migration: `theses.portfolio_holding_id TEXT NULL REFERENCES holdings(id)`
- [ ] Thesis editor: optional holding picker
- [ ] Portfolio review surfaces linked thesis alignment, confidence, validation
- [ ] Knowledge graph: optional holding nodes

### Acceptance
- [ ] End-to-end: create holding → link thesis → review shows alignment

---

## Phase 5 — Polish & Extras 📋 Planned

### Deliverables
- [ ] Broker sync (Wealthfolio `crates/connect`) — optional, gated add-on
- [ ] CSV import formats: generic + IBKR first
- [ ] FIRE / retirement calculator port (`planning::retirement`)
- [ ] Market-data refresh scheduling & quote caching
- [ ] Sweep remaining `unwrap()`/`expect()` in ported code
- [ ] Final full `pnpm check` + `cargo clippy -- -D warnings` clean

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
Phase 5 (polish, sync, FIRE)
```

Phase 1 → Phase 2 must precede everything. Phase 2.5 is independent of the
frontend and can proceed in parallel with Phase 3 planning. Phase 4 requires
Phase 3 (holding management UI) to be usable.

---

## Definition of Done (every phase)

- [ ] `cargo test` green
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] No new `unwrap()`/`expect()` outside tests
- [ ] Docs updated before the code ships
- [ ] PR merged via single-feature PR into `feature/portfolio-integration`

---

## References

- [Integration Plan](../PORTFOLIO_INTEGRATION_PLAN.md) — approved decisions D1–D10
- [API Specification](API_SPEC.md) — live command surface
- [Domain Models](DOMAIN_MODELS.md) — enum/struct reference
- [Frontend Integration](FRONTEND_INTEGRATION.md) — flagship UI plan
- [Wealthfolio Audit](../wealthfolio-audit/README.md) — 14 audit documents