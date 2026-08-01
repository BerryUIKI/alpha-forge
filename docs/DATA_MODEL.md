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

**Attributes.** Title, thesis statement, confidence level (0-100), status (draft/active/validating/validated/closed), validation date, outcome.

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

### Portfolio Account

**Purpose.** A brokerage or tracking account for portfolio positions.

**Attributes.** Name, account type, base currency.

**Relationships.** Has many positions and transactions.

**Lifecycle.** Created by the user. Can be archived (hidden) or deleted.

---

### Position

**Purpose.** A holding in a portfolio account — a specific quantity of a security.

**Attributes.** Symbol, quantity, cost basis.

**Relationships.** Belongs to one account.

**Lifecycle.** Created when a position is opened. Updated as quantity or basis changes. Closed when quantity reaches zero.

---

### Transaction

**Purpose.** A recorded buy or sell action in a portfolio account.

**Attributes.** Symbol, transaction type (buy/sell), quantity, price, execution timestamp.

**Relationships.** Belongs to one account.

**Lifecycle.** Immutable after creation (audit trail).

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
  │     └── Evidence ───► Source
  ├── Agent Task
  │     └── Agent Task Event
  │     └── Artifact ───► Plugin
  │           └── Artifact Session
  ├── Portfolio Account
  │     ├── Position
  │     └── Transaction
  └── Watchlist
```

## Design Rules

- UUIDs for all primary keys.
- `created_at` on every record.
- `updated_at` on every mutable record.
- Migrations are append-only after any release.
- Domain models are separate from database rows.
- Soft deletion preferred where historical integrity matters.
