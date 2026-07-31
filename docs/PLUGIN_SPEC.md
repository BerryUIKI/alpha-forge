# Plugin Specification

Plugins are the rendering layer for artifacts. Each plugin defines one artifact type — how its structured data should be displayed as an interactive window.

## Manifest

Every plugin must include a `manifest.json`:

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

### Fields

| Field | Required | Description |
|-------|----------|-------------|
| `id` | Yes | Unique plugin identifier. Used to route artifacts. |
| `name` | Yes | Human-readable display name. |
| `version` | Yes | SemVer version string. |
| `entry` | Yes | Entry HTML file for the artifact WebView. |
| `inputSchema` | Yes | Path to a JSON Schema file defining valid artifact input. |
| `permissions` | Yes | List of required capabilities (empty = no special permissions). |
| `window` | Yes | Default window dimensions and behavior. |

## Versioning

Plugins use semantic versioning (`MAJOR.MINOR.PATCH`).

- **MAJOR:** Breaking changes to the input schema or rendering contract.
- **MINOR:** New features or rendering improvements, backward-compatible.
- **PATCH:** Bug fixes, no schema or rendering changes.

The artifact runtime validates that a plugin version is compatible with the artifact data before rendering.

## Permissions

Plugins declare required permissions in their manifest. All permissions are denied by default.

| Permission | Description |
|-----------|-------------|
| (none) | Plugin renders static structured data. No external access needed. |
| `network:chart-data` | Plugin may fetch chart data from an allowed provider. |
| `filesystem:export` | Plugin may trigger a save dialog for export (PDF, CSV). |

In the MVP, no permissions are granted beyond `core:default`. All plugins are internal and statically reviewed.

## Input Schema

Each plugin defines a JSON Schema for its expected input. The artifact runtime validates agent output against this schema before rendering.

Example schema for `company-comparison`:

```json
{
  "type": "object",
  "required": ["companies", "comparisonDimensions"],
  "properties": {
    "companies": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["ticker", "name"],
        "properties": {
          "ticker": { "type": "string" },
          "name": { "type": "string" },
          "metrics": { "type": "object" }
        }
      }
    },
    "comparisonDimensions": {
      "type": "array",
      "items": { "type": "string" }
    }
  }
}
```

Validation fails (and the artifact does not render) if agent output does not match the schema.

## Lifecycle

```text
Registered → Loaded → Active → Unloaded
```

| State | Trigger |
|-------|---------|
| **Registered** | Plugin manifest is discovered and validated at application startup. |
| **Loaded** | Plugin code is loaded when the first artifact of its type is requested. |
| **Active** | Plugin is rendering an artifact window. |
| **Unloaded** | Plugin is unregistered or the application shuts down. |

## Error Isolation

- A plugin crash or render error must not affect the main application window.
- The artifact window displays an error state if rendering fails.
- Plugin errors are logged with the plugin ID and version for diagnosis.

## MVP Plugins

Five internal plugins are included in the initial release:

| Plugin ID | Artifact Type |
|-----------|--------------|
| `company-comparison` | Side-by-side financial metric comparison table |
| `valuation-model` | DCF and multiples-based company valuation |
| `portfolio-risk` | Portfolio risk exposure and concentration analysis |
| `industry-map` | Visual industry landscape and competitive positioning |
| `research-timeline` | Chronological research and thesis development timeline |

All MVP plugins are internal — no third-party plugin loading is supported.

## Future: Plugin Marketplace

Post-MVP, a curated plugin marketplace may allow third-party plugins. This will require:

- Plugin signing and verification.
- Sandboxed execution (separate process per plugin).
- Permission review process.
- Version compatibility checking.

This is explicitly out of scope for the MVP.
