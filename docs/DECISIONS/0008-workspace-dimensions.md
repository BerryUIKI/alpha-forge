# ADR-0008: Workspace Dimensions — Global Portfolio, Workspace-Scoped Research

## Status

Proposed — records the agreed direction (Product review, 2026-08-19). Implementation follows once this document is approved and the currently open feature PRs are merged.

## Context

### Current state (verified 2026-08-19)

- **Workspaces are plain name-only containers.** `workspaces` has only `{ id, name, created_at, updated_at }` — no type, no default marker (`apps/desktop/src-tauri/migrations/0001_initial.sql`).
- **Everything research-side is workspace-scoped** with `workspace_id NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE`: research projects/documents/reports (0007), theses (0006), knowledge entities, artifacts, agent tasks, and all option tables (0014).
- **Portfolio is ambiguous today.** The placeholder `portfolio_accounts`/`positions`/`transactions` tables gained a `workspace_id` index (0010), while the canonical `accounts` table defines `workspace_id` as **nullable** "to scope financial accounts to a research workspace" (`apps/desktop/src-tauri/migrations/0015_financial_platforms_accounts.sql`). `DATA_MODEL.md` documents `accounts` as "scoped to an optional workspace" and records that the placeholder tables are "retired once the portfolio UI lands (Phase 3)".
- **A research↔portfolio link already exists:** `portfolio_theme_links(workspace_id, symbol, entity_id)` (`apps/desktop/src-tauri/migrations/0011_portfolio_theme_links.sql`) binds a workspace to portfolio symbols and knowledge entities. `investment_theses.portfolio_holding_id` provides a second, optional thesis→holding reference.
- **There is no global active-workspace state.** Every page owns a local selector defaulting to the first workspace (Portfolio, Artifacts, Thesis, Options). The Research page is the only page that writes `?workspace=` to the URL. The Today/Dashboard page and the global search resolve "URL param, else first workspace" (`apps/desktop/src/pages/today/hooks/useDashboardData.ts`).

### The core tension

Portfolio and research have different cardinality:

- **Research is naturally per-theme**: one topic (AI infrastructure, semiconductors, …) is one workspace.
- **Portfolio is naturally per-investor/per-strategy and few**: the same brokerage account usually serves many research themes.

Forcing "portfolio ∈ workspace" (one workspace carries one portfolio) either bundles unrelated themes into one workspace or prevents an account from being shared across themes. The resolution is a dimension split rather than a workspace-type split.

## Decision

Portfolio and research are **two dimensions**, not two kinds of workspace.

### 1. Portfolio is a global dimension

Accounts, holdings, and valuation are not isolated by research workspace. The Portfolio page shows **all accounts regardless of the active workspace**.

- `accounts.workspace_id`: `NULL` means **global**. A non-null value is an **ownership/provenance marker** ("created by this workspace"), not a data-isolation boundary. The Portfolio page ignores it for filtering.
- The placeholder `portfolio_accounts`/`positions` UI is **retired** in favor of the canonical `accounts` model (already the documented plan in `DATA_MODEL.md`, Phase 3). The Portfolio page data source switches to `accounts`.

### 2. Research is a workspace-scoped dimension

Research projects/documents/reports, theses, knowledge entities, artifacts, agent tasks, and option analysis follow the active workspace. No research table changes.

**Options page (analysis tool).** The Options page is an analysis tool for pricing and comparing option instruments (chain, Greeks, strategy builder) — it is not a holdings/ownership module. Its data stays workspace-scoped (0014 unchanged) and the page reads the active workspace context. If real option *positions* are later tracked for portfolio valuation, they belong to the global portfolio dimension (see Out of scope).

### 3. The link between dimensions is explicit

Research references portfolio symbols through `portfolio_theme_links(workspace_id, symbol, entity_id)` and, where applicable, `investment_theses.portfolio_holding_id`. Research conclusions and portfolio holdings are never the same table; the linkage is intentional.

### 4. Switching (UX)

- A **single global workspace switcher** is placed at the top-left of the TopBar, next to the breadcrumb.
- The selection is **persisted in `localStorage`** and exposed through a `useActiveWorkspace` context.
- All research-dimension pages (Research, Theses, Knowledge, Journal, Artifacts, Options, Agent panel) read from `useActiveWorkspace` and **drop their per-page selectors**.
- The **Portfolio page ignores the switcher** (global view).
- The **Today/Dashboard** page is a mixed view: portfolio overview is global; research activity comes from the active workspace. `useActiveWorkspaceId` (today: first workspace) reads the context instead.
- The URL `?workspace=` parameter remains only as a **deep-link entry point**: when present and different from the stored preference it wins; otherwise the stored preference is used. The Research page stops being the only page that syncs it.

### 5. Classification

No `kind`/`type` column is added to `workspaces`. "组合/研究" is a **dimension** distinction, not a workspace type. The switcher groups nothing; the split is expressed by which pages follow the context.

## Consequences

### Positive

- Resolves the account-sharing problem: one account can serve many research themes.
- Zero schema migration for every research table; only the semantics of the already-nullable `accounts.workspace_id` change.
- Option tables are untouched.
- Switching behavior is consistent across all pages; the per-page selector duplication disappears.

### Negative

- The Portfolio page loses per-workspace scoping. Users who want a single-strategy view must filter accounts explicitly.
- Existing workspace-bound accounts need their ownership-marker semantics documented; optionally, a data migration may later normalize them.
- Option analysis remains scoped per research workspace and cannot be aggregated across a global portfolio.

### Risks and Mitigations

| Risk                                                         | Mitigation                                                                                                            |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| Users depend on a workspace-scoped portfolio view            | Portfolio page shows a global view with explicit account grouping/filters                                             |
| `accounts.workspace_id` misread as an isolation boundary     | Document `NULL = global`, non-null = provenance only; Portfolio page ignores it for filtering                         |
| Options analysis conflated with option holdings              | This ADR draws the boundary; real option positions later move to global `accounts`/positions                         |
| Local-only persistence diverges across devices               | `localStorage` is the v1 source of truth; a future sync/migration to URL or server state is tracked as future work    |

## Out of scope / Future work

- Real option positions in the global portfolio dimension (follows the canonical `accounts`/positions model).
- Cross-workspace references beyond `portfolio_theme_links`.
- Normalizing legacy `accounts.workspace_id` values (decide `NULL` vs. keep marker) via a data migration.
- Syncing the active-workspace preference across devices.

## References

- [Data Model](../DATA_MODEL.md) — `Workspace`, `Financial Account`, `Portfolio Theme Link`
- Migrations: `0001_initial.sql`, `0010_portfolio_management.sql`, `0011_portfolio_theme_links.sql`, `0014_options_support.sql`, `0015_financial_platforms_accounts.sql`
- [Global Search / Agent Chat](../GLOBAL_SEARCH_AGENT_CHAT.md) — workspace resolution contract to be updated to read `useActiveWorkspace`
- [Frontend/Backend Integration](../FRONTEND_BACKEND_INTEGRATION.md)
- `apps/desktop/src/features/workspace/hooks/useWorkspaces.ts`
- `apps/desktop/src/components/layout/TopBar/TopBar.tsx` — switcher placement
- `apps/desktop/src/pages/today/hooks/useDashboardData.ts` — `useActiveWorkspaceId`
