# System Design

AlphaForge is composed of nine major subsystems. Each has a clear purpose, bounded responsibilities, and defined interfaces.

---

## 1. Application Shell

**Purpose.** Provide the structural UI framework — layout, navigation, routing, theme, and global state infrastructure.

**Responsibilities.**

- Render the main application window with sidebar and content area.
- Route between pages: Today, Research, Journal, Portfolio, Settings.
- Manage global providers: QueryClient, theme, error boundary, toast notifications.
- Handle keyboard shortcuts and command palette.
- Coordinate loading, empty, error, and offline states at the app level.

**Dependencies.** None — it is the root that composes all other subsystems.

**Future extension points.** Multi-window support, customizable layouts, status bar.

---

## 2. Agent Runtime

**Purpose.** Execute research tasks asynchronously, streaming progress back to the UI.

**Responsibilities.**

- Accept task input from the user.
- Return a `task_id` immediately (non-blocking).
- Run the task in a background Tokio runtime.
- Stream progress events via Tauri events.
- Support cancellation, timeout, retry, and concurrency limits.
- Produce structured output (not free-form text).
- Persist task events to SQLite.

**Inputs.** User text input, research context, tool configurations.

**Outputs.** Structured JSON output, progress events, final artifact data.

**Dependencies.** Provider Layer (AI), Persistence Layer (SQLite), Event System (Tauri events).

**Future extension points.** Multi-agent coordination, custom tool registration, streaming partial results.

---

## 3. Task System

**Purpose.** Track the lifecycle of every agent task from creation to completion.

**Responsibilities.**

- Maintain task state machine: `queued → running → waiting_for_input → completed | failed | cancelled`.
- Enforce concurrency limits.
- Enforce timeouts and cost budgets.
- Persist task history.
- Support task resumption after application restart.

**Inputs.** Task creation requests.

**Outputs.** Task status updates, task history query responses.

**Dependencies.** Agent Runtime, Persistence Layer.

---

## 4. Event System

**Purpose.** Decouple subsystems through typed, streamed events.

**Responsibilities.**

- Emit agent events: started, thinking, tool_call, tool_result, streaming, completed, failed.
- Emit artifact events: opened, updated, closed.
- Emit system events: database migration complete, error.
- Allow React to subscribe to specific event types via Tauri's event listener API.

**Inputs.** Events emitted by Rust subsystems.

**Outputs.** Typed event payloads received by React components.

**Dependencies.** Tauri event system.

---

## 5. Research Engine

**Purpose.** Store, organize, and retrieve research documents, sources, and notes.

**Responsibilities.**

- CRUD operations on research documents.
- Link sources to documents with provenance metadata (URL, retrieval timestamp).
- Attach notes to documents.
- Search across documents and notes.
- Provide context for agent tasks.

**Inputs.** User-uploaded documents, agent-generated research, external sources.

**Outputs.** Structured research records, search results, agent context.

**Dependencies.** Persistence Layer, Provider Layer (for external source retrieval).

**Future extension points.** Full-text search, document chunking and embedding, research templates.

---

## 6. Artifact Runtime

**Purpose.** Render structured agent output as interactive, temporary windows.

**Responsibilities.**

- Validate artifact manifests and input schemas.
- Create temporary WebView windows with minimal permissions.
- Render structured data using predefined React components.
- Support user interaction within the artifact window.
- Allow users to persist or discard artifacts.
- Enforce strict permission isolation from the main window.

**Inputs.** Structured JSON from agent output.

**Outputs.** Interactive WebView windows.

**Dependencies.** Plugin System (rendering components), Tauri (window management), Security (permission enforcement).

**Future extension points.** Screenshot/PDF export, artifact sharing, custom renderers.

---

## 7. Plugin System

**Purpose.** Enable extensible artifact rendering through internal plugins.

**Responsibilities.**

- Register plugins with manifest, version, permissions, and input schema.
- Validate manifests at load time.
- Load and unload plugins without restarting the application.
- Isolate plugin errors from the main application.
- Provide a typed communication bridge between plugins and the host.

**Inputs.** Plugin manifests and rendering components.

**Outputs.** Rendered artifact windows, plugin lifecycle events.

**Dependencies.** Artifact Runtime, Security (permission enforcement).

**Future extension points.** Third-party plugin marketplace, plugin hot-reload during development, sandboxed plugin execution.

---

## 8. Persistence Layer

**Purpose.** Provide reliable local storage for all application data.

**Responsibilities.**

- Manage SQLite connection pool with WAL mode and foreign key enforcement.
- Run append-only database migrations.
- Provide repository-pattern data access.
- Keep database rows separate from domain models.
- Map database errors to typed application errors.

**Inputs.** Read/write requests from other subsystems.

**Outputs.** Typed domain objects.

**Dependencies.** SQLite, SQLx.

**Future extension points.** Optional backup/export, query optimization, migration rollback (development only).

---

## 9. Provider Layer

**Purpose.** Abstract external services behind stable interfaces.

**Responsibilities.**

- Define provider traits: AI, market data, news.
- Implement adapters for specific services (OpenAI, etc.).
- Handle rate limiting, retries, and timeouts.
- Never expose API keys to the frontend.

**Inputs.** Service configuration, API keys (from OS keychain).

**Outputs.** Structured data from external services.

**Dependencies.** Security (credential storage), Networking (reqwest).

**Future extension points.** Additional AI providers (Anthropic, local models), market data providers, news APIs.

---

## Dependency Direction

```text
Application Shell
  ├── Agent Runtime ──────► Provider Layer
  │     └── Task System
  ├── Research Engine ─────► Persistence Layer
  ├── Artifact Runtime ────► Plugin System
  │     └── Event System
  └── Security
```

Dependencies flow downward. No subsystem depends on the Application Shell. The Persistence Layer and Provider Layer are leaves — they depend on nothing internal.
