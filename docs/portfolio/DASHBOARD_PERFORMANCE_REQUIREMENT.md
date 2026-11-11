# Dashboard PerformanceTab Financial API Requirement

> **Document:** `docs/portfolio/DASHBOARD_PERFORMANCE_REQUIREMENT.md`
> **Status:** ✅ Implemented; retained as historical acceptance criteria
> **Date:** 2026-08-17
> **Resolved:** Present on `dev` as of the 2026-09-20 baseline review
> **Requested by:** GUI team (main `dev` branch)
> **Target team:** Portfolio integration team
> **Priority:** High

---

## 1. Background

The GUI redesign introduced a tabbed dashboard in
`apps/desktop/src/pages/today/TodayPage.tsx`. The Performance tab now consumes
the portfolio performance hook and renders real workspace-level portfolio data
with loading, error, empty, period-selection, and summary behavior. The
remaining sections preserve the requirement that guided that implementation.

The integrated Portfolio work supplied the required backend capability:

- **Rust commands** (`apps/desktop/src-tauri/src/commands/financial.rs`):
  - `get_performance_time_series(account_id)` → `Vec<PerformancePoint>`
  - `compute_performance_summary(account_id, start_date, end_date)` → `PerformanceSummary`
  - `get_valuation_series(account_id)` → `Vec<DailyAccountValuation>`
  - `calculate_all_valuations(date)` → `Vec<DailyAccountValuation>`
- **TS API client** (`apps/desktop/src/lib/desktop-api/financial.ts`)
- **TanStack Query hooks** (`apps/desktop/src/features/portfolio/hooks/useFinancialData.ts`)

This coordination requirement was satisfied when the Portfolio integration and
dashboard wiring reached `dev`.

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

| #   | Requirement                        | Detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| --- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | **Workspace-level aggregation**    | The Performance tab must show the **whole workspace** portfolio value over time. Consolidation now happens in Rust via `compute_consolidated_performance(account_ids, start_date, end_date, base_currency?)`: it builds a portfolio-level base-currency series from each account's last known valuation, treats an account's first appearance mid-period as an external inflow, and computes return/XIRR/TWR from portfolio cash flows. The hook only shapes points for the chart. |
| R2  | **Time period selector**           | Keep 1W / 1M / 3M / 1Y selector. Filter the merged series by date range based on the selected period. If insufficient data, show a meaningful empty state ("No valuation data for this period").                                                                                                                                                                                                                                                                                   |
| R3  | **Loading / Error / Empty states** | Follow AGENTS.md §9.2: skeleton on load, retry on error, empty state when no accounts or no valuations exist.                                                                                                                                                                                                                                                                                                                                                                      |
| R4  | **Currency handling**              | Use `total_value_base` (base currency) fields to consolidate. All included accounts must share one base currency: the command accepts an optional base currency, otherwise uses the first known valuation currency, and reports `skipped_accounts` for accounts in a different currency. The chart labels use the currency returned by the backend; nothing is hardcoded to USD.                                                                                                   |
| R5  | **Performance summary chips**      | Above the chart, show `total_return_pct`, `xirr_pct`, `twr_pct` from the consolidated response. These fields are **percentage points** (`"5"` = 5%) and are formatted directly. When `data_quality` is `partial` or `insufficient_data`, the tab shows explicit coverage copy instead of treating missing accounts as zero.                                                                                                                                                        |
| R6  | **No duplicate fetching**          | Reuse `financialKeys` query-key factory from `useFinancialData.ts`. Do not duplicate query keys across feature areas.                                                                                                                                                                                                                                                                                                                                                              |
| R7  | **Gentle degradation**             | If the workspace has zero accounts, show the empty state (reuse `EmptyState`), not an error.                                                                                                                                                                                                                                                                                                                                                                                       |

### 2.3 Out of Scope

- A live market-data feed for dashboard benchmark quotes.
- The Portfolio page (`/portfolio`) itself — its own dashboard is being built
  on the portfolio branch and will use `useFinancialData` hooks directly.
- Replacing the standalone `PortfolioDashboard` stub components
  (`AccountManagement.tsx`, `Analysis.tsx`) — separate follow-up.

---

## 3. Implementation Guidance (for whoever picks this up)

### Suggested files on `dev` (post-merge)

| File                                                        | Action                                                                                                         |
| ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `apps/desktop/src/pages/today/hooks/useDashboardData.ts`    | **Extend** with `usePortfolioPerformance(workspaceId)` or add a **new hook file** `usePortfolioPerformance.ts` |
| `apps/desktop/src/pages/today/tabs/PerformanceTab.tsx`      | Rewrite to render a recharts `LineChart` from the hook data                                                    |
| `apps/desktop/src/pages/today/tabs/PerformanceTab.test.tsx` | **New test**: loading, empty (no accounts), error, data-renders states                                         |
| `docs/gui/GUI_E_ENHANCEMENTS.md`                            | Update the E2 "PerformanceTab placeholder" risk item to "resolved"                                             |

### Suggested hook signature

```ts
// apps/desktop/src/pages/today/hooks/usePortfolioPerformance.ts
export interface PortfolioPerformancePoint {
  date: string; // YYYY-MM-DD
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

| Command          | Expected                                                                                            |
| ---------------- | --------------------------------------------------------------------------------------------------- |
| `pnpm typecheck` | Pass, zero errors                                                                                   |
| `pnpm lint`      | Pass                                                                                                |
| `pnpm test`      | Pass — includes new PerformanceTab tests                                                            |
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

_End of requirement_
