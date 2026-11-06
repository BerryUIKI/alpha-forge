# Changelog

All notable changes to AlphaForge (Investment OS) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Settings page for the AI provider: save or remove an OpenAI API key, stored in the OS keychain, with status-only reads.
- Comprehensive documentation suite (17+ documents)
- Architecture Decision Records (ADR) system
- Agent development workflow documentation (AGENTS.md)
- Product milestone roadmap
- Global toast notifications for user actions and errors.
- Light/dark/system theme switching with persistence and pre-render initialization.
- Agent tasks now execute in the background, persist output/error, and stream progress events to the frontend.
- Workspace task panel in the Today page: create tasks, start/cancel execution, and inspect a live execution timeline with output and error details.
- OpenAI-backed agent runtime: structured research prompts, schema-validated JSON output, and typed provider errors.
- Provider configuration commands with API keys stored in the OS keychain; status responses never expose the key.
- Agent queue semantics: at most two concurrent tasks, startup recovery for interrupted tasks, manual retry for failed/cancelled tasks, and automatic retries with backoff for transient provider errors.
- `alphaforge` CLI for workspaces and agent tasks, with human/JSON output, live progress, timeout, cancellation, and retry.
- CLI provider status/set-key/clear-key commands (hidden input, OS keychain) and artifact listing.
- Agent context and tools: workspace and recent-task context in prompts, an OpenAI tool-calling loop (max 4 rounds), and read-only tools for listing/searching tasks, reading task output, and arithmetic.
- Agent artifacts: validated comparison-table and timeline outputs are generated with tasks, persisted in SQLite, and rendered on the Artifacts page with predefined React components.
- Artifact windows: validated artifacts can open in a dedicated minimal-permission WebView (`core:default`) with the payload injected as parsed JSON.
- Research documents: workspace-scoped CRUD with a Research page for selecting a workspace and creating, editing, and deleting documents.

### Fixed
- Root `pnpm typecheck` now uses the pnpm workspace instead of a non-existent npm workspace.
- Workspace IPC timestamps now match serde's `created_at` / `updated_at` output.
- Migrations now run through SQLx's migrator; added `0003_agent_tasks_align` so agent tasks can persist the `created` state and nullable input.
- SQLite WAL mode and foreign-key enforcement are now configured for every pooled connection.

### Tests
- Added migration coverage for the full migration chain and agent task schema.
- Added agent task repository tests.
- Added agent task list, detail, and workspace panel tests covering start/cancel actions and event rendering.
- Expanded WorkspaceList tests to cover loading, empty, error, and data states.

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