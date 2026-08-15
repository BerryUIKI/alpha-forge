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

Command-boundary DTOs use explicit camelCase serialization where required. The Option and System desktop wrappers validate unknown responses with Zod; broader cross-family fixture coverage remains part of stabilization.

## Current IPC Command Families

The application registers command families for system/settings, credentials, workspaces, Agent tasks, research, theses, knowledge graph, portfolio, Artifacts, internal plugins, Options, and Goose scaffolding. Command registration alone is not completion evidence.

Current integration status is maintained in the [Frontend-Backend Integration and Functional Completeness Audit](reviews/INTEGRATION_GAP_AUDIT_2026-08-12.md). In particular:

- Agent queue/start and OpenAI credential repairs are merged with focused regression coverage; full Agent-to-Artifact verification remains pending.
- Research, thesis, knowledge graph, and portfolio commands have reachable primary UI surfaces.
- Artifact persistence, in-page renderers, and the isolated Artifact-window route are merged (#88) with focused route and permission tests; packaged smoke acceptance remains pending.
- Internal plugin commands and a Settings management surface exist; the controlled create-to-Artifact workflow remains incomplete.
- Option command-boundary DTOs and System information responses now use the reviewed camelCase/Zod contracts; the Option UI vertical slice remains incomplete.
- Goose commands are registered as scaffolding; the service is disabled and the feature is not accepted.

## Database

- **Engine**: SQLite via SQLx
- **Migrations**: SQL files under `apps/desktop/src-tauri/migrations/`, applied by the custom runner in `database/migrations.rs`
- **Current domains**: settings, workspaces, Agent tasks/events, Artifacts, research, theses, knowledge graph, portfolio, internal plugins, and Options
- **Pragmas**: WAL mode, foreign keys enabled
- **Schema reference**: See `DATA_MODEL.md`; verify it against migrations when changing persisted data

## Security

- API keys: Stored through the OS keychain under canonical `openai.api_key`; the legacy `api_key` entry is migrated by Rust, and plaintext secrets do not cross into React
- Artifact windows: Minimal permissions (`capabilities/artifact-window.json`)
- Main window: Controlled permissions (`capabilities/main-window.json`)
- Input validation: Zod is used in selected TypeScript boundaries and explicit validation is used in Rust; coverage is not yet uniform
