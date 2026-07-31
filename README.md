# AlphaForge (Investment OS)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**Desktop-first AI workspace for investment research** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md)

---

## Core Product Loop

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

AlphaForge transforms raw information into structured investment knowledge, turning research into testable theses and informed decisions.

> **⚠️ Important**: This is a **research workspace**, not a brokerage terminal. It does not execute trades or make autonomous investment decisions.

---

## Table of Contents

- [Status](#status)
- [Screenshots](#screenshots)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [Current Limitations](#current-limitations)
- [License](#license)

---

## Status

**Phase 1.5 — Application Foundation** (in progress)

| Milestone | Status | Description |
|-----------|--------|-------------|
| M0 | ✅ Complete | Project Foundation |
| M1 | ✅ Complete | Desktop Runtime Foundation |
| M1.5 | 🚧 In Progress | Application Foundation |
| M2-M7 | 📋 Planned | Agent Runtime → Plugin Ecosystem |
| M8 | 📅 Future | Production & Commercialization |

See [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) for detailed milestones.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | Tauri 2 |
| Backend | Rust, Tokio, SQLx, SQLite |
| Frontend | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| AI | OpenAI API (future) |
| Quality | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## Screenshots

> **Note**: AlphaForge is in early development (M1.5). Screenshots will be added as UI development progresses.

<!-- TODO: Add actual screenshots when UI is implemented -->
<!--
### Main Window
![Main Window](docs/assets/screenshot-main.png)

### Research Workspace
![Research Workspace](docs/assets/screenshot-research.png)

### Agent Task Progress
![Agent Task](docs/assets/screenshot-task.png)
-->

---

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
| [SECURITY.md](SECURITY.md) | Security policy, credential storage, vulnerability reporting |
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

---

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Start

1. Read [AGENTS.md](AGENTS.md) (**required** for all contributors)
2. Check [CONTRIBUTING.md](CONTRIBUTING.md) for workflow
3. Fork, branch, and submit a PR

All contributions must follow our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Roadmap

AlphaForge is being developed in 9 milestones:

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

- **M0-M1**: Foundation (Complete)
- **M1.5**: Application Foundation (Current)
- **M2-M7**: Intelligence & Features (Planned)
- **M8**: Production & Commercialization (Future)

See [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) for detailed milestones.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

AlphaForge is made possible by these open source projects:

- [Tauri](https://tauri.app) - Desktop application framework
- [React](https://react.dev) - UI library
- [Rust](https://www.rust-lang.org) - Systems programming language
- [shadcn/ui](https://ui.shadcn.com) - UI component library
- [Tailwind CSS](https://tailwindcss.com) - CSS framework

---

<p align="center">
  Built with ❤️ by the AlphaForge team
</p>
