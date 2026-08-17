# Dashboard PerformanceTab 接入 Financial API — 开发需求

> **Document:** `docs/portfolio/DASHBOARD_PERFORMANCE_REQUIREMENT.md`
> **Status:** 📋 Requirement — Ready for implementation on `feature/portfolio-integration`
> **Date:** 2026-08-17
> **Requested by:** GUI team (main `dev` branch)
> **Target team:** Portfolio integration team
> **Priority:** High

---

## 1. Background

The GUI redesign (merged to `dev` via PRs #105–#113) introduced a tabbed
dashboard in `apps/desktop/src/pages/today/TodayPage.tsx`. The **Performance
tab** (`PerformanceTab.tsx`) currently renders **placeholder bars** because the
main `dev` branch has no portfolio value history API.

The `feature/portfolio-integration` branch **already has** the required backend
capability (Phase 2/3 Wealthfolio port):

- **Rust commands** (`apps/desktop/src-tauri/src/commands/financial.rs`):
  - `get_performance_time_series(account_id)` → `Vec<PerformancePoint>`
  - `compute_performance_summary(account_id, start_date, end_date)` → `PerformanceSummary`
  - `get_valuation_series(account_id)` → `Vec<DailyAccountValuation>`
  - `calculate_all_valuations(date)` → `Vec<DailyAccountValuation>`
- **TS API client** (`apps/desktop/src/lib/desktop-api/financial.ts`)
- **TanStack Query hooks** (`apps/desktop/src/features/portfolio/hooks/useFinancialData.ts`)

This is a **coordination requirement**: once `feature/portfolio-integration`
merges into `dev`, the dashboard PerformanceTab should be wired to these APIs.

---

## 2. Requirement (Acceptance Criteria)

### 2.1 Scope: Dashboard PerformanceTab → Real Data

**Component:** `apps/desktop/src/pages/today/tabs/PerformanceTab.tsx` (on `dev`)

**Requirement:** Replace the placeholder bars with a real portfolio value chart
using the financial APIs from `feature/portfolio-integration`.

**Data flow:**

```
PerformanceTab (dashboard, /?tab=performance)
  └── usePortfolioPerformance(workspaceId)   [new hook, TODAY page]
        ├── listPortfolioAccounts(workspaceId)          (dev API, exists)
        ├── getPerformanceTimeSeries(accountId)         (portfolio API, per account)
        └── getAllHoldings / getValuationSeries         (fallback)
        → merge into a single workspace-level series
        → render line chart (recharts — already a dependency)
```

### 2.2 Specific Requirements

| # | Requirement | Detail |
|---|-------------|--------|
| R1 | **Workspace-level aggregation** | The Performance tab must show the **whole workspace** portfolio value over time, not a single account. Since the backend time-series API is per-account (`get_performance_time_series(account_id)`), a new **dashboard-level hook** must fetch all accounts for the workspace, fetch each account's series in parallel (`Promise.all`), and merge by date (sum `total_value_base` per date). |
| R2 | **Time period selector** | Keep 1W / 1M / 3M / 1Y selector. Filter the merged series by date range based on the selected period. If insufficient data, show a meaningful empty state ("No valuation data for this period"). |
| R3 | **Loading / Error / Empty states** | Follow AGENTS.md §9.2: skeleton on load, retry on error, empty state when no accounts or no valuations exist. |
| R4 | **Currency handling** | Use `total_value_base` (base currency) fields when available to consolidate mixed-currency accounts. Label the chart y-axis with the base currency. |
| R5 | **Performance summary chips** | Above the chart, show `total_return_pct`, `xirr_pct`, `twr_pct` from `compute_performance_summary` per account (or the workspace aggregate if a workspace-level summary becomes available). |
| R6 | **No duplicate fetching** | Reuse `financialKeys` query-key factory from `useFinancialData.ts`. Do not duplicate query keys across feature areas. |
| R7 | **Gentle degradation** | If the workspace has zero accounts, show the empty state (reuse `EmptyState`), not an error. |

### 2.3 Out of Scope

- Backend changes on the portfolio branch — the API already exists.
- The Portfolio page (`/portfolio`) itself — its own dashboard is being built
  on the portfolio branch and will use `useFinancialData` hooks directly.
- Replacing the standalone `PortfolioDashboard` stub components
  (`AccountManagement.tsx`, `Analysis.tsx`) — separate follow-up.

---

## 3. Implementation Guidance (for whoever picks this up)

### Suggested files on `dev` (post-merge)

| File | Action |
|------|--------|
| `apps/desktop/src/pages/today/hooks/useDashboardData.ts` | **Extend** with `usePortfolioPerformance(workspaceId)` or add a **new hook file** `usePortfolioPerformance.ts` |
| `apps/desktop/src/pages/today/tabs/PerformanceTab.tsx` | Rewrite to render a recharts `LineChart` from the hook data |
| `apps/desktop/src/pages/today/tabs/PerformanceTab.test.tsx` | **New test**: loading, empty (no accounts), error, data-renders states |
| `docs/gui/GUI_E_ENHANCEMENTS.md` | Update the E2 "PerformanceTab placeholder" risk item to "resolved" |

### Suggested hook signature

```ts
// apps/desktop/src/pages/today/hooks/usePortfolioPerformance.ts
export interface PortfolioPerformancePoint {
  date: string;            // YYYY-MM-DD
  total_value_base: number;
}

export function usePortfolioPerformance(
  workspaceId: string,
  period: "1W" | "1M" | "3M" | "1Y",
): UseQueryResult<PortfolioPerformancePoint[]> {
  // 1. listPortfolioAccounts(workspaceId)
  // 2. Promise.all(accounts.map(a => desktopApi.financial.getPerformanceTimeSeries(a.id)))
  // 3. merge by date, sum total_value_base
  // 4. filter by period window
}
```

### Reuse checklist

- `desktopApi.financial.getPerformanceTimeSeries(accountId)` — from portfolio branch
- `financialKeys.performance(accountId, start, end)` — from `useFinancialData.ts`
- `desktopApi.portfolio.listPortfolioAccounts(workspaceId)` — on dev
- `useWorkspaces()` / `useActiveWorkspaceId()` — on dev (`pages/today/hooks/useDashboardData.ts`)
- `EmptyState`, `ErrorState`, `LoadingSpinner` from `@/components/common`
- `recharts` — already a dependency (used by portfolio branch components)

---

## 4. Verification

| Command | Expected |
|---------|----------|
| `pnpm typecheck` | Pass, zero errors |
| `pnpm lint` | Pass |
| `pnpm test` | Pass — includes new PerformanceTab tests |
| `pnpm tauri dev` | Dashboard → Performance tab renders real chart when account+valuations exist; empty state otherwise |

---

## 5. Dependencies / Ordering

1. **`feature/portfolio-integration` must merge to `dev` first** — it provides
   the Rust commands, migrations (0015–0021), repositories, services, TS types,
   `financial.ts` client, and `useFinancialData` hooks.
2. Then the dashboard wiring PR (this requirement) lands on `dev`.

**Blocking relationship:** the GUI team cannot complete R1-R7 until the
portfolio branch merges. The portfolio team should treat this document as the
official handoff requirement.

---

## 6. Contact / Handoff

- **Requesting team:** GUI (main `dev` branch)
- **Implementing team:** Portfolio integration (`feature/portfolio-integration`)
- **Point of contact:** Berry (project owner)
- **Deliverable:** A PR on `dev` titled
  `feat(gui): wire dashboard performance tab to financial API` referencing this
  document.

---

*End of requirement*