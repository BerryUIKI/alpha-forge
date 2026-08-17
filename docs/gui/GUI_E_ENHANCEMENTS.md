# GUI-E1 & GUI-E2 — Enhancement Work Packages

> **Document:** `docs/gui/GUI_E_ENHANCEMENTS.md`
> **Status:** ✅ Complete
> **Date:** 2026-08-17
> **Base:** `dev` (after GUI-M0–M5 merged via PRs #105–#111)

---

## 1. Overview

Following the completion of the GUI redesign (GUI-M0–M5), two high-priority
enhancements were identified in the [completion summary](GUI_M2_COMPLETION_SUMMARY.md):

1. **GUI-E1 — TopBar action buttons**: restore theme toggle, language selector,
   and a "New" action to the new TopBar (these lived in the removed
   `UserOperations` component).
2. **GUI-E2 — Dashboard data wiring**: replace the placeholder sample data in
   the dashboard tabs with real `desktopApi` queries.

---

## 2. GUI-E1: TopBar Action Buttons

### Problem Statement

The old `UserOperations` component (theme toggle, language selector, settings)
was removed with the legacy layout (PR #110). The new `TopBar` currently only
has a search box and the agent-panel toggle. Theme switching and language
selection are still available in `SettingsPage`, but should be one click away
from every page.

### Scope

| Item | Description |
|------|-------------|
| Theme toggle | Sun/Moon icon button cycling light/dark (via `next-themes` `useTheme`) |
| Language selector | Dropdown (EN / 简体中文) via existing `useLocale()` context |
| Settings shortcut | Gear icon navigating to `/settings` (consistent with old UX) |
| Divider + Agent toggle | Keep existing agent panel toggle, add separator between groups |

### Implementation Details

- **File:** `apps/desktop/src/components/layout/TopBar/TopBar.tsx`
- **New component:** `ActionButtons` (or inline) using Radix `DropdownMenu`
  (already a dependency via `@radix-ui/react-dropdown-menu`)
- Theme toggle uses the existing `ThemeToggle` pattern (`next-themes`)
- Language options use `LOCALES` from `@/lib/i18n/locale` +
  `useLocale().setLocale`
- i18n keys required: `themeLight`, `themeDark`, `language`, `menuSettings` —
  some already exist in `locale.ts` (`lightMode`, `darkMode`, `language`,
  `menuSettings`); reuse where possible

### Acceptance Criteria

- [ ] Theme toggle in TopBar switches light/dark and persists (via
      `next-themes` internal storage)
- [ ] Language dropdown switches EN/ZH immediately
- [ ] Settings gear navigates to `/settings`
- [ ] Agent toggle still works (Ctrl+2)
- [ ] i18n keys are present in both catalogs and `locale.ts`
- [ ] `pnpm typecheck` / `pnpm lint` / `pnpm test` pass

---

## 3. GUI-E2: Dashboard Data Wiring

### Problem Statement

`OverviewTab`, `PerformanceTab`, and `ActivityTab` use static sample data
(`SAMPLE_HOLDINGS`, `SAMPLE_ACTIVITY`). The wires exist in `desktopApi`
(portfolio, thesis, agent, research modules) but are not connected.

### Data Sources (available via `desktopApi`)

| Dashboard Section | API | Notes |
|-------------------|-----|-------|
| Total Portfolio Value | `listPortfolioAccounts(workspaceId)` | Sum balances (no balance field yet — see Risk) |
| Unrealized P&L | `listPortfolioPositions(accountId)` + transactions | Requires per-account aggregation |
| Active Theses count | `listTheses(workspaceId)` | Count where `status` is active/draft |
| Top Holdings | `getPortfolioAllocation(workspaceId)` | symbol + weight_percent |
| Recent Activity | `listAgentTasks(workspaceId)`, `listResearchProjects(workspaceId)`, `listTheses(workspaceId)` | Merge by `created_at`, take latest 5 |
| Performance chart | No time-series endpoint yet | Keep placeholder bars, label as "sample" |

### Implementation Details

- **New hook:** `apps/desktop/src/pages/today/hooks/useDashboardData.ts`
  - `usePortfolioSummary(workspaceId)` — aggregate accounts → total value
  - `useActiveTheses(workspaceId)` — thesis count
  - `useRecentActivity(workspaceId)` — merged agent/research/thesis events
  - Uses TanStack Query keys (`dashboardKeys`) with `workspaceId` param
- **Files updated:**
  - `apps/desktop/src/pages/today/tabs/OverviewTab.tsx`
  - `apps/desktop/src/pages/today/tabs/PerformanceTab.tsx`
  - `apps/desktop/src/pages/today/tabs/ActivityTab.tsx`
  - `apps/desktop/src/components/activity/ActivityFeed.tsx` (accept real data)
  - `apps/desktop/src/components/portfolio/HoldingsList.tsx` (accept real data)
- Workspace id from URL params (`?workspace=`) or first workspace
  (`useWorkspaces()`)
- Loading / empty / error states per [AGENTS.md §9.2](../../AGENTS.md)

### Acceptance Criteria

- [ ] Dashboard shows real portfolio value when workspace has accounts
- [ ] Active theses count reflects `listTheses` data
- [ ] Recent activity merges agent tasks + research projects + theses
- [ ] Empty state (0 accounts) and error state (IPC failure) render correctly
- [ ] Loading skeletons shown during fetch
- [ ] `pnpm typecheck` / `pnpm lint` / `pnpm test` pass

---

## 4. Execution Plan

| Step | Work | PR Branch | Depends on |
|------|------|-----------|------------|
| 1 | Docs (this file) + milestone status update | `docs/gui-e-docs` | dev |
| 2 | GUI-E1 TopBar action buttons | `feat/gui-e1-topbar-actions` | docs PR |
| 3 | GUI-E2 dashboard data wiring | `feat/gui-e2-dashboard-data` | docs PR |
| 4 | Final verification (`pnpm` + `cargo`) | — | E1, E2 |

### Branch strategy

Follow [GIT_WORKFLOW.md](../../docs/GIT_WORKFLOW.md): branch from `dev`, one
feature per branch, PR to `dev`, squash merge.

---

## 5. Risks & Open Items

| Item | Impact | Mitigation |
|------|--------|------------|
| `PortfolioAccount` has no balance field | Portfolio value cannot be precisely computed | Use position quantity × latest transaction price as an estimate; document; or keep sample value + label "sample" |
| No performance time-series endpoint | PerformanceTab cannot show real data | Keep placeholder bars clearly labeled; note as future backend work |
| Activity feed has no unified activity table | Must merge 3 sources client-side | Implement `useRecentActivity` merge with `created_at` sort; document |
| Multiple workspaces | Dashboard scoping | Default to first workspace; allow `?workspace=` param override |

---

## 6. Outcome

After GUI-E1 and GUI-E2:

- Every page has one-click theme/language/settings access
- Dashboard reflects the user's real portfolio and research activity
- Loading / empty / error states are complete per AGENTS.md §9.2
- The GUI milestone doc statuses are updated to reflect M0–M5 completion

---

*End of document*