# Changelog

All notable changes to AlphaForge (AlphaForge) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Demo Data Orchestration & First-Run Onboarding Wizard (模拟演示数据与初始引导向导):**
  - Implemented `demoData.ts` providing institutional-grade seed data: Workspace ("AI & Semiconductor Alpha"), Knowledge Graph entities & linkages, investment theses with empirical evidence, prime brokerage financial account, portfolio assets (NVDA, AAPL, MSFT, TSM), cash deposits and lot activities, 30 days of historical daily OHLC/volume price quotes, asset taxonomy, and 70/30 target asset allocation with rebalancing drift bands.
  - Implemented `OnboardingWizardModal` automatically displaying on initial launch when 0 workspaces exist, offering a dual-path choice: 1-click loading the full institutional demo dataset or creating a custom workspace.
  - Added permanent on-demand "Demo Data Management" section to `SettingsPage` enabling demo data re-seeding at any time with TanStack Query cache invalidation and active workspace switching.
  - Added comprehensive unit tests for `OnboardingWizardModal` and `SettingsPage` demo data integration, verifying 100% locale catalog parity.
- **Thesis & Agent Deep Research Loop (投资研究与 Agent 深度联动闭环 - Direction 3):**
  - Integrated direct Agent investigation dispatch from `ThesisDetail` (`handleLaunchAgentResearch`), enabling 1-click background evidence harvesting and thesis falsification search.
  - Implemented automated evidence harvesting from Goose Shadow Analysis (`onComplete` -> `handleHarvestShadowEvidence`), persisting structured supporting and contradicting facts directly into `thesis_evidence`.
  - Added user feedback notices and confirmation banners for agent research loop lifecycle events.
  - Added 100% parity localized keys (`agentResearchLoop*`, `launchAgentEvidenceTask`, `evidenceHarvestSuccess`) across `zh-CN` and `en`.
- **Asset Taxonomies & Allocation Targets (资产分类体系与配置目标体系 - Category 11):**
  - Integrated Category 11 domain functionality with TanStack Query hooks in `useFinancialData`: `useListTaxonomies`, `useCreateTaxonomy`, `useListTaxonomyCategories`, `useCreateTaxonomyCategory`, `useListAllocationTargets`, `useCreateAllocationTarget`, `useListAllocationWeights`, `useAddAllocationWeight`.
  - Created `TaxonomyAllocationPanel` mounted in `PortfolioDashboard` displaying multi-dimensional taxonomy hierarchies, model allocation targets, and rebalancing drift bands.
  - Added target vs actual holding weight comparison with real-time drift compliance indicators (`withinDriftStatus` vs `outOfDriftStatus`) and rebalancing action guidance.
  - Added 100% parity localized strings for all taxonomy and allocation target workflows across `zh-CN` and `en`.
- **Market Quotes Dashboard (行情看板):**
  - Added standalone top-level route `/quotes` accessible via the Left Sidebar navigation under Tools.
  - Implemented real-time and historical asset quote tracking cards displaying latest price, day changes, and thesis linkage.
  - Added on-demand single asset quote refreshing and bulk active assets market data refresh (`useRefreshAllActiveQuotes`, `useRefreshAssetQuote`).
  - Added historical quote inspection table per active asset with full date, OHLC, volume, and data source breakdown.
  - Added visual price trend area chart (`AreaChart` with gradient fill, OHLC/volume tooltip, and high/low/average period metrics) with seamless Chart/Table view toggle.
  - Added multi-period timeframe filtering (`1W`, `1M`, `3M`, `1Y`, `ALL`) and technical indicators overlay (20-day Simple Moving Average, OHLC view mode).
  - Added localized navigation and content strings with 100% parity across `zh-CN` and `en`.
- **Investment & Decision Journal Timeline (决策与投资日志):**
  - Differentiated `/journal` into a dedicated chronological decision timeline capturing thesis lifecycle changes, research project activity, and background agent tasks.
  - Added category filtering (`all`, `theses`, `research`, `tasks`), confidence progression markers, and validation outcome badges.

### Fixed
- **Demo Data DB Constraint Compliance & Diagnostics (演示数据约束与诊断增强):**
  - Resolved SQLite foreign key constraint failure in thesis evidence creation by embedding source citations into evidence text and omitting invalid non-UUID `source_id` references.
  - Aligned allocation target creation parameters with schema CHECK constraints (`trigger_type: 'threshold'`, `rebalance_goal: 'nearest_band'`) and set category assignment weight to 10000 basis points (100.00%).
  - Enhanced error diagnostics in both `OnboardingWizardModal` and `SettingsPage` to report precise underlying error messages.
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
  - Connected `StatusBar` to `useSystemHealth` for real-time SQLite database health monitoring and localized operational indicators.
  - Linked `SecFilingFeed` dynamically to active portfolio assets and workspace investment theses with interactive ticker pill selection.
  - Replaced native browser dialogs (`window.confirm`) with accessible in-app `ConfirmDialog` modal components across `ArtifactsPage`, `OptionContractTable`, and `OptionStrategyPanel` with keyboard focus trap and escape handling.
  - Computed real unrealized P&L dollar values and percentage returns from holdings summaries in `OverviewTab` and individual top holdings.
  - Mounted previously orphaned `StrategyBuilder` component on `OptionsPage` for multi-leg payoff simulation and break-even calculations.
  - Integrated `ShadowAnalysis` component into `ThesisDetail` for AI shadow-mode validation and hypothesis stress-testing.
  - Added descriptive `disabledReason` tooltips to disabled navigation and palette actions in `WindowTitleBar`.
  - Replaced placeholder modules (`startup.ts`, `formatting/index.ts`, `validation/index.ts`, `types/index.ts`) with production runtime diagnostics, formatting utilities, schema re-exports, and shared domain models.

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