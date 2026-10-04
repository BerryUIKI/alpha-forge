# Changelog

All notable changes to AlphaForge (AlphaForge) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **GUI Placeholder & Parity Hardening:**
  - Synchronized Knowledge Graph TanStack Query keys between `KnowledgePage` and `useKnowledgeGraph`, resolving cache desynchronization.
  - Enabled entity creation in `KnowledgeGraphPanel` empty state.
  - Ensured `ArtifactWindowPage` persists artifact closure to backend SQLite via `desktopApi.artifacts.closeArtifact`.
  - Replaced hardcoded static emerald pulse in `LeftSidebar` and `RightSidebar` with dynamic `useAgentGlobalStatus()` indicators.
  - Wired live query invalidation and spinning indicator to `MarketPulseBar` refresh button.
  - Implemented automatic hash scrolling for navigation in `SettingsLayout`.
  - Modernized `WindowTitleBar` edit actions with `navigator.clipboard` integration.
  - Eliminated dead `window.alert` branches in `PortfolioDashboard` and wired comprehensive query cache invalidation on refresh.
  - Added user feedback banners on thesis status transitions and artifact launch failures.
  - Provided descriptive disabled state tooltips in `QuickActions` and `OptionStrategyPanel`.
  - Replaced placeholder calculations in `StrategyBuilder` with genuine multi-leg break-even analysis.

## [0.1.0-rc.1] - 2026-10-03

### Added
- **Milestone M11 Portfolio Release Candidate:**
  - Automated release packaging pipeline (`scripts/package-release.mjs`, `scripts/generate-attribution.mjs`, `scripts/smoke-packaged-flows.mjs`).
  - Third-party licensing & SBOM attribution documentation (`docs/releases/THIRD_PARTY_NOTICES.md`).
  - Cross-platform packaged smoke verification suite (`docs/releases/M11_PACKAGED_SMOKE_EVIDENCE.md`).
  - Formal Release Candidate Acceptance Gate (`docs/releases/M11_RELEASE_ACCEPTANCE.md`).
- **Milestone M10 Goose Agent Integration:** Supervised sidecar runtime with read-only MCP scopes, shadow analysis, proposal workflows, and strict zero-trading enforcement.
- **Milestone M9 Options Analytics:** Black-Scholes pricing models, Greeks calculation, implied volatility solver, and interactive strategy payoff diagrams.
- **Milestone M6 & M11 Financial Portfolio Capabilities:** Multi-account tracking, lot-level cost basis engine, transaction activities, Generic & IBKR statement CSV importing, valuation snapshots, and asset allocation constraint checks.
- **Thesis-Asset Linkage:** Bidirectional relational linkage between investment theses and portfolio assets (`theses.portfolio_asset_id` → `financial_assets.id`).
- **Performance Benchmarks:** Criterion benchmark suite covering 9 portfolio scale workloads and frontend sub-50ms transformation budgets (`docs/portfolio/PERFORMANCE_BASELINE.md`).

### Changed
- Retired 12 deprecated legacy portfolio commands in favor of canonical SQLx services, achieving 100% IPC registration parity (176 commands).
- Optimized allocation constraint queries from N+1 database queries to a single batched query (-19.6% execution time).
- Enhanced large CSV statement import resilience with chunked Tokio yields and 10,000-row batch safety limits.

### Fixed
- Eliminated 5 production panic points (`.unwrap()`) in `ActivityService` lot creation and disposal, replacing them with typed `AppError::Validation`.
- Closed and verified legacy portfolio deprecation issues #194 through #198 with full audit evidence.

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