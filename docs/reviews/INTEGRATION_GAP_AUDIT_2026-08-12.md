# Frontend-Backend Integration and Functional Completeness Audit

**Audit date:** 2026-08-12
**Audited branch baseline:** `dev`
**Audit work branch:** `codex/integration-gap-audit`
**Scope:** React routes and components, TypeScript desktop API, Tauri command registration, Rust services, domain serialization, SQLite integration, Artifact runtime, bundled plugins, tests, and current documentation.

## Executive conclusion

AlphaForge is not functionally complete and is not ready to be described as a release-ready MVP. The repository contains substantial implemented capability, but several critical user journeys are broken by cross-layer contract mismatches or are not reachable from the UI.

The highest-priority failures are:

1. The Agent create-to-run flow cannot start a newly created task.
2. The Settings page writes the OpenAI key under a different keychain name from the one used by the provider.
3. The Option module has incompatible request and response field naming across TypeScript and Rust, and one frontend command does not exist in the backend.
4. Artifact windows open a React route that is not registered.
5. The Rust module tree references a missing `database/timeout.rs` file.

M8 and M9 must therefore be reopened for stabilization. M10 remains planned because its frontend is unreachable, its service is disabled, and several bridge operations are placeholders.

**Remediation status (2026-08-13):** The dual-lockfile and npm-invoking root-script gap identified during this audit is addressed: root scripts use pnpm-native workspace commands, `pnpm-lock.yaml` is the sole tracked JavaScript lockfile, and the tracked `package-lock.json` is removed. The credential-contract repair replaces caller-controlled credential IPC with OpenAI-specific save, status, and delete operations; migrates the legacy `api_key` entry to `openai.api_key` in Rust; and routes Settings, Agent status, and provider lookup through that contract. Release acceptance remains subject to the verification commands listed below and the remaining stabilization work.

This audit was code-first. Compilation and build commands were deliberately deferred at the user's request. Findings are based on static contract tracing and direct source inspection; each rectification PR must run the prescribed verification commands before claiming completion.

## Status model

| Status | Meaning |
|---|---|
| Working | A reachable UI path maps to a registered backend command and a real service implementation with compatible data contracts. |
| Partial | Meaningful implementation exists, but the user journey is incomplete, unreachable, or insufficiently verified. |
| Broken | A confirmed contract or state-machine mismatch prevents the intended user journey. |
| Planned | Scaffolding or documentation exists, but the feature is not an accepted product capability. |

## Cross-layer capability matrix

| Capability | Frontend | Backend | Integration status | Evidence and gap |
|---|---|---|---|---|
| Workspace create/list/select | Reachable from Today and layout | Repository and service implemented | Working by inspection | Frontend and Rust both use the existing command boundary consistently. |
| Workspace rename/delete | Hooks and API wrappers exist | Commands and service implemented | Partial | No reachable management UI was found for rename or delete. |
| Agent task create/list | Reachable in right sidebar | Commands, service, repository implemented | Partial | Creation and listing exist, but execution is broken as described in P0-1. |
| Agent task queue/run | Start and Retry Start actions explicitly queue created tasks before starting and retry queued tasks without requeueing | Backend requires `created -> queued -> running` | Partial — remediation implemented, verification pending | `useRunAgentTask` preserves the queued state after backend start-admission failure and exposes a recoverable retry action; focused hook/component and command/service regression tests cover the transition, while full verification remains required. |
| OpenAI credential configuration | Reachable in Settings | OS keychain, legacy migration, and OpenAI provider implemented | Partial | The credential identifier mismatch is repaired with provider-specific IPC and regression tests; full stabilization acceptance remains pending. |
| Research projects/documents/sources/notes/reports | Reachable in Research | Commands, services, repositories implemented | Partial | Core CRUD is connected, but navigation query parameters are ignored and async/error states are inconsistent. |
| Research navigation context | Layout generates `?workspace=` and `?project=` links | Not applicable | Broken | `ResearchPage` uses local state and never reads those query parameters. |
| Thesis and knowledge graph | Reachable in Journal | Commands, services, repositories implemented | Partial | Main operations are connected; contract and workflow tests are incomplete. |
| Portfolio accounts/positions/analysis | Reachable in Portfolio | Commands, services, repositories implemented | Partial | Main panels are connected. Several backend capabilities are not clearly exposed, including direct theme linking from the current dashboard. |
| Option calculations | Reachable | Commands and pricing core implemented | Broken | Nested request objects use camelCase in TypeScript while Rust structs expect snake_case. |
| Option chains/contracts/strategies | Components and wrappers exist | CRUD commands and services exist | Broken | Request/response naming is incompatible; `create_option_chain` is not registered; chain selection is a console-only TODO; `OptionStrategyPanel` and `OptionContractTable` are not connected to the route. |
| Artifact list and in-page rendering | Reachable | Persistence and predefined renderers implemented | Partial | In-page rendering exists. Separate Artifact windows are broken as described in P0-4. |
| Artifact separate window | API hook and backend manager exist | Window manager opens `/artifact/:id/:type` | Broken | React router has no matching route. |
| Bundled plugin registry | API wrappers and renderer registry exist | Seven plugins sync, validate, and produce Artifacts | Partial | No user-facing plugin list, enable/disable control, or Artifact creation workflow is reachable. |
| Goose shadow analysis | Component, hook, and wrappers exist | Commands and service scaffolding exist | Planned | Component is unreachable; `AppState` sets the service to `None`; MCP functions return placeholders; cancellation parses an incompatible displayed run ID. |
| Backup and update check | Reachable in Settings | Implemented in Rust | Working by inspection | Requires packaged smoke verification before release acceptance. |
| Settings persistence | Locale uses persisted settings | Repository and commands implemented | Partial | Generic settings APIs exist, but most are not surfaced; API contracts are not schema-validated. |

## Confirmed findings

### P0-1: Agent tasks cannot move from creation to execution

**Frontend:** `apps/desktop/src/components/layout/RightSidebar/AgentPanel.tsx:231`
**Frontend hook:** `apps/desktop/src/features/agent/hooks/useAgentTasks.ts:71`
**Backend:** `apps/desktop/src-tauri/src/services/agent_service.rs:39` and `:70`

The UI shows Start when a task has `created` status and calls `start_agent_task`. The backend only allows `start_task` for a task already in `queued` status. A queue mutation exists but is not used by the panel.

**Impact:** Before remediation, the core `Create task -> Run task` product loop failed.

**Rectification:** Implemented the least disruptive explicit lifecycle contract: the UI queues a created task and starts only after the queued transition succeeds; queued tasks expose Retry Start, running tasks expose cancellation, and executor-admission failures are recoverable through queued-state refresh. Focused hook/component and Rust command/service tests cover call order, retry, failure messaging, cancellation, and requeue recovery. Full verification remains required before closure.

### P0-2: The configured API key is never read by the OpenAI provider

**Frontend:** `apps/desktop/src/pages/settings/SettingsPage.tsx:25` and `:81`
**Frontend status:** `apps/desktop/src/hooks/useAgentStatus.ts:47`
**Backend:** `apps/desktop/src-tauri/src/providers/ai/mod.rs:13`

The frontend reads and writes `api_key`. The provider reads `openai.api_key`.

**Impact:** Settings can report the Agent as configured while task execution fails with credentials unavailable.

**Rectification:** Define one shared credential identifier. Prefer `openai.api_key`, migrate or deliberately remove the legacy name, and test Settings status plus provider lookup without exposing the secret value.

**Remediation status (2026-08-13):** Implemented by the credential-contract repair. Rust owns the canonical and legacy identifiers, migrates only after a successful canonical write, and keeps plaintext out of React. Settings and Agent status use OpenAI-specific status/save calls, the provider shares the same migration path, and the editable Settings field no longer contains a reusable mask. Final closure depends on the recorded PR verification and merge.

### P0-3: The Option IPC contract is incompatible

**Frontend:** `apps/desktop/src/lib/desktop-api/options.ts:53`
**Frontend types:** `apps/desktop/src/types/option.ts:40`
**Backend:** `apps/desktop/src-tauri/src/commands/options.rs:10`
**Domain output:** `crates/domain/src/option.rs:118`

The frontend sends nested objects with fields such as `workspaceId`, `optionType`, and `underlyingPrice`. Rust input structs define `workspace_id`, `option_type`, and `underlying_price` without `#[serde(rename_all = "camelCase")]`. Rust domain outputs serialize fields such as `workspace_id` and `underlying_price`, while the UI reads `workspaceId` and `underlyingPrice`.

The frontend also invokes `create_option_chain`, but this command is not implemented or registered.

**Impact:** M9 Option calculations and CRUD paths cannot be treated as functional despite substantial code on both sides.

**Rectification:** Establish one versioned IPC DTO convention. Do not change database/domain naming merely to satisfy React. Add command-boundary DTOs with `#[serde(rename_all = "camelCase")]`, Zod response schemas, and serialization contract tests. Remove the unsupported wrapper or implement the command only if a real product workflow requires it.

**Remediation status (2026-08-13):** Implemented in `commands/options.rs`, `src/lib/desktop-api/options.ts`, and `src/types/option.ts`. Rust emits explicit camelCase response DTOs while domain/database models remain snake_case; nested requests use camelCase serde and scalar command arguments use Tauri's camelCase boundary. The unsupported `create_option_chain` wrapper was removed; `fetch_option_chain` remains the acquisition and persistence path. Focused Rust and Vitest fixtures cover serialization and malformed responses, and `scripts/check-option-ipc-registration.mjs` verifies wrapper/registration parity. Final closure depends on the recorded PR verification and merge.

### P0-4: Separate Artifact windows open an undefined route

**Backend:** `apps/desktop/src-tauri/src/artifacts/manager.rs:75`
**Frontend router:** `apps/desktop/src/app/router.tsx:11`

The window manager opens `/artifact/{artifact_id}/{artifact_type}`. The React router defines only the main application pages and has no Artifact route.

**Impact:** `start_viewing_artifact` can create a window that cannot render the requested Artifact view.

**Rectification:** Add a dedicated Artifact-window route and minimal provider tree, or route the new window to an existing renderer entrypoint. Verify the Artifact capability remains minimal and add an E2E test covering open, render, update, and close.

### P0-5: Rust declares a missing database module

**File:** `apps/desktop/src-tauri/src/database/mod.rs:4`

`pub mod timeout;` references a file that does not exist.

**Impact:** Rust source-tree tooling and compilation cannot be considered healthy.

**Rectification:** Determine from Git history whether the module was intentionally removed. Restore the required implementation or remove the orphan declaration in a narrowly scoped build-repair PR.

**Remediation status (2026-08-12):** Git history shows the declaration was accidentally added by `e0ca184`; no `database/timeout.rs` module, consumer, or historical implementation exists. This branch removes the orphan declaration. The finding is closed only after the required Rust verification passes.

### P1-1: Research navigation links do not select the requested context

**Navigation producers:** `apps/desktop/src/components/layout/MainContent/OperationBar.tsx:53`, `WorkspaceDropdown.tsx:31`
**Consumer:** `apps/desktop/src/pages/research/ResearchPage.tsx:14`

The layout navigates with `workspace` and `project` query parameters. `ResearchPage` never reads them.

**Impact:** Cross-page navigation appears to work but opens an unselected Research page.

**Rectification:** Make URL search parameters the initial and shareable selection source, validate referenced IDs, and keep local selection synchronized without loops.

### P1-2: Option UI components are incomplete or unreachable

**Page:** `apps/desktop/src/pages/options/OptionsPage.tsx:94`
**Components:** `OptionStrategyPanel.tsx`, `OptionContractTable.tsx`

The route renders a calculation-oriented `StrategyBuilder`, but not the CRUD-oriented `OptionStrategyPanel`. `OptionContractTable` is not opened when a chain is selected; the selection handler only logs to the console.

**Impact:** Backend strategy and contract CRUD functionality is not reflected in the product UI.

**Rectification:** After fixing the IPC contract, choose a single coherent Option workflow and connect chain selection to contract detail and strategy persistence. Remove or clearly mark duplicate experimental components.

### P1-3: Plugin capabilities are not exposed to users

**Frontend API:** `apps/desktop/src/lib/desktop-api/plugins.ts:53`
**Backend:** `apps/desktop/src-tauri/src/commands/plugins.rs:26`

The application can list plugins, enable or disable them, validate payloads, and create completed Artifacts. No reachable page or settings surface calls these APIs.

**Impact:** M7 exists as backend infrastructure and renderer code, not as a complete user workflow.

**Rectification:** Add a minimal internal-plugin settings surface and one controlled create-to-Artifact vertical slice. Do not add a public marketplace.

### P1-4: Goose is scaffolding, not an integrated feature

**Frontend:** `apps/desktop/src/features/goose/components/ShadowAnalysis.tsx`
**Backend state:** `apps/desktop/src-tauri/src/app/state.rs:172`
**Bridge:** `apps/desktop/src-tauri/src/goose/mcp.rs:242`
**Cancellation:** `apps/desktop/src-tauri/src/services/goose_service.rs:220`

The component is not mounted by any route or reachable component. `goose_service` is initialized to `None`. Several MCP tools return empty or placeholder data. Displayed run IDs use `goose-run-N`, while cancellation attempts to parse the entire string as an integer.

**Impact:** M10 cannot be marked complete.

**Rectification:** Keep M10 planned until its entry gate is reapproved. First complete a synthetic-data spike, initialize it only behind an explicit opt-in configuration, replace placeholders, and use one typed run-ID representation.

### P1-5: System information has a TypeScript response mismatch

**Frontend:** `apps/desktop/src/lib/desktop-api/system.ts:10`
**Backend:** `apps/desktop/src-tauri/src/services/system_service.rs:11`

TypeScript expects `os`, `arch`, and `version`. Rust returns `app_name`, `app_version`, `platform`, and `architecture` without a camelCase rename.

**Impact:** Any UI using `getSystemInfo` would receive an incompatible object. The wrapper is currently not used by a reachable page.

**Rectification:** Add an explicit camelCase IPC response DTO and a contract test before exposing the function.

**Remediation status (2026-08-13, pending merge):** The remediation branch adds the explicit `SystemInfo` camelCase serialization contract, updates the internal desktop wrapper to validate unknown responses with Zod, and adds focused Rust and Vitest contract tests. The wrapper remains unmounted; release acceptance remains pending merge and the required verification matrix.

### P1-6: Documentation contains contradictory completion claims

**Files:** `README.md`, `docs/MILESTONE_ROADMAP.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, and `docs/ROADMAP.md`

The documents variously describe M8-M10 as complete, planned, or in progress. Architecture still lists real commands as stubs and the database as containing only two tables. The README describes the application as both M1.5 and M10 complete.

**Impact:** Agents and maintainers cannot determine authoritative product status.

**Rectification:** Use the milestone roadmap as the status authority, mark M8/M9 as stabilization required and M10 as planned, and archive historical checklists as non-authoritative.

### P2-1: User-facing source still contains hard-coded Chinese strings

**File:** `apps/desktop/src/components/layout/tools-config.tsx:31`

Tool labels are hard-coded in Chinese rather than using the locale catalogs. Settings also contains Chinese fallback strings.

**Impact:** The source and product do not meet the requested English-only documentation/source-label policy, and English locale behavior can regress.

**Rectification:** Move all user-facing strings to the English catalog first, retain other locales only through translated catalog entries, and remove `as any` translation workarounds.

### P2-2: Backend-only APIs lack a clear product surface

Examples include generic settings enumeration, system directory discovery, workspace mutation hooks, Artifact lifecycle APIs, plugin management, and portions of Option CRUD.

**Impact:** Wrapper presence inflates apparent feature completeness and increases maintenance cost.

**Rectification:** For each API, either connect it to an approved workflow with tests or mark it internal and remove unused public frontend exports in a separate cleanup PR.

## Areas that appear aligned by static inspection

- Workspace creation, listing, selection, and SQLite persistence.
- Research CRUD command names and snake_case response models.
- Thesis and knowledge graph command names and primary UI hooks.
- Portfolio command names and primary dashboard hooks.
- Local backup export and manual release check wiring.
- Artifact list and predefined in-page rendering.
- OS-keychain storage implementation itself; the defect is the cross-layer credential name.
- Web import URL validation and PDF import UI-to-command connection.

These are not release acceptance claims until their tests and packaged smoke checks pass.

## Rectification principles

1. Branch from `dev`; never develop directly on `dev` or `main`.
2. One concern per pull request. Do not combine build repair, IPC normalization, UI wiring, and documentation cleanup.
3. Prefer command-boundary DTOs over changing domain or database naming conventions.
4. Add a regression test in the same PR as every confirmed defect fix.
5. Update English documentation in the same PR as behavior changes.
6. Do not mark a milestone complete based on file presence or mocked tests.
7. Do not add autonomous trading, brokerage execution, or unapproved external services.

## Proposed pull request sequence

| Order | Branch example | Scope | Target | Required proof |
|---|---|---|---|---|
| 1 | `fix/rust-module-tree` | Restore or remove `database::timeout` only | `dev` | Rust format, check, Clippy, tests |
| 2 | `fix/openai-credential-contract` | Standardize credential identifier and Agent status | `dev` | Keychain adapter tests and Settings/provider contract test |
| 3 | `fix/agent-task-start-flow` | Queue then start, correct UI states | `dev` | Service, hook, component, and command integration tests |
| 4 | `fix/option-ipc-contract` | Introduce compatible Option IPC DTOs and remove nonexistent call | `dev` | Serde fixtures, Zod schemas, TypeScript tests, Rust command tests |
| 5 | `fix/artifact-window-route` | Add isolated Artifact window route | `dev` | Router, permission, and E2E window lifecycle tests |
| 6 | `fix/research-route-context` | Consume and synchronize workspace/project parameters | `dev` | Router and Research page tests |
| 7 | `feat/option-vertical-slice` | Chain selection to contract detail and persisted strategy | `dev` | Full Option vertical-slice tests |
| 8 | `feat/internal-plugin-surface` | Minimal plugin settings and create-Artifact workflow | `dev` | Payload, disabled-state, Artifact-render tests |
| 9 | `chore/ci-quality-gates` | Enforce frontend and Rust checks | `dev` | Successful CI on Windows and supported release platforms |
| 10 | `docs/stabilization-acceptance` | Record final evidence and milestone decisions | `dev` | Links to merged PRs and retained verification output |

M10 Goose work should use separate post-stabilization PRs only after its documented entry gate is approved.

## Definition of stabilization complete

- All P0 findings are closed with regression tests.
- The Agent `Create -> Queue -> Run -> Progress -> Artifact -> Persist` flow passes E2E.
- The main Research, Thesis, Portfolio, Option, Artifact, Settings, and plugin workflows have explicit loading, success, empty, error, partial, and offline behavior where applicable.
- TypeScript/Rust IPC fixtures prove field-name and optional-value compatibility.
- No frontend invocation targets an unregistered command.
- No registered product command is called complete unless it has an approved UI or is explicitly documented as internal.
- Required checks and packaged smoke tests pass with recorded evidence.
- README, architecture, roadmap, user documentation, and milestone status agree.

## Verification deferred during this audit

At the user's request, this phase did not compile or build the project. The following must be run by the implementation PRs after the code-first fixes are made:

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
