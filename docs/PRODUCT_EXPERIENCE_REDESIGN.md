# AlphaForge Product Experience Redesign Blueprint

> Status: In progress
> Date: 2026-08-24
> Objective: Evolve the current collection of feature pages into a coherent desktop workspace built around the investment-research lifecycle.

## 1. Product Assessment

The reported issues are not cosmetic. They originate in three layers:

1. **Global information architecture**: high-frequency research actions, low-frequency system actions, and portfolio navigation currently share the same hierarchy.
2. **Disconnected domain workflows**: research, theses, knowledge, journals, and artifacts exist independently, forcing users to switch pages repeatedly to complete one decision record.
3. **Insufficient testable data**: an empty database only produces empty states, so evaluators cannot understand the intended experience. Some backend capabilities exist but lack discoverable entry points and explanations.

The redesign must continue to support the core AlphaForge loop:

```text
Information -> Knowledge -> Thesis -> Decision -> Validation -> Review -> Improvement
```

AlphaForge must not become a trading terminal, and an Agent must never autonomously make or execute a real investment decision.

## 2. Global Information Architecture

### 2.1 Application Shell

- Disable native main-window decorations and render a unified AlphaForge title bar
  with `File / Edit / View / Help`, draggable non-interactive regions, and custom
  minimize, maximize/restore, and close controls.
- Keep `shadow: true` for the frameless main window. Window buttons and menus must
  never be drag regions, and the sidebar toggle remains at the far left.
- Place a global toolbar below the title bar for workspace context, search, and
  primary actions. The logo is only a brand identifier.
- Keep Dashboard, Research, Theses, Investment Journal, Options, and Artifacts in the primary sidebar.
- Pin Knowledge and Portfolio at the bottom, followed by an account icon menu beside Portfolio.
- Provide Settings, API Usage, workspace or portfolio switching, and light/dark theme controls in the account menu.
- Allow Settings, Knowledge, and Portfolio to use dedicated sidebars so domain navigation does not compete with the main application navigation.

### 2.2 Settings Center

Settings is a standalone route shell and does not display the main application sidebar. Its sections are General, Appearance, Localization, Agents, Data and Backup, Internal Plugins, and About.

Appearance supports system, light, and dark modes, multiple accent colors, and two market-color conventions. Every market chart must read `--market-positive` and `--market-negative`; components must not hard-code red or green.

Localization includes an **Enable professional terminology localization** option. Professional terminology is separate from ordinary interface copy: common interface text is always localized, while domain terms such as company, industry, technology, and macro theme can preserve their source language or use user-defined translations. Adding a term requires synchronous updates to:

- `apps/desktop/src/lib/i18n/locale.ts`
- `apps/desktop/src/lib/i18n/catalogs/*`
- the professional-term defaults and settings editor
- `docs/i18n/ARCHITECTURE.md`

## 3. Dashboard and First-Run Experience

### 3.1 Distinct Data States

| State | Presentation | Actions |
|---|---|---|
| New installation or empty database | Clearly labeled demonstration data | Create portfolio, import data, open setup wizard |
| Database unavailable or missing | Recovery view; never disguise a failure as demo data | Select backup, create database, open diagnostics |
| Real data available | Real dashboard | Filter portfolio, account, period, and benchmark |
| Partial data | Available charts plus data-gap notices | Complete quotes, exchange rates, or activity records |

Demo data should use a read-only frontend projection or in-memory fixture with `dataMode: "demo"`. It must never be written to the user database. Every demo card must carry a visible **Demo data** label so it cannot be mistaken for real assets.

### 3.2 Setup Wizard

```text
Welcome -> Create or import data -> Create portfolio -> Add accounts
-> Import activities -> Configure market data -> Finish
```

Replace the phrase **Bind database** with **Import existing database**. Rust owns file selection, SQLite header and schema-version validation, path normalization, and migration. By default, AlphaForge copies a validated database into its managed application directory instead of retaining arbitrary access to an external path.

### 3.3 Dashboard Visualizations

- Total assets and net-worth history
- Return versus benchmark
- Allocation by asset class, industry, currency, and account
- Cash flow, dividends, and fees
- Maximum drawdown and volatility
- Thesis coverage, unvalidated risks, and recent research activity

## 4. Research, Theses, and Investment Journal

### 4.1 Research Workbench

Restructure Research around project context instead of three unrelated panels:

```text
Research project list | Overview / Sources / Documents / Notes / Theses / Artifacts
```

After a project is selected, all content shares one `research_project_id`. The header displays the research question, progress, source count, last update, and next action. Importing, searching, creating notes, running an Agent, drafting a thesis, and generating an artifact all start from the current project.

### 4.2 Relationship Between Research and Theses

Keep an independent **Thesis Portfolio** entry for cross-project review, validation, and retrospectives, while embedding a Theses tab inside each research project. This is one domain model presented through two working views, not duplicated data or persistence.

### 4.3 Investment Journal

Reframe Investment Journal as a **Decision and Review Journal**. Every entry should capture at least:

- decision type and date
- related portfolio, account, asset, research project, and thesis
- the information set available at the time
- supporting and contradicting evidence
- expectations, risks, triggers, and invalidation conditions
- later outcomes, sources of variance, and lessons learned

The journal offers timeline and calendar views plus thesis and asset filters. Agents may organize evidence and assist reviews, but the user must confirm every investment decision.

## 5. Knowledge Workspace

Knowledge uses a dedicated three-pane layout:

```text
Libraries / Tags / Collections | Notes, literature, and entities
| Editor, PDF reader, or graph inspector
```

### 5.1 Capability Layers

1. **Notes and knowledge network**: Markdown, bidirectional links, backlinks, tags, full-text search, graph view, orphan detection, and unresolved-link discovery.
2. **Literature management**: metadata, authors, DOI, journal, year, collections, attachments, citation keys, and CSL JSON/BibTeX import and export.
3. **PDF workflow**: embedded reading, page navigation, highlights, comments, area captures, annotation-to-note conversion, and links back to source pages.
4. **LaTeX support**: inline and block rendering, preservation of source LaTeX, citation-key completion, and BibTeX/CSL and Pandoc export contracts.
5. **Entity management**: open a detail inspector on click; edit, soft-delete, manage relationships and tags, inspect sources, and connect theses.

Zettlr is a useful reference for local-first authoring, Zettelkasten workflows, citations, LaTeX, and Pandoc. PDF viewing must not be treated as PDF annotation, which requires a separate design. Zotero-like functionality should begin with open metadata and citation formats rather than attempting to reproduce the entire product.

### 5.2 Incremental Backend Model

- `knowledge_tags`, `knowledge_entity_tags`
- `library_items`, `library_creators`, `library_collections`
- `document_assets`, `pdf_annotations`
- `note_links`, `citation_keys`

Entity removal uses soft deletion or a pre-deletion impact preview. When an entity has linked theses, relationships, or citations, AlphaForge must communicate the impact explicitly and must not silently cascade-delete evidence.

## 6. Portfolio and Accounts

Portfolio receives a dedicated sidebar: Overview, Accounts, Holdings, Activities, Performance, Allocation, Income, Import, and Settings. The current portfolio remains visible in the header and can be switched immediately.

One portfolio contains multiple accounts. Every aggregate query accepts optional `account_ids` and selects all accounts by default. Accounts supports creation, archiving, single-account views, and data-quality notices. The implementation must reuse the existing SQLx financial domain, account repositories, valuation, performance, allocation, net-worth, and income services instead of creating a second portfolio model.

The interface may follow the local Wealthfolio audit and `docs/PORTFOLIO_INTEGRATION_PLAN.md`, while retaining SQLx, typed recoverable errors, and the separation between financial and research domains.

## 7. Options

The Options workbench is divided into Chain, Strategy Builder, Payoff, Volatility, Risk, and Portfolio Links.

The first priority is a strategy payoff chart that includes:

- expiration payoff and theoretical payoff at a selected date
- break-even points and maximum profit/loss
- adjustable underlying price, implied volatility, and time-to-expiration scenarios
- multi-leg details and aggregate Greeks

Later phases add IV smile and term structure, probability distributions, historical backtesting, and portfolio-level Greeks. Model inputs, quote timestamps, and assumptions must remain visible; theoretical values must never be presented as guaranteed returns.

## 8. Research Artifacts

Research Artifacts becomes a reusable output library rather than a standalone demonstration page. It supports filtering by research project, thesis, asset, type, and status; card and table views; recent items, favorites, and versions; source and generation-parameter provenance; secure Artifact-window rendering; export, citation copying, and thesis linking; and recoverable errors with retry actions.

Artifact creation primarily starts inside Research or Thesis context. The Artifact library focuses on management and reuse.

## 9. Agent Settings and API Usage

### 9.1 Agent Configuration

Provider configuration includes display name, provider type, base URL, model, API key stored only in the operating-system keychain, timeout, maximum output, reasoning mode or effort, concurrency, retry policy, and a connection test. Provider-specific parameters use versioned discriminated unions; arbitrary JSON must never be forwarded directly to a privileged backend.

The OpenAI adapter uses the Responses protocol, while the DeepSeek adapter uses its official Chat Completions protocol. Models and parameter options must come from the active adapter capabilities or provider model discovery. Parameters from one provider must not be forced onto another.

### 9.2 Usage Ledger

A real usage center first requires normalized provider responses:

```text
Provider response -> normalized UsageMetrics -> Rust usage service
-> api_usage_events (SQLite) -> daily/monthly aggregation
-> React charts and request log
```

Recommended event fields are provider, model, task_id, request_id, input_tokens, cached_tokens, output_tokens, reasoning_tokens, total_tokens, latency_ms, status, occurred_at, price_version, and estimated_cost. Cost uses a price snapshot from the time of the request and is always labeled **Estimated**. When a provider does not return usage, the event records `unknown`; AlphaForge must not invent a value.

## 10. Phases and Acceptance Criteria

### P0: Application Shell and Settings Center

- Install the frameless, shadowed custom title/menu bar and window controls
- Move the sidebar control to the far-left title-bar position
- Add the bottom Knowledge, Portfolio, and account area
- Introduce a dedicated Settings shell
- Support light, dark, accent, and market-color preferences
- Add the professional terminology localization option and editor

### P1: Testable Dashboard and Data Wizard

- Explicit demo-data mode
- Separate empty, broken, and partial database states
- Portfolio and account creation wizard
- Core visualizations

### P2: Knowledge Loop

- Entity details, editing, deletion, and tags
- Notes, backlinks, search, and graph inspector
- Literature model plus the first PDF reading and annotation workflow

### P3: Research-Thesis-Journal Loop

- Project workbench
- Embedded theses and cross-project Thesis Portfolio
- Decision and review journal

### P4: Portfolio and Account Workspace

- Dedicated sidebar, portfolio switching, and account filters
- Account creation and single-account views
- Performance, allocation, cash-flow, and risk charts

### P5: Options and Artifacts

- Payoff charts and scenario analysis
- Artifact library, versioning, and contextual creation

### P6: Agents and Usage

- Typed multi-provider configuration
- Connection testing and model capabilities
- Token and estimated-cost ledger, trends, and request details

Every phase must include all relevant frontend states, Rust services, append-only migrations, IPC and Zod contracts, tests, and synchronous documentation. Every phase must finish with a green build.

## 11. References

- [Tauri 2 Window Menu](https://v2.tauri.app/learn/window-menu/)
- [CC Switch](https://github.com/farion1231/cc-switch)
- [Zettlr](https://github.com/Zettlr/Zettlr)
- [OpenAI Responses API](https://developers.openai.com/api/reference/resources/responses/methods/create)
- [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/)
- `docs/PORTFOLIO_INTEGRATION_PLAN.md`
- `docs/option/ROADMAP.md`
