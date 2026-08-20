# Daily Operations

This guide explains how to use every module of AlphaForge in day-to-day work.
Modules are listed in the same order as the left navigation.

> **Scope reminder**: AlphaForge is a research workspace. It records and analyzes
> information; it does not place trades or make autonomous investment decisions.

## Contents

- [Workspaces](#workspaces)
- [Today (Dashboard)](#today-dashboard)
- [Research](#research)
- [Theses / Journal](#theses--journal)
- [Portfolio](#portfolio)
- [Knowledge](#knowledge)
- [Options](#options)
- [Artifacts](#artifacts)
- [Global Search](#global-search)
- [Agent Tasks](#agent-tasks)

---

## Workspaces

AlphaForge uses **workspaces** to keep research organized. Research projects,
options analysis, and artifacts are scoped to the **active workspace**.

- **Create a workspace** — use the workspace switcher at the top of the window.
- **Switch workspaces** — your active workspace follows you across modules, so
  selecting a project or artifact in one module carries into the others.

---

## Today (Dashboard)

The **Today** page is your starting point, with three tabs:

| Tab | Shows |
|-----|-------|
| **Overview** | Stat cards, top holdings, and recent activity |
| **Performance** | Portfolio performance chart |
| **Activity** | Full activity feed |

The active tab is remembered across sessions and can be shared via the URL
(`?tab=performance`).

---

## Research

The **Research** page manages research projects inside the active workspace.

### Projects

- **Create a project** to group documents, notes, sources, and reports around a
  theme (for example, a company or an industry).
- The selected project can be deep-linked with `?project=<id>`.
- All lists show loading, empty, and error states, so a project with no content
  yet is clearly distinguishable from a failed load.

### Documents, Notes, Sources, and Reports

Within a project you can:

- **Documents** — track files used as research material.
- **Notes** — capture your own analysis.
- **Sources** — record evidence with a title and URL, so claims stay traceable.
- **Reports** — write structured reports (type: `analysis`, `summary`, `thesis`,
  or `recommendation`).

### Search inside research

Search supports two modes:

- **Lexical** — keyword matching (default).
- **Semantic** — meaning-based matching (requires the AI provider to be
  configured; see [Configuration](configuration.md#ai-provider-agent-configuration)).

---

## Theses / Journal

**Theses** (also reachable as **Journal**) is the investment-thesis management
interface.

- **Create a thesis** for an investment idea.
- **Track evidence** — attach supporting material so the thesis stays
  evidence-backed.
- **Confidence** — record and update how confident you are; the app keeps the
  confidence history over time.
- **Knowledge graph** — the panel shows how the thesis connects to related
  entities.

The core loop this module supports: *Information → Knowledge → Thesis →
Decision → Validation → Review → Improvement*.

---

## Portfolio

The **Portfolio** page is a dashboard for tracking holdings:

- **Accounts** — create accounts (e.g. brokerage, cash, retirement) and record
  activities against them.
- **Holdings** — see current holdings with quantities and market values.
- **Activities** — log buy/sell/income events with full history.
- **Allocation** — view asset allocation charts.
- **Valuation** — track valuations over time (snapshots).

Keep the data current by adding activities as they happen — the dashboard,
performance charts, and Today page all read from the same local data.

---

## Knowledge

The **Knowledge** page maintains a knowledge graph of entities you track:

| Entity type | Examples |
|-------------|----------|
| Company | A listed company |
| Industry | A sector |
| Technology | A technology/theme |
| Macro theme | A macroeconomic theme |

Create entities and they become linkable anchors for theses and research.

---

## Options

The **Options** page is a workspace-scoped options analysis toolkit:

- **Greeks Calculator** — compute option greeks for a contract.
- **Option Chains** — fetch an option chain for a symbol and pick a chain/date.
- **Contract Table** — inspect contracts in the selected chain.
- **Strategy Builder** — assemble option strategies.

### Fetching a chain

1. Enter a ticker symbol (letters/digits/`.`/`-`, up to 10 characters).
2. Fetch the chain; the app normalizes the symbol (uppercase, trimmed).
3. Select an expiration/chain, then inspect contracts and compute greeks.

> Chain data comes from the configured market-data provider. If a fetch fails,
> check the network connection and the symbol format; see
> [Troubleshooting](troubleshooting.md).

---

## Artifacts

**Artifacts** are the visual, interactive results produced by plugins and agent
workflows.

- **View** — open an artifact; it renders in a dedicated, isolated window.
- **Delete** — remove artifacts you no longer need.
- **Create** — some plugins let you build an artifact from a form (for example,
  the Company Comparison form: pick companies and generate a comparison table).

### Built-in artifact renderers

| Renderer | Used by |
|----------|---------|
| Comparison Table | Company Comparison plugin |
| Earnings Analyzer | Earnings Analyzer plugin |
| Industry Map | Industry Map plugin |
| Macro Dashboard | Macro Dashboard plugin |
| Risk Dashboard | Portfolio Risk plugin |
| Timeline | Timeline plugin |
| Valuation Model | Valuation Model plugin |

Artifacts are scoped to the active workspace.

---

## Global Search

Use **global search** (available from the top bar) to jump quickly across
projects, theses, entities, and artifacts instead of navigating module by module.

---

## Agent Tasks

Agent tasks run research/analysis jobs in the background and stream progress
into the UI.

- **Create a task** — describe what you want analyzed; the agent runs it
  asynchronously.
- **Statuses** — a task moves through `queued` → `running` → `completed`
  (`failed` or `cancelled` on problems).
- **Cancel** — long-running tasks can be cancelled at any time.
- **Prerequisite** — the AI provider must be configured first
  ([Configuration](configuration.md#ai-provider-agent-configuration)). If it is
  not, the Agent Configuration Guide dialog appears with a shortcut to Settings.

---

## Next Steps

- [Troubleshooting](troubleshooting.md) — fixes for common issues.
- [FAQ](faq.md) — questions and answers.
