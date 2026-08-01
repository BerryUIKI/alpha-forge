# AlphaForge Milestone Roadmap

> Vision: Build an AI-native investment research operating environment

## Core Product Loop

```text
Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
```

---

## Milestone Overview

| Milestone | Status | Timeline | Description |
|-----------|--------|----------|-------------|
| M0 | ✅ Complete | Week 1-2 | Project Foundation |
| M1 | ✅ Complete | Week 3-4 | Desktop Runtime Foundation |
| M1.5 | ✅ Complete | Week 5-6 | Application Foundation |
| M2 | ✅ Complete | Week 7-10 | Agent Runtime |
| M3 | ✅ Complete | Week 11-14 | Artifact Intelligence System |
| M4 | ✅ Complete | Week 15-18 | Research Workspace |
| M5 | ✅ Complete | Week 19-22 | Investment Knowledge System |
| M6 | ✅ Complete | Week 23-26 | Portfolio Intelligence |
| M7 | ✅ Complete | Week 27-30 | Plugin Ecosystem |
| M8 | 📅 Future | TBD | Production & Commercialization |

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
| Layer | Technology |
|-------|-----------|
| Desktop Shell | Tauri 2 |
| Backend | Rust, Tokio, SQLx |
| Frontend | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui |
| Quality | ESLint, Prettier, Vitest, Clippy |

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

## M2 — Agent Runtime ✅

**Status**: Complete

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
| State | Description |
|-------|-------------|
| `queued` | Waiting for execution |
| `running` | Actively executing |
| `waiting_for_input` | Awaiting user input |
| `completed` | Successfully finished |
| `failed` | Error occurred |
| `cancelled` | User cancelled |

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

## M3 — Artifact Intelligence System ✅

**Status**: Complete

### Goal
Enable Agents to create interactive research experiences.

### Deliverables

#### Artifact Runtime
- ✅ Structured output schema
- ✅ Temporary window creation
- ✅ Renderer registry and React component system
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

## M4 — Research Workspace 📋

**Status**: Planned

### Goal
Turn AlphaForge into a complete AI research environment.

### Deliverables

#### Research Projects
- [ ] Project CRUD operations
- [ ] Document management
- [ ] Source management
- [ ] Notes system
- [ ] Report generation

#### Document Intelligence
- [ ] PDF parsing
- [ ] Web source extraction
- [ ] Local document indexing
- [ ] Semantic search

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
| Entity | Purpose |
|--------|---------|
| ResearchProject | Container for research work |
| Document | PDFs, web pages, notes |
| Source | External references with provenance |
| Note | User annotations |
| Report | Generated outputs |

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

## M5 — Investment Knowledge System 🚧

**Status**: In Progress

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
- 🚧 Migration reconciliation for legacy schemas (in progress)

#### Frontend (TypeScript)
- ✅ Thesis management UI in the Journal workspace
- ✅ Evidence collection interface with supporting and contradicting evidence
- ✅ Confidence visualization, immutable history, and lifecycle controls

#### Tauri Commands
| Command | Description |
|---------|-------------|
| `create_thesis` | Create a new investment thesis |
| `get_thesis` | Get thesis by ID |
| `list_theses` | List all theses in workspace |
| `activate_thesis` | Activate a draft thesis |
| `start_thesis_validation` | Begin validation process |
| `complete_thesis_validation` | Record validation outcome |
| `update_thesis_confidence` | Update confidence score |
| `close_thesis` | Close a thesis |
| `delete_thesis` | Delete a thesis |
| `add_thesis_evidence` | Add supporting/contradicting evidence |
| `list_thesis_evidence` | List all evidence for thesis |
| `delete_thesis_evidence` | Remove evidence |
| `list_thesis_confidence_history` | Review confidence changes over time |

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

## M8 — Production & Commercialization 📅

**Status**: Future

### Goal
Prepare AlphaForge as a commercial product.

### Deliverables

#### Infrastructure
- [ ] User authentication
- [ ] Licensing system
- [ ] Subscription management
- [ ] Cloud backup (optional)

#### Marketplace
- [ ] Plugin marketplace
- [ ] Research templates
- [ ] Community contributions

#### Deployment
- [ ] Application icons
- [ ] Installer packaging (DMG, MSI, AppImage)
- [ ] Auto-update infrastructure
- [ ] Release automation

### Release Readiness
- [ ] Performance optimization
- [ ] Security hardening
- [ ] Documentation review
- [ ] Legal review
- [ ] Support infrastructure

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
- [ ] Workspace persistence (M1.5)
- [ ] Agent task execution (M2)
- [ ] Progress event streaming (M2)
- [ ] Structured output (M2)
- [ ] Artifact rendering (M3)
- [ ] Result persistence (M3)

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
- M8 prepares for scale

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
**M1.5 — Application Foundation** 🚧

### Next Milestones
1. Complete workspace persistence
2. Expand desktop API
3. Build UI foundation
4. Establish testing framework

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
