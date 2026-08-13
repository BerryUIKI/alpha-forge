# Option Module Integration Plan

## Purpose and schedule

This plan integrates the Option module into the current AlphaForge architecture as milestone M9. M9 begins only after the M8 local MVP gate is complete, unless the product owner explicitly changes the milestone order. This prevents a large derivatives feature from destabilizing MVP release work.

The plan supersedes the earlier assumption that `integration/option` can be merged into `dev` in one step. The historical candidate remains valuable, but current `dev` has evolved and the candidate changes shared agent, workspace, and application-state files.

## Baseline evidence

The current `dev` baseline contains:

- `crates/domain/src/option.rs`.
- `apps/desktop/src-tauri/migrations/0004_options_support.sql` (historical) and the canonical `0014_options_support.sql` runtime migration.
- Six Option repository modules under `apps/desktop/src-tauri/src/database/repositories/`, wired to the canonical schema.
- `apps/desktop/src/types/option.ts`.
- `crates/option-core`, Option services and Tauri commands, the desktop API module, route, hooks, and feature components.
- The Option specifications in this directory.

The runtime is not yet accepted as a complete vertical slice:

- The canonical persistence baseline is implemented and focused migration verification passes; repository CRUD/isolation coverage remains pending.
- Option IPC request/response naming is now normalized at the command boundary: Rust request/response DTOs use camelCase serde while domain and database models remain snake_case. The desktop API no longer invokes the unsupported `create_option_chain` command; `fetch_option_chain` is the acquisition and persistence path.
- Chain selection, contract detail, and persisted-strategy UI paths are incomplete or unreachable.
- No end-to-end Option workflow has been verified on `dev`.

`origin/integration/option` contains candidate implementations for many of these items. They must pass the gates below against the then-current `dev`.

## Integration principles

1. Integrate by vertical slice, not by a single historical branch merge.
2. Preserve current domain and repository abstractions where valid.
3. Repair persistence first; no service is integrated against a schema the runtime never creates.
4. Keep calculations pure and deterministic inside `option-core`.
5. Keep network, credentials, filesystem import, and background work in Rust.
6. Route every frontend call through typed `desktopApi` functions and Zod validation.
7. Preserve source, timestamp, calculation model, assumptions, and confidence with analytical output.
8. Never add trading, brokerage execution, or autonomous recommendations.

The repaired Option IPC contract is enforced by focused Rust serde fixtures, desktop API malformed-response tests, and `scripts/check-option-ipc-registration.mjs`, which compares every Option wrapper command with the handlers registered in `lib.rs`.

## Stage O0: Rebaseline and decision gate

**Actions**

- Confirm M8 is complete and M9 is active in [Milestone Roadmap](../MILESTONE_ROADMAP.md).
- Refresh `dev` and re-run the baseline inventory.
- Review the Option product boundary with a domain reviewer.
- Diff every candidate file against current `dev` and classify it: reuse, adapt, reject, or obsolete.
- Record ADRs for pricing models, market-data providers, and Option Artifact isolation before privileged implementation.
- Resolve the migration strategy without editing historical migrations.

**Exit criteria**

- Approved M9 scope and owner.
- File-level candidate disposition.
- Approved migration and provider decisions.
- No unresolved secret, data-licensing, or trade-execution ambiguity.

## Stage O1: Persistence repair

**Actions**

- Add and register the append-only `0014_options_support.sql` migration after the current highest migration to create or reconcile Option tables and indexes idempotently.
- Register it in the custom migration runner.
- Map database rows to existing domain types through repositories.
- Add fresh-database, historical-database, repeat-run, cascade, workspace-isolation, and repository CRUD tests.
- Keep the historical `0004_options_support.sql` label and file unchanged; document `0014_options_support` as the canonical runtime migration.

**Exit criteria**

- A new and upgraded database receives the same canonical Option schema.
- Running migrations twice is safe.
- Incompatible legacy Option tables fail recoverably without deleting existing rows.
- DDL and `_migrations` registration roll back together on failure.
- Repository failures map to stable application errors.
- Historical runtime migrations under `apps/desktop/src-tauri/migrations/` are not renamed, deleted, or modified; the unused incompatible nested `src/database/migrations/0014_option_chain_tables.sql` is removed.

## Stage O2: Pricing and provider core

**Actions**

- Create `crates/option-core` as a pure workspace crate.
- Implement and validate European pricing, Greeks, implied-volatility solving, and strategy payoff primitives in separately testable modules.
- Make model inputs explicit: spot, strike, time, rate, volatility, dividend yield, exercise style, multiplier, and valuation timestamp.
- Validate finite values, ranges, expiration, convergence, and numerical tolerance.
- Add a provider trait in Rust. Start with deterministic demo and validated file providers; keep live providers disabled until approved.
- Benchmark representative chains and document hardware and dataset size with results.

**Exit criteria**

- Published reference fixtures and boundary cases pass within documented tolerances.
- Invalid or non-convergent calculations return typed recoverable errors.
- Benchmarks are recorded; targets are not claimed without evidence.
- Provider output includes provenance and retrieval timestamps.

## Stage O3: Option-chain vertical slice

```text
Select workspace and symbol
  -> validate request in Tauri command
  -> OptionService calls approved provider
  -> calculate and persist chain/contracts/Greeks
  -> emit bounded progress events if long-running
  -> desktopApi validates response
  -> React renders loading, success, empty, partial, offline, and error states
  -> user opens a contract and retains provenance
```

**Actions**

- Add thin commands and an `OptionService`; do not put SQL or formulas in commands.
- Return a task ID for provider or chain work that can exceed the short IPC budget.
- Implement cancellation, timeout, retry, concurrency, token/cost (if AI is involved), and restart behavior.
- Add the typed desktop API, TanStack Query hooks, route, navigation, and virtualized chain UI.
- Localize the new surface using the [i18n architecture](../i18n/ARCHITECTURE.md).

**Exit criteria**

- Demo and file-backed chains complete end to end and persist.
- Every asynchronous UI state is tested.
- Workspace isolation and input/path validation pass.
- Live networking and keys remain disabled or fully approved.

## Stage O4: Strategy and Artifact slice

- Build a strategy from validated legs and immutable contract snapshots.
- Calculate cost, break-even points, payoff series, max profit/loss where bounded, and aggregate Greeks.
- Persist strategy assumptions and calculation version.
- Render validated JSON with a predefined React Option Artifact renderer.
- Keep Agent-generated HTML and arbitrary code out of privileged windows.
- Add unit, schema, component, IPC integration, and E2E tests for one representative spread and one unbounded-risk strategy.

**Exit criteria:** a user can build, save, reopen, and explain a strategy; the UI clearly distinguishes model output from a recommendation.

## Stage O5: Scenario and portfolio-risk slice

- Integrate Option positions through portfolio services, not direct cross-repository calls in commands.
- Calculate aggregate Greeks and bounded price/volatility/time scenarios.
- Preserve missing/stale quote states rather than fabricating current values.
- Link results to research, thesis evidence, and review artifacts through stable identifiers.
- Require human confirmation for any decision record derived from analysis.

**Exit criteria:** mixed equity/Option risk is explainable, provenance is visible, and no trade action exists.

## Stage O6: Release gate

Required evidence:

- Full Rust, TypeScript, migration, repository, component, IPC, and critical E2E suites.
- Calculation fixtures independently reviewed by an Option-domain reviewer.
- Performance measurements for representative chain sizes.
- Security review of providers, file import, permissions, Artifact payloads, and logs.
- Accessibility and both-locale UI review.
- Fresh install, historical upgrade, restart, cancellation, offline, partial-data, and packaged-build smoke tests.
- Documentation reconciled with final file paths and contracts.

## Integration PR checklist

- [ ] Branch is based on current `dev` and contains one scoped slice.
- [ ] Database changes are append-only and upgrade-tested.
- [ ] No credentials, shell capability, arbitrary filesystem scope, or privileged Artifact command was added without approval and documentation.
- [ ] Structured inputs and outputs are validated in Rust and TypeScript.
- [ ] Model assumptions, provider provenance, timestamps, and stale-data behavior are visible.
- [ ] Tests and benchmarks were actually run and recorded.
- [ ] No trading or autonomous recommendation path exists.
- [ ] Relevant Option and milestone documents are updated.

## Rollback

Before merge, each slice documents how to disable its route or feature flag without deleting persisted data. Rollback uses a normal revert PR; never rewrite `dev` history or remove applied migrations. A schema repair is a later append-only migration.

## Success criteria

M9 is complete only when all required slices are on `dev`, the release gate has evidence, and the milestone record is updated. Candidate-branch code, unchecked boxes, or a successful compile alone are insufficient.
