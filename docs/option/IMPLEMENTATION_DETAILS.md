# Option Module Implementation Details

This document translates the Option specifications into the current repository's concrete implementation path. It is not an authorization to implement M9 before the milestone gate.

**Rebaseline (M9-01):** ADRs for pricing models, data providers, and Artifact isolation are approved. See [ADR-0005](../DECISIONS/0005-option-pricing-models.md), [ADR-0006](../DECISIONS/0006-option-data-providers.md), and [ADR-0007](../DECISIONS/0007-option-artifact-isolation.md).

## Target module map

| Layer             | Existing baseline to reuse                                          | Planned additions or changes                                                                                                                                         |
| ----------------- | ------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Domain            | `crates/domain/src/option.rs`                                       | Validate model completeness; add inputs/outputs only when a vertical slice requires them                                                                             |
| Calculation       | **Completed** `crates/option-core`                                  | Black-Scholes pricing, analytical Greeks, IV solver, strategy payoffs - 11 tests passing                                                                             |
| Persistence       | **Implemented baseline** Option repositories and migration          | `0014_options_support.sql` is registered by the custom runner; focused migration tests cover clean, repeat, partial, legacy, preservation, and rollback paths        |
| Provider          | **In Progress** `apps/desktop/src-tauri/src/providers/market_data/` | `OptionsDataProvider` plus demo/file implementations - DemoProvider in option-core                                                                                   |
| Service           | **Completed** Existing service pattern                              | `option_service.rs`, `strategy_service.rs`, `portfolio_option_service.rs` - full stack                                                                               |
| Command           | **Completed** Existing thin command pattern                         | `commands/options.rs` - fetch_option_chain, calculate_greeks, calculate_option_price, IV                                                                             |
| Frontend protocol | **Completed** `apps/desktop/src/types/option.ts`                    | CamelCase command-boundary DTOs, strict Zod response parsing, malformed-response tests, and registration parity check in `scripts/check-option-ipc-registration.mjs` |
| Frontend state    | **Completed** TanStack Query conventions                            | `useOptions.ts` - hooks for chains, contracts, strategies, calculations                                                                                              |
| UI                | **Completed** Shared states and layout                              | `features/options` - GreeksCalculator, OptionChainList, StrategyBuilder components                                                                                   |
| Artifact          | Predefined renderer registry                                        | Validated Option chain/payoff/risk renderers; no free-form privileged HTML                                                                                           |
| Tests             | Existing Rust/Vitest patterns                                       | Numerical fixtures, migrations, repositories, services, schemas, states, IPC, E2E                                                                                    |

## Domain contracts

Reuse the existing `OptionType`, `DataSource`, `StrategyType`, `PositionType`, `PricingModel`, `OptionChain`, `OptionContract`, `Greeks`, `OptionStrategy`, `StrategyLeg`, and `OptionPosition` types after an audit.

Before adding fields, resolve these contract questions:

- European versus American exercise style.
- Quote currency and exchange/market identifier.
- Contract symbol and contract multiplier normalization.
- Quote timestamp versus retrieval timestamp.
- Bid/ask/last missing-value semantics; do not use zero as “unknown.”
- Risk-free-rate and dividend-yield source provenance.
- Calculation version and numerical tolerance.
- Open versus closed position lifecycle and immutable acquisition data.

Domain models remain free of SQLx, Tauri, HTTP, React, and display-localization concerns.

## Persistence path

The current custom migration runner skips historical migrations 0002-0006 and applies a reconciliation sequence. Therefore, adding an Option SQL file is insufficient.

The implementation now:

1. Uses the append-only `0014_options_support.sql` migration, greater than the current maximum.
2. Preflights existing Option tables and rejects incompatible legacy schemas without deleting data.
3. Registers `0014_options_support` in `database/migrations.rs` using the existing `_migrations` table.
4. Applies DDL and migration registration atomically, with rollback on failure.
5. Keeps historical migrations unchanged.

Required database constraints include workspace scoping, referential integrity, stable IDs, creation/update timestamps for mutable entities, quote/calculation timestamps, and indexes for symbol, expiration, strike, chain, and position access. Complex SQL stays inside repositories.

## Calculation core

`option-core` exposes pure, serializable inputs and deterministic outputs. A recommended boundary is:

```text
Validated PricingInput
  -> OptionPricer implementation
  -> PricingOutput { value, greeks, assumptions, model_version }
```

Implementation rules:

- Reject NaN, infinity, non-positive spot/strike, negative time, and invalid volatility/rates according to documented ranges.
- Define units in types and docs: years versus days, decimal volatility versus percent, per-day theta, and per-1-percent vega/rho.
- Treat expiration and zero-volatility boundaries explicitly.
- Bound iterative solvers by tolerance, iterations, and bracket; return a typed convergence error.
- Validate every pricing/Greeks/IV input through `pricing::validate_pricing_input`, which rejects NaN, infinity, and non-positive spot/strike/time/volatility.
- Keep the standard-normal density consistent between Greeks and IV modules: `norm_pdf(x) = exp(-x²/2) / sqrt(2π)`.
- Benchmark representative pricing, Greeks, IV, and strategy calls with Criterion and record hardware with results.

### Verification status (M9-03)

- Independent reference fixtures: Hull ATM call 10.4506 / put 5.5663, plus ITM/OTM spot-check values (call 17.66 at S=110, put 10.21 at S=90).
- Property tests: put-call parity (with and without dividend), call/put upper bounds, volatility monotonicity, delta moneyness ordering, delta call-put relationship, gamma/vega peaking at the money, and identical gamma for call and put.
- Boundary tests: expiring-option intrinsic convergence, near-zero volatility, and NaN/infinity/negative input rejection.
- IV convergence tests: round-trip at 20%, 8%, and 60% volatility, tight-tolerance accuracy to 4 decimal places, non-convergence with zero iterations, and non-finite/negative market-price rejection.
- Benchmarks: `crates/option-core/benches/bench.rs` measures Black-Scholes price, Greeks, IV solve, and two-leg strategy payoff via Criterion. Run with `cargo bench -p option-core --bench bench`.
- Compare against independent published fixtures and property tests such as put-call parity where applicable.
- Keep market-data retrieval, persistence, and logging outside the pure crate.

## Provider boundary

`OptionsDataProvider` belongs in Rust and returns validated provider-neutral data plus provenance. It should expose capabilities so the UI can distinguish demo, file, delayed, and live data.

Every network provider requires:

- Approved data license and attribution.
- Native credential storage; no key reaches React or logs.
- HTTPS allowlist and redirect validation.
- Request timeout, response-size cap, retry policy, and rate-limit handling.
- Schema validation, symbol normalization, freshness metadata, and partial-chain semantics.
- A deterministic offline/error path.

The initial M9 slice should use demo and file data. A live adapter is a separate approved PR.

## Service and command boundary

Commands perform:

```text
Validate request
  -> call service
  -> map typed result or AppError
```

Services coordinate providers, calculation, repositories, and Artifact creation. Commands contain no SQL and no pricing formulas. Long-running chain, surface, scenario, or backtest work returns `task_id`, executes in the Rust background runtime, emits typed events, supports cancellation, and reaches one of the existing task terminal states.

The API spec is a target, not proof every listed command belongs in the first slice. Add only commands required by the active vertical slice and keep request/response definitions centralized.

## Frontend path

1. Parse every Option IPC result with a Zod schema; TypeScript interfaces alone do not validate untrusted values. Nullable Rust `Option<T>` outputs are represented as explicit `null` values.
2. Expose functions from `apps/desktop/src/lib/desktop-api/options.ts` and re-export them through the unified `desktopApi`.
3. Use TanStack Query for chain, contract, strategy, and risk asynchronous state; use component state for filters and draft legs.
4. Add route-level pages under `pages/options` and composable components under `features/options`.
5. Reuse common Loading, Empty, Error, partial, and offline patterns.
6. Use the shared i18n formatters for currency, percentages, timestamps, strikes, and Greeks without changing calculation precision.
7. Provide keyboard navigation for chain grids and strategy legs, visible focus, and Escape behavior for temporary layers.

Avoid a second global store, direct `invoke` calls in components, and calculations duplicated in JavaScript.

## Structured Artifact output

Option analysis produces validated JSON with, at minimum:

```json
{
  "summary": "",
  "asOf": "",
  "dataSource": { "id": "", "title": "", "retrievedAt": "" },
  "assumptions": {},
  "claims": [],
  "evidence": [],
  "risks": [],
  "contracts": [],
  "strategy": null,
  "scenarios": [],
  "confidence": 0,
  "modelVersion": ""
}
```

The predefined React renderer displays assumptions, stale/missing data, contradictory evidence, and risk. The payload cannot request shell, SQLite, credentials, filesystem, navigation to arbitrary URLs, or undeclared Tauri commands.

## Test matrix

| Layer           | Minimum cases                                                                                         |
| --------------- | ----------------------------------------------------------------------------------------------------- |
| Domain/schema   | serialization parity, invalid enums, missing fields, numeric bounds                                   |
| Migration       | fresh, historical, partial Option schema, repeat run, workspace cascade                               |
| Repository      | CRUD, ordering/filtering, missing row, isolation, database error mapping                              |
| Calculation     | reference fixtures, boundaries, parity properties, solver failure, strategy aggregation               |
| Provider        | demo determinism, malformed file, path validation, partial data, timeout/rate limit for live adapters |
| Service/command | validation, persistence, cancellation, typed errors, provenance                                       |
| Frontend        | loading, success, empty, error, partial, offline, keyboard, both locales                              |
| Artifact        | schema rejection, renderer registry, permission isolation                                             |
| E2E             | load chain, inspect contract, build/save/reopen strategy, cancel task, restart persistence            |

## Observability and redaction

Record structured timings and counts for provider fetch, validation, pricing, persistence, and render payload size. Logs may include provider ID, normalized symbol, task ID, count, duration, and stable error code. They must redact keys, tokens, raw provider bodies, sensitive local paths, account identifiers, and user research content.

## Documentation update rule

Every implementation PR updates the relevant checkboxes and file paths in this directory. If code and documentation disagree, the PR must either change the implementation or record the divergence and approval; it must not silently leave a target specification presented as current behavior.
