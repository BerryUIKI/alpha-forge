# GUI Unification & Backend Connectivity Plan

> **Status:** Draft  
> **Created:** 2026-08-03  
> **Scope:** Merge the two parallel navigation architectures, eliminate all placeholder UI, and ensure every button invokes a real backend command or a documented deferred action.

---

## 1. Executive Summary

The current GUI has **two unreconciled navigation systems** that coexist without connecting to each other:

| System | Entry Point | Navigation | Backend | Status |
|--------|-------------|------------|---------|--------|
| **Model A: WorkspaceViews** (GUI-M1 Codex layout) | `MainLayout` → `MainContent` | `WorkspaceSelector` dropdown toggles 6 placeholder views | None | **All placeholders** |
| **Model B: Routed Pages** (UI Guidelines) | `router.tsx` → `MainLayout` (as parent route) | `Sidebar.tsx` NavLink icons | Full backend via `desktopApi` | **Dead code** (never rendered) |

The `Sidebar.tsx` navigation component is imported by **zero consumers**.  
`MainLayout.tsx` renders `LeftSidebar` + `MainContent` (WorkspaceViews) but **never renders `<Outlet />`**, so the 6 routed pages (`TodayPage`, `ResearchPage`, `JournalPage`, `PortfolioPage`, `ArtifactsPage`, `SettingsPage`) are unreachable.

**Resolution:** Remove the WorkspaceView system (Model A), make `MainLayout` render the `Sidebar.tsx` navigation + `<Outlet />` for routed pages (Model B), and retain the collapsible right sidebar (Agent panel) as an overlay.

---

## 2. Audit Findings

### 2.1 CRITICAL — Routing Architecture Break

| # | Issue | File | Detail |
|---|-------|------|--------|
| C1 | `MainLayout` has no `<Outlet />` | `components/layout/MainLayout.tsx:82-108` | Routed pages never render |
| C2 | `Sidebar.tsx` is imported nowhere | `components/navigation/Sidebar.tsx` | Icon navigation sidebar is dead code |
| C3 | `MainContent` renders 6 placeholder views | `components/layout/MainContent/MainContent.tsx:66-73` | All WorkspaceViews show "coming soon" |
| C4 | `MainContent` is the only content area | `components/layout/MainLayout.tsx:93-97` | No slot for routed pages |

### 2.2 HIGH — Placeholder WorkspaceViews (6 files)

All 6 files follow an identical pattern: icon + "coming soon" + dashed-border placeholder list.

| File | Placeholder Text |
|------|-----------------|
| `WorkspaceViews/AnalyzeView.tsx` | "Analysis tools and visualizations coming soon" |
| `WorkspaceViews/QuantificationView.tsx` | "Quantitative analysis tools coming soon" |
| `WorkspaceViews/ComprehensiveMarketView.tsx` | "Market overview and analysis tools coming soon" |
| `WorkspaceViews/OptionsView.tsx` | "Options analysis and strategy tools coming soon" |
| `WorkspaceViews/FuturesView.tsx` | "Futures market analysis tools coming soon" |
| `WorkspaceViews/OtherDerivativesView.tsx` | "Specialized derivatives analysis tools coming soon" |

### 2.3 HIGH — Disabled Buttons and Stub Handlers

| # | Component | Buttons/Handlers | Issue |
|---|-----------|-----------------|-------|
| H1 | `OperationBar` | Search button (`disabled`), Create button (`disabled`) | No onClick, titles say "coming soon" |
| H2 | `WelcomePage` | 4 quick action buttons (Open Project, New Research, Analyze Market, View Portfolio) | All `disabled`, titles say "coming soon" |
| H3 | `AgentPanel` | Message input (`disabled`), Send button (`disabled`), 4 tool buttons (`disabled`), 3 task buttons (`disabled`), Settings button | All `disabled` or `console.log` stub |
| H4 | `LeftSidebar.UserOperations` | "Profile" menu item | `console.log("Profile clicked")` stub |
| H5 | `LeftSidebar` | `handleMenuItemClick` | `console.log("Menu item clicked")` stub |
| H6 | `MainLayout` | `handleWorkspaceChange` | `console.log("Workspace changed to:", workspace)` stub |

### 2.4 HIGH — Hardcoded / Mock Data

| # | Component | Issue |
|---|-----------|-------|
| D1 | `ScrollableList` | `PLACEHOLDER_ITEMS` array with 5 fake items (lines 16-22) |
| D2 | `StatusBar` | Hardcoded `status="idle"` in `MainContent.tsx:82` |
| D3 | `AgentPanel` | Hardcoded `status="Ready to assist"` in `RightSidebar.tsx:108` |
| D4 | `RightSidebar` | Hardcoded `placeholder="Agent capabilities coming soon"` |
| D5 | `LeftSidebar` | Hardcoded `username="Investor"` in `LeftSidebar.tsx:138` |
| D6 | `WorkspaceSelector` | 6 workspace types (`WORKSPACE_OPTIONS`) not connected to any data model |

### 2.5 MEDIUM — Missing desktopApi Wrappers

15 backend commands have no frontend `desktopApi` wrapper:

| Backend Command | Module | Notes |
|----------------|--------|-------|
| `get_system_info` | system | |
| `get_config_dir` | system | |
| `get_data_dir` | system | |
| `check_database_health` | system | |
| `delete_setting` | settings | |
| `list_settings` | settings | |
| `get_research_project` | research | |
| `archive_research_project` | research | |
| `complete_research_project` | research | |
| `delete_research_project` | research | |
| `get_research_document` | research | |
| `delete_research_document` | research | |
| `get_research_report` | research | |
| `delete_research_report` | research | |
| `list_journal_entries` | journal | Backend itself is a placeholder returning `[]` |

### 2.6 MEDIUM — desktopApi Bug

| # | File | Issue |
|---|------|-------|
| B1 | `desktop-api/journal.ts:6` | Invokes `"list_thesis"` but registered command is `"list_theses"` — runtime error if called |

### 2.7 MEDIUM — Orphaned Feature Components

| Component | Exported In | Imported By |
|-----------|-------------|-------------|
| `AgentTaskList` | `features/agent/index.ts` | **Nobody** |
| `CreateAgentTask` | `features/agent/index.ts` | **Nobody** |
| `TaskStatusBadge` | `features/agent/index.ts` | **Nobody** |
| `WelcomePage` | `components/layout/MainContent/WelcomePage.tsx` | **Nobody** (import exists in MainContent but component is unused in render) |

### 2.8 LOW — Hardcoded English Strings (No i18n)

| Component | Strings |
|-----------|---------|
| `ThesisList` | "Failed to load theses.", "No theses yet", "Create a thesis to begin preserving your investment reasoning." |
| `ThesisDetail` | "Evidence", "Supporting", "Contradicting", "Source ID (optional)", "Add a fact, data point, or argument..." |
| `KnowledgeGraphPanel` | "Knowledge graph", entity type labels, "Entity name", "Add entity", "Add relationship", "Source", "Target" |
| `CreateAgentTask` | Placeholder strings in input fields |
| `WorkspaceViews/*` | All English labels (entire files) |
| `StatusBar` | "Idle", "Running", "Error", "Syncing", hint text |
| `OperationBar` | "Search", "Create new", "Agent", "Hide Agent panel", "Show Agent panel" |
| `AgentPanel` | "Agent", "Ready to assist", all section titles, button labels |
| `WelcomePage` | All labels and descriptions |
| `ScrollableList` | Empty message default "No items yet" |

### 2.9 LOW — Missing UI States (per AGENTS.md §9.2)

| Component | Missing States |
|-----------|---------------|
| `KnowledgeGraphPanel` | No loading/error/empty for `useKnowledgeEntities` and `useKnowledgeRelationships` |
| `ArtifactViewer` | Uses inline divs instead of standard `LoadingSpinner`/`ErrorState` components; no retry button |
| `PortfolioDashboard` | `AlignmentReviewPanel` error shows `unableToCreateAccount` message (semantic mismatch) |
| `PortfolioDashboard` | `usePortfolioReview` does not invalidate queries on success |
| `ThesisDashboard` | Calls `useTheses` redundantly — `ThesisList` calls it again internally |

---

## 3. Architecture Decision

### 3.1 Chosen Approach: Route-Based Navigation (Model B)

**Rationale:**
- The routed pages (`TodayPage`, `ResearchPage`, `JournalPage`, `PortfolioPage`, `ArtifactsPage`, `SettingsPage`) are **already connected to the backend** via `desktopApi` + TanStack Query.
- The `Sidebar.tsx` navigation component follows `UI_GUIDELINES.md` (the product spec).
- The WorkspaceViews are 100% placeholders with zero backend integration.
- Removing the WorkspaceView system eliminates 6 placeholder files, the `WorkspaceSelector`, `ScrollableList`, `useWorkspaceState` hook, and the `WorkspaceType` union type — all of which have no backend counterpart.

### 3.2 Target Layout

```
┌──────────────────────────────────────────────────────┐
│                   MainLayout                          │
│  ┌──────┐  ┌───────────────────────┐  ┌───────────┐ │
│  │      │  │     <Outlet />          │  │  Agent    │ │
│  │ Side │  │  (routed pages)        │  │  Panel    │ │
│  │ bar  │  │                        │  │ (toggle)  │ │
│  │(64px)│  │                        │  │           │ │
│  │      │  │                        │  │           │ │
│  └──────┘  └───────────────────────┘  └───────────┘ │
└──────────────────────────────────────────────────────┘
```

- **Left:** `Sidebar.tsx` (64px icon navigation, NavLink-based)
- **Center:** `<Outlet />` rendering routed pages (Today, Research, Journal, Portfolio, Artifacts, Settings)
- **Right:** `RightSidebar` with `AgentPanel` (collapsible, retained from current layout)
- **Removed:** `LeftSidebar` (WorkspaceSelector + ScrollableList + UserOperations), `MainContent` (OperationBar + WorkspaceViews + StatusBar + WelcomePage), `useWorkspaceState`, `WorkspaceType`

### 3.3 Agent Panel Integration

The right sidebar `AgentPanel` will be preserved as a collapsible overlay. Its "coming soon" elements will be replaced with real agent task components (`AgentTaskList`, `CreateAgentTask`, `TaskStatusBadge`) that are currently orphaned but already connected to the backend.

---

## 4. Task Breakdown

Tasks are organized into alternating **Phase 1 (Backend + Docs)** and **Phase 2 (Frontend)** PRs. Each task is a single PR.

### Phase 1: Backend + Documentation Synchronization

---

#### PR-01: Fix `desktop-api/journal.ts` Bug

**Branch:** `fix/journal-api-command-name`

| Item | Detail |
|------|--------|
| Problem | `journal.ts` invokes `"list_thesis"` but registered command is `"list_theses"` |
| Fix | Either fix the command name to `"list_theses"`, or redirect to the real `journal.list_journal_entries` command (which currently returns `[]`) |
| Files | `apps/desktop/src/lib/desktop-api/journal.ts` |
| Tests | Update `journal` desktopApi tests if they exist |
| Depends on | None |

---

#### PR-02: Add Missing `desktopApi` Wrappers for Backend Commands

**Branch:** `feat/desktopapi-missing-wrappers`

Add frontend wrappers for the 15 unregistered backend commands so all backend capabilities are accessible.

| Module | Commands to Add |
|--------|----------------|
| `system.ts` | `getSystemInfo()`, `getConfigDir()`, `getDataDir()`, `checkDatabaseHealth()` |
| `settings.ts` | `deleteSetting(key)`, `listSettings()` |
| `research.ts` | `getResearchProject(id)`, `archiveResearchProject(id)`, `completeResearchProject(id)`, `deleteResearchProject(id)`, `getResearchDocument(id)`, `deleteResearchDocument(id)`, `getResearchReport(id)`, `deleteResearchReport(id)` |
| `journal.ts` | `listJournalEntries()` (invoke `list_journal_entries`) |

| Item | Detail |
|------|--------|
| Files | `apps/desktop/src/lib/desktop-api/system.ts`, `settings.ts`, `research.ts`, `journal.ts` |
| Tests | Add test cases for each new wrapper |
| Docs | Update `docs/FRONTEND_BACKEND_INTEGRATION.md` with the new API surface |
| Depends on | PR-01 (journal fix) |

---

#### PR-03: Unify Layout — Replace WorkspaceViews with Routed Pages

**Branch:** `feat/layout-routing-unification`

This is the **core architectural PR** that merges the two navigation systems.

| Step | Action |
|------|--------|
| 1 | Rewrite `MainLayout.tsx` to use `Sidebar.tsx` (left) + `<Outlet />` (center) + `RightSidebar` (right) |
| 2 | Remove `LeftSidebar/` directory (WorkspaceSelector, ScrollableList, UserOperations) |
| 3 | Remove `MainContent/` directory (OperationBar, StatusBar, WelcomePage, WorkspaceViews/) |
| 4 | Remove `useWorkspaceState` hook from `hooks/layout/` |
| 5 | Remove `WorkspaceType`, `MenuBarConfig`, `MenuItemId`, `FileMenuItem`, `EditMenuItem`, `ViewMenuItem`, `HelpMenuItem` from `components/layout/types.ts` |
| 6 | Keep `SidebarState`, `DEFAULT_SIDEBAR_WIDTHS`, and right-sidebar related types |
| 7 | Move `SidebarState` and right-sidebar props to a simpler types file or inline them |
| 8 | Add an "Agent" toggle button inside the routed pages' layout (or as a floating button) to control `RightSidebar` visibility |
| 9 | Add keyboard shortcut (Ctrl+B or Ctrl+2) to toggle Agent panel |
| 10 | Ensure `UserOperations` functionality (theme toggle, language switch, settings nav) is accessible from the new layout — either via a small user menu in `Sidebar.tsx` or in `SettingsPage` |

| Item | Detail |
|------|--------|
| Files Modified | `components/layout/MainLayout.tsx`, `components/layout/types.ts`, `components/layout/index.ts`, `hooks/layout/index.ts` |
| Files Removed | `components/layout/LeftSidebar/`, `components/layout/MainContent/`, `hooks/layout/useWorkspaceState.ts` |
| Files Promoted | `components/navigation/Sidebar.tsx` becomes the primary left navigation |
| Tests | Update `app/router.test.tsx`, add `MainLayout.test.tsx` |
| Depends on | None (this is a self-contained layout change) |
| Risk | Medium — must verify all routed pages render correctly |

---

#### PR-04: Wire Agent Panel to Backend (Replace Placeholders)

**Branch:** `feat/agent-panel-backend`

Replace `AgentPanel` placeholders with real agent task components.

| Step | Action |
|------|--------|
| 1 | Replace the "Conversation" section placeholder with `CreateAgentTask` component |
| 2 | Replace the "Tasks" section placeholder with `AgentTaskList` component |
| 3 | Use `TaskStatusBadge` for task status indicators |
| 4 | Remove all `disabled` attributes from agent input/buttons |
| 5 | Wire `handleSendMessage` to `desktopApi.agent.createAgentTask` or a chat command |
| 6 | Add loading/empty/error states using `useAgentTasks` hook |
| 7 | Remove hardcoded `status="Ready to assist"` — derive from real task state |
| 8 | Remove "coming soon" text from Tools section or disable with tooltip explaining M10 scope |

| Item | Detail |
|------|--------|
| Files | `components/layout/RightSidebar/AgentPanel.tsx`, `components/layout/RightSidebar/RightSidebar.tsx` |
| Tests | Add `AgentPanel.test.tsx` |
| Depends on | PR-03 (new layout must be in place) |

---

### Phase 2: Frontend Development (During PR-03 Review)

---

#### PR-05: Add Missing desktopApi Consumers to Routed Pages

**Branch:** `feat/pages-missing-api-consumers`

Wire the newly-added desktopApi wrappers (from PR-02) into the relevant pages.

| Page | New Functionality |
|------|-------------------|
| `ResearchPage` | Add delete buttons for projects, documents, reports (using `deleteResearchProject`, `deleteResearchDocument`, `deleteResearchReport`) |
| `ResearchPage` | Add archive/complete actions for projects |
| `SettingsPage` | Add settings list view (using `listSettings`) and delete button (using `deleteSetting`) |
| `SettingsPage` | Add database health indicator (using `checkDatabaseHealth`) |
| `SettingsPage` | Add system info display (using `getSystemInfo`) |
| `JournalPage` | Wire `listJournalEntries` (currently backend returns `[]`, but frontend should be ready) |

| Item | Detail |
|------|--------|
| Files | `pages/research/ResearchPage.tsx`, `pages/settings/SettingsPage.tsx`, `pages/journal/JournalPage.tsx` |
| Tests | Update relevant page tests |
| Depends on | PR-02 must be merged first |

---

#### PR-06: i18n Hardcoded Strings in Feature Components ✅ COMPLETED

**Branch:** `feat/i18n-feature-components`
**PR:** https://github.com/BerryUIKI/alpha-forge/pull/66

| Item | Status |
|------|--------|
| CreateAgentTask: 11 strings | ✅ Done (existing keys) |
| AgentTaskList: 3 strings | ✅ Done (existing keys) |
| ThesisList: 4 strings | ✅ Done (new keys) |
| ThesisDetail: 27 strings | ✅ Done (new keys) |
| KnowledgeGraphPanel: 15 strings | ✅ Done (new keys) |

---

#### PR-07: Fix Missing UI States in Feature Components

**Branch:** `feat/ui-states-features`

Add loading/empty/error states per AGENTS.md §9.2.

| Component | Fix |
|-----------|-----|
| `KnowledgeGraphPanel` | Add `LoadingSpinner`/`ErrorState`/`EmptyState` for both entity and relationship queries |
| `ArtifactViewer` | Replace inline error divs with standard `ErrorState` component; add retry button |
| `PortfolioDashboard` | Fix `AlignmentReviewPanel` error message (use `t("reviewFailed")` instead of `t("unableToCreateAccount")`) |
| `PortfolioDashboard` | Add query invalidation in `usePortfolioReview` on success |

| Item | Detail |
|------|--------|
| Files | `features/thesis/components/KnowledgeGraphPanel.tsx`, `features/artifacts/components/ArtifactViewer.tsx`, `features/portfolio/components/PortfolioDashboard.tsx` |
| Tests | Add state-specific test cases |
| Depends on | PR-06 (i18n keys for new error messages) |

---

#### PR-08: Remove Dead Code and Clean Up Exports

**Branch:** `chore/remove-dead-code`

Clean up after the layout unification is complete.

| Step | Action |
|------|--------|
| 1 | Remove `WelcomePage.tsx` import from `MainContent` (already removed with MainContent in PR-03, verify) |
| 2 | Remove unused exports from `features/agent/index.ts` if they remain orphaned after PR-04 |
| 3 | Remove `PLACEHOLDER_ITEMS` from `ScrollableList` (already removed in PR-03, verify) |
| 4 | Clean up any remaining `console.log` stubs |
| 5 | Remove `.tmp` file: `database/repositories/mod.rs.tmp` |
| 6 | Verify `features/portfolio/hooks/usePortfolio.ts` exports match actual usage |

| Item | Detail |
|------|--------|
| Files | Various cleanup across `components/`, `features/`, `database/` |
| Tests | `pnpm lint`, `pnpm typecheck`, `cargo clippy` |
| Depends on | PR-03, PR-04 must be merged |

---

### Optional / Deferred Tasks (Post-MVP)

These are noted but **not in the current PR plan** per AGENTS.md §1.2 (do not expand scope):

| Task | Reason for Deferral |
|------|-------------------|
| Connect AgentPanel "Tools" buttons to real tool invocation | Agent tool execution is M10 scope (Goose integration) |
| Implement Command Palette (Ctrl+K) | No backend command routing exists yet |
| Add `list_journal_entries` backend implementation | Journal is empty placeholder on both sides; no data model defined |
| Portfolio theme linking UI improvements | `linkPortfolioTheme` works but needs UX refinement — low priority |
| View transition animations | AGENTS.md §1.2: no premature architecture |

---

## 5. Dependency Graph

```
PR-01 (fix journal bug)
  └── PR-02 (add missing desktopApi wrappers)
        └── PR-05 (wire new wrappers into pages)  ← can run during PR-03 review

PR-03 (layout unification — core)
  └── PR-04 (wire agent panel to backend)
  └── PR-08 (remove dead code cleanup)

PR-06 (i18n feature components) ← independent, can run in parallel with PR-03
  └── PR-07 (fix missing UI states) ← depends on PR-06 for i18n keys
```

### Parallel Execution

| Wave | PRs | Notes |
|------|-----|-------|
| Wave 1 | PR-01, PR-03, PR-06 | PR-01 is tiny; PR-03 is the core work; PR-06 is independent i18n |
| Wave 2 (during Wave 1 review) | PR-02 (after PR-01 merges), PR-07 (after PR-06 merges) | |
| Wave 3 (after Wave 1+2 merge) | PR-04, PR-05, PR-08 | Final wiring and cleanup |

---

## 6. Verification Checklist

After all PRs are merged, the following must hold:

- [ ] Every route defined in `router.tsx` renders its page when navigated to
- [ ] `Sidebar.tsx` is the active left navigation and highlights the current route
- [ ] No "coming soon" text appears anywhere in the application
- [ ] No `disabled` button exists without a documented reason (e.g., M9/M10 scope)
- [ ] No `console.log()` stubs remain in production code
- [ ] No `PLACEHOLDER_ITEMS` or hardcoded mock data arrays remain
- [ ] Every button triggers a real action (backend call, navigation, or state change)
- [ ] Agent panel shows real task list and allows task creation
- [ ] All user-facing strings use i18n (`t()` function)
- [ ] Every async component handles loading, empty, error, and success states
- [ ] `pnpm lint` passes
- [ ] `pnpm typecheck` passes
- [ ] `pnpm test` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --workspace` passes

---

## 7. Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| PR-03 layout change breaks existing page rendering | High | Test each routed page individually before merging; keep `RightSidebar` untouched |
| Agent panel wiring reveals backend bugs | Medium | PR-04 is isolated; can disable agent panel if backend issues found |
| i18n key proliferation causes catalog drift | Low | Run `catalog-parity.test.ts` after each i18n PR |
| Removing WorkspaceViews removes future Options/Futures UI entry points | Low | These are M9 scope; workspace selector concept does not match the data model. Pages can be added as new routes when needed. |

---

## 8. Files Touched (Summary)

| Category | Files Added | Files Modified | Files Removed |
|----------|-------------|----------------|---------------|
| Layout | 0 | `MainLayout.tsx`, `types.ts`, `layout/index.ts`, `hooks/layout/index.ts` | `LeftSidebar/` (4 files), `MainContent/` (9 files), `useWorkspaceState.ts` |
| Navigation | 0 | `Sidebar.tsx` (promoted to primary nav) | 0 |
| Agent Panel | 0 | `AgentPanel.tsx`, `RightSidebar.tsx` | 0 |
| desktopApi | 0 | `system.ts`, `settings.ts`, `research.ts`, `journal.ts` | 0 |
| Pages | 0 | `ResearchPage.tsx`, `SettingsPage.tsx`, `JournalPage.tsx` | 0 |
| Features | 0 | `ThesisList.tsx`, `ThesisDetail.tsx`, `KnowledgeGraphPanel.tsx`, `ArtifactViewer.tsx`, `PortfolioDashboard.tsx` | 0 |
| i18n | 0 | `en/*.ts`, `zh-CN/*.ts` (multiple catalogs) | 0 |
| Tests | ~8 | Various test files | `mod.rs.tmp` |
| Docs | 0 | `FRONTEND_BACKEND_INTEGRATION.md` | 0 |
