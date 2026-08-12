# AlphaForge Stabilization Roadmap

**Status:** Active
**Established:** 2026-08-12
**Purpose:** Restore a truthful, tested, end-to-end MVP baseline before feature expansion.

## Why stabilization is required

The 2026-08-12 integration audit found that substantial frontend and backend code exists, but several core user journeys are blocked by incompatible IPC contracts, unreachable UI, incomplete runtime wiring, and contradictory status documentation.

The program must not continue to describe M8 or M9 as accepted until the stabilization gates below are satisfied. M10 Goose remains planned.

See [Frontend-Backend Integration and Functional Completeness Audit](reviews/INTEGRATION_GAP_AUDIT_2026-08-12.md) for evidence and the detailed rectification sequence.

## Delivery rules

- Branch every change from `dev`.
- Open pull requests back to `dev`; never develop directly on `dev` or `main`.
- Keep each pull request limited to one defect or one vertical slice.
- Use Conventional Commits when the user explicitly requests commits.
- Update English documentation and tests in the same pull request as behavior.
- Do not merge based on mocked success alone; retain real command output and smoke evidence.

## Milestone overview

| Milestone | Status | Outcome |
|---|---|---|
| S0 — Baseline truth and build recovery | Active | Repository checks can run and status documents reflect reality. |
| S1 — Core Agent loop recovery | Blocked by S0 | A configured user can create, queue, run, cancel, and inspect a research task. |
| S2 — IPC contract normalization | Blocked by S0 | Frontend and Rust exchange validated, versioned DTOs without naming ambiguity. |
| S3 — Artifact and plugin vertical slice | Blocked by S1-S2 | Structured output opens safely in an Artifact window and internal plugins are usable. |
| S4 — Research and portfolio workflow closure | Blocked by S2 | Navigation and persistence workflows are complete and state-aware. |
| S5 — Option module re-acceptance | Blocked by S2 | One evidence-grounded Option workflow works end to end. |
| S6 — Release-readiness re-acceptance | Blocked by S1-S5 | Required checks, E2E flows, packages, security, and docs are accepted. |
| M10 — Goose integration | Planned after S6 | Opt-in, read-only, pinned, scoped Goose analysis passes its own entry gate. |

## S0 — Baseline truth and build recovery

### Scope

- Resolve the missing `database::timeout` module.
- Choose one JavaScript package manager and one authoritative lockfile.
- Add standard CI quality gates in addition to CodeQL.
- Align README, architecture, roadmap, and milestone status.
- Record a command-to-wrapper static contract check.

### Acceptance criteria

- Rust module resolution succeeds.
- `pnpm install --frozen-lockfile` is deterministic.
- CI runs lint, typecheck, frontend tests, Rust format, Clippy, and Rust tests.
- No document claims M8/M9 acceptance without retained evidence.
- No frontend invocation references an unregistered Tauri command.

## S1 — Core Agent loop recovery

### Scope

- Standardize `openai.api_key` across Settings, Agent status, and provider lookup.
- Correct the `created -> queued -> running` transition in the UI.
- Surface progress events, failure context, cancellation, and completion.
- Verify completed structured output is persisted and discoverable.

### Acceptance criteria

```text
Configure credential
-> Create task
-> Queue task
-> Start background execution
-> Display progress
-> Cancel or complete
-> Persist structured result
```

- Secrets never return to React.
- Every state transition has service and UI regression tests.
- Initialization readiness prevents commands from racing `AppState` setup.

## S2 — IPC contract normalization

### Scope

- Define command-boundary DTO naming and optional-value rules.
- Use camelCase serialized IPC DTOs while preserving Rust domain and database conventions.
- Add Zod parsing for untrusted command responses where practical.
- Repair Option and System DTO mismatches first.
- Add automated command registration and serialization fixture tests.

### Acceptance criteria

- TypeScript and Rust share checked fixtures for every repaired command family.
- No UI relies on TypeScript-only assertions for runtime response shape.
- Error responses preserve stable codes without leaking raw internal details.

## S3 — Artifact and plugin vertical slice

### Scope

- Add a dedicated Artifact-window route.
- Enforce a narrow Artifact-window IPC protocol.
- Render one completed Agent Artifact in a separate window.
- Add a minimal internal-plugin settings surface.
- Create one validated plugin payload and open its predefined renderer.

### Acceptance criteria

```text
Validated JSON
-> Persist Artifact
-> Open isolated Artifact window
-> Render predefined component
-> Receive bounded update
-> Close window
```

- Artifact windows cannot invoke main-window credential, settings, backup, or destructive commands.
- Disabled plugins cannot create Artifacts.
- No plugin source or Agent-generated HTML is evaluated.

## S4 — Research and portfolio workflow closure

### Scope

- Make Research URL context authoritative for selected workspace and project.
- Complete async initial/loading/success/empty/error/partial/offline states.
- Expose or intentionally retire unused workspace, settings, and portfolio APIs.
- Confirm provenance links remain visible from research to thesis and review.

### Acceptance criteria

- Shared Research links restore valid workspace/project context.
- Invalid or deleted IDs recover without a blank page.
- Main portfolio import, allocation, concentration, theme, and thesis-alignment flows are tested.

## S5 — Option module re-acceptance

### Scope

- Repair Option IPC DTOs.
- Connect chain acquisition/list to contract detail.
- Connect a persisted strategy workflow rather than a calculation-only mockup.
- Display assumptions, timestamp, source, model, and uncertainty.
- Preserve the no-trading boundary.

### Acceptance criteria

```text
Select workspace
-> Acquire demo/file chain
-> Persist chain and contracts
-> Inspect contract
-> Build and persist strategy
-> Calculate risk
-> Render controlled Artifact
```

- Numerical results pass independent fixtures.
- No `create_option_chain` call exists unless a real registered command and workflow require it.
- Workspace isolation and migration tests pass.

## S6 — Release-readiness re-acceptance

### Scope

- Run the full verification matrix.
- Add critical-flow E2E coverage.
- Perform Windows and macOS packaged smoke tests.
- Close high-severity dependency advisories.
- Complete security, privacy, legal, support, recovery, and rollback gates.
- Perform a final English documentation audit.

### Acceptance criteria

- Zero open P0 defects.
- No known critical security defects and no unjustified high-severity dependency advisories.
- All standard checks pass with retained output.
- Core workflows pass packaged smoke tests on supported platforms.
- README, Architecture, Product, Roadmap, and milestone documents agree.
- A release owner explicitly accepts the local MVP.

## M10 entry gate after stabilization

Goose remains planned until all of the following are true:

- S6 is accepted.
- The upstream version, license, CLI/API, packaging, and checksum are reverified.
- A threat model and ADR approve the topology.
- The Rust service initializes only through explicit opt-in configuration.
- Placeholder MCP methods are replaced with bounded, workspace-scoped reads.
- Run IDs, cancellation, timeout, concurrency, cost, token, and output budgets are tested.
- No direct SQLite, unrestricted filesystem, shell, secret, trade, or privileged Tauri capability is exposed.

## Required verification matrix

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
pnpm test:e2e
pnpm tauri build
```

Every milestone acceptance record must list the exact commands run, the environment, the result, and any remaining risk.
