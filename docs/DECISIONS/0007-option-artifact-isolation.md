# ADR-0007: Option Artifact Isolation

## Status

Accepted

## Context

The M9 Option module produces analytical output that may be rendered as an Artifact (a temporary interactive window). The existing Artifact runtime (M3) already enforces permission isolation: Artifacts receive validated JSON input only, have no SQLite, filesystem, shell, or API-key access, and are rendered by predefined React components.

The `OptionStrategyPanel` and `StrategyBuilder` components on `dev` display strategy analysis results inline on the Options page. A separate Artifact renderer for Option strategy payoff charts is not yet registered in the renderer registry (`apps/desktop/src/features/artifacts/renderers/registry.tsx`).

This ADR records the approved Artifact isolation boundary for Option output and confirms that the M9 release uses inline analysis components rather than privileged Artifact windows for strategy visualization.

## Decision

### Artifact scope for M9

1. **Option chain, Greeks, and strategy analysis are rendered inline** on the Options page using existing React components (`OptionChainList`, `OptionContractTable`, `GreeksCalculator`, `OptionStrategyPanel`, `StrategyBuilder`). These components operate within the main application window and follow the existing IPC security model.

2. **No dedicated Option Artifact renderer is required for M9.** The predefined renderers in the registry (`ComparisonTableRenderer`, `TimelineRenderer`, `ValuationModelRenderer`, `RiskDashboardRenderer`, `IndustryMapRenderer`, `EarningsAnalyzerRenderer`, `MacroDashboardRenderer`) are sufficient for any report-level output that references Option data.

3. If a future PR adds a dedicated Option Artifact renderer, it must:
   - Receive only validated JSON payloads matching a documented input schema.
   - Never request SQLite, filesystem, shell, credential, or undeclared Tauri command access.
   - Be registered in the predefined renderer registry (not loaded dynamically or from user-provided HTML).
   - Display a clear disclaimer: "This is an analytical estimate. Not investment advice."

### Inline component permission boundary

The existing Options page components already comply with the AlphaForge security model:

- React calls `desktopApi` functions (typed IPC wrappers), never `invoke` directly.
- All IPC results are validated with Zod schemas before rendering.
- No component accesses SQLite, the filesystem, or shell commands.
- No component reads or writes credentials.
- User-provided symbols are normalized and validated against `SYMBOL_PATTERN = /^[A-Z][A-Z0-9.-]{0,9}$/` before being sent to Rust.

### Strategy payload schema

When a strategy analysis result is rendered, the JSON payload must include:

```json
{
  "modelVersion": "option-core-0.1.0/black-scholes-european",
  "asOf": "2026-08-15T12:00:00Z",
  "dataSource": { "id": "", "title": "", "retrievedAt": "" },
  "legs": [],
  "assumptions": { "riskFreeRate": 0.05, "dividendYield": 0.0 },
  "breakEvenPoints": [],
  "maxProfit": null,
  "maxLoss": null,
  "confidence": 0,
  "disclaimer": "Analytical estimate for research purposes only. Not investment advice."
}
```

This schema is enforced by the `analyze_strategy` command's Zod validation on the frontend. The payload cannot request shell, SQLite, credentials, filesystem, navigation to arbitrary URLs, or undeclared Tauri commands.

### Disclaimer

Every Option analytical output displayed to the user (inline or Artifact) must include the following disclaimer or its i18n-localized equivalent:

> This is an analytical estimate for research and education purposes. It is not investment advice, a recommendation, or a trade signal.

## Consequences

### Positive

- No new Artifact permissions or security review scope for M9.
- Inline rendering keeps the strategy analysis flow simple and testable.
- The existing Artifact renderer registry can be extended later without breaking changes.

### Negative

- Strategy payoff diagrams are not available as standalone Artifact windows. Users must view them on the Options page.
- No interactive payoff chart Artifact is delivered in M9. The `StrategyBuilder` component provides a basic inline view.

### Risks and Mitigations

| Risk                                               | Mitigation                                                                                                                    |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Future Artifact renderer bypasses isolation        | The permission model is enforced by the Artifact runtime, not by the renderer; a new renderer cannot grant itself permissions |
| Users treat inline analysis as a recommendation    | The disclaimer is displayed on every analytical output surface                                                                |
| Strategy data is misinterpreted without provenance | Every persisted strategy includes model version, data source, and timestamps                                                  |

## References

- [Artifact System](../ARTIFACT_SYSTEM.md)
- [Plugin Specification](../PLUGIN_SPEC.md)
- [Option Implementation Details](../option/IMPLEMENTATION_DETAILS.md)
- `apps/desktop/src/features/artifacts/renderers/registry.tsx`
- `apps/desktop/src/features/options/components/StrategyBuilder.tsx`
- `apps/desktop/src/features/options/components/OptionStrategyPanel.tsx`
