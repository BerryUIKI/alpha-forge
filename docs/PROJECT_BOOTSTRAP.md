# AlphaForge Project Bootstrap

## 1. Purpose

This document defines the initial repository structure, implementation boundaries, development order, and minimum engineering standards for AlphaForge.

AlphaForge is a desktop-first AI workspace for investment research. Its core loop is:

```text
Information
→ Knowledge
→ Thesis
→ Decision
→ Validation
→ Review
→ Improvement
```

The product is not a brokerage terminal, automated trading system, stock-picking bot, or generic AI chat wrapper.

---

## 2. Core Technology Stack

### Desktop Application

- Tauri 2
- Rust
- React
- TypeScript
- Vite

### Frontend

- Tailwind CSS
- shadcn/ui
- Radix UI
- Lucide React
- React Router
- TanStack Query
- Zustand
- Zod
- React Hook Form

### Local Runtime

- SQLite
- SQLx
- Tokio
- Serde
- Reqwest
- Tracing
- thiserror
- anyhow

### AI and Data

- OpenAI API
- Structured Outputs
- Tool Calling
- Streaming responses
- Provider abstraction for market, news, and research data

### Testing and Quality

- Vitest
- React Testing Library
- Playwright
- cargo test
- cargo clippy
- cargo fmt
- ESLint
- Prettier

---

## 3. Recommended Repository Structure

```text
alpha-forge/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── release.yml
│   │   └── security.yml
│   ├── ISSUE_TEMPLATE/
│   └── pull_request_template.md
│
├── .vscode/
│   ├── extensions.json
│   ├── settings.json
│   └── tasks.json
│
├── apps/
│   └── desktop/
│       ├── public/
│       │   ├── icons/
│       │   └── assets/
│       │
│       ├── src/
│       │   ├── app/
│       │   │   ├── App.tsx
│       │   │   ├── router.tsx
│       │   │   ├── providers.tsx
│       │   │   └── startup.ts
│       │   │
│       │   ├── pages/
│       │   │   ├── today/
│       │   │   ├── research/
│       │   │   ├── journal/
│       │   │   ├── portfolio/
│       │   │   ├── artifacts/
│       │   │   └── settings/
│       │   │
│       │   ├── features/
│       │   │   ├── agent/
│       │   │   ├── research/
│       │   │   ├── journal/
│       │   │   ├── portfolio/
│       │   │   ├── artifacts/
│       │   │   ├── plugins/
│       │   │   ├── documents/
│       │   │   └── market-data/
│       │   │
│       │   ├── components/
│       │   │   ├── layout/
│       │   │   ├── navigation/
│       │   │   ├── feedback/
│       │   │   └── common/
│       │   │
│       │   ├── lib/
│       │   │   ├── desktop-api/
│       │   │   ├── query-client/
│       │   │   ├── validation/
│       │   │   ├── formatting/
│       │   │   └── errors/
│       │   │
│       │   ├── hooks/
│       │   ├── stores/
│       │   ├── styles/
│       │   ├── types/
│       │   ├── test/
│       │   └── main.tsx
│       │
│       ├── src-tauri/
│       │   ├── capabilities/
│       │   │   ├── main-window.json
│       │   │   └── artifact-window.json
│       │   │
│       │   ├── migrations/
│       │   │   └── 0001_initial.sql
│       │   │
│       │   ├── src/
│       │   │   ├── main.rs
│       │   │   ├── lib.rs
│       │   │   │
│       │   │   ├── app/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── state.rs
│       │   │   │   └── bootstrap.rs
│       │   │   │
│       │   │   ├── commands/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── agent.rs
│       │   │   │   ├── research.rs
│       │   │   │   ├── journal.rs
│       │   │   │   ├── portfolio.rs
│       │   │   │   ├── artifacts.rs
│       │   │   │   └── settings.rs
│       │   │   │
│       │   │   ├── agent/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── runtime.rs
│       │   │   │   ├── task.rs
│       │   │   │   ├── tools.rs
│       │   │   │   ├── events.rs
│       │   │   │   └── context.rs
│       │   │   │
│       │   │   ├── database/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── connection.rs
│       │   │   │   ├── migrations.rs
│       │   │   │   └── repositories/
│       │   │   │
│       │   │   ├── artifacts/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── manager.rs
│       │   │   │   ├── manifest.rs
│       │   │   │   ├── renderer.rs
│       │   │   │   └── security.rs
│       │   │   │
│       │   │   ├── plugins/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── registry.rs
│       │   │   │   ├── loader.rs
│       │   │   │   └── permissions.rs
│       │   │   │
│       │   │   ├── documents/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── parser.rs
│       │   │   │   ├── chunker.rs
│       │   │   │   └── indexer.rs
│       │   │   │
│       │   │   ├── providers/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── ai/
│       │   │   │   ├── market-data/
│       │   │   │   └── news/
│       │   │   │
│       │   │   ├── security/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── credentials.rs
│       │   │   │   ├── sandbox.rs
│       │   │   │   └── validation.rs
│       │   │   │
│       │   │   ├── windows/
│       │   │   │   ├── mod.rs
│       │   │   │   ├── main_window.rs
│       │   │   │   └── artifact_window.rs
│       │   │   │
│       │   │   ├── config/
│       │   │   ├── telemetry/
│       │   │   └── error.rs
│       │   │
│       │   ├── Cargo.toml
│       │   ├── tauri.conf.json
│       │   └── build.rs
│       │
│       ├── index.html
│       ├── package.json
│       ├── tsconfig.json
│       └── vite.config.ts
│
├── crates/
│   ├── domain/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── research.rs
│   │       ├── thesis.rs
│   │       ├── portfolio.rs
│   │       ├── artifact.rs
│   │       └── task.rs
│   ├── agent-core/
│   ├── artifact-core/
│   ├── provider-core/
│   └── shared/
│
├── packages/
│   ├── ui/
│   ├── schemas/
│   ├── artifact-sdk/
│   ├── financial-components/
│   ├── shared-types/
│   └── config/
│
├── plugins/
│   ├── company-comparison/
│   │   ├── manifest.json
│   │   ├── src/
│   │   └── package.json
│   ├── valuation-model/
│   ├── portfolio-risk/
│   ├── industry-map/
│   └── timeline/
│
├── docs/
│   ├── PRODUCT.md
│   ├── ARCHITECTURE.md
│   ├── DATA_MODEL.md
│   ├── SECURITY.md
│   ├── PLUGIN_SPEC.md
│   ├── AGENT_PROTOCOL.md
│   ├── UI_GUIDELINES.md
│   ├── ROADMAP.md
│   └── DECISIONS/
│
├── scripts/
│   ├── bootstrap.sh
│   ├── check.sh
│   ├── dev.sh
│   ├── build.sh
│   └── release.sh
│
├── tests/
│   ├── fixtures/
│   ├── integration/
│   └── e2e/
│
├── .editorconfig
├── .env.example
├── .gitignore
├── .node-version
├── rust-toolchain.toml
├── AGENTS.md
├── CONTRIBUTING.md
├── Cargo.toml
├── package.json
├── pnpm-workspace.yaml
├── README.md
└── LICENSE
```

---

## 4. Initialization Order

### Phase 1: Repository and Workspaces

- [ ] Create the Git repository.
- [ ] Initialize the pnpm workspace.
- [ ] Initialize the Rust workspace.
- [ ] Create `apps/desktop`.
- [ ] Create `packages`.
- [ ] Create `crates`.
- [ ] Create `plugins`.
- [ ] Create `docs`.
- [ ] Configure `.gitignore`.
- [ ] Configure `.editorconfig`.
- [ ] Pin Node, pnpm, and Rust versions.

Recommended version files:

```text
.node-version
rust-toolchain.toml
```

### Phase 2: Tauri Desktop Application

- [ ] Initialize Tauri 2.
- [ ] Integrate React, TypeScript, and Vite.
- [ ] Confirm that the macOS development build launches.
- [ ] Configure the application name, bundle identifier, and icons.
- [ ] Configure the default window dimensions.
- [ ] Configure the macOS title bar behavior.
- [ ] Configure development hot reload.
- [ ] Configure Tauri capability files.

Suggested bundle identifier:

```text
com.berry.alphaforge
```

### Phase 3: Frontend Foundation

- [ ] Install Tailwind CSS.
- [ ] Initialize shadcn/ui.
- [ ] Configure React Router.
- [ ] Configure TanStack Query.
- [ ] Configure Zustand.
- [ ] Configure Zod.
- [ ] Add a global error boundary.
- [ ] Add toast notifications.
- [ ] Add theme support.
- [ ] Add keyboard shortcut infrastructure.
- [ ] Create the main application layout.
- [ ] Create the left navigation.
- [ ] Create the Agent workspace.
- [ ] Create an Artifact window placeholder.

Initial routes:

```text
/today
/research
/journal
/portfolio
/settings
```

### Phase 4: Rust Foundation

- [ ] Create a shared `AppState`.
- [ ] Create a unified application error type.
- [ ] Configure `tracing`.
- [ ] Configure Tokio.
- [ ] Configure a shared Reqwest client.
- [ ] Configure the SQLite connection pool.
- [ ] Configure database migrations.
- [ ] Create the repository layer.
- [ ] Create the Tauri command registry.
- [ ] Create the application event publisher.
- [ ] Create the configuration module.
- [ ] Create secure credential storage.

All production Tauri commands should return:

```rust
Result<T, AppError>
```

Do not expose uncontrolled internal error strings directly to the frontend.

### Phase 5: Frontend–Rust Communication

- [ ] Create a unified `desktopApi`.
- [ ] Prevent components from calling `invoke` directly throughout the codebase.
- [ ] Define stable command names.
- [ ] Define request and response schemas.
- [ ] Define Agent events.
- [ ] Define Artifact events.
- [ ] Define stable error codes.
- [ ] Add IPC integration tests.

Suggested frontend structure:

```text
src/lib/desktop-api/
├── agent.ts
├── artifacts.ts
├── research.ts
├── journal.ts
├── portfolio.ts
└── index.ts
```

### Phase 6: Local Database

Initial tables:

```text
app_settings
workspaces
research_documents
research_sources
research_notes
investment_theses
thesis_evidence
agent_tasks
agent_task_events
artifacts
artifact_sessions
plugins
plugin_permissions
portfolio_accounts
positions
transactions
watchlists
```

Requirements:

- [ ] Use stable UUIDs.
- [ ] Include `created_at` on all tables.
- [ ] Include `updated_at` on mutable records.
- [ ] Prefer soft deletion where appropriate.
- [ ] Never modify a migration that has already been released.
- [ ] Keep database rows separate from domain models.
- [ ] Repositories must not leak storage-specific structures into the rest of the application.

### Phase 7: Agent Runtime

The first release should use a single-agent workflow.

- [ ] Create a task.
- [ ] Return a `task_id` immediately.
- [ ] Run the task asynchronously.
- [ ] Stream progress events.
- [ ] Support cancellation.
- [ ] Persist task events.
- [ ] Persist the final Artifact.
- [ ] Handle errors and retries.
- [ ] Limit concurrent tasks.
- [ ] Enforce timeouts.
- [ ] Enforce token and cost budgets.

Task states:

```text
queued
running
waiting_for_input
completed
failed
cancelled
```

Do not keep a long-running Tauri `invoke` call open for the duration of an Agent task.

### Phase 8: Artifact Runtime

- [ ] Define the Artifact manifest.
- [ ] Define the input schema.
- [ ] Define Artifact output events.
- [ ] Create the Artifact registry.
- [ ] Create temporary WebView windows.
- [ ] Create a controlled local asset protocol.
- [ ] Prevent Artifacts from inheriting main-window permissions.
- [ ] Prevent Artifacts from accessing API keys.
- [ ] Disable unrestricted external network access by default.
- [ ] Destroy temporary windows when closed.
- [ ] Support persisted Artifact state.
- [ ] Support screenshot or PDF export where appropriate.

Example manifest:

```json
{
  "id": "company-comparison",
  "name": "Company Comparison",
  "version": "0.1.0",
  "entry": "index.html",
  "inputSchema": "schema.json",
  "permissions": [],
  "window": {
    "width": 1100,
    "height": 760,
    "resizable": true
  }
}
```

### Phase 9: Plugin System

The MVP should allow internal plugins only.

- [ ] Plugin registration.
- [ ] Manifest validation.
- [ ] Version validation.
- [ ] Input schema validation.
- [ ] Permission declarations.
- [ ] Plugin loading.
- [ ] Plugin unloading.
- [ ] Plugin error isolation.
- [ ] Plugin event bridge.
- [ ] Plugin development template.

Initial internal plugins:

```text
company-comparison
valuation-model
portfolio-risk
industry-map
research-timeline
```

A third-party plugin marketplace is explicitly outside the MVP scope.

### Phase 10: Security

- [ ] Store API keys in the operating system keychain.
- [ ] Never expose plaintext credentials to React.
- [ ] Do not allow Artifacts to call unrestricted Rust commands.
- [ ] Isolate Tauri permissions by window.
- [ ] Validate all external URLs against an allowlist.
- [ ] Normalize and validate file paths.
- [ ] Prevent directory traversal.
- [ ] Limit plugin package size.
- [ ] Limit Agent output size.
- [ ] Limit task concurrency.
- [ ] Redact secrets from logs.
- [ ] Disable shell access by default.
- [ ] Disable unrestricted filesystem access by default.

---

## 5. MVP Page Scope

### Today

- Current research tasks
- Recently generated Artifacts
- Theses awaiting validation
- Portfolio risk summary
- Agent quick input

### Research

- Research documents
- Research tasks
- Sources
- Notes
- Artifacts

### Journal

- Investment theses
- Supporting and contradicting evidence
- Confidence level
- Validation date
- Outcome and review

### Portfolio

- Accounts
- Positions
- Exposure
- Concentration
- Theme allocation

### Settings

- AI providers
- API key status
- Data providers
- Local storage
- Plugins
- Privacy
- Diagnostics

---

## 6. Explicitly Out of Scope for the MVP

- Securities order execution
- Brokerage trading integration
- Social community features
- Third-party plugin marketplace
- Complex autonomous multi-agent orchestration
- Professional real-time market terminal
- Full TradingView-style charting platform
- Cloud-based team collaboration
- Full mobile applications
- Automated stock recommendations
- Autonomous investment decisions

---

## 7. Definition of Done

A feature is complete only when:

- [ ] The user workflow works end to end.
- [ ] TypeScript reports no errors.
- [ ] Rust compiles successfully.
- [ ] ESLint passes.
- [ ] `cargo fmt` passes.
- [ ] `cargo clippy` passes.
- [ ] Relevant unit tests pass.
- [ ] Critical integration tests pass.
- [ ] Loading states are implemented.
- [ ] Empty states are implemented.
- [ ] Error states are implemented.
- [ ] Permission boundaries are explicit.
- [ ] Documentation is updated.
- [ ] No unexplained temporary code remains.
- [ ] No plaintext secrets are present.
- [ ] Existing behavior is not unintentionally broken.

---

## 8. First Runnable Milestone

The first milestone is complete when the following loop works:

```text
Launch the application
→ Enter a research task
→ Rust creates a background task
→ React receives live progress events
→ The task produces structured output
→ A temporary Artifact window opens
→ The user closes the window
→ The result remains persisted in SQLite
```

Once this loop works, Research, Portfolio, Journal, and future plugins can be built on the same foundation.
