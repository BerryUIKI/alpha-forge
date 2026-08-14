# Portfolio Module — Frontend Integration Guide

> **Status:** Phase 3 (frontend) — not yet started.
> **Audience:** Main dev developer building the portfolio UI.
> **All documentation is in English.**

---

## 1. GUI Slots Required

The following GUI locations need to be reserved in the main application for the
portfolio module. These are slots that the portfolio UI will occupy once Phase 3
begins.

### 1.1 Sidebar Entry

**Location:** Left sidebar, bottom section (user area)
**Reference:** `src/components/layout/LeftSidebar/UserOperations.tsx`

A permanent **Portfolio (📊)** entry must be added to the sidebar. When clicked,
it navigates to the portfolio dashboard route.

```typescript
// Example sidebar entry (adjust for existing component structure)
{
  id: "portfolio",
  label: "portfolio",      // i18n key
  icon: "📊",              // or a proper icon component
  route: "/portfolio",
  placement: "bottom",     // User area, below workspace/tools
}
```

### 1.2 Route

**Route:** `/portfolio`
**Page component:** `src/pages/portfolio/PortfolioPage.tsx` (exists as placeholder)
**Layout:** Full-page layout. The portfolio page should be a route registered in
the router configuration alongside existing routes like `/workspace/:id/research`.

### 1.3 Portfolio Dashboard

The dashboard at `/portfolio` should display:

1. **Account cards** — summary of all financial accounts with balances
2. **Holdings table** — current positions with quantity, market value, gain/loss
3. **Activity feed** — recent transactions and cash movements
4. **Quick actions** — add account, import CSV, create snapshot

---

## 2. Page Structure (Phase 3 Plan)

The planned portfolio UI component tree:

```
PortfolioPage
└── PortfolioDashboard
    ├── AccountSummary
    │   ├── AccountCard (per account)
    │   │   ├── AccountBalance
    │   │   └── AccountActions (edit, archive, import)
    │   └── AddAccountButton
    ├── HoldingsSection
    │   ├── HoldingsTable
    │   │   ├── HoldingRow
    │   │   │   ├── AssetInfo (logo, symbol, name)
    │   │   │   ├── QuantityDisplay
    │   │   │   ├── MarketValueCell
    │   │   │   └── GainLossCell
    │   │   └── HoldingsSummaryRow
    │   └── HoldingsFilters
    ├── ActivitySection
    │   ├── ActivityList
    │   │   └── ActivityItem
    │   └── ActivityFilters
    ├── AllocationSection
    │   ├── AllocationChart (pie/donut chart)
    │   └── AllocationTable
    ├── ValuationChart
    │   └── TimeSeriesChart (account value over time)
    └── QuickActions
        ├── ImportCsvButton
        ├── CreateSnapshotButton
        └── AddActivityButton
```

---

## 3. State Management

### 3.1 TanStack Query Keys

Use TanStack Query for all server state. Suggested query key pattern:

```typescript
// Query keys for portfolio data
const portfolioKeys = {
  all: ["portfolio"] as const,
  platforms: () => [...portfolioKeys.all, "platforms"] as const,
  accounts: (workspaceId?: string) =>
    [...portfolioKeys.all, "accounts", workspaceId] as const,
  account: (id: string) => [...portfolioKeys.all, "account", id] as const,
  assets: () => [...portfolioKeys.all, "assets"] as const,
  asset: (id: string) => [...portfolioKeys.all, "asset", id] as const,
  activities: (accountId: string) =>
    [...portfolioKeys.all, "activities", accountId] as const,
  lots: (accountId: string) =>
    [...portfolioKeys.all, "lots", accountId] as const,
  valuations: (accountId: string) =>
    [...portfolioKeys.all, "valuations", accountId] as const,
  snapshots: (accountId: string) =>
    [...portfolioKeys.all, "snapshots", accountId] as const,
  taxonomies: () => [...portfolioKeys.all, "taxonomies"] as const,
  allocationTargets: () => [...portfolioKeys.all, "allocation-targets"] as const,
};
```

### 3.2 Local State

Keep local state for:
- Form inputs (create account, add activity)
- UI state (selected account, expanded rows)
- Filter state (date range, activity type filter)

### 3.3 Zustand (Minimal)

Use only if cross-page state is needed (e.g., selected account ID persists when
navigating between portfolio sub-pages). Otherwise, keep state in TanStack Query
and local component state.

---

## 4. Component Library

- **Use existing shadcn/ui components** from `packages/ui/` — do not import
  `@wealthfolio/ui` (the Wealthfolio frontend package).
- Chart components: use `recharts` (already in the repo) or build from
  `packages/ui/chart` shadcn components.
- Form components: use existing `Input`, `Select`, `Button`, `Dialog` from
  shadcn/ui.
- Table: use `@tanstack/react-table` (already in the repo) or shadcn `Table`.

---

## 5. Existing Frontend Files

The following files already exist and must be extended (not replaced):

| File | Status | Purpose |
|------|--------|---------|
| `src/pages/portfolio/PortfolioPage.tsx` | Placeholder (7 lines) | Route page |
| `src/features/portfolio/components/PortfolioDashboard.tsx` | Placeholder | Main dashboard |
| `src/features/portfolio/components/AccountManagement/` | Exists | CreateAccountForm, AccountList, PositionPanel |
| `src/features/portfolio/components/Analysis/` | Exists | AllocationPanel, ConcentrationPanel, etc. |
| `src/features/portfolio/hooks/usePortfolio.ts` | Exists | TanStack Query hooks |
| `src/lib/desktop-api/portfolio.ts` | Exists | IPC client (placeholder commands) |
| `src/lib/desktop-api/portfolio.test.ts` | Exists | Test file |
| `src/components/layout/LeftSidebar/` | Exists | Sidebar with navigation entries |

---

## 6. i18n Keys

Existing i18n catalogs for the portfolio module:

- `src/lib/i18n/catalogs/en/portfolio.ts`
- `src/lib/i18n/catalogs/zh-CN/portfolio.ts`

New keys will be needed for:
- Financial account labels (account types, tracking modes)
- Activity type labels (buy, sell, dividend, etc.)
- Asset labels (kind, instrument type)
- Valuation labels
- Allocation labels
- Lot/disposal labels

---

## 7. Page Layout Mockup

The portfolio dashboard should follow this layout:

```
┌─────────────────────────────────────────────────────┐
│  📊 Portfolio                                       │
│                                                     │
│  ┌─────────────────────────────────────────────────┐│
│  │ Account Cards (horizontal scroll)               ││
│  │ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐            ││
│  │ │Broker│ │ Cash │ │Credit│ │Crypto│  [+ Add]   ││
│  │ │$45k  │ │$12k  │ │ -$2k │ │$8k   │            ││
│  │ └──────┘ └──────┘ └──────┘ └──────┘            ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  ┌───────────────┬─────────────────────────────────┐│
│  │ Holdings      │ Quick Actions                   ││
│  │ ┌────────────┐│ ┌─────────────────────────────┐ ││
│  │ │ Symbol  Qty ││ │ 📥 Import CSV              │ ││
│  │ │ AAPL   100  ││ │ 📸 Take Snapshot           │ ││
│  │ │ NVDA   50   ││ │ ➕ Add Activity            │ ││
│  │ │ TSLA   200  ││ │ 🏷️ Manage Taxonomies      │ ││
│  │ └────────────┘│ └─────────────────────────────┘ ││
│  └───────────────┴─────────────────────────────────┘│
│                                                     │
│  ┌─────────────────────────────────────────────────┐│
│  │ Account Value (chart)                           ││
│  │   ╱╲      ╱╲                                    ││
│  │  ╱  ╲    ╱  ╲                                   ││
│  │ ╱    ╲  ╱    ╲                                  ││
│  │╱      ╲╱      ╲                                 ││
│  │ Aug  Sep  Oct  Nov                               ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  ┌─────────────────────────────────────────────────┐│
│  │ Recent Activity                                  ││
│  │ • 2026-08-13  BUY    AAPL x10 @ $220           ││
│  │ • 2026-08-12  DIV    $45.00                     ││
│  │ • 2026-08-11  DEP    $5,000.00                  ││
│  └─────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────┘
```

---

## 8. Required Desktop API Client

The file `src/lib/desktop-api/financial.ts` must be created with all the
Tauri command wrappers listed in `API_SPEC.md` Section 2. Follow the existing
pattern:

```typescript
import { invoke } from "@tauri-apps/api/core";

export function createFinancialAccount(input: CreateAccountInput): Promise<FinancialAccount> {
  return invoke("create_financial_account", { input });
}
```

The existing `src/lib/desktop-api/portfolio.ts` file contains the old placeholder
commands and will be replaced/extended once the new financial commands are registered.