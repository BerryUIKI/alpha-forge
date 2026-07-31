# Investment OS

Desktop-first AI workspace for investment research.

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

Investment OS transforms raw information into structured investment knowledge, turning research into testable theses and informed decisions.

**This is a research workspace, not a brokerage terminal.** It does not execute trades or make autonomous investment decisions.

## Status

**Phase 1 — Technical Foundation** (in progress)

- [x] Repository structure
- [x] pnpm workspace with 13 packages
- [x] Tauri 2 + React + TypeScript + Vite foundation
- [x] TypeScript strict mode — passes `pnpm typecheck`
- [x] ESLint + Prettier configured — passes
- [x] Vitest test framework configured
- [x] Rust module structure with `AppError`, `AppState`, IPC commands
- [x] SQLite migration system (SQLx)
- [ ] Rust compilation (`cargo check`) — blocked by sandbox (see Limitations)
- [ ] `pnpm tauri dev` — requires local Rust compilation

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | Tauri 2 |
| Backend | Rust, Tokio, SQLx, SQLite |
| Frontend | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| AI | OpenAI API (future) |
| Quality | ESLint, Prettier, Vitest, Rustfmt, Clippy |

## Getting Started

### Prerequisites

- Rust stable (MSVC toolchain on Windows)
- Node.js 22+
- pnpm 9+

### Development Commands

```bash
pnpm install          # Install all dependencies
pnpm dev:web          # Start Vite dev server (frontend only)
pnpm typecheck        # TypeScript type check (all packages)
pnpm lint             # ESLint
pnpm format:check     # Prettier format check
pnpm format           # Prettier auto-fix
pnpm test             # Vitest
```

### Tauri Development (requires local Rust)

```bash
pnpm tauri dev        # Start full Tauri desktop app
pnpm tauri build      # Production build
```

### Rust Commands (requires local Rust)

```bash
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Architecture

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the full architecture document.

Key boundaries:

- **React** owns pages, components, interaction, frontend state.
- **Rust** owns agent runtime, SQLite, filesystem, network, credentials.
- **Tauri** owns windows, IPC, permissions, OS integration.

## Documentation

| Document | Purpose |
|----------|---------|
| [AGENTS.md](AGENTS.md) | Agent coding standards and rules (highest priority) |
| [PRODUCT.md](docs/PRODUCT.md) | Product positioning, target users, MVP scope |
| [VISION.md](docs/VISION.md) | Long-term direction and design philosophy |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System boundaries, component responsibilities, IPC flow |
| [SYSTEM_DESIGN.md](docs/SYSTEM_DESIGN.md) | Nine subsystems — purpose, inputs, outputs, dependencies |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | Conceptual entities, relationships, lifecycles |
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | Agent task lifecycle, tool usage, structured output, events |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | Artifact concept, rendering model, permission model |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | Plugin manifest, versioning, permissions, lifecycle |
| [SECURITY.md](docs/SECURITY.md) | Credential storage, window permissions, input validation |
| [UI_GUIDELINES.md](docs/UI_GUIDELINES.md) | Design system, required UI states, navigation patterns |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | Local setup, dev commands, agent workflow, troubleshooting |
| [ROADMAP.md](docs/ROADMAP.md) | 12-phase development roadmap (technical phases) |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | Product milestones with deliverables and acceptance criteria |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Branch strategy, commit conventions, PR process |
| [PROJECT_BOOTSTRAP.md](docs/PROJECT_BOOTSTRAP.md) | Full initialization plan with 10 implementation phases |
| [DECISIONS/](docs/DECISIONS/) | Architecture Decision Records (3 ADRs) |

## Current Limitations

1. **Rust compilation in sandbox**: The WorkBuddy sandbox prevents native binary execution. `cargo check`, `cargo test`, `cargo clippy` must be run locally.
2. **`pnpm tauri dev`**: Depends on Rust compilation. Must be run locally.
3. **No application icons**: Placeholder directories only. Icons needed before release build.
4. **No real AI integration**: Agent commands return stubs. Real integration in Phase 7.
5. **No tests written yet**: Vitest framework configured but no test files.

## License

MIT
