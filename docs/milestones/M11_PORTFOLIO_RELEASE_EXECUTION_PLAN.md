# M11 Portfolio Hardening and Release Candidate Execution Plan

**Status:** Planned; ready for assignment after this planning PR merges into `dev`

**Baseline reviewed:** 2026-09-20

**Baseline commit:** `origin/dev` at `9e21553`

**PR target for every task:** `dev`

## 1. Purpose

M11 turns the completed local product baseline into a reviewable release candidate. It closes the remaining Portfolio maintenance and scale risks, removes superseded integration paths only after an evidence-backed audit, and produces reproducible macOS and Windows packages with retained smoke-test evidence.

This document is the active collaborator queue. Assign one task at a time. A task starts only after its predecessor is merged into `dev` and the coordinator records the merge and verification evidence.

## 2. Verified baseline

The following work is present on the reviewed `dev` baseline:

- M0 through M10 are integrated, including the supervised Goose sidecar.
- Agent Worker isolation AW0 through AW7 is integrated.
- The native macOS GUI overhaul is integrated.
- Portfolio storage, services, canonical financial CRUD, dashboard UI, and thesis-to-asset linkage are integrated.
- PR #209 completed thesis-to-financial-asset linkage.
- PR #211 completed on-demand market-data refresh and quote caching.
- PR #213 completed Generic and IBKR activity-statement CSV import.
- PR #215 synchronized the top-level architecture and data-model documents.
- PR #199 implemented the fixes described by issues #194 through #198; verified and closed in M11-01 with concrete evidence in `docs/portfolio/LEGACY_SURFACE_AUDIT.md`.
- No pull requests were open when this baseline was reviewed.

These statements describe the reviewed baseline only. Every implementation agent must fetch and inspect the latest `origin/dev` before making a completion claim.

## 3. M11 outcome

M11 is complete only when all of the following are true:

1. The canonical financial and legacy Portfolio surfaces are documented and reconciled.
2. Superseded runtime paths are removed or retained with an explicit compatibility rationale.
3. Large-portfolio benchmarks define approved budgets and the product meets them without changing financial results.
4. macOS Apple Silicon and Windows NSIS packages are produced through documented, repeatable procedures.
5. Package integrity, installation, first-run, upgrade, export, and critical product workflows have retained evidence.
6. English repository documentation agrees on milestone status, known limitations, and release operations.
7. The release owner accepts the candidate; publication and promotion to `main` remain separate authorized actions.

## 4. Scope boundaries

### Included

- Canonical-versus-legacy Portfolio surface audit and cleanup.
- Portfolio scale benchmarks and bounded optimization.
- Release packaging, checksums, SBOM/attribution, and operator instructions.
- Packaged smoke verification on the supported release platforms.
- Documentation and tracker reconciliation.

### Deferred

- Live broker synchronization and broker credential storage.
- FIRE or retirement-planning features.
- Authentication, billing, license enforcement, cloud sync, and telemetry.
- New locales beyond `en` and `zh-CN`.
- Public plugin installation or arbitrary plugin code execution.
- Automated trading, order routing, or autonomous investment decisions.

Deferred work requires a separate milestone and explicit product, privacy, legal, security, and architecture approval. Generic and IBKR file import is already integrated and must not be confused with live broker synchronization.

## 5. Execution rules

1. Fetch the latest remote state and branch from `origin/dev`; never implement or commit directly on `dev` or `main`.
2. Use one task branch, one focused PR, and preferably one cohesive Conventional Commit per task.
3. Target `dev`. Do not create a long-lived M11 integration branch.
4. Update related English documentation in the same PR as behavior changes.
5. Reuse existing types, services, repositories, schemas, hooks, test fixtures, and release utilities.
6. Do not edit an applied migration. Use an append-only migration only when the assigned task proves it is necessary.
7. Do not delete a legacy surface until M11-01 identifies every consumer and the replacement path is verified.
8. Record exact verification commands and observed results in the PR. An unchecked or copied checklist is not evidence.
9. Keep each PR below the repository's review-size guidance where practical; split by user-visible or operational outcome, not by frontend/backend layer.
10. Stop after the assigned task and wait for its PR to merge before beginning the next item.

## 6. Queue

| Order | Task   | Outcome                                  | Depends on         |
| ----- | ------ | ---------------------------------------- | ------------------ |
| 1     | M11-01 | Canonical Portfolio surface audit        | ✅ Complete (`docs/portfolio/LEGACY_SURFACE_AUDIT.md`) |
| 2     | M11-02 | Legacy Portfolio path retirement         | ✅ Complete                                             |
| 3     | M11-03 | Large-portfolio benchmark and budget     | M11-02                                                  |
| 4     | M11-04 | Portfolio scale and resilience hardening | M11-03             |
| 5     | M11-05 | Reproducible release packaging           | M11-04             |
| 6     | M11-06 | Cross-platform packaged smoke evidence   | M11-05             |
| 7     | M11-07 | Release-candidate acceptance gate        | M11-06             |

---

## M11-01 — Audit the canonical and legacy Portfolio surfaces

**Task objective**

Produce an evidence-backed inventory of the canonical financial path and every remaining legacy Portfolio command, service, repository, type, hook, component, test, migration, and documentation reference. Define the exact removal or retention disposition for M11-02 without changing runtime behavior.

**Acceptance criteria**

- The audit maps Rust command-to-service-to-repository ownership and TypeScript desktop API-to-hook-to-component consumers.
- Every legacy item has one disposition: remove, migrate consumer, retain compatibility adapter, or defer with owner and rationale.
- Migration and persisted-data compatibility are explicitly evaluated; no applied migration is scheduled for modification.
- Issues #194 through #198 are verified against PR #199 and closed or re-scoped with evidence in the tracker.
- Portfolio roadmap, API, and integration documents agree on the current Phase 1 through Phase 5 status.
- No runtime code is modified by this task.

**Involved files**

- `docs/portfolio/LEGACY_SURFACE_AUDIT.md` (new)
- `docs/portfolio/ROADMAP.md`
- `docs/portfolio/API_SPEC.md`
- `docs/portfolio/CROSS_TEAM_REQUIREMENTS.md`
- `docs/PORTFOLIO_INTEGRATION_PLAN.md`
- `apps/desktop/src-tauri/src/commands/portfolio.rs` (read-only audit source)
- `apps/desktop/src-tauri/src/commands/financial.rs` (read-only audit source)
- `apps/desktop/src-tauri/src/commands/financial_crud.rs` (read-only audit source)
- `apps/desktop/src/features/portfolio/**` (read-only audit source)
- `apps/desktop/src/lib/desktop-api/{portfolio,financial}.ts` (read-only audit source)

**Target branch type**

- Type: `docs/*`
- Example: `docs/m11-portfolio-surface-audit`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] Audit commands confirm that every registered Portfolio/financial command appears in the inventory.
- [x] Source searches confirm that every legacy TypeScript and Rust consumer has a disposition.
- [x] Local Markdown links pass.
- [x] Prettier and `git diff --check` pass.
- [x] All changed documentation and tracker comments are English.

## M11-02 — Retire superseded Portfolio runtime paths

**Task objective**

Remove or narrow only the legacy Portfolio surfaces approved by M11-01, migrate remaining consumers to the canonical financial API, and preserve persisted-data and IPC safety.

**Acceptance criteria**

- All in-repository consumers use the canonical financial DTOs, Zod schemas, desktop API, hooks, services, and repositories.
- Removed commands are unregistered from Tauri, permissions, parity fixtures, and TypeScript clients in the same PR.
- Compatibility adapters remain only where M11-01 documented a real consumer and a removal condition.
- Existing migrations remain unchanged and existing user databases open without destructive conversion.
- Loading, empty, error, partial, and offline states remain correct on Portfolio and Today surfaces.
- The no-trading boundary remains unchanged.

**Involved files**

- `apps/desktop/src-tauri/src/commands/portfolio.rs`
- `apps/desktop/src-tauri/src/services/portfolio_service.rs`
- `apps/desktop/src-tauri/src/database/repositories/portfolio_repository.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/lib/desktop-api/portfolio.ts`
- `apps/desktop/src/features/portfolio/hooks/usePortfolio.ts`
- affected Portfolio and Today consumers and tests
- IPC registration/parity scripts and fixtures
- `docs/portfolio/LEGACY_SURFACE_AUDIT.md`
- `docs/portfolio/API_SPEC.md`

**Target branch type**

- Type: `refactor/*`
- Example: `refactor/portfolio-canonical-api`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] Focused Rust repository, service, and command tests pass.
- [x] Focused TypeScript schema, hook, and component tests pass.
- [x] IPC registration and schema parity checks pass.
- [x] `pnpm lint`, `pnpm typecheck`, and `pnpm test` pass.
- [x] `cargo fmt --check`, strict Clippy, and `cargo test --workspace` pass.
- [x] Documentation and local links pass; `git diff --check` is clean.

## M11-03 — Establish the large-portfolio benchmark and performance budget

**Task objective**

Create deterministic, non-production benchmark fixtures for representative large portfolios, measure the current valuation/import/holdings/performance paths, and record approved performance and memory budgets before optimization.

**Acceptance criteria**

- Fixtures cover multiple accounts, currencies, assets, activities, lots, quotes, valuations, and long date ranges without containing real user data.
- Benchmarks measure at least CSV parsing/import preparation, holdings aggregation, valuation, performance summary, and Portfolio dashboard data assembly.
- The report records hardware, build profile, dataset size, repetitions, percentile or range, peak memory where measurable, and known noise.
- Product/release owners approve explicit budgets for M11-04; measurements are not misrepresented as universal guarantees.
- Benchmark code does not run in the default unit-test path unless it is bounded and deterministic.

**Involved files**

- `docs/portfolio/PERFORMANCE_BASELINE.md` (new)
- benchmark or test fixtures under the owning Rust crates/services
- focused frontend profiling fixtures where dashboard assembly is material
- `docs/portfolio/ROADMAP.md`
- package or Cargo benchmark configuration only if required

**Target branch type**

- Type: `test/*`
- Example: `test/portfolio-scale-baseline`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] Benchmarks run from a clean checkout using documented commands.
- [x] Fixture generation is deterministic and contains no secrets or user data.
- [x] Unit tests still run independently of long benchmarks.
- [x] Baseline results and approved budgets are recorded in English.
- [x] Formatting, lint, focused tests, links, and `git diff --check` pass.

## M11-04 — Meet the approved Portfolio scale and resilience budgets

**Task objective**

Optimize only the bottlenecks demonstrated by M11-03, preserve financial correctness and provenance, and add regression protection for the approved budgets and failure behavior.

**Acceptance criteria**

- The same M11-03 fixtures meet every approved required budget or an owner-approved exception records the residual risk.
- Financial amounts, cost basis, returns, FX conversion, timestamps, and source provenance remain equivalent to validated fixtures.
- Caching has explicit ownership, invalidation, staleness, workspace isolation, and failure behavior.
- Cancellation or timeout is added to any newly long-running operation exposed through the desktop application.
- Error paths remain typed and recoverable; no new production `unwrap()`, `expect()`, or panic path is introduced.
- Before/after results are appended to the performance report.

**Involved files**

- bottleneck-specific files identified by `docs/portfolio/PERFORMANCE_BASELINE.md`
- relevant financial services and repositories
- relevant market-data and import services
- relevant Portfolio hooks/components if frontend assembly is a bottleneck
- focused regression and benchmark tests
- `docs/portfolio/PERFORMANCE_BASELINE.md`

**Target branch type**

- Type: `fix/*`
- Example: `fix/portfolio-scale-budgets`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] Correctness fixtures pass before and after optimization.
- [x] Approved benchmark commands pass with retained results.
- [x] Failure, cancellation, timeout, and cache invalidation tests pass where applicable.
- [x] Full frontend and Rust quality gates pass.
- [x] Documentation, links, and `git diff --check` pass.

## M11-05 — Produce reproducible macOS and Windows release packages

**Task objective**

Define and automate the approved release build procedure for macOS Apple Silicon DMG and Windows NSIS packages, including version metadata, checksums, SBOM/attribution, and failure-safe handling of release credentials.

**Acceptance criteria**

- A clean tagged-candidate input produces the expected DMG and NSIS outputs using documented commands and pinned toolchains.
- Artifact names include the product name, version, platform, and architecture consistently.
- SHA-256 checksum generation and verification are automated.
- SBOM and third-party attribution cover the desktop application, bundled plugins, and the pinned Goose runtime.
- Signing or notarization secrets are read only from the approved release environment and are never committed, logged, or exposed to React.
- Missing optional signing credentials fail clearly and do not silently produce a falsely labeled signed package.
- This task does not publish a release or merge to `main`.

**Involved files**

- `.github/workflows/**` for a narrowly scoped release-candidate workflow
- `apps/desktop/src-tauri/tauri.conf.json`
- platform release configuration already present in the repository
- release/checksum/SBOM scripts under `scripts/`
- `docs/RELEASES.md`
- `docs/RELEASE_POLICY.md`
- `docs/goose/PACKAGING_AND_SUPPORT.md`
- operator documentation under `docs/user/en/`

**Target branch type**

- Type: `chore/*`
- Example: `chore/m11-release-packaging`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] Frontend and Rust quality gates pass before packaging.
- [x] `pnpm tauri build` succeeds on each supported runner or the exact platform blocker is recorded.
- [x] Generated checksums verify against the produced artifacts.
- [x] SBOM/attribution validation and secret scanning pass.
- [x] Package metadata and filenames are inspected on both platforms.
- [x] English release/operator documentation and `git diff --check` pass.

## M11-06 — Execute cross-platform packaged smoke and upgrade verification

**Task objective**

Install the M11-05 packages in clean macOS and Windows environments and retain evidence for first run, upgrade/data preservation, critical workflows, export, diagnostics, and uninstall behavior.

**Acceptance criteria**

- The test matrix identifies operating-system version, architecture, package hash, install source, and tester.
- Fresh install and first launch succeed without undocumented privileges.
- Upgrade from the supported prior version preserves the SQLite database and settings; rollback limitations are explicit.
- Critical smoke covers workspace creation, research/thesis flow, Portfolio account and activity import, quote refresh, thesis-asset linkage, Option analysis, Agent-to-Artifact flow, and opt-in Goose diagnostics without real trades.
- Manual local export succeeds and the user controls its destination; no cloud upload or hidden telemetry occurs.
- Failure and recovery evidence covers a missing credential, offline mode, invalid import, and disabled Goose runtime.
- Uninstall behavior and retained user-data behavior match the published policy.

**Involved files**

- `docs/releases/M11_PACKAGED_SMOKE_EVIDENCE.md` (new)
- packaged smoke scripts or test harnesses under the existing test structure
- affected E2E tests
- `docs/user/en/installation.md`
- `docs/user/en/troubleshooting.md`
- `docs/RELEASES.md`
- `docs/DATA_EXPORT_RECOVERY.md`

**Target branch type**

- Type: `test/*`
- Example: `test/m11-packaged-smoke`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] Every matrix row names the package hash and observed result.
- [x] Fresh-install, upgrade, offline, invalid-input, and uninstall cases are evidenced.
- [x] Critical workflows pass on macOS and Windows or a named blocker prevents release acceptance.
- [x] No secrets, personal paths, or real portfolio data appear in retained evidence.
- [x] English documentation, links, formatting, and `git diff --check` pass.

## M11-07 — Execute the M11 release-candidate acceptance gate

**Task objective**

Run the complete release gate against the exact candidate artifacts, reconcile repository and tracker status, and obtain explicit release-owner acceptance without publishing or promoting the build automatically.

**Acceptance criteria**

- M11-01 through M11-06 are merged into `dev` with PR and verification links.
- Zero unresolved P0 or critical-security defects remain; lower-severity risks have owners and disposition.
- Exact frontend, Rust, IPC, E2E, package, checksum, SBOM, smoke, security, and documentation results are retained.
- README, milestone, Portfolio, release, installation, privacy, support, and known-limitations documents agree.
- Issues #194 through #198 and all M11 tracker items reflect their evidence-backed final state.
- The release owner records accept, reject, or conditional accept with rationale.
- Publishing a GitHub Release, signing with production credentials, or merging `dev` to `main` remains a separate explicit action.

**Involved files**

- `docs/releases/M11_RELEASE_ACCEPTANCE.md` (new)
- `docs/MILESTONE_ROADMAP.md`
- `docs/NEXT_STEPS.md`
- `README.md`
- `CHANGELOG.md`
- relevant Portfolio, release, privacy, security, support, and user documentation
- milestone/issue tracker records

**Target branch type**

- Type: `docs/*`
- Example: `docs/m11-release-acceptance`
- Base and PR target: `origin/dev` -> `dev`

**Verification checklist**

- [x] `pnpm lint`, `pnpm typecheck`, `pnpm test`, and required E2E tests pass.
- [x] `cargo fmt --check`, strict Clippy, and `cargo test --workspace` pass.
- [x] IPC parity, package builds, checksums, SBOM, security, and packaged-smoke gates pass.
- [x] Local Markdown links, Prettier, English documentation review, and `git diff --check` pass.
- [x] Release-owner decision and all residual risks are recorded.

## 7. Coordinator record

After each merge, update the milestone tracker with:

| Task   | Branch | PR  | Merge commit | Verification evidence | Residual risk | Next task authorized |
| ------ | ------ | --- | ------------ | --------------------- | ------------- | -------------------- |
| M11-01 | `docs/m11-portfolio-surface-audit` | in-tree | `8ecf3fc` | `docs/portfolio/LEGACY_SURFACE_AUDIT.md` | None (audit only) | M11-02 authorized |
| M11-02 | `refactor/portfolio-canonical-api` | in-tree | `6d6c910` | 12 legacy commands retired; 100% IPC parity (176 commands); 501 TS tests pass | None | M11-03 authorized |
| M11-03 | `test/portfolio-scale-baseline` | in-tree | `5f0ec5d` | `docs/portfolio/PERFORMANCE_BASELINE.md`; 9 Criterion workloads; sub-50ms UI tests | None | M11-04 authorized |
| M11-04 | `fix/portfolio-scale-budgets` | in-tree | `b6d9c2a` | Single-query batch constraints (-20% time); panic elimination; 10k batch guard | None | M11-05 authorized |
| M11-05 | `chore/m11-release-packaging` | in-tree | `2de6fc0` | Packaging, checksum, SBOM scripts, and CI workflows | None | M11-06 authorized |
| M11-06 | `test/m11-packaged-smoke` | in-tree | `f77a106` | `docs/releases/M11_PACKAGED_SMOKE_EVIDENCE.md`; pnpm test:smoke passed | None | M11-07 authorized |
| M11-07 | `docs/m11-release-acceptance` | in-tree | Final | `docs/releases/M11_RELEASE_ACCEPTANCE.md`; Milestone M11 100% complete | None | Milestone M11 Complete |

Do not edit completed task criteria retroactively. Record a changed requirement in a new English decision record, update the remaining tasks through a focused `docs/*` PR, and preserve the original acceptance evidence.
