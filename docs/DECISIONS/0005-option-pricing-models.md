# ADR-0005: Option Pricing Models

## Status

Accepted

## Context

Milestone M9 requires integrating evidence-grounded Option pricing into AlphaForge. The `option-core` crate (merged via PR #69, commit `171d435`) already implements Black-Scholes European pricing, analytical Greeks, a Newton-Raphson implied-volatility solver, and strategy payoff calculations. This ADR records the approved pricing-model scope, supported exercise styles, and numerical conventions before the Option release gate.

The current `option-core` implementation on `dev` provides:

- **Black-Scholes**: European call/put pricing with dividend yield, using `erf`-based CDF approximation (Abramowitz and Stegun).
- **Analytical Greeks**: Delta, Gamma, Theta (per-day), Vega (per-1%-IV), and Rho (per-1%-rate).
- **Implied volatility solver**: Newton-Raphson with bounded iterations and tolerance.
- **Strategy payoff**: Multi-leg payoff at expiry, break-even detection, and max profit/loss calculation.

## Decision

### Pricing models

1. **European Black-Scholes** is the only approved pricing model for the M9 Option release. It is accurate for cash-settled European index and equity options, which cover the majority of listed A-share, Hong Kong, and US equity option contracts relevant to research.

2. **American exercise** (binomial tree or finite-difference) is explicitly deferred. It will be added in a later PR if and when American-style options become a researched product in the workspace. The current `option-core` architecture is modular enough that an `AmericanPricer` implementing the same trait can be added without refactoring existing code.

3. **No exotic pricing model** (barrier, Asian, lookback, digital, etc.) is approved for M9. Exotic options are outside the Option product scope defined in the product specification.

### Numerical conventions

| Parameter            | Unit               | Convention                            |
| -------------------- | ------------------ | ------------------------------------- |
| Volatility (`sigma`) | Decimal            | 0.25 means 25% annualized             |
| Time (`t`)           | Years              | Calendar days / 365                   |
| Risk-free rate (`r`) | Decimal            | 0.05 means 5% annualized              |
| Dividend yield (`q`) | Decimal            | 0.02 means 2% annualized              |
| Theta                | Per calendar day   | Calculated per-year, divided by 365   |
| Vega                 | Per 1% IV change   | Calculated per-1-vol, divided by 100  |
| Rho                  | Per 1% rate change | Calculated per-1-rate, divided by 100 |

### Tolerance and convergence

| Parameter                     | Value                                    | Documentation            |
| ----------------------------- | ---------------------------------------- | ------------------------ |
| Black-Scholes price tolerance | 0.01 (1 cent) against reference fixtures | `pricing.rs` test values |
| Greeks tolerance              | See each test assertion                  | `greeks.rs` tests        |
| IV solver max iterations      | 100                                      | `volatility.rs`          |
| IV solver precision           | 0.0001 (1 bp)                            | `volatility.rs`          |
| IV solver initial guess       | 0.50 (50%)                               | `volatility.rs`          |
| IV solver minimum sigma       | 0.01 (1%)                                | `volatility.rs`          |

### Input validation

The pricing module rejects:

- Non-positive spot, strike, or volatility.
- Non-positive or zero time to expiration (boundary: treat as intrinsically priced).
- NaN or infinite values (handled by `f64` arithmetic — the `OptionError::InvalidParameters` catch covers visible cases; an explicit `is_finite` check is added in the next maintenance pass).

### Model version

The calculation model version is defined as `"option-core-0.1.0/black-scholes-european"`. This string is recorded in persisted analysis output so that future model changes can be retroactively compared.

## Consequences

### Positive

- A single, well-understood pricing model reduces numerical regression risk.
- The modular crate structure allows adding American and exotic models without touching existing code.
- Explicit tolerance and unit conventions prevent silent precision errors.
- The model version stamp enables provenance tracing.

### Negative

- American-style options (e.g., most US equity options) are priced with a European approximation. This is acceptable for research and education purposes, as documented in the product specification.
- The `erf`-based CDF approximation (Abramowitz and Stegun) has a maximum absolute error of ~1.5×10⁻⁷, which is well below the 1-cent pricing tolerance.

### Risks and Mitigations

| Risk                      | Mitigation                                                                                           |
| ------------------------- | ---------------------------------------------------------------------------------------------------- |
| American exercise bias    | Documented in the product specification and UI; user is warned that the model uses European exercise |
| Put-call parity drift     | Weekly automated test; parity is verified in the CI suite                                            |
| IV solver non-convergence | Returns a typed `IvConvergenceFailed` error; the UI shows a recoverable error state                  |

## References

- [Option Architecture](../option/ARCHITECTURE.md)
- [Option Implementation Details](../option/IMPLEMENTATION_DETAILS.md)
- [Option Integration Plan](../option/INTEGRATION_PLAN.md)
- `crates/option-core/src/pricing.rs`
- `crates/option-core/src/greeks.rs`
- `crates/option-core/src/volatility.rs`
