# AlphaForge Stabilization Roadmap

**Status:** Active
**Established:** 2026-08-12
**Last reviewed:** 2026-08-15
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
| S0 — Baseline truth and build recovery | ✅ Complete (PR #151) | Repository checks run cleanly, React Fast Refresh clean, status documents aligned. |
| S1 — Core Agent loop recovery | ✅ Complete (PR #152) | End-to-end task lifecycle operational: credential → create → queue → run → real-time progress → failure context / structured output. |
| S2 — IPC contract normalization | ✅ Complete (PR #153-#156) | Frontend and Rust exchange validated, versioned DTOs without naming ambiguity (176/176 commands checked). |
| S3 — Artifact and plugin vertical slice | ✅ Complete | Structured output opens safely in an isolated Artifact window and internal plugins are usable with least privilege. |
| S4 — Research and portfolio workflow closure | ✅ Complete | Navigation, provenance, portfolio imports, allocations, risks, themes, and thesis alignment workflows are complete and state-aware. |
| S5 — Option module re-acceptance | ✅ Complete | End-to-end chain acquisition, contract inspection, strategy persistence, and numerical pricing workflows verified. |
| S6 — Release-readiness re-acceptance | Active | Verification matrix, E2E flows, packaging, security, and English docs audited. |
| M10 — Goose integration | Planned after S6 | Opt-in, read-only, pinned, scoped Goose analysis passes its own entry gate. |

## S0 — Baseline truth and build recovery

**Acceptance reference:** [M8 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-9)
**Status:** ✅ Complete (PR #151)

### Scope & Completion Evidence

- Orphan `database::timeout` declaration removed (PR #78).
- The pnpm 9.0.0 and `pnpm-lock.yaml` baseline merged (PR #79).
- React Fast Refresh violations and mixed component/hook export architectures cleanly resolved across 28 files (PR #151).
- CI quality gates, TypeScript checks (0 errors), ESLint (0 errors), Vitest tests (448 tests passing), Rust formatting, Clippy, and Rust tests (314+ passing) are fully operational.

### Acceptance criteria

- [x] Rust module resolution succeeds.
- [x] `pnpm install --frozen-lockfile` is deterministic.
- [x] CI runs lint, typecheck, frontend tests, Rust format, Clippy, and Rust tests.
- [x] No document claims M8/M9 acceptance without retained evidence.
- [x] No frontend invocation references an unregistered Tauri command (verified via `scripts/check-ipc-registration.mjs`).

## S1 — Core Agent loop recovery

**Acceptance references:** [M2 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-3) and [M8 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-9)
**Status:** ✅ Complete (PR #152)

### Scope & Completion Evidence

- **Startup Race Condition Fix**: `lib.rs` emits `app:ready` / `app:init-failed` upon async `AppState` initialization; `useAppReady` hook gates frontend commands.
- **Real-Time Event Streaming**: `useTaskEventStream` subscribes to `task:progress`, `task:completed`, `task:failed`, `task:cancelled` Tauri events, invalidating TanStack Query cache and maintaining a bounded 20-message real-time log.
- **Failure Context Surfacing**: `AgentPanel` displays the exact failure reason payload from `TaskFailed` events (e.g. missing API key, provider error) with full EN/ZH-CN i18n support.
- **Structured Research Output**: `ResearchCompletion` Zod schema and `ResearchResultCard` component render summary, key claims, evidence, risks, and colored confidence score badges.
- **Credential Flow**: Pinned OpenAI API key in OS keychain (`openai.api_key`), plaintext secrets never returned to React.

### Acceptance criteria

```text
Configure credential
-> Create task
-> Queue task
-> Start background execution
-> Display real-time progress
-> Cancel or complete
-> Persist structured result & render ResearchResultCard
```

- [x] Secrets never return to React.
- [x] Every state transition has service and UI regression tests.
- [x] Initialization readiness (`app:ready`) prevents commands from racing `AppState` setup.

## S2 — IPC contract normalization

**Acceptance references:** [M8 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-9) and [M9 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-10)
**Status:** ✅ Complete (PR #153, #154, #155, #156)

### Scope

- Define command-boundary DTO naming and optional-value rules.
- Use `camelCase` serialized IPC DTOs while preserving Rust domain and database conventions (`snake_case`).
- Add Zod parsing for untrusted command responses across all desktop API wrappers.
- Unified IPC registration checker script (`scripts/check-ipc-registration.mjs`) verifies 1:1 parity between frontend wrappers and `lib.rs` handlers.

### Phase Status

- **Foundation & Core (Phase 2.1 Complete - PR #153)**:
  - `scripts/check-ipc-registration.mjs` scans all 176 commands and confirms 100% registration parity.
  - `WorkspaceDto` with `camelCase` (`createdAt`, `updatedAt`) + strict `WorkspaceSchema` Zod validation in `desktop-api/workspace.ts`.
  - `SettingItemDto` (`key`, `value`) + strict `AppInfoSchema` and `SettingItemSchema` in `desktop-api/settings.ts`.
  - `desktop-api/credentials.ts` strict Zod validation + malformed response rejection tests.
- **Agent, Artifacts & Plugins (Phase 2.2 Complete - PR #154)**:
  - `AgentTaskDto` and `AgentTaskEventDto` with `camelCase` serialization (`workspaceId`, `createdAt`, `updatedAt`, `taskId`, `eventType`) + roundtrip tests in `commands/agent.rs`.
  - `desktop-api/agent.ts` with strict `AgentTaskSchema`, `AgentTaskEventSchema`, `TaskStatusSchema`, `TaskEventTypeSchema` + runtime `.parse()`.
  - `ArtifactDto` with `camelCase` serialization (`workspaceId`, `taskId`, `artifactType`, `createdAt`, `updatedAt`) + roundtrip tests in `commands/artifacts.rs`.
  - `desktop-api/artifacts.ts` with strict `ArtifactSchema`, `ArtifactStatusSchema` + runtime `.parse()`.
  - `commands/plugins.rs` updated to return `ArtifactDto`; `desktop-api/plugins.ts` aligned with `ArtifactSchema`.
  - All UI consumers (`AgentTaskList`, `AgentPanel`, `ArtifactViewer`, `ArtifactWindowPage`, `ArtifactsPage`, `useGlobalSearch`, `useDashboardData`) updated to camelCase properties.
- **Research, Thesis & KnowledgeGraph (Phase 2.3 Complete - PR #155)**:
  - `commands/research.rs` with `ResearchProjectDto`, `ResearchDocumentDto`, `ResearchSourceDto`, `ResearchNoteDto`, `ResearchReportDto`, `ResearchSearchMatchDto` (`camelCase`) + Rust roundtrip tests.
  - `commands/thesis.rs` with `InvestmentThesisDto`, `ThesisEvidenceDto`, `ThesisConfidenceSnapshotDto` (`camelCase`) + Rust roundtrip tests.
  - `commands/knowledge_graph.rs` with `KnowledgeEntityDto`, `KnowledgeRelationshipDto`, `ThesisEntityLinkDto` (`camelCase`) + Rust roundtrip tests.
  - `desktop-api/research.ts` with strict `ResearchProjectSchema`, `ResearchDocumentSchema`, `ResearchSourceSchema`, `ResearchNoteSchema`, `ResearchReportSchema`, `ResearchSearchMatchSchema` + runtime `.parse()`.
  - `desktop-api/thesis.ts` with strict `InvestmentThesisSchema`, `ThesisEvidenceSchema`, `ThesisConfidenceSnapshotSchema` + runtime `.parse()`.
  - `desktop-api/knowledge-graph.ts` with strict `KnowledgeEntitySchema`, `KnowledgeRelationshipSchema`, `ThesisEntityLinkSchema` + runtime `.parse()`.
  - All UI consumers (`ResearchDocumentsSection`, `ThesisDetail`, `KnowledgeGraphPanel`, `KnowledgePage`, `useGlobalSearch`, `useDashboardData`) updated to camelCase properties.
- **Portfolio & Financial (Phase 2.4 Complete - PR #156)**:
  - `commands/portfolio.rs` with `PortfolioAccountDto`, `PositionDto`, `PortfolioTransactionDto`, `PortfolioAllocationDto`, `ConcentrationRiskDto`, `ThemeExposureDto`, `ThesisAlignmentDto`, `PortfolioReviewDto` (`camelCase`) + Rust roundtrip tests.
  - `desktop-api/portfolio.ts` with strict `PortfolioAccountSchema`, `PositionSchema`, `TransactionTypeSchema`, `PortfolioTransactionSchema`, `PortfolioAllocationSchema`, `ConcentrationSeveritySchema`, `ConcentrationRiskSchema`, `ThemeExposureSchema`, `ThesisAlignmentSchema`, `PortfolioReviewSchema` + runtime `.parse()`.
  - `desktop-api/financial.ts` with strict Zod validation across all 53 Wealthfolio service and CRUD commands + rejection tests.
  - All UI consumers and tests aligned with validated schemas.

### Acceptance criteria

- [x] Automated static command registration parity checker (`scripts/check-ipc-registration.mjs`).
- [x] TypeScript and Rust share checked fixtures for every repaired command family.
- [x] No UI relies on TypeScript-only assertions for runtime response shape.
- [x] Error responses preserve stable codes without leaking raw internal details.

## S3 — Artifact and plugin vertical slice

**Acceptance references:** [M3 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-4) and [M8 acceptance criteria](MILESTONE_ROADMAP.md#acceptance-criteria-9)
**Status:** ✅ Complete

### Scope & Architecture

- **Dedicated Isolated Artifact Window Route**: `/artifact/:artifactId/:artifactType` strictly validates UUID and safe alphanumeric route segments before initiating IPC.
- **Narrow IPC Capability Protocol**: `capabilities/artifact-window.json` restricts `artifact-*` windows to `artifact-read` (`get_artifact` only) along with window closure and event listening (`artifact:update`, `artifact:theme`). Privileged main commands (credentials, settings, backups, database, filesystem, execution) are forbidden.
- **Plugin Registry & Lifecycle Controls**: Internal plugins (`company-comparison`, `valuation-model`, `industry-map`, `portfolio-risk`, `research-timeline`, `earnings-analyzer`, `macro-dashboard`) are validated; disabled plugins are blocked (`PermissionDenied`) from producing Artifacts.
- **Predefined Safe React Renderers**: Artifacts render through validated components in `artifactRegistry` (`ComparisonTableRenderer`, etc.) without evaluating untrusted HTML or dynamic script payloads.
- **Synchronized Window Lifecycle**: Native window destruction and command-based closure stay in sync with in-memory tracking in `ArtifactManager`.

### Acceptance criteria

```text
Validated JSON
-> Persist Artifact
-> Open isolated Artifact window
-> Render predefined component
-> Receive bounded update
-> Close window
```

- [x] Artifact windows cannot invoke main-window credential, settings, backup, or destructive commands (`capabilities/artifact-window.json` least privilege).
- [x] Disabled plugins cannot create Artifacts (enforced by `PluginService::prepare_artifact`).
- [x] No plugin source or Agent-generated HTML is evaluated (predefined React component registry).
- [x] Artifact route parsing validates UUID and type strings before IPC.
- [x] Isolated window update events (`artifact:update`, `artifact:theme`) and close events are fully tested.

## S4 — Research and portfolio workflow closure

**Status:** ✅ Complete

### Scope

- Make Research URL context authoritative for selected workspace and project with deep-link restoration and stale ID cleanup.
- Complete async initial/loading/success/empty/error/partial/offline states across Research and Portfolio dashboards.
- Expose and test portfolio accounts, positions, CSV transactions import, allocation, concentration risks, theme exposure, thesis alignment, and portfolio review generation.
- Confirm provenance links remain visible from research documents to thesis evidence and portfolio review.

### Acceptance criteria

- [x] Shared Research links restore valid workspace/project context (`ResearchPage.test.tsx`).
- [x] Invalid or deleted IDs recover without a blank page (replace navigation and graceful fallback).
- [x] Main portfolio import, allocation, concentration, theme, and thesis-alignment flows are tested (`usePortfolio.test.tsx`, `useFinancialData.test.tsx`, `dialogs.test.tsx`).
- [x] Provenance links connect research evidence, knowledge entities, and thesis validation outcomes (`ThesisDetail.tsx`).

## S5 — Option module re-acceptance

**Status:** ✅ Complete

### Scope

- Option IPC DTO repair and canonical `0014_options_support` persistence baseline are verified across all repository CRUD and cross-workspace isolation tests.
- Connected demo chain acquisition and listing to contract detail and selection.
- Connected persisted strategy workflow with leg persistence, validation, and cascading deletes.
- Displayed assumptions, timestamp, source, model, and pricing/Greeks calculations.
- Strictly preserved the no-trading boundary (pure research, valuation, and scenario analysis).

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

- [x] Numerical results pass independent fixtures (`option-core` 47 Black-Scholes, Greeks, Payoff, and Newton-Raphson IV tests).
- [x] No `create_option_chain` call exists; `fetch_option_chain` is the registered acquisition and persistence path (`scripts/check-option-ipc-registration.mjs`).
- [x] Chain acquisition, contract inspection, custom strategy building, persistence, and deletion flows pass (`OptionsPage.test.tsx`).
- [x] Workspace isolation, cascading deletes, and migration tests pass (`strategy_service_test.rs`, `option_service_test.rs`, repositories test suite).

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
