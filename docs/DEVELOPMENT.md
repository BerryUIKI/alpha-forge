# Development Guide

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | stable (MSVC on Windows) | Backend, Tauri |
| Node.js | 22+ | Frontend tooling |
| pnpm | 9+ | Package management |
| Git | 2+ | Version control |

## Local Setup

```bash
# Clone and enter the repository
git clone <repo-url>
cd alpha-forge

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies from the authoritative lockfile
corepack pnpm install --frozen-lockfile

# Start development
corepack pnpm dev
```

## Package Manager and Lockfile Policy

This repository uses Corepack-managed pnpm 9.0.0, as declared in the root `package.json`; Corepack activates that pinned version for delegated workspace commands. The tracked `pnpm-lock.yaml` is the sole authoritative JavaScript lockfile; `package-lock.json` is not used. Use `corepack pnpm install --frozen-lockfile` for normal setup and reproducible installs. Regenerate the lockfile only as intentional dependency maintenance after changing package manifests, then review the resulting diff.

## Repository Structure

```
alpha-forge/
├── apps/desktop/          Tauri desktop application
│   ├── src/               React frontend
│   ├── src-tauri/         Rust backend
│   └── package.json
├── crates/                Rust workspace crates
├── packages/              TypeScript workspace packages
├── plugins/               Internal artifact plugins
├── docs/                  Documentation
├── scripts/               Build/dev scripts
└── tests/                 Integration and E2E tests
```

## Development Commands

### Frontend

```bash
corepack pnpm dev:web        # Start Vite dev server (frontend only, no Tauri)
corepack pnpm typecheck      # TypeScript type check (all packages)
corepack pnpm lint           # ESLint
corepack pnpm format         # Prettier auto-fix
corepack pnpm format:check   # Prettier check
corepack pnpm test           # Vitest
corepack pnpm test:watch     # Vitest in watch mode
```

### Rust

```bash
cargo check --workspace                      # Compile check (fast, no codegen)
cargo fmt --check                            # Format check
cargo fmt                                    # Auto-format
cargo clippy --all-targets --all-features    # Lint
cargo test --workspace                       # Run tests
```

### Tauri

```bash
corepack pnpm tauri dev       # Full Tauri desktop app (dev mode)
corepack pnpm tauri build     # Production build
```

### Full Check

```bash
./scripts/check.sh   # Runs typecheck, lint, cargo fmt, cargo clippy
```

## Branch Workflow

See [`docs/GIT_WORKFLOW.md`](GIT_WORKFLOW.md) for the full workflow.

Summary:

- Never develop on `main`.
- Create `feature/<name>` branches for new work.
- Use conventional commits: `feat:`, `fix:`, `docs:`, `chore:`, etc.
- Run relevant checks before every commit.
- Do not push or merge without explicit permission.

## AI Agent Development Workflow

When an AI agent (like this one) works on the project:

1. **Read first.** Inspect `AGENTS.md`, relevant docs, and the codebase before making changes.
2. **Plan.** State what will change and why.
3. **Implement.** Make focused, scoped changes.
4. **Verify.** Run relevant checks.
5. **Report.** Summarize what was done, what was verified, and what remains.

Agents must follow all rules in `AGENTS.md` — especially:

- Section 1.1: Understand before editing.
- Section 1.2: User requirements take priority.
- Section 14: Git safety rules.

## Testing Expectations

- Every new feature must include tests.
- Rust: unit tests for domain logic, integration tests for commands.
- React: component tests for critical UI, hook tests, schema tests.
- E2E: core workflows (task creation → artifact display → persistence).

## Editor Configuration

VS Code is the recommended editor. The `.vscode/` directory includes:

- `extensions.json` — Recommended extensions.
- `settings.json` — Workspace settings.
- `tasks.json` — Build and test tasks.

## Troubleshooting

### pnpm install fails

Clear the pnpm store, then retry the frozen install:

```bash
corepack pnpm store prune
corepack pnpm install --frozen-lockfile
```

### cargo check fails with "not a valid Win32 application"

This typically occurs in sandboxed environments. Ensure you're running commands in a local terminal with full system access.

### Tauri dev window appears blank

Check the Vite dev server is running on port 5173. Check `apps/desktop/src-tauri/tauri.conf.json` for correct `devUrl`.
