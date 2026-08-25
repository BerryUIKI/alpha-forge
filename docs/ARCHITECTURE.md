# Architecture

## System Overview

```text
┌────────────────────────────────────────────┐
│                  Tauri 2                    │
│  ┌──────────┐           ┌───────────────┐  │
│  │  React   │◄─────────►│     Rust      │  │
│  │  (UI)    │   IPC     │  (Backend)    │  │
│  └──────────┘           └───────────────┘  │
│                              │              │
│                          ┌───┴───┐          │
│                          │ SQLite │          │
│                          └───────┘          │
└────────────────────────────────────────────┘
```

## Boundary Rules

### React + TypeScript (Presentation Layer)

**Owns:**

- Pages, components, interaction, frontend state
- Forms, charts, artifact rendering
- Loading, empty, partial, offline, and error states
- Client-side validation
- Calling the `desktopApi` layer

**Must NOT:**

- Access SQLite directly
- Read arbitrary system files
- Read plaintext API keys
- Invoke shell commands
- Bypass the unified IPC layer (`desktopApi`)

### Rust (Local Capability Layer)

**Owns:**

- Agent runtime (`src/agent/`)
- SQLite access (`src/database/`)
- Filesystem access
- Network requests (`src/providers/`)
- Credentials (`src/security/`)
- Background tasks
- Cancellation
- Document and PDF processing
- Plugin registration (`src/plugins/`)
- Artifact window creation (`src/artifacts/`)
- Permission enforcement
- Structured logging (`src/telemetry/`)

**Must NOT:**

- Build large HTML strings
- Own page layout
- Own React state
- Own pure presentation logic

### Tauri (Desktop Runtime)

**Owns:**

- Desktop windows (`src/windows/`)
- React–Rust communication (IPC)
- Permission boundaries (`capabilities/`)
- Operating system integration
- Temporary Artifact WebViews
- Menus and keyboard shortcuts
- Application lifecycle
- Update and release infrastructure

## Directory Structure (Actual)

```
alpha-forge/
├── apps/desktop/                 Tauri 2 desktop application
│   ├── src/                      React frontend
│   │   ├── app/                  App entry, router, providers
│   │   ├── pages/                React route pages
│   │   ├── components/           Layout, navigation, feedback, common
│   │   ├── features/             Feature modules
│   │   ├── lib/desktop-api/      Unified IPC layer
│   │   ├── hooks/                React hooks
│   │   ├── stores/               Frontend state stores
│   │   ├── styles/               Tailwind CSS 4 globals
│   │   └── types/                Shared TypeScript types
│   ├── src-tauri/                Rust backend
│   │   ├── src/
│   │   │   ├── app/              AppState, bootstrap
│   │   │   ├── commands/         Tauri command modules
│   │   │   ├── agent/            Agent runtime and execution
│   │   │   ├── database/         SQLx pool, migrations
│   │   │   ├── security/         Credentials, sandbox, validation
│   │   │   ├── windows/          Main + artifact window management
│   │   │   ├── config/           App configuration
│   │   │   ├── telemetry/        Tracing/logging setup
│   │   │   └── error.rs          AppError enum
│   │   ├── capabilities/         Tauri permission files
│   │   └── migrations/           SQL migration files
│   ├── index.html
│   ├── package.json
│   ├── tsconfig.json
│   └── vite.config.ts
├── crates/                       Rust workspace crates
│   ├── domain/                   Core domain models
│   ├── agent-core/               Agent execution engine
│   ├── artifact-core/            Artifact lifecycle
│   ├── provider-core/            Provider adapters
│   └── shared/                   Error codes, shared utilities
├── packages/                     TypeScript workspace packages
│   ├── ui/                       Shared UI components
│   ├── schemas/                  Zod validation schemas
│   ├── artifact-sdk/             Artifact plugin SDK
│   ├── financial-components/     Finance-specific components
│   ├── shared-types/             Shared TypeScript types
│   └── config/                   Configuration constants
├── plugins/                      Internal artifact plugins
│   ├── company-comparison/
│   ├── valuation-model/
│   ├── portfolio-risk/
│   ├── industry-map/
│   └── timeline/
├── docs/                         Documentation
├── scripts/                      Build and dev scripts
├── tests/                        Integration and E2E tests
└── Root configs                  .editorconfig, .gitignore, etc.
```

## IPC Communication Flow

```text
React Component
  → desktopApi.settings.healthCheck()
    → invoke("health_check")
      → Rust #[tauri::command] fn health_check()
        → Result<"ok", AppError>
          → JSON response
            → React receives "ok"
```

All IPC goes through the `src/lib/desktop-api/` layer. Components never call `invoke()` directly.

Command-boundary DTOs use explicit `camelCase` serialization (`#[serde(rename_all = "camelCase")]`) while internal Rust domain models retain `snake_case`. The `desktop-api/` TypeScript wrappers parse and validate all responses with strict Zod schemas. Static 1:1 IPC registration parity between frontend `invoke` calls and Rust `lib.rs` handlers is verified via `scripts/check-ipc-registration.mjs`.

## Agent Task Execution & Event Streaming

The current production path executes provider requests in background Tokio tasks
inside the Tauri Rust process. ADR-0010 accepts a staged migration to managed
worker subprocesses for long-running, tool-using, and third-party Agent workloads.
Until that roadmap is implemented and accepted, process isolation must not be
reported as complete for the generic Agent runtime.

```text
React (AgentPanel)
  → useRunAgentTask / useCreateAgentTask
    → desktopApi.agent.createAgentTask() & startAgentTask()
      → Rust TaskExecutor background Tokio task
        → Emits Tauri events (task:progress, task:completed, task:failed, task:cancelled)
          → React useTaskEventStream (real-time 20-message stream + TanStack Query invalidation)
          → Persisted in agent_task_events SQLite table
          → Structured ResearchCompletion parsed via Zod & rendered in ResearchResultCard
```

- **Startup Race Protection**: Rust emits `app:ready` upon `AppState` initialization; `useAppReady` gates frontend IPC.
- **Failure Context**: Exact failure payloads are surfaced to users in `AgentPanel` with complete EN/ZH-CN i18n.
- **Target Worker Boundary**: Rust will supervise ephemeral Agent workers while
  retaining provider credentials, network calls, tools, SQLite, domain writes,
  budgets, and audit state in the trusted host. See
  [`docs/agent/README.md`](agent/README.md).

## Current IPC Command Families

The application registers command families for system/settings, credentials, workspaces, Agent tasks, research, theses, knowledge graph, portfolio, Artifacts, internal plugins, Options, and Goose scaffolding. Registration parity across all 176 commands is verified by `scripts/check-ipc-registration.mjs`.

Current integration status:

- **Agent Runtime (S1 Complete)**: End-to-end task creation, background execution, real-time event streaming, cancellation, failure surfacing, and structured research output rendering are fully accepted.
- **IPC Normalization (S2 Complete)**: 100% of command families (`Workspace`, `Settings`, `Credentials`, `System`, `Options`, `Agent`, `Artifacts`, `Plugins`, `Research`, `Thesis`, `KnowledgeGraph`, `Portfolio`, `Financial`) are normalized to `camelCase` DTOs with strict runtime Zod schema validation and checked serialization fixtures.
- **Research, Thesis, Knowledge Graph, Portfolio (S4 Complete)**: Functional UI surfaces connected to SQLx persistence with URL context authority, provenance tracking, and full state coverage.
- **Options (S5 Complete)**: Option chain acquisition, contract selection, strategy building, persistence, Greeks/pricing calculations, and no-trading boundary enforcement are fully verified.
- **Artifacts & Plugins (S3 Complete)**: Isolated Artifact-window route, least-privilege capability boundary (`capabilities/artifact-window.json`), disabled plugin enforcement, and predefined safe React renderers are fully verified.
- **Release Readiness (S6 Complete)**: Full 6-stage stabilization program (S0-S6) accepted; 100% IPC parity (176/176), 0 typecheck errors, 55 test files passing, clean lint, fmt, clippy, and Rust workspace test suite.
- **Managed Agent Workers (Planned)**: ADR-0010 is accepted; the generic worker
  protocol, supervisor, brokers, and platform isolation remain an implementation
  workstream. Goose keeps its specialized sidecar policy and will converge on the
  common lifecycle where doing so does not weaken its controls.

## Database

- **Engine**: SQLite via SQLx
- **Migrations**: SQL files under `apps/desktop/src-tauri/migrations/`, applied by the custom runner in `database/migrations.rs`
- **Current domains**: settings, workspaces, Agent tasks/events, Artifacts, research, theses, knowledge graph, portfolio, internal plugins, and Options
- **Pragmas**: WAL mode, foreign keys enabled
- **Schema reference**: See `DATA_MODEL.md`; verify it against migrations when changing persisted data

## Security

- API keys: Stored through the OS keychain under canonical `openai.api_key`; plaintext secrets never cross into React
- Artifact windows: Minimal permissions (`capabilities/artifact-window.json`)
- Main window: Controlled permissions (`capabilities/main-window.json`)
- Input validation: Strict Zod parsing on frontend IPC boundaries and Serde/Domain validation in Rust
