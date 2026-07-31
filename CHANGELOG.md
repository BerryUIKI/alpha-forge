# Changelog

All notable changes to AlphaForge (Investment OS) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation suite (17+ documents)
- Architecture Decision Records (ADR) system
- Agent development workflow documentation (AGENTS.md)
- Product milestone roadmap

## [0.1.0] - 2026-07-31

### Added
- Initial project structure with pnpm + Cargo workspaces
- Tauri 2 desktop application shell
- React 19 + TypeScript + Vite 6 foundation
- SQLite migration system with SQLx
- IPC communication layer (12 commands: 4 active, 8 stubs)
- Rust module structure with AppError and AppState
- Comprehensive documentation suite
  - AGENTS.md - Agent coding standards (733 lines)
  - docs/ARCHITECTURE.md - System architecture
  - docs/SYSTEM_DESIGN.md - Nine subsystems
  - docs/DATA_MODEL.md - Entity relationships
  - docs/AGENT_PROTOCOL.md - Task lifecycle
  - docs/ARTIFACT_SYSTEM.md - Plugin rendering
  - docs/PLUGIN_SPEC.md - Plugin framework
  - docs/SECURITY.md - Security model
  - docs/UI_GUIDELINES.md - Design system
  - docs/DEVELOPMENT.md - Development guide
  - docs/ROADMAP.md - 12-phase technical roadmap
  - docs/MILESTONE_ROADMAP.md - Product milestones
  - docs/GIT_WORKFLOW.md - Git workflow
  - docs/PROJECT_BOOTSTRAP.md - Initialization plan
  - docs/PRODUCT.md - Product positioning
  - docs/VISION.md - Long-term vision
- Architecture Decision Records
  - ADR-0001: Initial Technology Stack
  - ADR-0002: Tauri over Electron
  - ADR-0003: Local-First Architecture
- Multilingual README (English, Chinese)
- GitHub PR template
- TypeScript strict mode enabled
- ESLint + Prettier + Vitest configured
- Vitest test framework setup

### Technical Foundation
- Desktop Shell: Tauri 2
- Backend: Rust, Tokio, SQLx, SQLite
- Frontend: React 19, TypeScript, Vite 6
- UI: Tailwind CSS 4, shadcn/ui, Radix UI, Lucide
- Quality: ESLint, Prettier, Vitest, Rustfmt, Clippy

### Project Status
- Phase 0 (Project Foundation): ✅ Complete
- Phase 1 (Desktop Runtime Foundation): ✅ Complete
- Phase 1.5 (Application Foundation): 🚧 In Progress

## [0.0.1] - 2026-07-25

### Added
- Repository initialization
- Basic documentation structure
- Git workflow configuration

---

## Version History

| Version | Date | Milestone |
|---------|------|-----------|
| 0.1.0 | 2026-07-31 | M0 & M1 Complete |
| 0.0.1 | 2026-07-25 | Repository Init |

---

For more details on planned milestones, see [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md).