# Portfolio Module — Frontend Integration Guide

> **Status:** Phase 3 (frontend) — ✅ Implementation in progress.
> **Audience:** Main dev developer building the portfolio UI.
> **All documentation is in English.**

---

## 1. GUI Slots

### 1.1 Sidebar Entry

The portfolio sidebar entry is added under the **analyze** functional view in
`src/components/layout/tools-config.tsx`. It navigates to `/portfolio` and uses
the `BarChart3` icon from lucide-react.

### 1.2 Route

- **Route:** `/portfolio`
- **Page component:** `src/pages/portfolio/PortfolioPage.tsx`
- **Layout:** Full-page layout inside `MainLayout`
- **Registration:** `src/app/router.tsx` — already registered

### 1.3 Portfolio Dashboard

The dashboard at `/portfolio` displays:

1. **Account cards** — summary of all financial accounts with balances
2. **Holdings table** — current positions with quantity, market value, gain/loss
3. **Activity feed** — recent transactions and cash movements
4. **Allocation chart** — donut/bar chart of asset allocation
5. **Valuation chart** — time series of account value over time
6. **Quick actions** — add account (future), import CSV (future), create snapshot

---

## 2. Page Structure (Phase 3 Implementation)

```
PortfolioPage (src/pages/portfolio/PortfolioPage.tsx)
└── PortfolioDashboard (src/features/portfolio/components/PortfolioDashboard.tsx)
    ├── WorkspaceSelector (dropdown to pick workspace)
    ├── AccountCards (horizontal scroll)
    │   ├── AccountCard (per account)
    │   │   ├── Account name, type, balance
    │   │   └── Holdings count, last valuation date
    │   └── NetWorthCard (total net worth)
    ├── HoldingsSection
    │   └── HoldingsTable
    │       ├── HoldingRow (symbol, name, qty, market value, gain/loss, weight)
    │       └── HoldingsSummaryRow (totals)
    ├── AllocationSection
    │   └── AllocationChart (donut chart via recharts)
    ├── ValuationChart
    │   └── TimeSeriesChart (account value over time via recharts)
    ├── ActivitySection
    │   └── ActivityList (recent activities from holdings context)
    └── QuickActions
        ├── CreateSnapshotButton
        └── RefreshButton
```

### 2.1 Component Files

| Component | File | Purpose |
|-----------|------|---------|
| `PortfolioPage` | `src/pages/portfolio/PortfolioPage.tsx` | Route page, wraps dashboard |
| `PortfolioDashboard` | `src/features/portfolio/components/PortfolioDashboard.tsx` | Main orchestrator |
| `AccountCards` | `src/features/portfolio/components/AccountCards.tsx` | Account summary cards + net worth |
| `HoldingsTable` | `src/features/portfolio/components/HoldingsTable.tsx` | Holdings data table |
| `AllocationChart` | `src/features/portfolio/components/AllocationChart.tsx` | Donut chart of allocation |
| `ValuationChart` | `src/features/portfolio/components/ValuationChart.tsx` | Time series line chart |
| `ActivityList` | `src/features/portfolio/components/ActivityList.tsx` | Recent activity feed |
| `QuickActions` | `src/features/portfolio/components/QuickActions.tsx` | Action buttons |
| `useFinancialData` | `src/features/portfolio/hooks/useFinancialData.ts` | TanStack Query hooks |

---

## 3. Service Architecture

### 3.1 Data Flow

```
React Component
    ↓
TanStack Query hook (useFinancialData.ts)
    ↓
desktopApi.financial.* (src/lib/desktop-api/financial.ts)
    ↓
invoke("command_name") → Tauri → Service → Repository → SQLite
```

### 3.2 Available Commands (Phase 2 — 18 commands)

These are the Tauri commands wired in `commands/financial.rs`:

| Command | Purpose | Used by |
|---------|---------|---------|
| `get_holdings` | Holdings for one account | HoldingsTable |
| `get_all_holdings` | Holdings for all accounts | AccountCards, HoldingsTable |
| `get_valuation_series` | Valuation time series | ValuationChart |
| `get_allocation` | Allocation breakdown | AllocationChart |
| `compute_net_worth` | Net worth snapshot | NetWorthCard |
| `compute_performance_summary` | XIRR/TWR metrics | (future) |
| `create_snapshot` | Take a snapshot | QuickActions |
| `list_snapshots` | List snapshots | (future) |

### 3.3 Missing Commands (Future Phase)

The following repository-level CRUD commands are NOT yet exposed as Tauri commands
and will be needed for full account/asset/activity management:

- `list_financial_accounts` — needed for AccountCards
- `create_financial_account` — needed for AddAccount
- `list_activities_by_account` — needed for ActivityList
- `create_activity` — needed for AddActivity
- `list_assets` / `create_asset` — needed for asset management

**Workaround:** The old placeholder portfolio commands (`desktopApi.portfolio.*`)
still provide `listPortfolioAccounts` which returns the old `PortfolioAccount` type.
Phase 3 uses this for account listing until the repository CRUD commands are wired.

---

## 4. TanStack Query Hooks

Defined in `src/features/portfolio/hooks/useFinancialData.ts`:

```typescript
// Query keys
const financialKeys = {
  all: ["financial"] as const,
  holdings: (accountId: string, asOfDate: string) =>
    [...financialKeys.all, "holdings", accountId, asOfDate] as const,
  allHoldings: (asOfDate: string) =>
    [...financialKeys.all, "allHoldings", asOfDate] as const,
  valuations: (accountId: string) =>
    [...financialKeys.all, "valuations", accountId] as const,
  allocation: (scope: string, scopeId: string | null, date: string) =>
    [...financialKeys.all, "allocation", scope, scopeId, date] as const,
  netWorth: (date: string, currency?: string) =>
    [...financialKeys.all, "netWorth", date, currency] as const,
  performance: (accountId: string, start: string, end: string) =>
    [...financialKeys.all, "performance", accountId, start, end] as const,
  snapshots: (accountId: string) =>
    [...financialKeys.all, "snapshots", accountId] as const,
};
```

---

## 5. State Management

### 5.1 TanStack Query

All server state (holdings, valuations, allocation, net worth) uses TanStack Query
with the `useFinancialData` hooks. Query key factories enable fine-grained
invalidation.

### 5.2 Local State

- `selectedAccountId` — which account is selected
- `asOfDate` — the current view date (defaults to today)
- `isCreatingSnapshot` — loading state for snapshot creation

### 5.3 Zustand

Not used for portfolio state. Keep cross-page state minimal.

---

## 6. Component Library

- **shadcn/ui** — `packages/ui/` for Button, Card, Dialog, Select, Table
- **recharts** — for donut chart (allocation) and line chart (valuation)
- **lucide-react** — for icons (BarChart3, Wallet, TrendingUp, etc.)
- **@tanstack/react-table** — (optional) for advanced table features

---

## 7. Existing Frontend Files

| File | Status | Purpose |
|------|--------|---------|
| `src/pages/portfolio/PortfolioPage.tsx` | ✅ Updated (Phase 3) | Route page wrapping dashboard |
| `src/features/portfolio/components/PortfolioDashboard.tsx` | ✅ Replaced (Phase 3) | Main orchestrator |
| `src/features/portfolio/components/AccountCards.tsx` | ✅ New (Phase 3) | Account summary cards |
| `src/features/portfolio/components/HoldingsTable.tsx` | ✅ New (Phase 3) | Holdings data table |
| `src/features/portfolio/components/AllocationChart.tsx` | ✅ New (Phase 3) | Donut chart |
| `src/features/portfolio/components/ValuationChart.tsx` | ✅ New (Phase 3) | Line chart |
| `src/features/portfolio/components/ActivityList.tsx` | ✅ New (Phase 3) | Activity feed |
| `src/features/portfolio/components/QuickActions.tsx` | ✅ New (Phase 3) | Action buttons |
| `src/features/portfolio/hooks/useFinancialData.ts` | ✅ New (Phase 3) | TanStack Query hooks |
| `src/features/portfolio/hooks/usePortfolio.ts` | ⏳ Legacy (kept) | Old hooks for legacy portfolio commands |
| `src/lib/desktop-api/financial.ts` | ✅ New (Phase 3) | IPC client for Phase 2 commands |
| `src/lib/desktop-api/portfolio.ts` | ⏳ Legacy (kept) | Old IPC client (placeholder) |
| `src/types/financial.ts` | ✅ New (Phase 3) | TypeScript domain types |
| `src/components/layout/tools-config.tsx` | ✅ Updated (Phase 3) | Portfolio sidebar entry |
| `src/lib/desktop-api/index.ts` | ✅ Updated (Phase 3) | Registered financial API |

---

## 8. i18n Keys

### 8.1 Existing Keys

The i18n catalogs at `src/lib/i18n/catalogs/{en,zh-CN}/portfolio.ts` already contain
keys for:
- Portfolio page header
- Workspace selection
- Account CRUD forms
- Holdings display
- Transaction import
- Allocation/concentration panels
- Theme exposure
- Alignment review

### 8.2 New Keys Needed (Phase 3)

New keys added for Phase 3 components:

| Key | English | Chinese |
|-----|---------|---------|
| `netWorth` | Net Worth | 净资产 |
| `totalValue` | Total Value | 总价值 |
| `totalHoldings` | Holdings | 持仓 |
| `allAccounts` | All Accounts | 所有账户 |
| `marketValue` | Market Value | 市值 |
| `gainLoss` | Gain/Loss | 盈亏 |
| `weight` | Weight | 权重 |
| `allocation` | Allocation | 资产配置 |
| `accountValue` | Account Value | 账户价值 |
| `recentActivity` | Recent Activity | 最近活动 |
| `quickActions` | Quick Actions | 快速操作 |
| `createSnapshot` | Create Snapshot | 创建快照 |
| `refresh` | Refresh | 刷新 |
| `snapshotCreated` | Snapshot created | 快照已创建 |
| `failedToCreateSnapshot` | Failed to create snapshot | 创建快照失败 |
| `noHoldings` | No holdings found | 未找到持仓 |
| `noValuationData` | No valuation data | 无估值数据 |
| `noAllocationData` | No allocation data | 无配置数据 |

---

## 9. Page Layout

```
┌──────────────────────────────────────────────────────────────┐
│  📊 Portfolio                                                │
│                                                              │
│  [Workspace: My Workspace ▼]  [Date: 2026-08-15]            │
│                                                              │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ Net Worth: $125,430.00                                   ││
│  └──────────────────────────────────────────────────────────┘│
│                                                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │ Broker   │ │ Cash     │ │ Credit   │ │ Crypto   │        │
│  │ $92,340  │ │ $25,000  │ │ -$2,100  │ │ $10,190  │        │
│  │ 12 holdings│ │ 1 holding│ │ 0 holdings│ │ 3 holdings│        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│                                                              │
│  ┌───────────────────────┬──────────────────────────────────┐│
│  │ Holdings (12)         │ Allocation (by category)         ││
│  │ ┌───┬──────┬────┬────┐│         ┌──────────┐            ││
│  │ │Sym│ Qty  │ MV │ G/L││    ┌────┤ Tech 45% │            ││
│  │ │AAPL│ 100 │22k │+3k ││    │    │           │            ││
│  │ │NVDA│ 50  │18k │+5k ││    │    └──────────┘            ││
│  │ │TSLA│ 200 │14k │-2k ││ ┌──┴──┐                         ││
│  │ └───┴──────┴────┴────┘│ │Fin 25%                        ││
│  └───────────────────────┘ └─────┘                         ││
│                                                              ││
│  ┌──────────────────────────────────────────────────────────┐│
│  │ Account Value Over Time                                  ││
│  │         ╱╲                                               ││
│  │   ╱╲  ╱  ╲    ╱╲                                        ││
│  │  ╱  ╲╱    ╲  ╱  ╲                                       ││
│  │ ╱          ╲╱    ╲                                      ││
│  │ May  Jun  Jul  Aug                                       ││
│  └──────────────────────────────────────────────────────────┘│
│                                                              │
│  ┌───────────────────────┬──────────────────────────────────┐│
│  │ Recent Activity       │ Quick Actions                    ││
│  │ • 08-13 BUY AAPL x10  │ 📸 Create Snapshot               ││
│  │ • 08-12 DIV $45.00   │ 🔄 Refresh                       ││
│  │ • 08-11 DEP $5,000   │                                  ││
│  └───────────────────────┴──────────────────────────────────┘│
└──────────────────────────────────────────────────────────────┘
```

---

## 10. Desktop API Client

The file `src/lib/desktop-api/financial.ts` wraps all 18 Phase 2 Tauri commands
with camelCase function names. See `API_SPEC.md` Section 9 for the full command
reference.

The existing `src/lib/desktop-api/portfolio.ts` is kept for backward compatibility
with the old placeholder commands (used for account listing until repository CRUD
commands are wired).

---

## 11. Future Work

- **Phase 3.5:** Wire repository-level CRUD Tauri commands (create_platform,
  list_financial_accounts, create_asset, list_activities, etc.)
- **Phase 4:** Thesis ↔ holding linkage
- **Phase 5:** Broker sync, market data integration, CSV import via new commands