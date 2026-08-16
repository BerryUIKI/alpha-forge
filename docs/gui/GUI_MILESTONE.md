# AlphaForge GUI Redesign — Milestone Document

> **Document:** `docs/gui/GUI_MILESTONE.md`
> **Status:** Draft — Implementation Plan
> **Version:** 1.0
> **Date:** 2026-08-16

---

## 1. Overview

The AlphaForge GUI redesign modernizes the desktop application interface by adopting a **clean, navigation-focused layout** inspired by Wealthfolio's design language while preserving AlphaForge's unique investment research identity.

### Design Principles

1. **Clarity over density** — Information hierarchy guides the user, not visual clutter
2. **Consistent rhythm** — Reusable card primitives, uniform spacing, predictable layouts
3. **Glass-morphism aesthetics** — Subtle transparency and blur for depth without distraction
4. **Keyboard-first navigation** — All panels collapsible, global search, shortcuts
5. **Dark-first** — Optimized for dark theme (investment research is a nighttime activity)
6. **Progressive disclosure** — Agent panel collapses, sidebar collapses, tabs organize content

### Core Product Loop Alignment

Every GUI change is measured against the product loop:

```
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

The new layout makes each step visible and accessible:
- **Dashboard** surfaces key metrics and recent activity at a glance
- **Research** takes center stage with dedicated navigation
- **Theses/Journal** has its own nav item for easy access
- **Portfolio** and **Options** are one click away
- **Agent panel** provides AI-assisted insights throughout

---

## 2. Layout Architecture

### High-Level Structure

```
┌────────────────┬──────────────────────────────────────────┬────────────────┐
│  Left Sidebar  │           Main Content Area               │  Agent Panel   │
│  (220px→64px)  │                                           │  (320px→0px)   │
│                │  ┌────────────────────────────────────┐   │                │
│  Logo/Brand    │  │  Top Bar                           │   │  Agent Header  │
│                │  │  Breadcrumb │ Search │ Actions     │   │                │
│  ────────────  │  └────────────────────────────────────┘   │  Messages      │
│  Workspace     │  ┌────────────────────────────────────┐   │  (scrollable)  │
│   · Dashboard  │  │                                    │   │                │
│   · Research   │  │  Page Content (React Router Outlet) │   │  Agent Input   │
│   · Theses     │  │                                    │   │                │
│   · Portfolio  │  │  Dashboard tabs:                    │   │                │
│   · Knowledge  │  │  [Overview] [Performance] [Activity]│   │                │
│   · Journal    │  │                                    │   │                │
│                │  │  ┌──────┬──────┬──────┐            │   │                │
│  ────────────  │  │  │Stat 1│Stat 2│Stat 3│            │   │                │
│  Tools         │  │  └──────┴──────┴──────┘            │   │                │
│   · Options    │  │  ┌────────────────┬─────────┐      │   │                │
│   · Artifacts  │  │  │ Top Holdings   │Activity │      │   │                │
│                │  │  └────────────────┴─────────┘      │   │                │
│  ────────────  │  │  ┌──────────────────────────────┐  │   │                │
│   · Settings   │  │  │ Portfolio Performance Chart  │  │   │                │
│                │  │  └──────────────────────────────┘  │   │                │
│                │  └────────────────────────────────────┘   │                │
│                │  ┌────────────────────────────────────┐   │                │
│                │  │  Status Bar                        │   │                │
│                │  └────────────────────────────────────┘   │                │
└────────────────┴──────────────────────────────────────────┴────────────────┘
```

### Component Tree

```
App
└── Providers
    └── RouterProvider
        └── MainLayout
            ├── LeftSidebar
            │   ├── Logo/Brand
            │   ├── NavGroup("Workspace")
            │   │   ├── NavItem("Dashboard")
            │   │   ├── NavItem("Research")
            │   │   ├── NavItem("Theses")
            │   │   ├── NavItem("Portfolio")
            │   │   ├── NavItem("Knowledge")
            │   │   └── NavItem("Journal")
            │   ├── NavGroup("Tools")
            │   │   ├── NavItem("Options")
            │   │   └── NavItem("Artifacts")
            │   └── NavGroup("Account")
            │       └── NavItem("Settings")
            ├── MainContent
            │   ├── TopBar
            │   │   ├── Breadcrumb
            │   │   ├── SearchBar
            │   │   └── ActionButtons
            │   ├── Outlet (page content)
            │   │   ├── TodayPage (Dashboard)
            │   │   │   ├── TabBar (Overview | Performance | Activity)
            │   │   │   ├── OverviewTab
            │   │   │   │   ├── StatCard × 3
            │   │   │   │   ├── HoldingsList
            │   │   │   │   └── ActivityFeed
            │   │   │   ├── PerformanceTab
            │   │   │   │   └── DashboardCard > MiniChart
            │   │   │   └── ActivityTab
            │   │   │       └── ActivityFeed (full)
            │   │   ├── ResearchPage
            │   │   ├── JournalPage (Theses)
            │   │   ├── PortfolioPage
            │   │   ├── OptionsPage
            │   │   ├── ArtifactsPage
            │   │   └── SettingsPage
            │   └── StatusBar
            │       ├── SystemStatus
            │       ├── SyncTime
            │       └── Version
            └── RightSidebar (Agent Panel)
                ├── AgentHeader
                ├── AgentMessageList
                │   ├── AgentMessage (Research)
                │   ├── AgentMessage (Alert)
                │   └── AgentMessage (Thesis)
                └── AgentInput
```

---

## 3. Milestones

| ID | Name | Status | Dependencies | Est. Effort |
|----|------|--------|-------------|-------------|
| GUI-M0 | Core Layout Shell | 🔲 Planned | None | 1 sprint |
| GUI-M1 | Navigation & Routing | 🔲 Planned | GUI-M0 | 1 sprint |
| GUI-M2 | UI Component System | 🔲 Planned | None (parallel with M1) | 1 sprint |
| GUI-M3 | Dashboard Page | 🔲 Planned | GUI-M1, GUI-M2 | 1 sprint |
| GUI-M4 | Agent Panel Refinement | 🔲 Planned | GUI-M0 | 1 sprint |
| GUI-M5 | Polish & Cleanup | 🔲 Planned | GUI-M3, GUI-M4 | 1 sprint |

### Dependency Graph

```
GUI-M0 (Core Layout Shell)
  ├── GUI-M1 (Navigation & Routing) — depends on M0 nav infrastructure
  │   └── GUI-M3 (Dashboard Page) — depends on M1 + M2
  ├── GUI-M2 (UI Component System) — independent, parallel with M1
  └── GUI-M4 (Agent Panel) — depends on M0 sidebar structure
      └── GUI-M5 (Polish & Cleanup) — depends on M3 + M4
```

---

## 4. Navigation Data Model

```typescript
interface NavItem {
  id: string;
  label: string;         // i18n key
  icon: LucideIcon;      // Lucide icon component
  route: string;         // React Router path
  keywords?: string[];   // For search/command palette
}

interface NavGroup {
  id: string;
  label: string;         // i18n section label
  items: NavItem[];
}
```

### Navigation Groups

| Group | ID | Label (EN) | Items |
|-------|-----|-----------|-------|
| Workspace | `workspace` | `nav.workspace` | Dashboard (`/`), Research (`/research`), Theses (`/theses`), Portfolio (`/portfolio`), Knowledge (`/knowledge`), Journal (`/journal`) |
| Tools | `tools` | `nav.tools` | Options (`/options`), Artifacts (`/artifacts`) |
| Account | `account` | `nav.account` | Settings (`/settings`) |

---

## 5. Theme & Styling

### Current Theme (Kept)

The existing Tailwind CSS v4 theme with HSL-based CSS custom properties is preserved:

- `--background`: `222.2 84% 4.9%` (dark) / `0 0% 100%` (light)
- `--card`: `222.2 84% 4.9%` (dark) / `0 0% 100%` (light)
- `--border`: `217.2 32.6% 17.5%` (dark) / `214.3 31.8% 91.4%` (light)
- `--primary`: `210 40% 98%` (dark) / `222.2 47.4% 11.2%` (light)
- `--accent`: `217.2 32.6% 17.5%` (dark) / `210 40% 96.1%` (light)

### New CSS Variables (GUI-M2)

```css
@theme {
  --color-success: hsl(var(--success));
  --color-warning: hsl(var(--warning));
  /* Add sidebar width variables */
  --sidebar-width: 220px;
  --sidebar-collapsed-width: 64px;
  --agent-panel-width: 320px;
  --topbar-height: 52px;
  --statusbar-height: 28px;
}
```

### Glass-Morphism Pattern

```css
.glass-card {
  background-color: hsl(var(--card) / 0.7);
  backdrop-filter: blur(12px);
  border: 1px solid hsl(var(--border) / 0.4);
  border-radius: 0.75rem; /* 12px */
}
```

---

## 6. Component Specifications

### GUI-M0 Components

| Component | File | Key Props | Description |
|-----------|------|-----------|-------------|
| `MainLayout` | `components/layout/MainLayout.tsx` | — | Three-zone layout orchestrator |
| `LeftSidebar` | `components/layout/LeftSidebar/LeftSidebar.tsx` | — | Collapsible, resizable nav sidebar |
| `NavItem` | `components/layout/LeftSidebar/NavItem.tsx` | `{ item: NavItem; collapsed: boolean }` | Single navigation item |
| `NavGroup` | `components/layout/LeftSidebar/NavGroup.tsx` | `{ label: string; children; collapsed: boolean }` | Named navigation section |
| `TopBar` | `components/layout/TopBar/TopBar.tsx` | `{ isRightSidebarExpanded; onToggleRightSidebar }` | App header bar |
| `MainContent` | `components/layout/MainContent/MainContent.tsx` | `{ children; isRightSidebarExpanded; onToggleRightSidebar }` | Center content stack |
| `StatusBar` | `components/layout/MainContent/StatusBar.tsx` | — | Bottom status bar |
| `RightSidebar` | `components/layout/RightSidebar/RightSidebar.tsx` | `{ state; onStateChange }` | Agent panel sidebar |

### GUI-M2 Components

| Component | File | Key Props | Description |
|-----------|------|-----------|-------------|
| `DashboardCard` | `components/ui/DashboardCard.tsx` | `{ title, subtitle?, action?, meta?, padded?, elevated?, children? }` | Reusable glass-morphism card |
| `StatCard` | `components/ui/StatCard.tsx` | `{ label, value, change?, changeDirection?, icon? }` | Metric display card |
| `TabBar` | `components/ui/TabBar.tsx` | `{ tabs, activeTab, onTabChange }` | Segmented control tab bar |
| `SearchBar` | `components/ui/SearchBar.tsx` | `{ placeholder?, value?, onChange? }` | Styled search input |

---

## 7. Migration from Old Layout

### What Stays

- `RightSidebar` — structure preserved, visual refinement only
- `AgentPanel` — functionality preserved, messaging UI added in GUI-M4
- `UserOperations` — theme toggle and language selector moved to TopBar or Settings
- `useSidebarState` — layout state persistence hook (kept)
- `useResize` — drag-to-resize hook (kept)
- `useSidebarShortcuts` — keyboard shortcuts hook (kept)
- All page components — remain unchanged, only the layout shell changes

### What's Removed (GUI-M5)

| File | Reason |
|------|--------|
| `FunctionalViewSelector.tsx` | Replaced by simplified NavItem-based sidebar |
| `ToolsList.tsx` | Replaced by NavGroup(Tools) |
| `ScrollableList.tsx` | Not used in current layout |
| `WorkspaceSelector.tsx` | Not used in current layout |
| `tools-config.tsx` | Replaced by navigation config |
| `OperationBar.tsx` | Replaced by TopBar |
| `WorkspaceDropdown.tsx` | Functionality moved to workspace selector in TopBar |
| `navigation/Sidebar.tsx` | Old sidebar code, fully replaced |

### What's Refactored

| File | Change |
|------|--------|
| `types.ts` | Remove `FunctionalView`, `ToolItem`, `VIEW_ROUTE_MAP`, etc. |
| `hooks/layout/index.ts` | Fix broken import paths for `useNetworkStatus` and `useAgentStatus` |
| `router.tsx` | Add `/theses` route, add `/knowledge` route |
| `navigation.ts` (i18n) | Add new keys: `theses`, `knowledge`, `options`, `dashboard` |

---

## 8. Verification Checklist

### Per-Milestone Verification

- [ ] **GUI-M0**: `pnpm lint` passes, `pnpm typecheck` passes, `pnpm test` passes
- [ ] **GUI-M1**: `pnpm typecheck` passes, `pnpm test` passes, all routes accessible
- [ ] **GUI-M2**: `pnpm lint` passes, `pnpm typecheck` passes, `pnpm test` passes
- [ ] **GUI-M3**: `pnpm tauri dev` runs, dashboard renders correctly
- [ ] **GUI-M4**: `pnpm test` passes, agent panel renders messages
- [ ] **GUI-M5**: `pnpm test --coverage` meets threshold, `cargo test` passes

### Full Integration Verification

- [ ] All 8 pages render correctly (Today, Research, Theses, Portfolio, Knowledge, Journal, Options, Artifacts, Settings)
- [ ] Left sidebar collapses and expands with smooth animation
- [ ] Left sidebar width is resizable within [180px, 320px]
- [ ] Right sidebar (Agent panel) collapses and expands
- [ ] TopBar breadcrumb shows correct current page
- [ ] Search input has proper focus styling
- [ ] StatusBar shows system status, sync time, and version
- [ ] Keyboard shortcuts work: Ctrl+1 (left toggle), Ctrl+2 (right toggle)
- [ ] Theme toggle works (light/dark/system)
- [ ] i18n switches between English and Chinese
- [ ] Artifact window remains isolated (no layout change affects it)
- [ ] All existing tests pass after migration
- [ ] No console errors or warnings

---

## 9. Future Considerations

### Post-MVP GUI Enhancements

- **Resizable content panels** — Allow users to resize the holdings list / activity feed split
- **Customizable dashboard** — Users can choose which stat cards and widgets appear
- **Layout presets** — Save/load different layout configurations for different workflows
- **Command palette** — Full Cmd+K search across everything (tickers, theses, pages, settings)
- **Floating navigation bar** — Alternative to sidebar for narrow windows (from Wealthfolio)
- **Mobile/tablet support** — Responsive layout for smaller screens
- **Multi-window workspace** — Separate research and portfolio into independent windows
- **Focus mode** — Hide all panels, full-screen page content

---

*End of document*