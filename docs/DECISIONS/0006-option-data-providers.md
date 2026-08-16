# ADR-0006: Option Data Providers

## Status

Accepted

## Context

The Option module requires market-data providers to supply chain and contract pricing data. The `option-core` crate defines an `OptionsDataProvider` trait with a `DemoProvider` implementation. A `ProviderFactory` creates providers by `DataSource` enum. The current `dev` baseline contains:

- **`DemoProvider`** in `option-core/src/provider.rs` — generates 22 contracts (11 strikes × call/put) around a fixed 150.00 underlying price, with 30-day expiration, 25% flat volatility, and deterministic UUIDs.
- **`ProviderFactory`** — returns `DemoProvider` for `DataSource::Demo`, and returns `OptionError::ProviderError` for `DataSource::Live` and `DataSource::File`.
- **`OptionsDataProvider` trait** — async, requires `Send + Sync`, returns `FetchedOptionChain` with a chain and its contracts.
- **`apps/desktop/src-tauri/src/providers/market_data/`** — contains a `mod.rs` placeholder with no registered provider.

This ADR records the approved provider boundary, the scope of each implementation, and the rules for future live providers.

## Decision

### Provider catalog

| Provider        | Status on `dev`       | M9 release scope | Notes                                                                 |
| --------------- | --------------------- | ---------------- | --------------------------------------------------------------------- |
| Demo            | ✅ Implemented        | ✅ Included      | Deterministic, fixed price, flat IV                                   |
| File (CSV/JSON) | ❌ Not implemented    | ❌ Excluded      | Deferred to a post-M9 PR                                              |
| Live (API)      | ❌ Stub returns error | ❌ Excluded      | Requires separate licensing, credential, security, and privacy review |

### Demo provider contract

1. **Determinism**: The same symbol and workspace ID always produce the same number of contracts (22) with the same structure. UUIDs are generated fresh each call, so IDs differ between calls. This is acceptable — the demo is for UI exploration, not deterministic replay.
2. **Provenance**: `DataSource::Demo` is stamped on every chain and contract. The UI displays a "Demo data — not for investment decisions" label.
3. **Underlying price**: Fixed at 150.00 for all symbols. Symbol-specific pricing is a file or live provider feature.
4. **Volatility**: Flat 25% across all strikes and expirations. No skew or term structure.
5. **Expiration**: 30 days from retrieval time. Single expiration per chain.

### File provider (deferred)

A file provider will be added in a post-M9 PR. It must:

- Accept only `.csv` and `.json` formats.
- Reject files larger than 10 MB.
- Validate every row against the `OptionContract` schema, rejecting malformed, non-finite, or out-of-range values.
- Prevent path traversal by validating the resolved path is within the workspace-import directory.
- Record source filename, import timestamp, and row count as provenance.
- Report partial-data semantics (N of M rows imported successfully) instead of failing silently.

### Live provider (deferred)

A live provider is not approved for M9. Any future live provider PR must separately satisfy:

- Approved data license and attribution (shown in the About dialog).
- Native credential storage via the OS keyring (`apps/desktop/src-tauri/src/security/credentials.rs`); no API key reaches React, logs, CLI arguments, or recipe files.
- HTTPS allowlist with redirect validation, 15-second timeout, 5 MB response cap, and retry policy.
- Request-rate limiting and 429-response handling.
- Provider-neutral output that includes source ID, quote/retrieval timestamps, capabilities, and missing-field semantics.
- A deterministic offline/error path when the provider is unreachable.

### Provider registration

The `OptionService` on `dev` currently calls `ProviderFactory::create(source)` directly in `fetch_chain`. For the M9 release, this is acceptable. A future PR may refactor provider registration into a configurable `ProviderRegistry` with runtime selection, but this is not required for the release gate.

## Consequences

### Positive

- The demo provider gives a predictable, always-available UI development and testing surface.
- The trait abstraction makes adding file and live providers straightforward.
- Provenance is stamped at the data source level.

### Negative

- The demo provider's fixed price and flat IV do not resemble real market data. Users must understand this limitation.
- No file import means users cannot load their own option chain data until a post-M9 PR.

### Risks and Mitigations

| Risk                                   | Mitigation                                                                                                             |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Demo data mistaken for real data       | `DataSource::Demo` is stamped and displayed in the UI with a warning label                                             |
| No file provider for M9 release        | Accepted as a scope limitation; documented in the release notes                                                        |
| Future live provider leaks credentials | The credential model is already defined in `credentials.rs` and will be enforced before any live provider is activated |

## References

- [Option Architecture](../option/ARCHITECTURE.md)
- [Option Implementation Details](../option/IMPLEMENTATION_DETAILS.md)
- [Option Integration Plan](../option/INTEGRATION_PLAN.md)
- `crates/option-core/src/provider.rs`
- `apps/desktop/src-tauri/src/providers/market_data/`
- `apps/desktop/src-tauri/src/security/credentials.rs`
