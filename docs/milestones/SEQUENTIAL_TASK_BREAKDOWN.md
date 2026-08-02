# Sequential Child-Agent Task Breakdown

This document is the canonical execution queue for M8 through M10. A coordinator assigns exactly one task to one child agent at a time. The next task does not start until the previous task is merged into `dev` and its acceptance evidence is recorded.

## Execution contract

Every child agent must follow these rules:

1. Fetch the latest remote state and create the task branch from `origin/dev`. Never implement or commit directly on `dev` or `main`.
2. Use one task branch and one small-granularity PR per task. Target `dev` unless this document explicitly names an approved parent branch.
3. Prefer one cohesive Conventional Commit. Use a second commit only when implementation and independently reviewable test/documentation evidence cannot reasonably be kept together. Do not accumulate unrelated cleanup commits.
4. Inspect the current repository, related tests, migrations, and documentation before editing. Reuse existing abstractions and audit any unmerged candidate branch before copying code.
5. Update related documentation in the same implementation PR. All documentation, code comments intended as documentation, PR titles, and PR descriptions must be written in English.
6. Do not edit an applied migration. Add a later append-only migration and test fresh, upgrade, partial, and repeat-run paths.
7. Keep Rust responsible for SQLite, filesystem, network, credentials, background tasks, cancellation, and permission enforcement. Keep React responsible for presentation and route all IPC through `desktopApi`.
8. Do not add securities execution, brokerage order routing, autonomous investment decisions, unrestricted shell/filesystem access, or privileged Agent-generated HTML.
9. Run the verification checklist and attach actual results to the PR. Never report an unrun check as passed.
10. Stop after the assigned task. Do not begin, partially scaffold, or commit the next task.

## Queue summary

| Order | Task range       | Milestone               | Entry condition                                                      |
| ----- | ---------------- | ----------------------- | -------------------------------------------------------------------- |
| 1     | M8-01 to M8-14   | M8 Local MVP Completion | M8 is activated and product-owner decisions can be resolved          |
| 2     | M9-01 to M9-08   | M9 Option Integration   | M8 is complete unless the product owner explicitly changes the order |
| 3     | M10-01 to M10-09 | M10 Goose Integration   | M8 is complete and M9 is complete or independently waived            |

File lists below are the minimum expected scope. The child agent must inspect the current tree and list any additional necessary files in the PR without broadening the task objective.

---

## M8 — Local MVP Completion & Release Readiness

### M8-01 — Finalize M8 product and release decisions

**Task objective**

Resolve the launch locale, platforms, privacy/export policy, update method, signing/notarization policy, legal reviewer, security contact, support owner, and release owner before implementation begins.

**Acceptance criteria**

- Every blocking `TODO` required for the local MVP is resolved or explicitly deferred with an owner and rationale.
- `zh-CN` and `en` support, launch default, and translation reviewer are approved.
- No authentication, billing, cloud sync, telemetry, or commercial entitlement work is implicitly authorized.
- Milestone status remains accurate and does not claim implementation completion.

**Involved files**

- `docs/M8_DECISION_RECORD.md`
- `docs/MILESTONE_ROADMAP.md`
- `docs/NEXT_STEPS.md`
- `docs/i18n/README.md`

**Target branch type**

- Type: `docs/*`
- Example: `docs/m8-decisions`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Product owner and named reviewers approved the recorded decisions.
- [ ] `git diff --check` passes.
- [ ] Markdown formatting and local links pass.
- [ ] All changed documentation is English.

### M8-02 — Inventory UI strings and approve the terminology guide

**Task objective**

Inventory every user-visible string and asynchronous state, classify ownership, and create the English terminology guide used by later i18n tasks. Translated application values belong in locale catalog data, not documentation.

**Acceptance criteria**

- Inventory covers navigation, settings, common states, workspace, research, journal, portfolio, Agent, Artifacts, errors, and deferred Option UI.
- Every critical string group has a namespace and owner.
- The English terminology guide defines canonical source terms for research, thesis, evidence, portfolio, risk, and Options without translating user content.
- Documentation contains English only; Simplified-Chinese values are added to code-owned locale catalogs by the implementation tasks.
- The i18n index and milestone registry link the terminology guide and inventory.

**Involved files**

- `docs/i18n/README.md`
- `docs/i18n/IMPLEMENTATION_PLAN.md`
- `docs/i18n/TERMINOLOGY_GUIDE.md` (new)
- `docs/MILESTONE_ROADMAP.md`
- `apps/desktop/src/**/*.tsx` (read-only inventory source)

**Target branch type**

- Type: `docs/*`
- Example: `docs/i18n-inventory`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] String inventory has no unassigned critical surface.
- [ ] A bilingual reviewer approved the terminology rules and recorded any translated values in the implementation review, not in documentation.
- [ ] `git diff --check` and Markdown formatting pass.
- [ ] Local links pass and all explanatory documentation is English.

### M8-03 — Implement the typed i18n runtime foundation

**Task objective**

Add one typed locale runtime with deterministic fallback, persisted locale selection, shared `Intl` formatters, source/translated catalog parity, and provider tests.

**Acceptance criteria**

- `SupportedLocale` accepts only `en` and `zh-CN`.
- Locale precedence follows the approved decision record and invalid persisted values recover safely.
- Locale changes persist through the existing Settings service and rerender without restart.
- Common, navigation, and settings catalogs have parity; date, number, percent, and currency helpers are tested.
- No second locale framework, direct SQLite access, or plaintext credential path is introduced.

**Involved files**

- `apps/desktop/src/lib/i18n/**` (new or reconciled)
- `apps/desktop/src/app/providers.tsx`
- `apps/desktop/src/lib/desktop-api/settings.ts`
- `apps/desktop/src-tauri/src/services/settings_service.rs`
- `apps/desktop/src-tauri/src/commands/settings.rs`
- `docs/i18n/ARCHITECTURE.md`
- `docs/i18n/IMPLEMENTATION_PLAN.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-runtime`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Locale validation, fallback, persistence, provider, catalog parity, and formatter tests pass.
- [ ] `pnpm lint`, `pnpm typecheck`, and focused/full frontend tests pass.
- [ ] Relevant Rust tests pass if Settings code changes.
- [ ] `git diff --check` passes and i18n documentation is updated in English.

### M8-04 — Localize the application shell and common states

**Task objective**

Localize navigation, settings, command surfaces, dialogs, tooltips, accessibility labels, and reusable loading/empty/error/partial/offline components.

**Acceptance criteria**

- Shell and common states render correctly in both locales and switch at runtime.
- Keyboard navigation, focus indicators, `aria-label` values, and error recovery remain functional.
- No newly introduced user-visible shell string bypasses the catalog.
- Components tolerate at least 30 percent text expansion without clipping.

**Involved files**

- `apps/desktop/src/components/navigation/Sidebar.tsx`
- `apps/desktop/src/pages/settings/SettingsPage.tsx`
- `apps/desktop/src/components/common/**`
- `apps/desktop/src/components/CommandPalette.tsx` if present
- `apps/desktop/src/lib/i18n/catalogs/**`
- related component tests
- `docs/i18n/IMPLEMENTATION_PLAN.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-shell`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Shell and shared-state tests pass in both locales.
- [ ] Keyboard and accessibility assertions pass.
- [ ] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M8-05 — Localize Workspace and Today workflows

**Task objective**

Complete the create/select workspace and Today dashboard workflows in both locales, including all asynchronous and validation states.

**Acceptance criteria**

- Workspace creation, selection, update, deletion confirmation, and Today dashboard states are localized.
- Validation and stable Rust error codes map to safe localized messages.
- Dates and counts use shared formatters without changing stored values.
- Critical workflow tests run once per locale.

**Involved files**

- `apps/desktop/src/features/workspace/**`
- `apps/desktop/src/pages/today/TodayPage.tsx`
- `apps/desktop/src/lib/i18n/catalogs/*/workspace.ts`
- `apps/desktop/src/lib/i18n/catalogs/*/common.ts`
- related tests
- `docs/i18n/IMPLEMENTATION_PLAN.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-workspace-today`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Workspace/Today component and validation tests pass in both locales.
- [ ] Initial, loading, success, empty, error, partial, and offline states are reviewed.
- [ ] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M8-06 — Localize the Research workflow

**Task objective**

Localize research projects, documents, sources, notes, search, imports, and reports while preserving source quotations and provenance unchanged.

**Acceptance criteria**

- Research controls and every asynchronous state are available in both locales.
- User content, imported text, URLs, evidence quotations, and generated reports are not silently translated.
- File/URL validation errors use localized stable error mappings without exposing paths or provider details.
- Search and import behavior remains unchanged.

**Involved files**

- `apps/desktop/src/pages/research/ResearchPage.tsx`
- `apps/desktop/src/features/research/**` if present
- `apps/desktop/src/lib/i18n/catalogs/*/research.ts`
- research component/API tests
- `docs/i18n/IMPLEMENTATION_PLAN.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-research`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Research tests cover both locales and all async states.
- [ ] Provenance and user-content non-translation are regression-tested.
- [ ] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M8-07 — Localize Journal, thesis, and knowledge workflows

**Task objective**

Localize thesis creation, evidence direction, confidence history, validation, knowledge graph, and destructive confirmations without changing persisted enums or scores.

**Acceptance criteria**

- Journal/thesis/knowledge UI and all async states render in both locales.
- Persisted statuses, evidence directions, confidence values, and identifiers remain locale-neutral.
- Destructive and status-transition actions have explicit localized confirmation.
- Evidence quotations and thesis content remain user-authored content.

**Involved files**

- `apps/desktop/src/pages/journal/JournalPage.tsx`
- `apps/desktop/src/features/thesis/**`
- `apps/desktop/src/lib/i18n/catalogs/*/journal.ts`
- thesis/knowledge component and schema tests
- `docs/i18n/IMPLEMENTATION_PLAN.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-journal`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Thesis lifecycle and evidence tests pass in both locales.
- [ ] Stable enum/protocol serialization tests remain unchanged and pass.
- [ ] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M8-08 — Localize Portfolio workflows and financial formatting

**Task objective**

Localize account, position, transaction import, allocation, concentration, theme exposure, thesis alignment, and review surfaces with safe financial formatting.

**Acceptance criteria**

- Portfolio UI and all async states render in both locales.
- Currency codes are preserved, no conversion is implied, and percent/decimal units cannot be multiplied incorrectly.
- CSV import semantics remain locale-neutral and deterministic.
- Risk labels follow the approved terminology guide and do not become recommendations.

**Involved files**

- `apps/desktop/src/pages/portfolio/PortfolioPage.tsx`
- `apps/desktop/src/features/portfolio/**`
- `apps/desktop/src/lib/i18n/catalogs/*/portfolio.ts`
- formatter and portfolio tests
- `docs/i18n/IMPLEMENTATION_PLAN.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-portfolio`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Currency, percent, decimal, and risk-label tests pass in both locales.
- [ ] Portfolio component/API tests pass.
- [ ] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M8-09 — Localize Agent tasks, Artifacts, and stable errors

**Task objective**

Localize Agent task lifecycle, cancellation/failure states, Artifact registry/renderers, and stable error-code presentation without localizing logs or structured domain protocols.

**Acceptance criteria**

- Task statuses and all Artifact/common failure states render in both locales.
- Unknown error codes fall back safely; sensitive context is never interpolated.
- Artifact windows receive only the narrow locale/message data required by predefined renderers.
- Agent output and persisted Artifact JSON remain locale-neutral unless explicitly generated in a user-selected language.

**Involved files**

- `apps/desktop/src/features/agent/**`
- `apps/desktop/src/features/artifacts/**`
- `apps/desktop/src/pages/artifacts/**`
- `apps/desktop/src/lib/errors/**`
- `apps/desktop/src/lib/i18n/catalogs/*/{agent,artifacts}.ts`
- related tests
- `docs/i18n/ARCHITECTURE.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/i18n-agent-artifacts`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Task cancellation/failure and Artifact renderer tests pass in both locales.
- [ ] Unknown-error and redaction tests pass.
- [ ] `pnpm lint`, `pnpm typecheck`, `pnpm test`, and relevant Rust tests pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M8-10 — Implement user-controlled local backup export

**Task objective**

Deliver a native, user-initiated SQLite backup export with safe destination handling, consistent database copying, explicit success/failure UI, and privacy documentation.

**Acceptance criteria**

- React requests export only through `desktopApi`; Rust owns the save dialog and database operation.
- Export never overwrites an existing file silently and never exposes the source database path to React.
- Cancellation and I/O failures are recoverable and localized.
- Exported data remains local; no network or telemetry path is introduced.

**Involved files**

- `apps/desktop/src-tauri/src/services/system_service.rs`
- `apps/desktop/src-tauri/src/commands/system.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/lib/desktop-api/system.ts`
- `apps/desktop/src/pages/settings/SettingsPage.tsx`
- related Rust/frontend tests
- `docs/PRIVACY.md`
- `docs/RELEASES.md`
- `docs/MILESTONE_ROADMAP.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/m8-local-export`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Success, user cancellation, existing-target, and I/O failure tests pass.
- [ ] Exported database integrity and restart/open behavior are verified on a copy.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and privacy/release documentation is updated in English.

### M8-11 — Implement manual update discovery

**Task objective**

Add an explicit user-triggered update check against the approved release source, with strict URL policy, timeout, version validation, and manual download/install behavior.

**Acceptance criteria**

- No background polling, auto-download, or auto-install occurs.
- Rust performs the bounded network request; React receives a validated release summary only.
- Release URLs are allowlisted and redirects are validated.
- Offline, timeout, malformed response, current-version, and update-available states are localized and tested.

**Involved files**

- `apps/desktop/src-tauri/src/services/system_service.rs`
- `apps/desktop/src-tauri/src/commands/system.rs`
- `apps/desktop/src-tauri/src/security/url_policy.rs`
- `apps/desktop/src/lib/desktop-api/system.ts`
- `apps/desktop/src/pages/settings/SettingsPage.tsx`
- related tests
- `docs/RELEASES.md`
- `docs/PRIVACY.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/m8-manual-update`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] URL, redirect, timeout, schema, and semantic-version tests pass.
- [ ] Offline/current/update/error UI tests pass in both locales.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and release/privacy documentation is updated in English.

### M8-12 — Configure supported installer packaging

**Task objective**

Configure the approved macOS and Windows installer targets, application identity, icons, bundle metadata, and release build documentation without committing signing secrets.

**Acceptance criteria**

- Supported platform/architecture targets match the M8 decision record.
- Bundle identifiers, product name, version source, icons, and installer formats are consistent.
- Signing/notarization requirements and known unsigned-build warnings are documented.
- No certificate, token, private key, or release credential is committed.

**Involved files**

- `apps/desktop/src-tauri/tauri.conf.json`
- `apps/desktop/src-tauri/icons/**`
- `apps/desktop/package.json`
- release workflow files under `.github/workflows/` if approved
- `docs/RELEASES.md`
- `README.md`
- `docs/MILESTONE_ROADMAP.md`

**Target branch type**

- Type: `chore/*`
- Example: `chore/m8-packaging`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Configuration/schema checks pass.
- [ ] macOS and Windows build jobs or documented platform-specific runs complete.
- [ ] Generated packages contain the expected identity and no secrets.
- [ ] `git diff --check` passes and packaging documentation is updated in English.

### M8-13 — Complete release, privacy, legal, and support documentation

**Task objective**

Publish the operator and user documentation required for a local MVP release, including privacy, investment-research disclaimer, support, incident contact, update, backup/export, known limitations, and rollback ownership.

**Acceptance criteria**

- Named owners and contacts from M8-01 appear consistently.
- Documentation states local data ownership, no telemetry/cloud account, manual update behavior, and backup responsibility accurately.
- Disclaimer clearly states research-only behavior and no trade execution or personalized investment advice.
- All public documentation is English; translated application catalogs are data files reviewed under the i18n process.

**Involved files**

- `docs/PRIVACY.md`
- `docs/INVESTMENT_RESEARCH_DISCLAIMER.md`
- `docs/RELEASES.md`
- `docs/SECURITY.md`
- `README.md`
- `docs/M8_DECISION_RECORD.md`
- `docs/MILESTONE_ROADMAP.md`

**Target branch type**

- Type: `docs/*`
- Example: `docs/m8-release-operations`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Product, privacy/legal, security, and release owners approve their sections.
- [ ] Markdown formatting and local/external link checks pass.
- [ ] Terminology matches the approved terminology guide and product guardrails.
- [ ] `git diff --check` passes and all documentation is English.

### M8-14 — Execute and record the M8 release gate

**Task objective**

Run the complete M8 quality, accessibility, localization, security, migration, backup, update, and supported-platform package validation; fix only test/gate defects within scope and record evidence.

**Acceptance criteria**

- Critical MVP workflows pass in `en` and `zh-CN` on supported platforms.
- Backup export, manual update, install, first run, restart, offline, cancellation, and failure recovery are verified.
- Security, privacy, legal, and support approvals are recorded.
- M8 is marked complete only after all gates pass and evidence is linked.

**Involved files**

- existing frontend/Rust/E2E test suites
- release/build validation scripts or workflows
- `docs/MILESTONE_ROADMAP.md`
- `docs/NEXT_STEPS.md`
- `docs/i18n/IMPLEMENTATION_PLAN.md`
- `docs/RELEASES.md`

**Target branch type**

- Type: `test/*`
- Example: `test/m8-release-gate`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [ ] `cargo fmt --check`, strict Clippy, and `cargo test` pass.
- [ ] Critical E2E and supported-platform `pnpm tauri build` runs pass.
- [ ] Accessibility, catalog parity, link, secret-scan, and docs-only English checks pass.

---

## M9 — Option Module Integration

### M9-01 — Rebaseline Option scope and approve implementation decisions

**Task objective**

Reinspect current `dev`, audit historical Option candidate branches file by file, and approve the pricing-model, exercise-style, data-provider, migration, provenance, and Artifact decisions before code integration.

**Acceptance criteria**

- Every candidate path is classified as reuse, adapt, reject, or obsolete.
- ADRs define calculation units/tolerances, supported contracts, provider boundary, data freshness, and Artifact permissions.
- Product scope excludes trade execution, brokerage order routing, and autonomous recommendations.
- The Option roadmap and baseline matrix reflect current `dev` accurately.

**Involved files**

- `docs/option/README.md`
- `docs/option/ROADMAP.md`
- `docs/option/INTEGRATION_PLAN.md`
- `docs/option/IMPLEMENTATION_DETAILS.md`
- new Option ADRs under `docs/DECISIONS/`
- current Option code and candidate branches (read-only audit)

**Target branch type**

- Type: `docs/*`
- Example: `docs/option-rebaseline`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Product, architecture, Option-domain, and security reviewers approve the decisions.
- [ ] Markdown formatting and links pass.
- [ ] `git diff --check` passes.
- [ ] All documentation is English and no implementation completion is overstated.

### M9-02 — Repair Option schema application and repositories

**Task objective**

Add a new append-only Option reconciliation migration, register it in the custom runner, and prove repository correctness across fresh and historical databases.

**Acceptance criteria**

- Fresh, pre-Option, partial-Option, and repeat-run databases converge on one schema.
- Existing migrations are unchanged.
- Option repositories enforce workspace isolation, constraints, timestamps, and stable error mapping.
- Repository and migration tests cover CRUD, cascades, missing rows, and upgrade safety.

**Involved files**

- new migration under `apps/desktop/src-tauri/migrations/`
- `apps/desktop/src-tauri/src/database/migrations.rs`
- `apps/desktop/src-tauri/src/database/migrations_test.rs`
- `apps/desktop/src-tauri/src/database/repositories/{option_chain_repository,option_contract_repository,greeks_repository,option_strategy_repository,option_position_repository}.rs`
- related repository tests
- `docs/option/DATA_MODEL.md`
- `docs/option/ROADMAP.md`

**Target branch type**

- Type: `fix/*`
- Example: `fix/option-schema-runtime`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Fresh/historical/partial/repeat migration tests pass.
- [ ] Option repository and workspace-isolation tests pass.
- [ ] Rust formatting, strict Clippy, and tests pass.
- [ ] `git diff --check` passes and Option documentation is updated in English.

### M9-03 — Implement and validate the pure Option calculation core

**Task objective**

Add a pure `option-core` crate for approved pricing, Greeks, implied-volatility solving, and strategy payoff primitives with explicit units, typed errors, reference fixtures, and benchmarks.

**Acceptance criteria**

- Inputs reject non-finite/out-of-range values and define all units.
- Boundary cases and solver non-convergence return typed recoverable errors.
- Independent fixtures and applicable financial identities pass within approved tolerances.
- Benchmarks record hardware, dataset size, and observed results without unsupported claims.

**Involved files**

- `Cargo.toml`
- `Cargo.lock`
- `crates/option-core/Cargo.toml` (new)
- `crates/option-core/src/**` (new)
- calculation fixtures/benchmarks
- `docs/option/ARCHITECTURE.md`
- `docs/option/IMPLEMENTATION_DETAILS.md`
- `docs/option/ROADMAP.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/option-pricing-core`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Reference, property, boundary, and convergence tests pass.
- [ ] Criterion or approved benchmarks run and results are attached.
- [ ] Workspace Rust formatting, strict Clippy, and tests pass.
- [ ] `git diff --check` passes and calculation documentation is updated in English.

### M9-04 — Implement demo and validated file Option providers

**Task objective**

Implement the Rust provider abstraction plus deterministic demo and validated file providers, with provenance, freshness, path validation, size limits, and partial-data semantics. Keep live providers disabled.

**Acceptance criteria**

- Provider-neutral output includes source ID, quote/retrieval timestamps, capabilities, and missing-field semantics.
- Demo data is deterministic for fixed inputs.
- File import accepts only approved formats/sizes and rejects traversal, malformed, or non-finite data.
- No API key, live endpoint, or frontend filesystem access is added.

**Involved files**

- `apps/desktop/src-tauri/src/providers/market_data/**`
- provider input/output types in the appropriate Rust crate
- provider fixtures and tests
- `docs/option/ARCHITECTURE.md`
- `docs/option/API_SPEC.md`
- `docs/option/IMPLEMENTATION_DETAILS.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/option-data-providers`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Demo determinism and file validation tests pass.
- [ ] Path traversal, size, malformed, partial, and stale-data tests pass.
- [ ] Rust formatting, strict Clippy, and tests pass.
- [ ] `git diff --check` passes and provider documentation is updated in English.

### M9-05 — Deliver the Option-chain vertical slice

**Task objective**

Deliver validated symbol/workspace input through Rust service/provider/calculation/persistence, typed IPC and Zod validation, TanStack Query state, route/navigation, and an accessible Option-chain UI.

**Acceptance criteria**

- Demo and file chains load, calculate, persist, reopen, and show provenance end to end.
- Long work uses the background task lifecycle with cancellation, timeout, and typed events.
- UI covers initial, loading, success, empty, error, partial, and offline states in both locales.
- React does not calculate prices or call `invoke` outside `desktopApi`.

**Involved files**

- `apps/desktop/src-tauri/src/services/option_service.rs`
- `apps/desktop/src-tauri/src/commands/options.rs`
- `apps/desktop/src-tauri/src/app/state.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/lib/desktop-api/options.ts`
- `apps/desktop/src/types/option.ts`
- `apps/desktop/src/features/options/OptionChainViewer/**`
- `apps/desktop/src/pages/options/**`
- router/navigation/i18n catalogs and tests
- relevant Option documentation

**Target branch type**

- Type: `feature/*`
- Example: `feature/option-chain-slice`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Service, command, schema, component, cancellation, and persistence tests pass.
- [ ] Option-chain E2E flow passes for demo and file data.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and Option/i18n/API documentation is updated in English.

### M9-06 — Deliver the Option strategy and payoff Artifact slice

**Task objective**

Deliver validated strategy-leg construction, pricing/payoff analysis, persistence, reopen behavior, and a predefined controlled Artifact renderer.

**Acceptance criteria**

- At least one bounded-risk spread and one unbounded-risk strategy are calculated and explained correctly.
- Strategy assumptions, model version, source snapshots, break-even points, cost, risk, and aggregate Greeks persist.
- Artifact input is schema-validated JSON; no generated privileged HTML or new broad permission is used.
- User-facing output is explicitly analytical, not a recommendation.

**Involved files**

- `crates/option-core/src/strategy.rs`
- Option strategy service/command/repositories
- `apps/desktop/src/lib/desktop-api/options.ts`
- `apps/desktop/src/features/options/StrategyBuilder/**`
- Option Artifact renderer/registry files
- plugin manifest/schema files only if the approved architecture requires them
- related tests and Option documentation

**Target branch type**

- Type: `feature/*`
- Example: `feature/option-strategy-slice`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Strategy calculation, persistence, schema, renderer, and permission tests pass.
- [ ] Strategy build/save/reopen E2E flow passes in both locales.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and relevant documentation is updated in English.

### M9-07 — Deliver Option scenario and portfolio-risk integration

**Task objective**

Integrate Option positions, aggregate Greeks, and bounded price/volatility/time scenarios through portfolio services with visible provenance and missing/stale-data behavior.

**Acceptance criteria**

- Mixed equity/Option exposure is workspace-scoped, explainable, and reproducible.
- Missing/stale quotes are visible and never replaced with fabricated current values.
- Scenario inputs are bounded and cancellable when long-running.
- Results can link to research/thesis review only through user-confirmed existing service flows.

**Involved files**

- Option/portfolio services and commands
- `apps/desktop/src/features/options/PortfolioRisk/**`
- `apps/desktop/src/pages/options/PortfolioRiskPage.tsx`
- portfolio/Option desktop API and schemas
- controlled risk Artifact renderer if approved
- related tests and Option documentation

**Target branch type**

- Type: `feature/*`
- Example: `feature/option-risk-slice`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Aggregate Greeks, scenario bounds, stale/missing data, isolation, and cancellation tests pass.
- [ ] Mixed portfolio risk E2E flow passes in both locales.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and relevant documentation is updated in English.

### M9-08 — Execute and record the Option release gate

**Task objective**

Run the complete Option numerical, migration, provider, security, accessibility, i18n, performance, persistence, cancellation, restart, E2E, and packaged-build gate and record evidence.

**Acceptance criteria**

- Independent Option-domain review approves calculation fixtures and units.
- Fresh/historical database, demo/file chain, strategy, risk, offline, cancellation, restart, and packaged flows pass.
- No trade execution, live-provider credential, unsafe Artifact, or unrestricted permission path exists.
- M9 is marked complete only after all evidence is linked.

**Involved files**

- Option Rust/frontend/E2E/benchmark suites
- release/security validation scripts or workflows
- `docs/option/**`
- `docs/MILESTONE_ROADMAP.md`
- `docs/NEXT_STEPS.md`

**Target branch type**

- Type: `test/*`
- Example: `test/option-release-gate`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Frontend lint/typecheck/tests and critical E2E pass.
- [ ] Rust formatting, strict Clippy, tests, migration tests, and benchmarks pass.
- [ ] Security, accessibility, both-locale, secret-scan, and package smoke checks pass.
- [ ] `git diff --check`, Markdown formatting, and links pass.

---

## M10 — Goose Agent Integration

### M10-01 — Reverify upstream Goose and approve the architecture/threat model

**Task objective**

Reverify current AAIF Goose ownership, license, releases, supported platforms, CLI/API/library surfaces, recipes, permissions, MCP behavior, and security policy; then approve the AlphaForge integration ADR and threat model.

**Acceptance criteria**

- ADR compares pinned sidecar, direct library, and authenticated loopback API options and selects one with rationale/removal plan.
- Threat model covers prompt injection, malicious extensions/recipes, process escape, path traversal, secret leakage, unbounded output, unauthorized writes, and binary supply chain.
- Exact upstream version/source, integrity mechanism, license obligations, supported platforms, and credential owner are recorded.
- First use case is read-only and explicitly post-MVP.

**Involved files**

- `docs/goose/README.md`
- `docs/goose/INTEGRATION_ROADMAP.md`
- new Goose ADR/threat-model files under `docs/DECISIONS/` or `docs/goose/`
- `docs/MILESTONE_ROADMAP.md`
- upstream sources (read-only research)

**Target branch type**

- Type: `docs/*`
- Example: `docs/goose-architecture-decision`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Architecture, security, legal/license, and release reviewers approve the records.
- [ ] Upstream URLs and version/integrity evidence are verified.
- [ ] Markdown formatting, links, and `git diff --check` pass.
- [ ] All documentation is English.

### M10-02 — Execute the synthetic Goose spike and record findings

**Task objective**

Prove start/stream/cancel/timeout/exit/cleanup and structured recipe output using synthetic data in a disposable branch; merge only the English findings and approved contract, not unsafe or throwaway runtime code.

**Acceptance criteria**

- Spike uses the pinned runtime without a shell and enables no Developer, filesystem, arbitrary MCP, extension-manager, computer-control, or subagent capability.
- Structured output validates against a fixed schema and output/turn/time budgets are enforced.
- Cancellation, failure, and restart leave no orphan process or sensitive temporary data.
- Findings state what is reusable, rejected, and required for the production adapter.

**Involved files**

- disposable spike code/tests on the task branch only
- synthetic fixtures in a task-owned temporary location
- `docs/goose/SPIKE_REPORT.md` (new merged deliverable)
- `docs/goose/INTEGRATION_ROADMAP.md`
- `docs/MILESTONE_ROADMAP.md`

**Target branch type**

- Type: `docs/*` for the mergeable result; prototype work remains disposable
- Example: `docs/goose-spike-findings`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Start, streaming, structured output, cancellation, timeout, crash, and cleanup observations are recorded.
- [ ] Process/output/temp-directory checks use synthetic data only.
- [ ] PR diff contains no Goose binary, secret, generated cache, or throwaway prototype code.
- [ ] Markdown formatting, links, `git diff --check`, and English review pass.

### M10-03 — Implement the supervised Goose runtime adapter

**Task objective**

Implement the approved Rust-owned `GooseAdapter` lifecycle behind a trait with a fixed executable/resource path, bounded arguments/output, task events, timeout, cancellation, concurrency, and cleanup.

**Acceptance criteria**

- Rust starts the approved pinned runtime directly without a shell or user-supplied executable path.
- Adapter maps lifecycle events to existing AlphaForge task states and stable errors.
- Output bytes, turns, duration, concurrency, token/cost budgets, and stderr retention are bounded.
- Success, cancellation, timeout, nonzero exit, malformed output, missing binary, and restart cleanup are tested.

**Involved files**

- new Goose adapter module under `apps/desktop/src-tauri/src/agent/` or approved service location
- task runtime/state integration files
- configuration and stable error types
- synthetic adapter fixtures/tests
- `docs/goose/INTEGRATION_ROADMAP.md`
- relevant architecture/security docs

**Target branch type**

- Type: `feature/*`
- Example: `feature/goose-runtime-adapter`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Adapter contract and every lifecycle/failure test passes.
- [ ] No shell, arbitrary path, secret argument, or unbounded log/output path exists.
- [ ] Rust formatting, strict Clippy, and tests pass.
- [ ] `git diff --check` passes and adapter documentation is updated in English.

### M10-04 — Implement the read-only AlphaForge MCP bridge

**Task objective**

Expose the approved read-only workspace/research/thesis/Artifact tools through Rust services with task-bound scope, schemas, pagination, provenance, redaction, and size limits.

**Acceptance criteria**

- Only approved read-only tools are registered; unknown/write tools fail closed.
- Rust attaches the authorized workspace/task scope rather than trusting model-supplied IDs.
- No SQLite handle, SQL, arbitrary path/URL, Tauri proxy, or credential tool is exposed.
- Cross-workspace IDs, traversal strings, oversized requests, malformed parameters, and prompt-injected write attempts are rejected.

**Involved files**

- new Goose MCP bridge modules under the approved Rust service/agent location
- existing research, thesis, Artifact, and workspace services
- MCP schemas and policy tests
- `docs/goose/INTEGRATION_ROADMAP.md`
- `docs/AGENT_PROTOCOL.md`
- `docs/SECURITY.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/goose-mcp-bridge`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Tool schema, allowlist, scope, pagination, size, provenance, and redaction tests pass.
- [ ] Cross-workspace, traversal, unknown-tool, and write-attempt tests pass.
- [ ] Rust formatting, strict Clippy, and tests pass.
- [ ] `git diff --check` passes and protocol/security documentation is updated in English.

### M10-05 — Deliver opt-in Goose shadow-mode research

**Task objective**

Deliver an opt-in read-only research task from source/budget review through Goose execution, validated structured output, provenance persistence, progress/cancellation UI, and controlled Artifact review.

**Acceptance criteria**

- User reviews workspace/source scope, provider/model, and budgets before starting.
- Structured claims, evidence, contradictions, risks, unknowns, confidence, source IDs, provider/model, and recipe version validate before persistence.
- UI covers initial, running, waiting, completed, empty, partial, offline, failed, and cancelled states in both locales.
- No result automatically changes a note, thesis, evidence item, portfolio, or decision record.

**Involved files**

- Goose Rust service/commands and task integration
- Goose recipe/schema resources
- `apps/desktop/src/lib/desktop-api/**`
- Agent task UI/hooks and new shadow-mode components
- controlled Artifact renderer/registry
- persistence/migration files only if existing task storage is insufficient
- i18n catalogs/tests
- Goose/Agent/Artifact documentation

**Target branch type**

- Type: `feature/*`
- Example: `feature/goose-shadow-mode`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Service, command, schema, persistence, UI-state, cancellation, and provenance tests pass.
- [ ] Synthetic shadow-mode E2E flow passes in both locales.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and Goose/Agent/Artifact documentation is updated in English.

### M10-06 — Deliver human-approved structured proposals

**Task objective**

Allow Goose to propose research notes, evidence candidates, report outlines, or Artifact payloads while requiring field-level user review and Rust revalidation before existing services persist anything.

**Acceptance criteria**

- Proposal schemas exclude trades, target positions, autonomous portfolio changes, and generic write/delete tools.
- UI shows source links, changed fields, uncertainty, and accept/reject controls.
- Every accepted write is revalidated in Rust, uses the normal domain service, and records proposer/recipe/user timestamps and resulting entity IDs.
- Rejected proposals and partial validation failures do not mutate domain records.

**Involved files**

- Goose proposal schemas and service logic
- existing research/thesis/Artifact services and commands
- proposal preview/approval UI and desktop API
- append-only persistence changes only if required
- related tests and Goose/security documentation

**Target branch type**

- Type: `feature/*`
- Example: `feature/goose-proposals`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Proposal validation, preview, accept, reject, partial failure, and audit tests pass.
- [ ] Unauthorized write/trade proposal tests fail closed.
- [ ] Frontend and Rust standard checks plus proposal E2E pass.
- [ ] `git diff --check` passes and documentation is updated in English.

### M10-07 — Implement the approved Goose credential/provider policy

**Task objective**

Implement the credential model selected in M10-01, enforce provider/model allowlists and budgets, and prove that secrets never reach React, CLI arguments, recipes, logs, crash output, SQLite, or plaintext fallback storage.

**Acceptance criteria**

- Exactly one approved credential owner/path is implemented.
- File-based secret fallback is disabled unless separately approved by security.
- Provider/model/data-retention disclosures and budget limits are visible before execution.
- Missing, revoked, inaccessible, or malformed credentials fail safely with redacted stable errors.

**Involved files**

- `apps/desktop/src-tauri/src/security/credentials.rs`
- Goose adapter/provider configuration modules
- Settings service/commands/UI only for non-secret metadata
- redaction and credential tests
- `docs/goose/INTEGRATION_ROADMAP.md`
- `docs/SECURITY.md`
- `docs/PRIVACY.md`

**Target branch type**

- Type: `feature/*`
- Example: `feature/goose-credentials`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Credential success/failure/revocation/keyring-unavailable tests pass on supported platforms.
- [ ] Secret scanning and log/argument/recipe/SQLite redaction tests pass.
- [ ] Frontend and Rust standard checks pass.
- [ ] `git diff --check` passes and security/privacy documentation is updated in English.

### M10-08 — Package the pinned Goose runtime and diagnostics

**Task objective**

Package the exact approved Goose runtime for supported platforms with integrity verification, SBOM/license attribution, version diagnostics, fail-closed mismatch behavior, and a disable/rollback procedure.

**Acceptance criteria**

- Build inputs identify the exact upstream source/version and verify integrity reproducibly.
- No runtime is downloaded or upgraded automatically by the application.
- Missing/corrupt/wrong-version binaries fail closed and existing persisted results remain viewable.
- Diagnostics expose versions, policy profile, process state, and stable errors without secrets or user content.

**Involved files**

- `apps/desktop/src-tauri/tauri.conf.json`
- approved sidecar/resource packaging configuration and scripts
- release workflows and checksums/SBOM/attribution files
- Goose diagnostic service/UI if approved
- `docs/RELEASES.md`
- `docs/goose/README.md`
- `docs/goose/INTEGRATION_ROADMAP.md`

**Target branch type**

- Type: `chore/*`
- Example: `chore/goose-packaging`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Integrity, missing/corrupt/mismatch, disable, and diagnostic tests pass.
- [ ] macOS and Windows packaged smoke tests pass.
- [ ] SBOM, license, attribution, and secret scans pass.
- [ ] `git diff --check` passes and release/Goose documentation is updated in English.

### M10-09 — Execute and record the Goose release gate

**Task objective**

Run the complete Goose functional, safety, reliability, research-quality, credential, binary-integrity, workspace-isolation, prompt-injection, localization, packaging, rollback, and support gate and record evidence.

**Acceptance criteria**

- Developer-fixture, internal shadow, opt-in real-workspace, and human-approved proposal stages have explicit evidence.
- Tool allowlist, workspace scope, task budgets, cancellation, crash/restart, output validation, provenance, and confirmation boundaries pass.
- Kill switch prevents new Goose runs without hiding existing validated results.
- M10 is marked complete only after security, release, and product approvals are linked.

**Involved files**

- Goose Rust/frontend/E2E/security/package test suites
- release validation workflows and fixtures
- `docs/goose/**`
- `docs/MILESTONE_ROADMAP.md`
- `docs/NEXT_STEPS.md`
- `docs/SECURITY.md`
- `docs/RELEASES.md`

**Target branch type**

- Type: `test/*`
- Example: `test/goose-release-gate`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [ ] Frontend lint/typecheck/tests and Goose E2E pass in both locales.
- [ ] Rust formatting, strict Clippy, tests, policy/security tests, and lifecycle stress tests pass.
- [ ] macOS/Windows package, integrity, SBOM, secret-scan, rollback, and kill-switch checks pass.
- [ ] `git diff --check`, Markdown formatting, links, and English documentation review pass.

---

## Coordinator completion record

After each merge, the coordinator records the PR URL, merge commit, verification evidence, and next-task authorization in the milestone issue or tracking system. Do not edit completed task criteria retroactively; document changed requirements in a new English decision record and update the remaining queue through a separate `docs/*` PR.
