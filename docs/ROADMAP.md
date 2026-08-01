# Roadmap

## Phase 0 — Documentation Foundation ✅

Establish project documentation, repository structure, and Git workflow.

- [x] Repository structure with pnpm + Cargo workspaces.
- [x] `AGENTS.md` — coding standards and agent rules.
- [x] `docs/PROJECT_BOOTSTRAP.md` — full initialization plan.
- [x] `docs/GIT_WORKFLOW.md` — branching, commits, PR process.
- [x] Complete documentation suite (PRODUCT, VISION, ARCHITECTURE, etc.).
- [x] Architecture Decision Records.

## Phase 1 — Desktop Runtime Foundation ✅

Create a runnable Tauri 2 + React + TypeScript + Vite foundation.

- [x] pnpm workspace with all dependencies.
- [x] Tauri 2 desktop application shell.
- [x] React router with 6 page stubs.
- [x] Unified desktop-api IPC layer.
- [x] Rust module tree with AppError, AppState.
- [x] SQLite migration system (SQLx).
- [x] 12 IPC commands (4 active, 8 stubs).
- [x] TypeScript strict mode — passes typecheck.
- [x] ESLint + Prettier + Vitest configured.
- [ ] Rust compilation verification (blocked by sandbox — local only).

## Phase 2 — Frontend Foundation

Build the complete application shell with production-quality UI.

- [ ] Initialize shadcn/ui components.
- [ ] Global error boundary with recovery.
- [ ] Toast notification system.
- [ ] Theme support (light/dark).
- [ ] Keyboard shortcut infrastructure.
- [ ] Complete MainLayout with resizable panels.
- [ ] Agent workspace UI: AgentInput, TaskStatus, TaskHistory.
- [ ] All page skeletons with proper loading/empty/error states.

## Phase 3 — Frontend–Rust IPC

Verify and expand IPC communication.

- [ ] Confirm health_check round-trip.
- [ ] Wire agent commands to real state.
- [ ] Implement settings persistence (read/write).
- [ ] Add IPC integration tests.
- [ ] Define stable error codes.

## Phase 4 — Local Database

Implement the full data model.

- [ ] Apply complete migration (16+ tables).
- [ ] Create repository layer for all entities.
- [ ] Implement soft deletion.
- [ ] Add database integration tests.

## Phase 5 — Agent Runtime

Implement the single-agent execution engine.

- [ ] Task creation, queuing, and execution.
- [ ] Async background execution with Tokio.
- [ ] Progress event streaming via Tauri events.
- [ ] Cancellation and timeout support.
- [ ] Structured output parsing.
- [ ] OpenAI API integration.
- [ ] Tool registration and invocation.

## Phase 6 — Artifact System

Implement temporary interactive windows for agent output.

- [ ] Artifact manifest validation.
- [ ] Artifact registry and routing.
- [ ] Temporary WebView window creation.
- [ ] Plugin rendering of structured data.
- [ ] Artifact persistence.
- [ ] Permission isolation for artifact windows.

## Phase 7 — Plugin System

Implement the internal plugin framework.

- [ ] Plugin registration and loading.
- [ ] Manifest and version validation.
- [ ] Input schema validation.
- [ ] Plugin error isolation.
- [ ] Five internal plugins: company-comparison, valuation-model, portfolio-risk, industry-map, research-timeline.

## Phase 8 — Research Workspace

Build the research management system.

- [ ] Research document CRUD.
- [ ] Source management with provenance tracking.
- [ ] Notes system.
- [ ] Document search.
- [ ] Agent-assisted research workflows.

## Phase 9 — Journal (Thesis Tracking)

Build the investment thesis management system.

- [ ] Thesis CRUD with status workflow.
- [ ] Evidence collection (supporting/contradicting).
- [ ] Confidence tracking.
- [ ] Validation scheduling.
- [ ] Outcome recording and review.

## Phase 10 — Portfolio Intelligence

Build portfolio management and analysis.

- [ ] Account and position management.
- [ ] Transaction import.
- [ ] Exposure and concentration analysis.
- [ ] Theme allocation tracking.
- [ ] Risk dashboards (via portfolio-risk plugin).

## Phase 11 — Security Hardening 🚧

- [x] OS keychain credential storage with validated credential names, bounded values, idempotent deletion, and redacted platform errors.
- [ ] URL allowlist enforcement.
- [x] Artifact route validation: UUID artifact IDs, safe type segments, bounded window sizes, and recoverable invalid-route errors.
- [x] Artifact route traversal prevention.
- [ ] Plugin sandbox enforcement.
- [x] Startup and task-event logs use stable error codes and contextual messages without raw local paths or underlying error strings.
- [ ] Security audit.

## Phase 12 — Production Readiness

- [ ] Application icons and branding.
- [ ] Installer packaging (DMG, MSI, AppImage).
- [ ] Auto-update infrastructure.
- [ ] Performance optimization.
- [ ] Final documentation review.
- [ ] Release checklist.

## First Runnable Milestone

```text
Launch → Enter research task
  → Background agent runs (async)
    → Live progress events stream to UI
      → Agent produces structured output
        → Artifact window opens with interactive content
          → User closes or persists
            → Result saved in SQLite
```

This milestone spans Phases 2–7 and represents the core product loop.
