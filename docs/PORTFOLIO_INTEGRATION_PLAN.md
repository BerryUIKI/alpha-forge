# Portfolio Integration Plan (Wealthfolio → AlphaForge)

**Status:** Phases 0-4 integrated; Phase 5 hardening is governed by M11
**Historical branch:** `feature/portfolio-integration` (merged into `dev`)
**Originally approved:** 2026-08-13
**Baseline revalidated:** 2026-09-20 at `origin/dev` commit `9e21553`
**Author:** Handoff from planning session (see conversation history)
**Goal:** Fully integrate Wealthfolio's portfolio functionality into this
repository's Portfolio module, resolving Wealthfolio's technical debt instead
of copying it.

---

## 1. Purpose

This document is the single source of truth for the Wealthfolio integration.
It records the decisions already made with the user, the current state of both
codebases, and the phased execution plan. Read it before writing any code.

Related reference material (do not re-derive):

- `docs/wealthfolio-audit/` — 14 audit documents of the Wealthfolio codebase
- `F:\dev\wealthfolio` — local clone of the Wealthfolio source (read-only reference)
- `docs/DATA_MODEL.md`, `docs/ARCHITECTURE.md` — this repo's own model

---

## 2. Why

Wealthfolio is a mature open-source portfolio tracker (Tauri 2 + React + Rust,
AGPL-3.0, v3.7.0) with a full financial domain: accounts, holdings, activities
(transactions), lots, valuation, performance, snapshots, goals, net worth, and
imports. AlphaForge (this repo) is an AI-native research workspace whose
current `portfolio` domain module is a placeholder (4 basic structs, ~150 lines).

The user decided: **full monorepo merge (Route C)** — absorb Wealthfolio's
capability into AlphaForge's Portfolio module, resolve all Wealthfolio
technical debt in the process, and keep development unfrozen by shipping in
phases.

---

## 3. Decisions (already made — do not revisit without the user)

| # | Decision | Rationale |
|---|----------|-----------|
| D1 | **Full monorepo merge.** Wealthfolio code is absorbed crate-by-crate into this repo. | Long-term maintainability of a single product. |
| D2 | **Single SQLite database.** All tables (research + financial) in one DB file. | Fewer databases, simpler backups/sync, local-first. |
| D3 | **ORM = SQLx.** Wealthfolio uses Diesel; we migrate all financial persistence to SQLx. | Matches this repo's existing ORM. No Diesel in this repo. |
| D4 | **Domain model unification.** Financial domain (Account, Holding, Activity, Lot, Portfolio, Snapshot) and research domain (Thesis, Research, Artifact, KnowledgeGraph) remain separate modules in the same `crates/domain` crate, linked by foreign keys. | Keeps modules independent; avoids a god-model. |
| D5 | **Data linking:** `InvestmentThesis.portfolio_asset_id: Option<String>` links a thesis to a canonical financial asset (user-optional). | One asset may appear in multiple accounts/lots, so the canonical asset is the stable research link. |
| D6 | **Sidebar placement:** permanent **Portfolio (📊)** entry in the left sidebar (Workspace group). | User-approved mockup. Note (2026-08-16): implemented in the redesigned `LeftSidebar` nav groups (`NavItem`, route `/portfolio`). |
| D7 | **Phased execution** so the app never freezes. | Development continuity. |
| D8 | **Do NOT copy Wealthfolio's debt:** 3,264 unwrap/expect panic points, Diesel-specific repo layer, hardcoded prod URLs, addon runtime. | Debt resolution is a primary goal. |
| D9 | **Historical integration phases allowed parallel agents** for independent workstreams (for example, migration and UI). The active M11 queue is strictly sequential. | Preserve the original delivery decision without overriding the current execution contract. |
| D10 | **No autonomous trading.** The app tracks portfolios; it never executes trades or auto-decides investments. | Product constraint (AGENTS.md §15). |

---

## 4. Historical pre-integration state

This section records the original baseline used to approve the integration. It
is not the current implementation inventory. Use the
[Portfolio Roadmap](portfolio/ROADMAP.md), current source, and the
[M11 execution plan](milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md) for
current work. Do not use this historical snapshot to recreate superseded code.

### 4.1 Domain (`crates/domain/src/portfolio.rs`, ~150 lines)

Placeholder models: `PortfolioAccount`, `Position`, `PortfolioTransaction`,
`PortfolioAllocation`, `ConcentrationRisk`, `ThemeExposure`,
`CreatePortfolioThemeLinkInput`, `ThesisAlignment`, `PortfolioReview`,
`TransactionType` (Buy/Sell), `ConcentrationSeverity`.
Positions use `f64` quantity — replace with `rust_decimal` where financial
precision matters (Wealthfolio uses `rust_decimal`).

`crates/domain` also contains `thesis.rs` (`InvestmentThesis`, `ThesisEvidence`,
`ThesisConfidenceSnapshot`), `workspace.rs`, `research.rs`, `artifact.rs`,
`knowledge_graph.rs`, `option.rs`, `task.rs`. Workspace deps include `sqlx`
0.8, `chrono`, `uuid`, `serde`, `thiserror`, `tokio`.

### 4.2 Persistence (`apps/desktop/src-tauri/`)

- SQLx migrations dir: `apps/desktop/src-tauri/migrations/` (0010_portfolio_management.sql
  adds workspace-scoped portfolio_accounts indexes; 0011 adds portfolio_theme_links).
- `src/database/connection.rs`, `src/database/migrations.rs` — SQLx setup.
- `src/database/repositories/portfolio_repository.rs` (288 lines) — SQLx-based,
  minimal repos (accounts, positions, transactions, allocation, concentration,
  theme links, thesis alignment, review).
- Repos exist for: agent_task, artifact, greeks, knowledge_graph,
  option_chain/contract/position/strategy, plugin, portfolio, research_*,
  settings, strategy_leg, thesis, workspace.

### 4.3 Services (`apps/desktop/src-tauri/src/services/portfolio_service.rs`)

Existing `portfolio_service` with `create_account`, `list_accounts`,
`create_position`, `list_positions`, `import_transactions_csv`,
`list_transactions`, `allocation_by_workspace`, `concentration_risks`,
`link_theme`, `theme_exposure`, `thesis_alignment`, `review`.

### 4.4 Commands (`apps/desktop/src-tauri/src/commands/portfolio.rs`)

Thin Tauri commands (14) wrapping the service — the existing pattern to follow.

### 4.5 Frontend (`apps/desktop/src/`)

- Route: `portfolio` → `pages/portfolio/PortfolioPage.tsx` (7 lines) →
  `features/portfolio/components/PortfolioDashboard.tsx`.
- `features/portfolio/components/` has `AccountManagement/` (CreateAccountForm,
  AccountList, PositionPanel) and `Analysis/` (AllocationPanel,
  ConcentrationPanel, ThemeExposurePanel, AlignmentReviewPanel).
- Hooks: `features/portfolio/hooks/usePortfolio.ts` (TanStack Query).
- Desktop API client: `src/lib/desktop-api/portfolio.ts`.
- Sidebar: redesigned `LeftSidebar` with `NavItem`/`NavGroup` navigation groups
  (Workspace: Dashboard, Research, Theses, Portfolio, Knowledge, Journal;
   Tools: Options, Artifacts; Account: Settings). The old `FunctionalViewSelector`,
  `ToolsList`, `UserOperations` were removed in the GUI redesign (PRs #105–#111).
- i18n catalogs exist for Portfolio (zh/en) — extend, don't fork.

---

## 5. Target architecture

```
apps/desktop (Tauri)  ── IPC ──►  commands/  ──►  services/
                                                          │
crates/domain ◄─── (pure models, no I/O) ──── repositories (SQLx)
   ├── portfolio (financial domain, ported from Wealthfolio)
   ├── thesis / research / workspace / ...   (research domain, existing)
   └── cross-links: thesis.portfolio_asset_id → financial_assets.id
                                                          │
                                            SQLx migrations (single SQLite DB)
```

- **Financial domain module** (`crates/domain/src/portfolio/` or
  `crates/domain-portfolio/`): port Wealthfolio's models from
  `crates/core/src/portfolio/` — **models only**, no Diesel.
- **Persistence:** new SQLx migrations ported from Wealthfolio's
  `crates/storage-sqlite/migrations/` (48 Diesel migrations → fewer, cleaner
  SQLx migrations; schema spec in `docs/wealthfolio-audit/04-data-structure-spec.md`).
- **Business logic:** port Wealthfolio's `crates/core` portfolio services
  (performance, valuation, holdings, lots, allocation, snapshot, FIRE,
  net-worth, income) onto SQLx repositories — *without* the unwrap/expect
  panic points; return typed errors (`AppError` / domain `Result`).
- **Market data:** port `crates/market-data` (Fully-Reusable, zero internal
  deps) for quotes/price enrichment.
- **Import:** port CSV import (activities) and broker sync (`crates/connect`)
  later, as optional add-ons (Phase 4/5).

---

## 6. Wealthfolio debt NOT to copy (from `07-code-debt-risk-report.md`)

1. **3,264 unwrap/expect** across 140 files (1,935 unwrap + 1,329 expect).
   Hotspots to rewrite with `?` + typed errors: `performance_service.rs` (191),
   `holdings_service.rs` (102), `lots.rs` (88), `valuation_service.rs` (84).
2. **Diesel bake-in:** `WriteHandle::exec(FnOnce(&mut SqliteConnection))`,
   r2d2 pool, `immediate_transaction()` — the write-actor pattern is worth
   keeping conceptually, the Diesel API is not.
3. **`crates/core` hub coupling:** one `Error` type aggregates 8 sub-errors;
   storage-sqlite is Non-Detachable. We port models + services, not the crate
   graph.
4. **Hardcoded production URLs** (auth/connect/wealthfolio.app) and the addon
   runtime (`apps/frontend/src/addons/`, Non-Detachable) — out of scope; we
   don't port the addon system.
5. **Dead/duplicated code** flagged in `12-optimization-todo-list.md`.

---

## 7. Phased execution plan

The phases below record the historical integration sequence completed on
`feature/portfolio-integration`. The active M11 work must instead follow the
[M11 execution plan](milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md), with
one task branch and one PR executed sequentially from the latest `origin/dev`.

### Phase 0 — Prerequisite: debt baseline ✅ Done

- [x] Agree the target error model (`AppError` shape: code/message/context/recoverable).
- [x] Port **only** the cleanest Wealthfolio services as the "reference
      implementation" to learn the domain; do not copy panic-prone code.
- [x] Decide model precision policy: `rust_decimal` for money/quantity
      (matches Wealthfolio); keep `f64` only for display-only ratios.

### Phase 1 — Storage: financial schema on SQLx ✅ Done

- [x] Add SQLx migrations porting the financial schema from
      `docs/wealthfolio-audit/04-data-structure-spec.md` (accounts, holdings,
      activities, lots, valuations/snapshots, goals, net-worth, tax/cash-flow
      tables as needed for the chosen feature slice).
- [x] Keep existing research tables intact and add the nullable
      `portfolio_asset_id` link through append-only migration 0023 in Phase 4.
- [x] Build SQLx repositories following `portfolio_repository.rs` conventions
      (module + `_test.rs` per repo).
- [x] **Verify:** `cargo test` green; migration runs from clean DB;
      no `unwrap()` in new repo code.

### Phase 2 — Core: financial business logic (est. 3–4 weeks) ✅ DONE

- [x] Port domain models into `crates/domain` (financial module), replacing
      the placeholder structs. Use `rust_decimal`.
- [x] Port services: holdings, lots (FIFO/cost-basis), valuation,
      performance (XIRR, time-weighted), allocation/allocation-targets,
      snapshots, net-worth. (Income and FIRE deferred to Phase 2.5.)
- [x] Every ported service: typed `Result`, no panic paths; unit tests ported
      from Wealthfolio (`crates/core` has existing tests to adapt).
- [x] **Verify:** `cargo test`; `pnpm type-check` unaffected (domain is Rust-only).

### Phase 2.5 — Market data + Income ✅ Done

Split from the original Phase 2 scope. Independent of the frontend — can be
developed and tested without the main application.

- [x] Port `crates/market-data` as a new workspace crate (quotes, asset profile,
      provider registry, resolver chain, fixture + Yahoo providers).
- [x] Port `IncomeService` — income aggregation (by month, type, asset, currency,
      account) with YoY growth, on the existing `ActivityRepository`.
- [x] Add domain models for income summaries to `crates/domain/src/financial.rs`.
- [x] **Verify:** `cargo test`; `cargo clippy -- -D warnings` clean.

### Phase 3 — Frontend: portfolio UI (est. 3–4 weeks) ✅ DONE

- [x] Sidebar: add permanent **Portfolio (📊)** entry (Workspace nav group) per
      D6 — integrated in the redesigned LeftSidebar nav groups (`NavItem`, route `/portfolio`).
- [x] Build pages from Wealthfolio's financial UX reusing AlphaForge dark aesthetic
      and UI components.
- [x] Integrate full financial CRUD dialogs: `AddActivityDialog` (automatic FIFO lots on buy,
      FIFO liquidation on sell), `CreateAccountDialog`, `AddAssetDialog`.
- [x] Switch `AccountCards` and `ActivityList` to reactive financial hooks with TanStack Query.
- [x] Extend i18n catalogs (full bilingual parity for English and Chinese).
- [x] Live SEC EDGAR filing stream integrated with automated thesis sentiment mapping.
- [x] **Verify:** `pnpm lint`, `pnpm typecheck`, `pnpm test` (512 passing tests), `cargo test` (347+ passing).

### Phase 4 — Data linkage: research ↔ portfolio (est. 2 weeks) ✅ DONE

- [x] Migration `0023_theses_portfolio_asset_link.sql`: add `portfolio_asset_id TEXT NULL REFERENCES assets(id)`
      to `investment_theses` (binding canonical asset entity to all associated holdings/lots).
- [x] Domain & Repository: update `InvestmentThesis`, `CreateThesisInput`, `ThesisRepository`, and `ThesisService`
      with `portfolio_asset_id` and `link_asset(thesis_id, asset_id)`.
- [x] Tauri IPC & API: expose `link_thesis_asset` command, update `create_thesis` DTO and `desktopApi.thesis`.
- [x] Thesis UI: optional "Link to Financial Asset" picker in `CreateThesisForm`, linked asset management in `ThesisDetail`,
      and linked asset badge pill in `ThesisCard`.
- [x] Portfolio review: enhance `thesis_alignment` in `portfolio_repository.rs` to prioritize authoritative
      `portfolio_asset_id` asset linkage alongside symbol matching.
- [x] **Verify:** `pnpm lint`, `pnpm typecheck`, `pnpm test` (513 tests passing), `cargo test` (348+ passing),
      `cargo clippy` and `cargo fmt` clean.

### Phase 5 — Polish & debt cleanup (est. 2–3 weeks, ongoing)

- [x] Generic and IBKR activity-statement file import (PR #213).
- [x] Market-data refresh scheduling & quote caching (MarketDataService, Tauri commands, TanStack Query mutations, and UI refresh triggers).
- [x] Audit canonical versus legacy Portfolio runtime paths (M11-01, see [`portfolio/LEGACY_SURFACE_AUDIT.md`](portfolio/LEGACY_SURFACE_AUDIT.md)).
- [ ] Retire superseded Portfolio runtime paths (M11-02).
- [ ] Establish and meet approved large-portfolio performance budgets (M11-03).
- [ ] Sweep remaining production panic paths and scale hardening (M11-04).
- [ ] Produce reproducible macOS Apple Silicon and Windows NSIS packages (M11-05).
- [ ] Retain packaged smoke and release-candidate acceptance evidence (M11-06, M11-07).

The active sequence and acceptance criteria are defined in the
[M11 execution plan](milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md).
Live broker synchronization and FIRE/retirement planning are deferred to a
separately approved milestone.

The original estimate was approximately 15 weeks. It is historical and must not
be used as a forecast for M11.

---

## 8. Key risks & mitigations

| Risk | Mitigation |
|------|------------|
| Diesel schema port is lossy (48 migrations, unusual types like `numeric`) | Use `04-data-structure-spec.md` as schema source of truth; verify with row-level tests; keep the write-actor pattern. |
| Ported services carry panic-prone code | Phase 0 policy: typed errors only; clippy `-D warnings`; review gate. |
| Frontend scope creep (charts, tables) | Reuse existing `packages/ui` + `financial-components` package; no `@wealthfolio/ui` import. |
| DB migration conflicts with research tables | Append-only migrations; name financial migrations `0015_...` onward; never edit applied migrations. |
| Thesis-holding link breaks existing thesis flow | `Option<String>` FK, nullable; UI defaults to "no link". |
| Multiple collaborators collide on shared files | Assign one M11 task at a time and merge each focused PR into `dev` before the next task starts. |

### Historical agent file ownership

| Agent | Owns |
|-------|------|
| A (storage) | `migrations/` financial tables, `database/repositories/` new repos |
| B (core) | `crates/domain` financial module, `services/` financial services |
| C (frontend) | `src/features/portfolio/`, `src/pages/portfolio/`, i18n, `LeftSidebar` nav groups |
| D (linkage) | `theses` migration + thesis service + portfolio review (after B) |

---

## 9. Definition of done (every phase)

- [ ] `cargo test` green
- [ ] `pnpm lint` + `pnpm typecheck` green (frontend phases)
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] No new `unwrap()`/`expect()` outside tests
- [ ] Migration tested from a clean DB
- [ ] Docs updated where behavior changed

---

## 10. Recorded scope dispositions

1. The first integrated release uses the approved local Portfolio subset;
   FIRE/retirement planning is deferred.
2. Market data begins with deterministic fixture behavior and the approved
   provider path; additional providers require their own review.
3. File import supports Generic and IBKR activity statements. Other formats
   and live broker synchronization are deferred.

---

## 11. Handoff prompt

The companion handoff prompt for the next agent is provided in the
conversation that created this document (user pastes it into the next session).
