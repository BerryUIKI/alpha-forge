# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Required reading and product context

`AGENTS.md` is the authoritative coding standard for this repository — architecture boundaries, security model, testing requirements, and Git safety rules. Read it before changing code. The notes below supplement it with commands and the current implementation map.

AlphaForge (product name "Investment OS") is a desktop-first, local-first AI investment-research workspace. Its product loop is:

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

It is a research tool, not a brokerage terminal: it must never execute trades or make autonomous investment decisions. All user data lives locally in SQLite; there is no cloud backend (ADR-0003). The project is at milestone M1.5 (Application Foundation); the agent runtime, artifact system, and plugin runtime are largely scaffolding.

## Commands

Prerequisites: Node.js 22+, pnpm 9+, Rust stable with MSVC on Windows (`rust-toolchain.toml` pins `stable-x86_64-pc-windows-msvc`). On Windows, run Cargo from a Developer Command Prompt or after loading `vcvars64.bat`; otherwise `link.exe` may not find system libraries such as `bcrypt.lib`.

Install dependencies:

```bash
pnpm install
```

Frontend / desktop development:

```bash
pnpm dev          # full Tauri desktop app (Vite + Rust)
pnpm dev:web      # Vite dev server only; Tauri IPC calls fail outside the desktop shell
pnpm build        # web frontend only (tsc + vite build) — not a desktop installer
pnpm tauri build  # production Tauri bundle
```

Quality checks:

```bash
pnpm lint
pnpm typecheck
pnpm test                                       # Vitest, run once
pnpm test:watch
pnpm format
pnpm format:check
```

Rust checks (run from the repository root):

```bash
cargo check --workspace
cargo fmt --check                               # cargo fmt to apply
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

Single tests:

```bash
pnpm exec vitest run apps/desktop/src/features/workspace/components/WorkspaceList.test.tsx
pnpm exec vitest run -t "renders Today page at root path"
cargo test -p domain test_task_status_is_terminal
cargo test -p investment-os migrations
```

`./scripts/check.sh` runs typecheck, lint, `cargo fmt --check`, and Clippy with `-D warnings`.

## Repository layout

```text
apps/desktop/src/                    React 19 frontend
apps/desktop/src-tauri/src/          Rust backend (Tauri commands, services, repositories)
apps/desktop/src-tauri/migrations/   SQL migrations (append-only)
apps/desktop/src-tauri/capabilities/ Tauri permission files
crates/domain/                       Pure domain models (used by the backend and tests)
crates/{agent-core,artifact-core,provider-core,shared}/   Mostly scaffolding/stubs
packages/*                           TypeScript workspace packages
plugins/*                            Internal artifact plugin manifests/stubs
docs/                                Architecture, product, ADRs, and workflow docs
```

The pnpm workspace covers `apps/*`, `packages/*`, and `plugins/*`; the Cargo workspace covers `apps/desktop/src-tauri` and `crates/*`.

## Architecture

### IPC data flow

```text
React component
  → feature hook (TanStack Query)
    → desktopApi.<domain>.<fn>()              apps/desktop/src/lib/desktop-api/
      → invoke("<command>", args)             @tauri-apps/api/core
        → #[tauri::command]                   apps/desktop/src-tauri/src/commands/
          → service                            apps/desktop/src-tauri/src/services/
            → repository                       apps/desktop/src-tauri/src/database/repositories/
              → SQLx / SQLite
```

- All IPC must go through `desktopApi` (`apps/desktop/src/lib/desktop-api/`). Components must not call `invoke()` directly. Add new commands to the relevant module and export them through `desktop-api/index.ts`.
- Register every new Tauri command in the `invoke_handler` list in `apps/desktop/src-tauri/src/lib.rs`; otherwise the frontend call fails at runtime.
- Keep commands thin: validate → call service → map result. Services own validation, business rules, and state transitions. Repositories own SQL and convert private SQLx row structs into domain models.
- `AppState` (`src-tauri/src/app/state.rs`) owns the SQLite pool and service instances. Wire new repositories/services there; database initialization and `AppState` registration happen in `app/bootstrap.rs` and `lib.rs` setup.
- Errors use `AppError` (`Internal`, `NotFound`, `Validation`, `PermissionDenied`, `Timeout`) and serialize as `{ code, message, recoverable }`. Use typed errors rather than `unwrap()`/`panic!` in production paths.
- Domain models live in `crates/domain`. The other `crates/*` packages are placeholders — do not assume runtime logic lives there.

### Frontend

- React 19 + TypeScript strict mode + Vite + Tailwind CSS 4 + shadcn/Radix. The `@/` alias points at `apps/desktop/src` in both Vite and Vitest.
- Feature modules follow `features/<domain>/{components,hooks,index.ts}`; pages are thin route entries registered in `app/router.tsx`; shared loading/empty/error components live in `components/common/`.
- Use TanStack Query for asynchronous/server state (query keys are centralized per feature, e.g. `WORKSPACE_KEYS` and `AGENT_KEYS`), local component state for UI behavior, and Zustand only for lightweight cross-page state.
- Every async view should handle loading, success, empty, error, partial, and offline states (see `AGENTS.md` §9.2).
- `packages/ui` currently exports a shared `Badge` and `cn`; `packages/schemas`, `packages/shared-types`, `packages/config`, `packages/artifact-sdk`, and `packages/financial-components` are placeholders.

### Database and migrations

- SQLite via SQLx. The database file is `investment_os.db` in Tauri's app-data directory. WAL mode and foreign keys are enabled in `database/connection.rs`; the pool uses max 5 connections.
- Migrations are append-only. Never edit an already-applied migration. Add a new numbered SQL file under `apps/desktop/src-tauri/migrations/`; SQLx embeds and applies them in order, tracking state in `_sqlx_migrations`.
- Table-rebuild migrations run outside a transaction with `-- no-transaction` and manage `PRAGMA foreign_keys` themselves (see `0003_agent_tasks_align.sql`). Keep the schema, repositories, and migration tests in sync when changing persisted data.
- `0001_initial.sql` already defines most planned tables (research documents/sources/notes, theses/evidence, agent tasks/events, artifacts, plugins, portfolio accounts/positions/transactions, watchlists), even though many corresponding commands and UI pages are still stubs.

### Agent runtime and artifacts

- Canonical task model: `crates/domain/src/task.rs` — `TaskStatus` is `created → queued → running → waiting_for_input → completed/failed/cancelled`, with typed `AgentTaskEvent`s. `commands/agent.rs` and `services/agent_service.rs` currently implement CRUD plus status transitions only.
- `agent/runtime.rs` defines `AgentRuntime` and a simulated `LocalDemoRuntime`; it is not wired into commands/services yet. `agent/task.rs` is an older placeholder with a duplicate `TaskStatus` — use `domain::task`, not that type. `agent/events.rs` (Tauri event streaming) is a stub, so tasks are not actually executed in the background yet.
- Artifact and plugin code (`artifacts/*`, `plugins/*`, `documents/*`, `providers/*`, `security/*`) is placeholder scaffolding. The intended contracts are documented in `docs/ARTIFACT_SYSTEM.md`, `docs/PLUGIN_SPEC.md`, and the manifests in `plugins/*/manifest.json` (`id`, `name`, `version`, `entry`, `inputSchema`, `permissions`, `window`).
- Treat agent output as untrusted: validate it against a schema and render it with predefined React plugins. Never inject agent-generated HTML into the privileged main-window DOM.
- Artifact windows receive only `core:default` (`capabilities/artifact-window.json`); the main window additionally gets opener, shell-open, and store permissions (`capabilities/main-window.json`).
- The research, journal, portfolio, and artifact list commands (`commands/research.rs`, `journal.rs`, `portfolio.rs`, `artifacts.rs`) are stubs returning empty collections.

### Security boundaries

- Rust owns credentials, filesystem/network access, and long-running work. React must never receive plaintext keys or bypass `desktopApi`. Follow `AGENTS.md` §8 and `docs/SECURITY.md`.
- `.env.example` currently lists `VITE_OPENAI_API_KEY`. `VITE_`-prefixed variables are bundled into the frontend, so this must not become the credential path. Credentials belong in Rust/keychain storage (currently a stub at `security/credentials.rs`).
- New privileged operations need the justification, allowed inputs, validation, and misuse prevention documented in `AGENTS.md` §8.

## Testing

- Vitest is configured at the repository root: jsdom, `apps/desktop/src/test/setup.ts`, the `@` alias, and test discovery across `apps/**` and `packages/**`.
- Frontend tests mock Tauri with `vi.mock("@tauri-apps/api/core", ...)`; they never launch the desktop shell or a real database.
- Rust tests use `#[tokio::test]` and in-memory SQLite, running the real migration chain through `database::migrations::run`.
- `.github/workflows/ci.yml` runs typecheck, lint, Vitest, the web build, `cargo fmt --check`, Clippy, and `cargo test --workspace` on `windows-latest` for pushes to `main` and all pull requests.
- Add tests at the layer being changed (domain, repository, service, command, component, hook, schema) per `AGENTS.md` §12, and add a regression test for bug fixes where practical.

## Known gaps

- `AGENTS.md` mentions `pnpm test:e2e`, but no such script or E2E suite exists yet.
- `pnpm build` only produces the web frontend; use `pnpm tauri build` for a desktop release.

## Git workflow

- `main` is protected. Never develop on `main`; use `feature/<name>`, `fix/<name>`, `docs/<name>`, or `refactor/<name>`.
- Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `test:`, `refactor:`, ...).
- Never commit, push, or merge unless explicitly requested. Never run destructive Git commands (`git reset --hard`, `git clean -fd`, `git push --force`, `git branch -D`). Preserve existing uncommitted user changes.
- See `docs/GIT_WORKFLOW.md` and `docs/PR_BEST_PRACTICES.md`.

## Documentation map

- `AGENTS.md` — authoritative coding and agent standards.
- `docs/ARCHITECTURE.md` — architecture overview (partly stale relative to the current command set).
- `docs/DEVELOPMENT.md` — local setup and troubleshooting.
- `docs/AGENT_PROTOCOL.md` — task lifecycle and asynchronous agent rules.
- `docs/ARTIFACT_SYSTEM.md`, `docs/PLUGIN_SPEC.md` — artifact and plugin contracts.
- `docs/DATA_MODEL.md` — conceptual entities and relationships.
- `docs/DECISIONS/` — ADRs (stack, Tauri over Electron, local-first).
- `docs/MILESTONE_ROADMAP.md` — milestone status from M0 through M8.
