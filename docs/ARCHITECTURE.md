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
investment-os/
├── apps/desktop/                 Tauri 2 desktop application
│   ├── src/                      React frontend
│   │   ├── app/                  App entry, router, providers
│   │   ├── pages/                6 route pages (Today hosts workspace + agent tasks; others placeholders)
│   │   ├── components/           Layout, navigation, feedback, common
│   │   ├── features/             8 feature modules (workspace and agent active; others placeholders)
│   │   ├── lib/desktop-api/      Unified IPC layer (6 modules)
│   │   ├── hooks/                (future)
│   │   ├── stores/               (future)
│   │   ├── styles/               Tailwind CSS 4 globals
│   │   └── types/                Shared TypeScript types
│   ├── src-tauri/                Rust backend
│   │   ├── src/
│   │   │   ├── app/              AppState, bootstrap
│   │   │   ├── commands/         7 command modules (20 commands)
│   │   │   ├── agent/            Agent runtime, progress events, cancellation
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
│   ├── agent-core/               Agent execution engine (stub)
│   ├── artifact-core/            Artifact lifecycle (stub)
│   ├── provider-core/            Provider adapters (stub)
│   └── shared/                   Error codes, shared utilities
├── packages/                     TypeScript workspace packages
│   ├── ui/                       Shared UI components (stub)
│   ├── schemas/                  Zod validation schemas (stub)
│   ├── artifact-sdk/             Artifact plugin SDK (stub)
│   ├── financial-components/     Finance-specific components (stub)
│   ├── shared-types/             Shared TypeScript types (stub)
│   └── config/                   Configuration constants (stub)
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

## Current IPC Commands

| Command(s) | Module | Status |
|------------|--------|--------|
| `health_check`, `get_app_info`, `get_setting`, `set_setting` | settings | ✅ Active |
| `create_workspace`, `list_workspaces`, `get_workspace`, `update_workspace`, `delete_workspace` | workspace | ✅ Active (SQLite) |
| `create_agent_task`, `get_agent_task`, `list_agent_tasks`, `get_task_events`, `queue_agent_task`, `start_agent_task`, `cancel_agent_task` | agent | ✅ Active (background execution, event streaming, cancellation/timeout; workspace task panel with live timeline; real AI provider pending) |
| `list_research_documents` | research | Stub — returns [] |
| `list_theses` | journal | Stub — returns [] |
| `list_portfolio_accounts` | portfolio | Stub — returns [] |
| `list_artifacts` | artifacts | Stub — returns [] |

## Database

- **Engine**: SQLite via SQLx
- **Migrations**: SQL files in `apps/desktop/src-tauri/migrations/`, embedded by `sqlx::migrate!`, tracked in `_sqlx_migrations`
- **Current tables**: `app_settings`, `workspaces`, research tables, thesis/evidence tables, `agent_tasks`, `agent_task_events`, artifacts, plugins, portfolio tables, watchlists
- **Pragmas**: WAL mode and foreign keys configured through `SqliteConnectOptions` for every pooled connection
- **Full schema**: See `DATA_MODEL.md` for conceptual entities

## Security

- API keys: Will use OS keychain (stub in `security/credentials.rs`)
- Artifact windows: Minimal permissions (`capabilities/artifact-window.json`)
- Main window: Controlled permissions (`capabilities/main-window.json`)
- Input validation: Will use Zod (TS) + manual validation (Rust)
