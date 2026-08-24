# AlphaForge Milestone Roadmap

> Vision: Build an AI-native investment research operating environment

**Status reviewed:** 2026-08-15

## Core Product Loop

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

---

## Milestone Overview

| Milestone | Status                    | Timeline              | Description                              |
| --------- | ------------------------- | --------------------- | ---------------------------------------- |
| M0        | ✅ Complete               | Week 1-2              | Project Foundation                       |
| M1        | ✅ Complete               | Week 3-4              | Desktop Runtime Foundation               |
| M1.5      | ✅ Complete               | Week 5-6              | Application Foundation                   |
| M2        | ✅ Stabilized (S1)        | Week 7-10             | Agent Runtime                            |
| M3        | ✅ Stabilized (S3)        | Week 11-14            | Artifact Intelligence System             |
| M4        | ✅ Complete               | Week 15-18            | Research Workspace                       |
| M5        | ✅ Complete               | Week 19-22            | Investment Knowledge System              |
| M6        | ✅ Complete               | Week 23-26            | Portfolio Intelligence                   |
| M7        | ✅ Stabilized (S3)        | Week 27-30            | Internal Plugin Ecosystem                |
| M8        | ✅ Complete (S6)          | Rebaseline 2026-08-24 | Local MVP Completion & Release Readiness |
| M9        | ✅ Complete (S5)          | Rebaseline 2026-08-24 | Option Module Integration                |
| M10       | ✅ Complete (#161-#166)   | Rebaseline 2026-08-24 | Goose Agent Integration (Supervised Sidecar) |

## Delivery document registry

This roadmap is the program entry point. Implementation agents must follow the linked execution documents for the active milestone.

| Milestone/workstream | Supporting documents                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| All milestones       | [Sequential Task Breakdown](milestones/SEQUENTIAL_TASK_BREAKDOWN.md), [Delivery Playbook](milestones/DELIVERY_PLAYBOOK.md), [Git Workflow](GIT_WORKFLOW.md), [Architecture](ARCHITECTURE.md), [Security](SECURITY.md)                                                                                                                                                                                                                                                                                                                                                                                           |
| M8 i18n              | [i18n Index](i18n/README.md), [i18n Architecture](i18n/ARCHITECTURE.md), [i18n Implementation Plan](i18n/IMPLEMENTATION_PLAN.md), [Terminology Guide](i18n/TERMINOLOGY_GUIDE.md), [String Inventory](i18n/STRING_INVENTORY.md), [M8 Decision Record](M8_DECISION_RECORD.md)                                                                                                                                                                                                                                                                                                                                     |
| M9 Option            | [Option Index](option/README.md), [Product](option/PRODUCT.md), [Use Cases](option/USE_CASES.md), [Architecture](option/ARCHITECTURE.md), [Data Model](option/DATA_MODEL.md), [API Specification](option/API_SPEC.md), [Roadmap](option/ROADMAP.md), [Implementation Details](option/IMPLEMENTATION_DETAILS.md), [Integration Plan](option/INTEGRATION_PLAN.md), [Git Workflow](option/GIT_WORKFLOW.md), [ADR-0005 Pricing](DECISIONS/0005-option-pricing-models.md), [ADR-0006 Providers](DECISIONS/0006-option-data-providers.md), [ADR-0007 Artifact Isolation](DECISIONS/0007-option-artifact-isolation.md) |
| M10 Goose            | [Goose Index](goose/README.md), [Goose Integration Roadmap](goose/INTEGRATION_ROADMAP.md), [Agent Protocol](AGENT_PROTOCOL.md), [Artifact System](ARTIFACT_SYSTEM.md), [Plugin Specification](PLUGIN_SPEC.md)                                                                                                                                                                                                                                                                                                                                                                                                   |

Status in this file is authoritative at the program level. Detailed documents own work-package checklists; historical branch names and focused layer-level tests do not override the acceptance gates.

The current corrective program is defined by the [Stabilization Roadmap](STABILIZATION_ROADMAP.md). Evidence for the 2026-08-12 rebaseline is recorded in the [Frontend-Backend Integration and Functional Completeness Audit](reviews/INTEGRATION_GAP_AUDIT_2026-08-12.md).

---

## M0 — Project Foundation ✅

**Status**: Complete

### Goal

Create a stable foundation for AI-assisted development.

### Deliverables

- ✓ Repository structure (pnpm + Cargo workspaces)
- ✓ Documentation system
- ✓ `AGENTS.md` — coding standards and agent rules
- ✓ Git workflow documentation
- ✓ Architecture Decision Records
- ✓ Complete documentation suite

### Key Documents

- [AGENTS.md](../AGENTS.md) — Development guidelines
- [docs/ARCHITECTURE.md](ARCHITECTURE.md) — System architecture
- [docs/GIT_WORKFLOW.md](GIT_WORKFLOW.md) — Branching and PR process

### Acceptance Criteria

```text
Repository initialized
    ↓
Documentation complete
    ↓
Branch strategy established
    ↓
Ready for development
```

---

## M1 — Desktop Runtime Foundation ✅

**Status**: Complete

### Goal

Build the native desktop application foundation.

### Deliverables

- ✓ Tauri 2 application shell
- ✓ Rust backend structure
- ✓ React frontend setup
- ✓ TypeScript + Vite configuration
- ✓ IPC communication layer
- ✓ SQLite migration system (SQLx)
- ✓ Basic application shell

### Tech Stack

| Layer         | Technology                       |
| ------------- | -------------------------------- |
| Desktop Shell | Tauri 2                          |
| Backend       | Rust, Tokio, SQLx                |
| Frontend      | React 19, TypeScript, Vite 6     |
| UI            | Tailwind CSS 4, shadcn/ui        |
| Quality       | ESLint, Prettier, Vitest, Clippy |

### Acceptance Criteria

```text
Launch application
    ↓
React shell renders
    ↓
IPC commands work
    ↓
SQLite initializes
    ↓
Application closes cleanly
```

---

## M1.5 — Application Foundation ✅

**Status**: Complete

### Goal

Transform the technical skeleton into an extensible application platform.

### Deliverables

#### Backend (Rust)

- ✅ Service layer architecture
- ✅ Repository layer abstraction
- ✅ Workspace domain models
- ✅ Workspace persistence
- ✅ Desktop API expansion (22+ commands)

#### Frontend (React)

- ✅ Shared UI foundation (shadcn/ui integration)
- ✅ Error/loading states
- ✅ Toast notification system
- ✅ Theme support (light/dark)
- ✅ Global error boundary

#### Testing

- ✅ Vitest test suite setup
- ✅ Component tests
- ✅ Hook tests
- ✅ IPC integration tests
- ✅ Repository tests (48 tests total)

### Architecture Layers

```text
┌─────────────────────────────────────┐
│         React Frontend              │
│  Pages, Components, UI State        │
└──────────────┬──────────────────────┘
               │ IPC (Tauri)
┌──────────────▼──────────────────────┐
│         Rust Backend                │
│  Commands → Services → Repositories │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         SQLite Database             │
└─────────────────────────────────────┘
```

### Acceptance Criteria

```text
User creates Workspace
    ↓
Workspace saved locally
    ↓
Application restarts
    ↓
Workspace restored
    ↓
All tests pass
```

### Definition of Done

- [ ] All deliverables complete
- [ ] Tests written and passing
- [ ] Documentation updated
- [ ] Code reviewed
- [ ] Demo recording created

---

## M2 — Agent Runtime ⚠️

**Status**: Lifecycle and credential repairs are merged (#80, #81); milestone acceptance remains incomplete pending the full Agent-to-Artifact verification gates.

### Goal

Create the core intelligence engine of AlphaForge.

### Deliverables

#### Agent Task System

- ✅ Task lifecycle management
- ✅ Background execution (Tokio)
- ✅ Event streaming (Tauri events)
- ✅ Cancellation support
- ✅ Retry handling (infrastructure ready)
- ✅ Timeout enforcement

#### Context System

- ✅ Workspace context
- 📋 Conversation context (Future)
- 📋 Research context (Future)

#### Tool System

- 📋 Document tools (Future)
- 📋 Search tools (Future)
- 📋 Data tools (Future)
- 📋 Calculation tools (Future)

#### Provider Integration

- ✅ Typed provider contract and validated structured research-output parser
- ✅ OpenAI Responses API adapter with keychain-only credential lookup, bounded output, and strict structured output
- ✅ Agent executor routes tasks through the configured provider with timeout and cancellation handling
- ✅ Structured research output is persisted as a task-completion event
- ✅ Safe provider failure messages are persisted and streamed without exposing credentials or raw provider responses

### Task Lifecycle

```text
Create Task
    ↓
Queue Task
    ↓
Execute (Background)
    ↓
Stream Progress Events
    ↓
Complete/Failed/Cancelled
    ↓
Persist Result
```

### Task States

| State               | Description           |
| ------------------- | --------------------- |
| `queued`            | Waiting for execution |
| `running`           | Actively executing    |
| `waiting_for_input` | Awaiting user input   |
| `completed`         | Successfully finished |
| `failed`            | Error occurred        |
| `cancelled`         | User cancelled        |

### Acceptance Criteria

```text
User inputs research request
    ↓
Agent creates task
    ↓
Agent executes workflow
    ↓
Structured result returned
    ↓
Result persisted in SQLite
```

---

## M3 — Artifact Intelligence System ⚠️

**Status**: Persistence, renderers, permission primitives, and the isolated Artifact-window route are merged (#88) with focused verification; packaged smoke acceptance remains pending, so milestone acceptance remains incomplete.

### Goal

Enable Agents to create interactive research experiences.

### Deliverables

#### Artifact Runtime

- ✅ Structured output schema
- ✅ Temporary window creation
- ✅ Renderer system (predefined React components)
- ✅ Permission isolation

#### Persistence Layer

- ✅ Artifact database migration
- ✅ ArtifactRepository with CRUD operations
- ✅ ArtifactService with business logic
- ✅ Artifact Tauri commands (11 commands)
- ✅ Frontend artifact API and hooks

#### Built-in Artifacts

- ✅ Comparison table renderer
- ✅ Timeline renderer
- ✅ Industry map renderer
- ✅ Valuation model renderer
- ✅ Risk dashboard renderer

### Artifact Flow

```text
Agent produces validated JSON
    ↓
Artifact manifest validated
    ↓
Temporary WebView created
    ↓
Renderer plugin activated
    ↓
Interactive content displayed
    ↓
User explores and closes
    ↓
Result persisted (optional)
```

### Permission Model

Artifacts are isolated from main application:

- ✓ Receive validated JSON input only
- ✗ No SQLite access
- ✗ No filesystem access
- ✗ No API keys
- ✗ No shell execution

### Verification

- ✅ Rust repository and artifact-runtime tests cover artifact persistence, safe window configuration, and renderer data flow
- ✅ Frontend artifact API and renderer registry are covered by the workspace test suite

### Acceptance Criteria

```text
Agent generates research result
    ↓
Interactive Artifact opens
    ↓
User explores content
    ↓
Artifact closes cleanly
```

---

## M4 — Research Workspace ✅

**Status**: Complete

### Goal

Turn AlphaForge into a complete AI research environment.

### Deliverables

#### Research Projects

- [x] Project CRUD operations
- [x] Document management
- [x] Source management with recorded provenance and validated public HTTPS links
- [x] Notes system
- [x] Report persistence

#### Document Intelligence

- [x] Local content parsing, chunking, and lexical query-ranking primitives
- [x] PDF parsing through a Rust-owned native picker (25 MB limit; extracted text only)
- [x] Web source extraction through bounded Rust-side HTTPS retrieval
- [x] Local semantic ranking for related investment terms

The current local primitives normalize plain text and HTML supplied to the app, extract text from a user-selected PDF in Rust, retrieve validated public HTTPS pages in Rust, split content into deterministic chunks, and rank matching chunks through the Research page. PDF imports are limited to 25 MB and persist extracted text and title, never the selected local path. Web imports accept HTML or plain text only, cap responses at 5 MB, use a 15-second timeout, validate every redirect (maximum three), and preserve source provenance. Semantic mode is local and explainable: it expands a curated investment vocabulary (for example, revenue/sales and earnings/profit), while exact matches retain the highest score. It does not send content to an external embedding provider or generate investment recommendations.

#### Research Workflow

```text
Collect Sources
    ↓
Analyze Documents
    ↓
Generate Thesis
    ↓
Create Report
    ↓
Persist Knowledge
```

### Data Model

| Entity          | Purpose                             |
| --------------- | ----------------------------------- |
| ResearchProject | Container for research work         |
| Document        | PDFs, web pages, notes              |
| Source          | External references with provenance |
| Note            | User annotations                    |
| Report          | Generated outputs                   |

### Acceptance Criteria

```text
User creates research project
    ↓
Adds documents and sources
    ↓
Agent assists with analysis
    ↓
User generates report
    ↓
All artifacts persisted
```

---

## M5 — Investment Knowledge System ✅

**Status**: Complete

### Goal

Build persistent investment intelligence.

### Deliverables

#### Thesis Management

- [x] Investment thesis CRUD
- [x] Evidence collection
- [x] Counter-evidence tracking
- [x] Confidence scoring
- [x] Review history
- [x] Validation scheduling

#### Knowledge Graph

- ✅ Company entities
- ✅ Industry entities
- ✅ Technology entities
- ✅ Macro theme entities
- ✅ Relationship mapping and thesis links

### Knowledge Graph Example

```text
NVIDIA
    ↓ (produces)
CUDA Platform
    ↓ (enables)
AI Infrastructure
    ↓ (requires)
Data Center Hardware
    ↓ (depends on)
HBM Memory
```

### Thesis Lifecycle

```text
Create Thesis
    ↓
Collect Evidence
    ↓
Track Confidence
    ↓
Schedule Validation
    ↓
Record Outcome
    ↓
Review and Improve
```

### Acceptance Criteria

```text
User creates investment thesis
    ↓
Links to companies and themes
    ↓
Collects supporting evidence
    ↓
Tracks confidence over time
    ↓
Records validation results
```

### Implementation

#### Backend (Rust)

- ✅ Domain models: `InvestmentThesis`, `ThesisEvidence`, `ThesisStatus`, `EvidenceDirection`
- ✅ Database migration: `0006_theses.sql`
- ✅ Thesis repository with CRUD operations
- ✅ Thesis service with business logic
- ✅ 13 Tauri commands for thesis management
- ✅ 13 repository tests passing
- ✅ Migration reconciliation for legacy schemas

#### Frontend (TypeScript)

- ✅ Thesis management UI in the Journal workspace
- ✅ Evidence collection interface with supporting and contradicting evidence
- ✅ Confidence visualization, immutable history, and lifecycle controls

#### Tauri Commands

| Command                          | Description                           |
| -------------------------------- | ------------------------------------- |
| `create_thesis`                  | Create a new investment thesis        |
| `get_thesis`                     | Get thesis by ID                      |
| `list_theses`                    | List all theses in workspace          |
| `activate_thesis`                | Activate a draft thesis               |
| `start_thesis_validation`        | Begin validation process              |
| `complete_thesis_validation`     | Record validation outcome             |
| `update_thesis_confidence`       | Update confidence score               |
| `close_thesis`                   | Close a thesis                        |
| `delete_thesis`                  | Delete a thesis                       |
| `add_thesis_evidence`            | Add supporting/contradicting evidence |
| `list_thesis_evidence`           | List all evidence for thesis          |
| `delete_thesis_evidence`         | Remove evidence                       |
| `list_thesis_confidence_history` | Review confidence changes over time   |

---

## M6 — Portfolio Intelligence ✅

**Status**: Complete

### Goal

Connect research with actual investment decisions.

### Deliverables

#### Portfolio Management

- [x] Account management — workspace-scoped accounts with a desktop UI
- [x] Holdings tracking — manual position capture and account-level review
- [x] Transaction import — validated CSV import into immutable account history
- [x] Allocation analysis — workspace cost-basis allocation by symbol
- [x] Exposure calculation — per-symbol and cross-account concentration weights

#### AI Analysis

- [x] Risk concentration analysis — transparent cost-basis thresholds (moderate 10%, high 25%)
- [x] Theme exposure mapping — explicit symbol-to-knowledge-entity links with cost-basis aggregation
- [x] Thesis alignment checking — transparent held-symbol matches against workspace thesis content
- [x] Historical review automation — on-demand review summarizing concentration and unaligned symbols

### Important Constraints

```text
✓ Portfolio tracking and analysis
✓ Research-thesis alignment
✓ Risk visualization
✗ NO automated trading
✗ NO autonomous investment decisions
✗ NO trade execution
```

### Acceptance Criteria

```text
User imports portfolio
    ↓
System analyzes holdings
    ↓
AI identifies risks
    ↓
Maps to research theses
    ↓
User reviews insights
```

---

## M7 — Plugin Ecosystem ✅

**Status**: Complete

### Goal

Make AlphaForge extensible.

### Deliverables

#### Plugin SDK

- [x] Plugin manifest specification — strict internal manifest validation and safe relative-path checks
- [x] Permission model — declared permissions are persisted and checked against the validated manifest
- [x] Input/output schemas — bundled JSON Schemas validate payloads before a plugin artifact is created
- [x] Artifact renderer API — plugin IDs map only to predefined artifact renderers; no generated HTML is evaluated
- [x] Lifecycle management — bundled manifests synchronize at startup; users can enable or disable a registered plugin

#### Official Plugins

1. **company-comparison** — Compare multiple companies side-by-side
2. **valuation-model** — Interactive valuation scenarios
3. **industry-map** — Visualize industry relationships
4. **earnings-analyzer** — Parse and analyze earnings reports
5. **macro-dashboard** — Macro indicator dashboard

### Plugin Architecture

```text
Plugin Manifest
    ↓
Validation
    ↓
Permission Check
    ↓
Input Validation
    ↓
Execution
    ↓
Output Rendering
```

### Acceptance Criteria

```text
Plugin registered
    ↓
Input schema validated
    ↓
Permission granted
    ↓
Plugin executes
    ↓
Artifact rendered
```

**Current boundary**: The five official plugins are bundled alongside the existing internal `portfolio-risk` and `research-timeline` tools. Dynamic plugin installation and arbitrary code execution are intentionally unsupported. A validated payload becomes a completed artifact rendered by a predefined component; no plugin source code is evaluated.

---

## M8 — Local MVP Completion & Release Readiness 🚧

**Status**: Reopened on 2026-08-12. Baseline, credential, Agent, Option schema/IPC, and System IPC repairs are merged, but release acceptance remains withdrawn until Artifact routing, remaining workflows, and full verification gates pass.

### Goal

Ship a safe, local-first, documentable desktop MVP and close the release-quality gaps without adding unapproved commercial services. Prepare AlphaForge for a free, open-source, local desktop MVP while deferring commercial services.

### Deliverables

#### Infrastructure

- [x] Record the local-only MVP decisions (launch market, locale, platform, privacy, support, release-owner)
- [x] Complete the `en` and `zh-CN` i18n foundation and critical-workflow rollout
- [x] Add user-controlled local SQLite backup export
- [x] Add privacy and manual-update controls in Settings
- [x] Provide user-controlled local data export and recovery documentation
- [ ] User authentication (deferred)
- [ ] Licensing system (deferred)
- [ ] Subscription management (deferred)
- [ ] Cloud backup (deferred)

#### Marketplace

- [x] Keep the third-party plugin marketplace deferred; MVP plugins remain internal
- [x] Document bundled research templates and internal plugin compatibility where shipped

#### Deployment

- [x] Finalize application identity and icons
- [x] Configure DMG and Windows EXE (NSIS) targets
- [x] Define signing/notarization, update, rollback, and release-ownership policy
- [x] Document manual GitHub Release checks
- [ ] Keep release credentials in approved CI secrets only
- [ ] Signing and release automation (release-owner operation)

### Release Readiness

- [x] Performance optimization
- [x] Security hardening
- [x] Documentation review
- [ ] Legal review
- [x] Support infrastructure
- [ ] Close all P0 findings in the 2026-08-12 integration audit
- [ ] Pass and retain the complete verification and packaged-smoke matrix

### Execution path

1. Resolve the [M8 Decision Record](M8_DECISION_RECORD.md).
2. Execute [i18n Implementation Plan](i18n/IMPLEMENTATION_PLAN.md) from decision/inventory through packaged QA.
3. Complete local export, privacy, installer, update, security, legal, and support gates in scoped vertical slices.
4. Run full repository checks plus macOS/Windows packaged smoke tests.
5. Record acceptance evidence and declare the local MVP complete before M9 or M10 implementation begins.

### Acceptance criteria

- Critical MVP workflows complete in both approved locales.
- Local data remains the source of truth and no unapproved cloud service or telemetry is enabled.
- Installation, first run, backup/export, failure recovery, update, and rollback paths are documented and tested on supported platforms.
- Security, privacy, investment-research disclaimer, and support contacts are approved.
- Standard tests and packaged smoke tests have retained evidence.

---

## M9 — Option Module Integration 🚧

**Status**: In execution. The rebaseline and decision gate (M9-01) is complete on 2026-08-17: the Option baseline was revalidated against current `dev` (canonical schema and Option IPC repairs merged via #83 and #84; chain-to-strategy UI merged via #95, #97, and #98), and pricing-model, data-provider, and Artifact-isolation decisions are approved in [ADR-0005](DECISIONS/0005-option-pricing-models.md), [ADR-0006](DECISIONS/0006-option-data-providers.md), and [ADR-0007](DECISIONS/0007-option-artifact-isolation.md). M9 acceptance remains incomplete until the remaining vertical slices and the release gate pass.

### Goal

Integrate evidence-grounded Option pricing, chain analysis, strategies, and portfolio-risk research as safe, tested vertical slices.

### Entry gate

- [x] M8 local MVP is complete.
- [x] [Option baseline](option/README.md) is revalidated against current `dev` (M9-01).
- [x] Pricing-model, provider, migration, and Artifact decisions are approved ([ADR-0005](DECISIONS/0005-option-pricing-models.md), [ADR-0006](DECISIONS/0006-option-data-providers.md), [ADR-0007](DECISIONS/0007-option-artifact-isolation.md)).
- [x] Historical `integration/option` code consensus: all historical Option branches have been merged into `dev`; no remaining unmerged candidate branch exists.

### Execution path

1. Retain the merged Option schema baseline and focused migration tests.
2. Integrate a pure pricing/provider core with independent numerical fixtures.
3. Deliver the Option-chain input-to-persistence UI slice.
4. Deliver strategy construction and a controlled Artifact renderer.
5. Deliver scenario and portfolio-risk integration with provenance.
6. Pass calculation, migration, security, accessibility, i18n, E2E, performance, and packaged-build gates.

Detailed steps are in [Option Integration Plan](option/INTEGRATION_PLAN.md) and [Implementation Details](option/IMPLEMENTATION_DETAILS.md).

### Acceptance criteria

- Required Option slices are accepted on `dev`; historical branch presence is not completion.
- Data, assumptions, timestamps, model versions, uncertainty, and source provenance are visible.
- Workspace isolation, background-task lifecycle, typed IPC, and Artifact permissions follow current architecture.
- No trading, brokerage execution, or autonomous recommendation capability exists.
- Independent domain review and real verification evidence are recorded.

---

## M10 — Goose Agent Integration ✅

**Status**: Complete. All 7 slices (M10-G0 through M10-G6) merged into `dev` via PRs #161, #162, #163, #164, #165, and #166.

### Goal

Use a version-pinned Goose runtime to improve evidence-grounded research while AlphaForge retains task control, permissions, credentials, persistence, and mandatory human review.

### Entry gate

- [x] M8 is complete; local MVP baseline established.
- [x] M9 is complete (Option module integrated in S5).
- [x] The upstream Goose source, license (Apache 2.0 / Linux Foundation AAIF), version, API/CLI, permission, recipe, and MCP behavior verified.
- [x] [ADR-0004](DECISIONS/0004-goose-integration-topology.md) and [ADR-0009](DECISIONS/0009-goose-credential-and-provider-policy.md) select supervised sidecar topology, OS keyring credential model, packaging, and support strategy.
- [x] Threat model and read-only MCP tool allowlist approved.

### Execution path

- [x] 1. Complete an isolated synthetic-data lifecycle spike (M10-G0/G1, #161).
- [x] 2. Add an allowlisted, read-only AlphaForge MCP bridge through Rust services with `AuthorizedScope` validation (M10-G2, #162).
- [x] 3. Ship opt-in shadow-mode research with structured output, TanStack Query hooks, and UI controls (M10-G3, #163).
- [x] 4. Add human-approved proposals with zero-trading guardrails persisting through Rust services (M10-G4, #164).
- [x] 5. Pass OS Keyring credential policy, provider/model allowlists, and secret redaction gates (M10-G5, #165).
- [x] 6. Establish packaging, release integrity, SBOM attribution, diagnostics API, and support operations (M10-G6, #166).

Detailed documentation: [Goose Integration Roadmap](goose/INTEGRATION_ROADMAP.md), [Credential Policy](goose/CREDENTIAL_POLICY.md), [Packaging and Support](goose/PACKAGING_AND_SUPPORT.md).

### Acceptance criteria

- [x] Goose has no direct SQLite, unrestricted filesystem, shell, secret, trade, or privileged Tauri access.
- [x] Only allowlisted, typed, bounded, workspace-scoped tools are available (`AuthorizedScope` enforced).
- [x] Outputs are structured, validated via Zod, source-grounded, persisted with provenance, and rendered through controlled Artifacts.
- [x] Domain writes require explicit user confirmation and pass through existing Rust domain services.
- [x] Cancellation, timeout, retry policy, concurrency, restart, token, cost, and output limits are tested.
- [x] Packaged builds verify binary integrity via SHA-256 and fail closed on mismatch.

---

## Deferred commercialization

Authentication, billing, licensing enforcement, cloud sync, telemetry, and a public plugin marketplace remain outside the local MVP and are not authorized by M9 or M10. They require a separate product, privacy, legal, security, and architecture decision milestone.

---

## First Runnable Milestone (M1.5 → M2)

The first complete product experience spans M1.5 through M2.

### User Journey

```text
Launch AlphaForge
    ↓
Create/select Workspace
    ↓
Enter research task
    ↓
Agent runs in background
    ↓
Live progress streams to UI
    ↓
Agent produces structured output
    ↓
Artifact window opens
    ↓
User explores interactive content
    ↓
Result persisted
    ↓
User reviews later
```

### Technical Requirements

- [x] Workspace persistence (M1.5)
- [x] Agent task execution (M2)
- [x] Progress event streaming (M2)
- [x] Structured output (M2)
- [x] Artifact rendering (M3)
- [x] Result persistence (M3)

---

## Development Principles

### 1. Documentation Before Implementation

Every feature starts with documentation:

- What problem does it solve?
- What is the user journey?
- What are the technical constraints?
- How does it fit the architecture?

### 2. Build Foundations Before Features

- M0 → M1 → M1.5 establish the platform
- M2 → M3 add intelligence capabilities
- M4 → M5 build research workflows
- M6 → M7 extend functionality
- M8 completes the local MVP release gate
- M9 integrates Option research after the MVP
- M10 integrates Goose only after the MVP boundary is complete

### 3. Complete Vertical Slices

Every milestone delivers an end-to-end experience:

```text
Input → Processing → Output → Persistence
```

Not isolated modules, but working features.

### 4. AI Capability Architecture

```text
Agent (Intelligence)
    +
Artifact (Visualization)
    +
Persistent Knowledge (Memory)
```

All three pillars must work together.

### 5. Product Identity Guardrails

**AlphaForge is:**

- An AI-native investment research workspace
- A thesis development and tracking tool
- A knowledge management system
- A research collaboration environment

**AlphaForge is NOT:**

- A trading terminal
- A stock recommendation engine
- A simple AI chatbot
- An automated trading system

---

## Progress Tracking

### Current Phase

**Stabilization S0 — Baseline truth and build recovery** is active. M8 and M9 remain reopened until the [Stabilization Roadmap](STABILIZATION_ROADMAP.md) acceptance gates are met.

### Completed stabilization repairs

- [x] Rust module-tree repair and pnpm workspace baseline (#78, #79)
- [x] OpenAI credential and Agent lifecycle contracts (#80, #81)
- [x] Canonical Option schema, Option IPC, and System IPC contracts (#83, #84, #85)

### Next Milestones

1. Complete packaged Artifact-window smoke acceptance and retain route/permission evidence.
2. Review the controlled company-comparison create-to-Artifact workflow.
3. Retain its validation, disabled-state, Artifact-window, and renderer evidence.
4. Enforce CI, E2E, packaged smoke, security, and release gates before re-accepting M8/M9.
5. Reverify upstream and integrate Goose as M10 only after stabilization acceptance.

### Long-term Vision

Transform AlphaForge into the definitive AI-powered investment research platform where professionals develop, test, and refine investment theses with persistent knowledge and interactive visualizations.

---

## References

- [Architecture Documentation](ARCHITECTURE.md)
- [Product Definition](PRODUCT.md)
- [Vision Statement](VISION.md)
- [Agent Protocol](AGENT_PROTOCOL.md)
- [Artifact System](ARTIFACT_SYSTEM.md)
- [Plugin Specification](PLUGIN_SPEC.md)
- [Security Model](SECURITY.md)
- [Development Guide](DEVELOPMENT.md)
- [Milestone Delivery Playbook](milestones/DELIVERY_PLAYBOOK.md)
- [Sequential Child-Agent Task Breakdown](milestones/SEQUENTIAL_TASK_BREAKDOWN.md)
- [i18n Documentation](i18n/README.md)
- [Option Documentation](option/README.md)
- [Goose Integration Documentation](goose/README.md)
