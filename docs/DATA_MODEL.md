# Data Model

Conceptual entities and their relationships. The database schema implements these concepts; this document defines _what_ the data means, not _how_ it is stored.

---

## Core Entities

### Workspace

**Purpose.** A container for all research activity. Supports multiple isolated workspaces for different investment strategies or clients.

**Relationships.** One workspace contains many research documents, theses, portfolios, and agent tasks.

**Lifecycle.** Created by the user. Can be archived. Deletion cascades to all contained entities.

---

### Research Document

**Purpose.** A unit of research — could be an uploaded PDF, a web article, an agent-generated summary, or user-written notes.

**Relationships.** Belongs to one workspace. Has many sources and notes.

**Lifecycle.** Created by user upload or agent generation. Edited over time. Can be archived or deleted.

---

### Source

**Purpose.** Provenance metadata for information used in research. Tracks where evidence came from and when it was retrieved.

**Attributes.** URL or local path, title, retrieval timestamp, publication date.

**Relationships.** Attached to one research document. Referenced by thesis evidence entries.

**Lifecycle.** Created alongside or after a document. Read-only after creation (immutable provenance).

---

### Research Note

**Purpose.** A user annotation on a research document. Free-form text capturing insights, questions, or action items.

**Relationships.** Belongs to one research document.

**Lifecycle.** Created, edited, and deleted by the user.

---

### Investment Thesis

**Purpose.** A testable claim about an investment opportunity. The central artifact of the research workflow.

**Attributes.** Title, thesis statement, confidence level (0-100), immutable confidence history, status (draft/active/validating/validated/closed), validation date, outcome.

**Relationships.** Belongs to one workspace. Has many evidence entries. May reference research documents and sources. Independent of portfolio positions (a thesis can exist without a position).

**Lifecycle.**

```text
Draft → Active → Validating → Validated → (outcome recorded)
                         → Closed → (outcome recorded)
```

**State transitions.** Draft theses are private works-in-progress. Active theses are published for tracking. Validating theses are under active review. Validated theses have been confirmed by evidence or market outcome. Closed theses have been completed or discontinued.

---

### Evidence

**Purpose.** A specific fact, data point, or argument that supports or contradicts a thesis.

**Attributes.** Direction (supporting/contradicting), evidence text, source reference.

**Relationships.** Belongs to one thesis. Optionally linked to a source.

**Lifecycle.** Created alongside thesis development. Immutable after creation (preserves the evidentiary record).

### Thesis Confidence Snapshot

**Purpose.** An immutable record of a thesis confidence assessment, supporting retrospective review of how conviction changed as evidence accumulated.

**Relationships.** Belongs to one investment thesis.

**Lifecycle.** Created when a thesis is created and whenever its confidence is updated. Never modified or deleted independently.

### Knowledge Entity and Relationship

**Purpose.** A workspace-scoped graph of companies, industries, technologies, and macro themes. Directed relationships explain how entities affect one another.

**Relationships.** Entities may connect to other entities within the same workspace. A thesis may link to many entities, and an entity may support multiple theses.

**Lifecycle.** Entities and relationships are created intentionally by the user or a validated workflow. Cross-workspace relationships and thesis links are rejected.

---

### Agent Task

**Purpose.** A unit of work submitted to the AI agent. Represents one research operation.

**Attributes.** Status (queued/running/waiting_for_input/completed/failed/cancelled), input text, output JSON, error message.

**Relationships.** Produces artifacts. Generates task events.

**Lifecycle.**

```text
Queued → Running → Waiting For Input → Completed
                  → Failed
                  → Cancelled
```

Cancellation can occur at any non-terminal state. Failed tasks record the error for diagnosis.

---

### Agent Task Event

**Purpose.** A log entry recording one step in a task's execution. Enables progress streaming and post-hoc analysis.

**Attributes.** Event type (started/thinking/tool_call/tool_result/streaming/completed/failed), payload JSON, timestamp.

**Relationships.** Belongs to one agent task.

**Lifecycle.** Append-only. Events are never modified or deleted.

---

### Artifact

**Purpose.** A structured, interactive output produced by an agent task. Rendered in a temporary window.

**Attributes.** Artifact type (references a plugin), input data, output data, status.

**Relationships.** Optionally linked to an agent task. Rendered by a plugin.

**Lifecycle.**

```text
Pending → Generating → Completed
                    → Failed
```

Artifacts can be persisted (saved to workspace) or discarded after viewing.

---

### Artifact Session

**Purpose.** Tracks one viewing session of an artifact — when it was opened and closed.

**Relationships.** Belongs to one artifact.

**Lifecycle.** Created when an artifact window opens. Closed timestamp recorded when the window closes.

---

### Plugin

**Purpose.** A registered rendering component that can display specific artifact types.

**Attributes.** Identifier, name, version, manifest JSON, enabled flag.

**Relationships.** Has many permission grants. Renders artifacts of matching type.

**Lifecycle.** Installed (registered), enabled/disabled, uninstalled.

---

### Plugin Permission

**Purpose.** A specific capability granted to a plugin (e.g., network access for fetching chart data).

**Relationships.** Belongs to one plugin.

**Lifecycle.** Created when a plugin is installed or updated. Revoked when a plugin is uninstalled.

---

### Platform

**Purpose.** A brokerage or custodian that hosts one or more accounts.

**Relationships.** Has many accounts.

**Lifecycle.** Created by the user or a broker sync. Referenced by accounts.

---

### Financial Account

**Purpose.** A brokerage, cash, credit-card, or cryptocurrency account tracked for portfolio valuation. The canonical financial account model; the pre-existing placeholder `portfolio_accounts` tables are retired once the portfolio UI lands (Phase 3).

**Attributes.** Name, account type (SECURITIES / CASH / CREDIT_CARD / CRYPTOCURRENCY), base currency, tracking mode (TRANSACTIONS / HOLDINGS / NOT_SET), account number, archived flag.

**Relationships.** Scoped to an optional workspace. May belong to a platform. Has many activities, tax lots, holding snapshots, and daily valuation rows.

**Lifecycle.** Created by the user. Can be archived (hidden) or deleted — deletion cascades to all contained financial rows.

---

### Asset

**Purpose.** A tradeable or trackable instrument — equity, crypto, FX, metal, cash, or other tracked value (property, vehicle, collectible). The canonical replacement for the placeholder `positions.symbol` model.

**Attributes.** Kind, display code, quote mode (MARKET/MANUAL), quote currency, instrument type/symbol/exchange, and a derived canonical instrument key.

**Relationships.** Has quotes, tax lots, activities, and taxonomy assignments.

**Lifecycle.** Created by the user or during an import. Can be deactivated. Deletion cascades to its quotes and lots.

---

### Quote

**Purpose.** A price point for an asset on a day from a source (market provider or manual entry).

**Attributes.** Day, source, open/high/low/close/adjclose, volume, currency.

**Relationships.** Belongs to one asset. One row per (asset, day, source).

**Lifecycle.** Appended by market-data providers and manual entry. Rebuilt when prices change.

---

### Activity

**Purpose.** A single transaction or cash movement in the canonical ledger — buy, sell, split, dividend, interest, deposit, withdrawal, transfer, fee, tax, credit, adjustment. The canonical replacement for the placeholder `transactions` model.

**Attributes.** Type, status (POSTED/PENDING/CANCELED), activity date, settlement date, quantity, unit price, amount, fee, tax, currency, FX rate, idempotency key.

**Relationships.** Belongs to one account. May reference an asset and an import run.

**Lifecycle.** Immutable audit trail. The unique idempotency key makes re-importing the same source record a no-op.

---

### Import Run

**Purpose.** A batch import of activities from a CSV file or broker sync, with status, checkpoint, and warning tracking.

**Relationships.** Belongs to one account. Produces activities.

**Lifecycle.** Started → finished → applied; records warnings/errors for review.

---

### Tax Lot

**Purpose.** A FIFO cost-basis inventory unit — the quantity and remaining cost basis of shares acquired at one time in one account. The persisted inventory from which current holdings and realized P&L are derived.

**Attributes.** Open date, original/remaining quantity, cost per unit, original/remaining cost basis, allocated fees and taxes, split ratio, closed flag, close date.

**Relationships.** Belongs to one account and one asset. Opened by a buy activity; consumed by sell activities via lot disposals.

**Lifecycle.** Created when shares are acquired. Consumed FIFO by sells; closed when remaining quantity reaches zero. Rebuilt by the valuation service from activities.

---

### Lot Disposal

**Purpose.** A realized disposal of part of a lot, recording the realized P&L for one sell.

**Relationships.** Belongs to one lot, one account, and one asset; references the selling activity.

**Lifecycle.** Created when a sell is processed. Immutable.

---

### Holding Snapshot

**Purpose.** A point-in-time position record for an account — the full position list at a date, used for HOLDINGS-mode accounts, manual entries, and CSV/broker imports.

**Attributes.** Snapshot date, currency, cash balances, cost basis, net contribution, source (CALCULATED / MANUAL_ENTRY / CSV_IMPORT / BROKER_IMPORTED / SYNTHETIC).

**Relationships.** Belongs to one account. Contains many snapshot positions (one per asset).

**Lifecycle.** CALCULATED rows are derived read models rebuilt from activities; user/import rows are source data and never dropped.

---

### Daily Account Valuation

**Purpose.** The derived per-day valuation time series for one account — the read model behind valuation and performance charts.

**Attributes.** Valuation date, cash balance, investment market value, total value, cost basis, net contribution, external flows, FX to base currency, valuation/basis coverage quality.

**Relationships.** Belongs to one account. Exactly one row per (account, valuation date).

**Lifecycle.** Rebuildable at any time from activities + quotes + snapshots.

---

### Taxonomy

**Purpose.** A classification system for assets (instrument type, asset classes, GICS industries, regions, risk category, custom groups).

**Relationships.** Has many categories (hierarchical via parent). Assets are assigned to categories via assignments.

**Lifecycle.** System taxonomies are seeded by migration; users may create custom taxonomies.

---

### Taxonomy Category

**Purpose.** A node within a taxonomy; categories nest via `parent_id`.

### Asset Taxonomy Assignment

**Purpose.** A weighted assignment of an asset to a taxonomy category (0–10000 bps). Single-select taxonomies keep at most one assignment per asset.

---

### Allocation Target

**Purpose.** A rebalancing target expressed against one taxonomy (e.g., "60/40 equities/bonds") with drift bands, minimum trade size, turnover limits, and whole-share/sell policy.

**Relationships.** Scoped to all accounts, a portfolio, or one account. Has weights (per category) and constraints (per subject).

### Allocation Target Weight

**Purpose.** One category's target weight within an allocation target. The weight's taxonomy must match the owning target's taxonomy (enforced by trigger).

### Allocation Target Constraint

**Purpose.** A buy/sell/trade rule that constrains rebalancing for a target — block or avoid — against an asset, account, or category.

---

### Watchlist

**Purpose.** A user-curated list of symbols to monitor, independent of portfolio positions.

**Attributes.** Name, list of symbols.

**Relationships.** Standalone — not linked to accounts or theses.

**Lifecycle.** Created, edited, and deleted by the user.

---

## Entity Relationship Summary

```text
Workspace
  ├── Research Document
  │     ├── Source
  │     └── Research Note
  ├── Investment Thesis
  │     ├── Evidence ───► Source
  │     └── Thesis Confidence Snapshot
  ├── Knowledge Entity
  │     └── Knowledge Relationship ───► Knowledge Entity
  ├── Investment Thesis ───► Knowledge Entity
  ├── Agent Task
  │     └── Agent Task Event
  │     └── Artifact ───► Plugin
  │           └── Artifact Session
  ├── Financial Account ───► Platform          (scoped to workspace)
  │     ├── Import Run ──► Activity
  │     ├── Tax Lot ──► Lot Disposal ──► Activity
  │     ├── Holding Snapshot ──► Snapshot Position ──► Asset
  │     ├── Daily Account Valuation
  │     └── Allocation Target ──► Weight / Constraint
  ├── Asset
  │     ├── Quote
  │     └── Taxonomy Assignment ──► Taxonomy Category ──► Taxonomy
  └── Watchlist
```

Research ↔ financial linkage (Phase 4): an Investment Thesis may reference a
portfolio holding (`theses.portfolio_holding_id` → holding), making the
research → decision → validation loop traceable to actual positions.

## Design Rules

- UUIDs for all primary keys.
- `created_at` on every record.
- `updated_at` on every mutable record.
- Migrations are append-only after any release.
- Domain models are separate from database rows.
- Soft deletion preferred where historical integrity matters.
- Money and quantity values are decimal strings stored as TEXT and parsed as
  `rust_decimal` in Rust — never REAL/float.
- Dates are TEXT (YYYY-MM-DD); timestamps are TEXT (RFC3339 UTC).
- The research and financial domains are separate modules in `crates/domain`;
  the financial module is ported from Wealthfolio (AGPL-3.0) onto SQLx with
  typed, recoverable errors (no `unwrap`/`expect` panic points).
