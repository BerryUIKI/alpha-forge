# Artifact System

## What Is an Artifact

An artifact is a **temporary, interactive window** that renders structured output from an agent task. It is the bridge between agent reasoning and human decision-making.

Instead of reading a wall of text about a company comparison, the user sees an interactive comparison table. Instead of a paragraph describing portfolio risk, they explore a risk dashboard.

## Why Artifacts

Text is a dead end. Once read, it scrolls away. There is no interaction, no exploration, no persistence path.

Artifacts solve this:

- **Interaction.** Users can sort, filter, drill down, and explore agent output.
- **Structured data.** Agent output is validated JSON, not free-form prose.
- **Controlled rendering.** Each artifact type has a dedicated, pre-built renderer — no arbitrary HTML from the agent.
- **Temporary by default.** Artifacts open in sandboxed windows. Users close them when done, or persist them for later.

## Artifact Flow

```text
User submits task
  → Agent executes
    → Agent produces structured JSON
      → JSON matches a registered artifact type
        → Artifact runtime creates a temporary WebView window
          → Plugin renderer draws the interactive content
            → User interacts, then closes or persists
```

## Lifecycle

```text
Pending → Generating → Completed → Viewing → Closed
                    → Failed
```

| State | Meaning |
|-------|---------|
| **Pending** | Artifact record created, agent not yet started. |
| **Generating** | Agent is producing the artifact data. |
| **Completed** | Structured data is ready. Waiting for user to open. |
| **Viewing** | Artifact window is open. User is interacting. |
| **Closed** | Window closed. Data still persisted if user chose to save. |

## Input Model

Artifacts receive a validated JSON input from the agent. The input must conform to a schema defined by the artifact type.

Example input for a company comparison artifact:

```json
{
  "companies": [
    {
      "ticker": "AAPL",
      "name": "Apple Inc.",
      "metrics": {
        "pe_ratio": 28.5,
        "revenue_growth": 0.05,
        "market_cap": 2800000000000
      }
    }
  ],
  "comparisonDimensions": ["pe_ratio", "revenue_growth", "market_cap"]
}
```

## Output Model

Within the artifact window, the user can:

- View and interact with the rendered content.
- Apply filters, sorts, and toggles (defined by the plugin).
- Export as screenshot or PDF (future).
- Close the window (discard) or persist the artifact to the workspace.

## Rendering Model

Artifacts are rendered by **plugins** — pre-built React components registered with the system. The agent never generates HTML directly.

```text
Structured JSON
  → Artifact Runtime validates against schema
    → Routes to matching Plugin
      → Plugin renders React component in WebView
```

This ensures:

- **Security.** No arbitrary code execution. Plugins are pre-vetted.
- **Consistency.** Each artifact type renders the same way every time.
- **Quality.** Rich, interactive UIs that a text-only agent could not produce.

## Permission Model

Artifact windows run with **minimal permissions** — significantly less than the main application window.

| Capability | Main Window | Artifact Window |
|-----------|-------------|-----------------|
| Tauri core | Full | Default only |
| Shell access | Not registered | None |
| Filesystem | Controlled | None |
| Network | Rust services only; no frontend HTTP plugin | None |
| Store (persistence) | Full | None |
| API keys | Never exposed | Never exposed |

Artifacts cannot:

- Access the filesystem.
- Make network requests.
- Read API keys or credentials.
- Execute shell commands.
- Access the main window's DOM or state.

## Communication Protocol

Artifacts communicate with the main application through a narrow, typed message protocol:

```text
Artifact Window → Main Window:
  - close (user closes the window)
  - persist (user wants to save)
  - action { type, payload } (plugin-specific action)

Main Window → Artifact Window:
  - update { data } (data has changed)
  - theme { mode } (light/dark theme change)
```

No arbitrary message passing. Every message type is defined in the artifact SDK.

## Persistence

When a user chooses to persist an artifact:

1. The artifact's structured data is saved to SQLite.
2. A reference is stored in the workspace.
3. The artifact appears in the Artifacts page for later recall.
4. Reopening re-renders the artifact from saved data.

Artifacts that are not persisted are discarded when the window closes.

## MVP Artifact Types

Five internal plugins provide the initial artifact types:

| Plugin | Artifact Type | Description |
|--------|--------------|-------------|
| Company Comparison | `company-comparison` | Side-by-side financial comparison table |
| Valuation Model | `valuation-model` | DCF and multiples-based valuation |
| Portfolio Risk | `portfolio-risk` | Risk exposure and concentration dashboard |
| Industry Map | `industry-map` | Visual industry landscape and positioning |
| Research Timeline | `research-timeline` | Chronological view of research and thesis development |
