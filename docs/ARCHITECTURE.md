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
│   │   ├── pages/                6 route pages (all placeholders)
│   │   ├── components/           Layout, navigation, feedback, common
│   │   ├── features/             8 feature modules (all placeholders)
│   │   ├── lib/desktop-api/      Unified IPC layer (6 modules)
│   │   ├── hooks/                (future)
│   │   ├── stores/               (future)
│   │   ├── styles/               Tailwind CSS 4 globals
│   │   └── types/                Shared TypeScript types
│   ├── src-tauri/                Rust backend
│   │   ├── src/
│   │   │   ├── app/              AppState, bootstrap
│   │   │   ├── commands/         6 command modules (12 commands)
│   │   │   ├── agent/            Agent runtime types (stubs)
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

| Command | Module | Status |
|---------|--------|--------|
| `health_check` | settings | ✅ Active |
| `get_app_info` | settings | ✅ Active |
| `get_setting` | settings | ✅ Active (SQLite) |
| `set_setting` | settings | ✅ Active (SQLite) |
| `create_task` | agent | Stub — returns completed task |
| `list_tasks` | agent | Stub — returns [] |
| `list_research_documents` | research | Stub — returns [] |
| `list_theses` | journal | Stub — returns [] |
| `list_portfolio_accounts` | portfolio | Stub — returns [] |
| `list_artifacts` | artifacts | Stub — returns [] |

## Database

- **Engine**: SQLite via SQLx
- **Migrations**: Embedded in `database/migrations.rs`
- **Current tables**: `app_settings`, `_migrations` (tracking)
- **Pragmas**: WAL mode, foreign keys enabled
- **Full schema**: See `DATA_MODEL.md` for planned tables

## Security

- API keys: Will use OS keychain (stub in `security/credentials.rs`)
- Artifact windows: Minimal permissions (`capabilities/artifact-window.json`)
- Main window: Controlled permissions (`capabilities/main-window.json`)
- Input validation: Will use Zod (TS) + manual validation (Rust)
