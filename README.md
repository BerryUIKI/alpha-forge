# AlphaForge (Investment OS)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**Desktop-first AI workspace for investment research** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## 🎯 What is AlphaForge?

AlphaForge is an **AI-native investment research workspace** designed to transform raw information into structured investment knowledge.

### Core Product Loop

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

AlphaForge helps you:
- 📊 **Research efficiently** — AI-assisted document analysis and information gathering
- 💡 **Build theses** — Track investment theses with evidence and confidence levels
- 📈 **Make informed decisions** — Structured research workflow, not chatbot-style interaction
- ✅ **Validate outcomes** — Track thesis performance and learn from results

> **⚠️ Important**: This is a **research workspace**, not a brokerage terminal. It does NOT execute trades or make autonomous investment decisions.

---

## 📋 Table of Contents

- [Status](#status)
- [Features](#features)
- [Screenshots](#screenshots)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [Security](#security)
- [Current Limitations](#current-limitations)
- [License](#license)

---

## 📊 Status

**Phase 1.5 — Application Foundation** (in progress)

| Milestone | Status | Description |
|-----------|--------|-------------|
| M0 | ✅ Complete | Project Foundation |
| M1 | ✅ Complete | Desktop Runtime Foundation |
| M1.5 | 🚧 In Progress | Application Foundation |
| M2 | 📋 Planned | Agent Runtime |
| M3 | 📋 Planned | Artifact Intelligence System |
| M4 | 📋 Planned | Research Workspace |
| M5 | 📋 Planned | Investment Knowledge System |
| M6 | 📋 Planned | Portfolio Intelligence |
| M7 | 📋 Planned | Plugin Ecosystem |
| M8 | 📅 Future | Production & Commercialization |

See [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) for detailed milestones.

---

## ✨ Features

### Current (M0-M1.5)
- ✅ Tauri 2 desktop application shell
- ✅ React 19 + TypeScript + Vite foundation
- ✅ Rust backend with SQLite persistence
- ✅ IPC communication layer
- ✅ Comprehensive documentation (17+ documents)

### Planned (M2+)
- 📋 AI-powered research assistance
- 📋 Investment thesis tracking
- 📋 Interactive artifacts (charts, tables, visualizations)
- 📋 Document analysis and semantic search
- 📋 Portfolio-thesis alignment
- 📋 Plugin ecosystem

---

## 🖼️ Screenshots

> **Note**: AlphaForge is in early development (M1.5). Screenshots will be added as UI development progresses.

---

## 🛠️ Tech Stack

| Layer | Technology |
|-------|-----------|
| **Desktop Shell** | Tauri 2 |
| **Backend** | Rust, Tokio, SQLx, SQLite |
| **Frontend** | React 19, TypeScript, Vite 6 |
| **UI** | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| **AI** | OpenAI API (planned) |
| **Quality** | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## 🚀 Getting Started

### Prerequisites

- **Rust stable** (MSVC toolchain on Windows)
- **Node.js 22+**
- **pnpm 9+**

### Development Commands

```bash
# Install dependencies
pnpm install

# Frontend development
pnpm dev:web          # Start Vite dev server (frontend only)
pnpm typecheck        # TypeScript type check
pnpm lint             # ESLint
pnpm test             # Vitest

# Desktop development (requires Rust)
pnpm tauri dev        # Start full Tauri desktop app
pnpm tauri build      # Production build

# Rust commands (requires Rust)
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 🏗️ Architecture

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for full architecture details.

### Key Boundaries

```
┌─────────────────────────────────────────┐
│            Tauri 2 Desktop              │
│  ┌────────────┐       ┌──────────────┐  │
│  │   React    │◄─────►│    Rust      │  │
│  │ Frontend   │  IPC  │   Backend    │  │
│  └────────────┘       └──────────────┘  │
│                            │             │
│                        ┌───┴───┐         │
│                        │SQLite │         │
│                        └───────┘         │
└─────────────────────────────────────────┘
```

**React** owns:
- Pages, components, interaction
- Frontend state
- User interface

**Rust** owns:
- Agent runtime
- SQLite database
- Filesystem & network access
- Credentials management

**Tauri** owns:
- Desktop windows
- IPC communication
- Permissions & security
- OS integration

---

## 📚 Documentation

### Core Documents

| Document | Purpose |
|----------|---------|
| [AGENTS.md](AGENTS.md) | Agent coding standards (**required reading**) |
| [PRODUCT.md](docs/PRODUCT.md) | Product positioning and MVP scope |
| [VISION.md](docs/VISION.md) | Long-term direction |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System architecture |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | Product milestones |

### Technical Documentation

| Document | Purpose |
|----------|---------|
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | Agent task lifecycle |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | Artifact rendering |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | Plugin development |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | Entity relationships |
| [SECURITY.md](SECURITY.md) | Security policy |

### Development Guides

| Document | Purpose |
|----------|---------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guide |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Git and PR workflow |
| [PR_BEST_PRACTICES.md](docs/PR_BEST_PRACTICES.md) | PR guidelines |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | Local setup guide |

---

## 🤝 Contributing

We welcome contributions! 

### 🔒 Branch Protection Notice

**Main branch is protected. Direct pushes are BLOCKED.**

All changes must go through Pull Request:
1. Create feature branch
2. Make changes and commit
3. Create Pull Request
4. Get at least 1 approval
5. Merge to main

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed workflow.

### Quick Start

1. Read [AGENTS.md](AGENTS.md) (**required**)
2. Check [CONTRIBUTING.md](CONTRIBUTING.md)
3. Fork, create branch, submit PR

All contributions must follow our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## 🗺️ Roadmap

### Development Timeline

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

### Phase Overview

**Foundation (M0-M1.5)**:
- Project setup
- Desktop runtime
- Application foundation

**Intelligence (M2-M3)**:
- Agent runtime
- AI integration
- Artifact system

**Features (M4-M6)**:
- Research workspace
- Thesis tracking
- Portfolio analysis

**Extensibility (M7-M8)**:
- Plugin ecosystem
- Production release

See [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) for details.

---

## 🔐 Security

Security is a top priority. See [SECURITY.md](SECURITY.md) for:
- Vulnerability reporting process
- Security architecture
- Credential management
- Permission model

**Reporting**: Please report security issues privately via GitHub Security.

---

## ⚠️ Current Limitations

1. **Rust compilation in sandbox**: `cargo check/test/clippy` must run locally
2. **`pnpm tauri dev`**: Requires local Rust compilation
3. **No application icons**: Placeholder only
4. **No real AI integration**: Agent commands return stubs
5. **No tests written**: Framework configured, no test files

---

## 📄 License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPLv3)** - see the [LICENSE](LICENSE) file for details.

### Why AGPLv3?

AGPLv3 ensures that:
- ✅ All modifications must be shared back to the community
- ✅ Network use (SaaS) triggers copyleft requirements
- ✅ Users always have access to the source code
- ✅ Commercial use is allowed with proper licensing

This protects the open-source nature of AlphaForge while allowing sustainable development.

---

## 🙏 Acknowledgments

AlphaForge is made possible by these open source projects:

- [Tauri](https://tauri.app) - Desktop application framework
- [React](https://react.dev) - UI library
- [Rust](https://www.rust-lang.org) - Systems programming language
- [shadcn/ui](https://ui.shadcn.com) - UI components
- [Tailwind CSS](https://tailwindcss.com) - CSS framework

---

## 📞 Contact

- **Issues**: [GitHub Issues](https://github.com/BerryUIKI/alpha-forge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions)

---

<p align="center">
  <strong>Built with ❤️ by the AlphaForge team</strong>
</p>

<p align="center">
  <sub>Transforming information into investment intelligence</sub>
</p>