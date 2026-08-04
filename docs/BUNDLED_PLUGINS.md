# Bundled Research Templates and Internal Plugins

**Status**: MVP Internal Plugins Only
**Owner**: @BerryUIKI
**Last updated**: 2026-08-03

This document describes the bundled artifact renderers (internal plugins) shipped with AlphaForge MVP.

---

## Overview

AlphaForge MVP ships with a set of internal artifact renderers. These renderers are:

1. **Bundled with the application** — No external installation required
2. **Reviewed and tested** — Part of the core application codebase
3. **Permission-constrained** — No network, filesystem, or database access
4. **Statically registered** — IDs map to predefined React components

The MVP does **not** support a third-party plugin marketplace. All plugins are internal and reviewed as part of the application release.

---

## Bundled Artifact Renderers

### 1. Comparison Table Renderer

**ID**: `comparison-table`
**Purpose**: Side-by-side comparison of multiple entities

**Use Cases:**
- Compare company financial metrics
- Analyze multiple investment options
- Review competitor landscapes

**Input Schema:**
```json
{
  "type": "object",
  "required": ["entities", "dimensions"],
  "properties": {
    "entities": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "name": { "type": "string" },
          "values": { "type": "object" }
        }
      }
    },
    "dimensions": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "label": { "type": "string" },
          "format": { "type": "string" }
        }
      }
    }
  }
}
```

**Permissions Required**: None (data is passed from agent output)

---

### 2. Earnings Analyzer Renderer

**ID**: `earnings-analyzer`
**Purpose**: Visualize earnings data and trends

**Use Cases:**
- Quarterly earnings analysis
- Revenue trend visualization
- Margin and profitability review

**Input Schema:**
```json
{
  "type": "object",
  "required": ["company", "quarters"],
  "properties": {
    "company": {
      "type": "object",
      "properties": {
        "ticker": { "type": "string" },
        "name": { "type": "string" }
      }
    },
    "quarters": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "period": { "type": "string" },
          "revenue": { "type": "number" },
          "eps": { "type": "number" },
          "netMargin": { "type": "number" }
        }
      }
    }
  }
}
```

**Permissions Required**: None

---

### 3. Industry Map Renderer

**ID**: `industry-map`
**Purpose**: Visualize industry relationships and structure

**Use Cases:**
- Industry supply chain mapping
- Competitor positioning analysis
- Market structure visualization

**Input Schema:**
```json
{
  "type": "object",
  "required": ["nodes", "edges"],
  "properties": {
    "nodes": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "label": { "type": "string" },
          "type": { "type": "string" }
        }
      }
    },
    "edges": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "source": { "type": "string" },
          "target": { "type": "string" },
          "label": { "type": "string" }
        }
      }
    }
  }
}
```

**Permissions Required**: None

---

### 4. Macro Dashboard Renderer

**ID**: `macro-dashboard`
**Purpose**: Display macroeconomic indicators

**Use Cases:**
- Economic indicator tracking
- Market sentiment overview
- Policy impact analysis

**Input Schema:**
```json
{
  "type": "object",
  "required": ["indicators"],
  "properties": {
    "indicators": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "name": { "type": "string" },
          "value": { "type": "number" },
          "change": { "type": "number" },
          "period": { "type": "string" }
        }
      }
    }
  }
}
```

**Permissions Required**: None

---

### 5. Risk Dashboard Renderer

**ID**: `risk-dashboard`
**Purpose**: Visualize risk factors and exposures

**Use Cases:**
- Portfolio risk assessment
- Thesis risk tracking
- Exposure analysis

**Input Schema:**
```json
{
  "type": "object",
  "required": ["risks"],
  "properties": {
    "risks": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "name": { "type": "string" },
          "severity": { "type": "string", "enum": ["low", "medium", "high", "critical"] },
          "probability": { "type": "number" },
          "impact": { "type": "string" },
          "mitigation": { "type": "string" }
        }
      }
    }
  }
}
```

**Permissions Required**: None

---

### 6. Timeline Renderer

**ID**: `timeline`
**Purpose**: Display events and milestones chronologically

**Use Cases:**
- Company event timeline
- Product launch history
- Earnings announcement tracking

**Input Schema:**
```json
{
  "type": "object",
  "required": ["events"],
  "properties": {
    "events": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "date": { "type": "string", "format": "date" },
          "title": { "type": "string" },
          "description": { "type": "string" },
          "type": { "type": "string" }
        }
      }
    }
  }
}
```

**Permissions Required**: None

---

### 7. Valuation Model Renderer

**ID**: `valuation-model`
**Purpose**: Display valuation analysis and models

**Use Cases:**
- DCF model visualization
- Comparable company analysis
- Valuation sensitivity analysis

**Input Schema:**
```json
{
  "type": "object",
  "required": ["company", "valuations"],
  "properties": {
    "company": {
      "type": "object",
      "properties": {
        "ticker": { "type": "string" },
        "name": { "type": "string" },
        "currentPrice": { "type": "number" }
      }
    },
    "valuations": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "method": { "type": "string" },
          "value": { "type": "number" },
          "range": {
            "type": "object",
            "properties": {
              "low": { "type": "number" },
              "high": { "type": "number" }
            }
          },
          "assumptions": { "type": "object" }
        }
      }
    }
  }
}
```

**Permissions Required**: None

---

## Plugin Lifecycle

### Registration

Internal plugins are registered at application startup:

```typescript
// In registry.tsx
artifactRegistry.register(
  "comparison-table",
  ComparisonTableRenderer,
  { type: "comparison-table", name: "Comparison Table", description: "..." }
);
```

### Artifact Creation

When an agent task produces structured output with an artifact type:

1. Backend validates output against the plugin's JSON Schema
2. Backend persists artifact with validated input
3. Frontend requests artifact rendering
4. Registry maps artifact type to React component
5. Component renders with validated data

### No Runtime Evaluation

The MVP does **not** evaluate plugin code at runtime. All plugins are:

- Bundled at build time
- Reviewed in source code
- Tested as part of application tests

---

## Security Model

### Permission Enforcement

| Permission | Bundled Plugins | Enforcement |
|-----------|-----------------|-------------|
| Filesystem | Denied | No filesystem API available |
| Network | Denied | No network API available |
| Database | Denied | No database API available |
| Shell | Denied | No shell execution capability |
| Credentials | Denied | No credential access |
| IPC | Denied | Only message passing from parent |

### Data Flow

```
Agent Task (Rust)
  ↓ Structured JSON output
Backend Validation
  ↓ Schema validation
Artifact Persistence
  ↓ Read from SQLite
Frontend ArtifactViewer
  ↓ Registry lookup
Renderer Component
  ↓ React render
User Interface
```

The renderer receives only validated JSON data. It cannot:

- Access the database directly
- Make network requests
- Read files
- Execute commands
- Access parent window state

---

## Compatibility

### Version Compatibility

Bundled plugins are versioned with the application. When the application updates, plugins update together.

### Backward Compatibility

Older artifacts may not render correctly if:

- Schema has changed significantly
- Required fields are missing
- Data format is incompatible

**Mitigation**: The application includes schema version in artifact metadata. Renderers can check version and provide graceful degradation.

---

## Future Extensions

### Post-MVP Considerations

The following are **not** in the MVP but may be considered later:

1. **User-defined renderers** — Allow users to create custom visualizations
2. **Third-party plugins** — Support external plugin installation (requires security review)
3. **Plugin marketplace** — Public plugin repository (requires infrastructure)
4. **Plugin sandboxing** — Isolated plugin execution environment

### Extension Points

The registry pattern allows future extensions:

```typescript
// Future: External plugin loading
artifactRegistry.registerExternalPlugin(manifest, component);

// Future: Plugin marketplace
artifactRegistry.downloadFromMarketplace(pluginId);
```

These are not implemented in the MVP.

---

## Developer Guidelines

### Creating a New Renderer

1. Create component in `apps/desktop/src/features/artifacts/renderers/`
2. Define input schema JSON
3. Register in `registry.tsx`
4. Add tests for schema validation
5. Document in this file

### Renderer Best Practices

1. **Validate props** — Use TypeScript strict typing
2. **Handle missing data** — Graceful degradation for optional fields
3. **No side effects** — Renderers should be pure presentational
4. **Accessibility** — Include ARIA labels and keyboard navigation
5. **Localization** — Use `useLocale()` for user-facing strings

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-08-03 | Initial MVP documentation |

---

## See Also

- [Plugin Specification](PLUGIN_SPEC.md)
- [Artifact System](ARTIFACT_SYSTEM.md)
- [Architecture](ARCHITECTURE.md)
