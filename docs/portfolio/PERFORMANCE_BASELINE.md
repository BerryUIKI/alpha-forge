# Large-Portfolio Benchmark and Performance Baseline (M11-03)

> **Document:** `docs/portfolio/PERFORMANCE_BASELINE.md`  
> **Status:** ✅ Approved Baseline (M11-03 complete, governing M11-04)  
> **Date:** 2026-10-03  
> **Target Release:** Milestone 11 — Portfolio Integration & Release Candidate  
> **Branch:** `test/portfolio-scale-baseline`  
> **Authors:** AlphaForge Core Team

---

## 1. Executive Summary

As part of Milestone 11 (Task **M11-03**), this document establishes the empirical scale, latency, and throughput baseline for AlphaForge's Portfolio system. Representative large-portfolio synthetic fixtures were executed against canonical Rust SQLite/SQLx services and frontend transformation pipelines.

Key findings:
1. **Core calculations are sub-20ms:** Multi-currency holdings aggregation across 5 accounts (50 assets, 500 tax lots) executes in **~14.1 ms**. Daily valuation calculations across all accounts execute in **~14.3 ms**.
2. **Polynomial XIRR & TWR compounding is sub-5ms:** Calculating annualized IRR (via Newton-Raphson polynomial convergence) and geometric TWR across 250 daily valuation periods executes in **~3.4 ms**.
3. **High-volume CSV statement ingestion is sub-65ms:** Full statement parsing, asset resolution, SHA-256 idempotency hashing, activity ledger insertion, and FIFO tax lot inventory allocation for **1,000 transactions** completes in **~61.6 ms** (Generic CSV) and **~53.9 ms** (Interactive Brokers CSV).
4. **UI transformations are sub-1ms:** Client-side time-series downsampling (1,250 points), activity search/filtering (1,000 rows), and holdings locale formatting execute in **< 1.0 ms**, well within the 60 fps (16.6 ms) frame budget.

All measured workloads comfortably beat the M11-04 performance budgets by **5× to 20×**.

---

## 2. Test Environment

All benchmarks were captured under controlled local conditions using native release builds.

| Dimension | Specification |
|-----------|---------------|
| **Operating System** | Microsoft Windows 11 Enterprise (64-bit, Build 26100) |
| **Processor** | 12th Gen Intel(R) Core(TM) i5-12600KF (10 cores, 16 logical threads, up to 4.90 GHz) |
| **System Memory** | 32 GB RAM (33,393,304 KB visible) |
| **Storage** | NVMe PCIe Gen4 SSD |
| **Rust Toolchain** | Rust 1.85.0+ (MSVC target: `x86_64-pc-windows-msvc`) |
| **Node.js / PNPM** | Node.js v20.18.0 / PNPM v9.0.0 |
| **Build Profile** | Cargo `bench` profile (`opt-level = 3`, `lto = thin`, `codegen-units = 16`, debug symbols enabled) |
| **Database Engine** | SQLite 3 via SQLx `0.8.6` (in-memory WAL mode, zero network latency) |

---

## 3. Benchmark Dataset & Fixtures

All fixtures are 100% deterministic, self-contained, and contain **zero user or real-world proprietary financial data**.

### 3.1 Dataset Dimensions

| Entity | Quantity | Characteristics |
|--------|----------|-----------------|
| **Accounts** | 5 | Multi-currency (USD, EUR), Securities, Cash, and Crypto portfolios |
| **Assets** | 50 | 35 US Equities, 5 ETFs, 5 Cryptocurrencies, 5 FX Currency Pairs |
| **Quotes** | 250 trading days × 50 assets | Daily OHLCV pricing spanning full year 2025 (12,500 quote records) |
| **FX Rates** | 250 days | EUR/USD daily historical exchange rates for currency conversion |
| **Open Lots** | 500 lots | FIFO cost basis tracking distributed across accounts |
| **Activities** | 1,000 rows | Deposits, Buys, Sells, Dividends, and Fees with fractional pricing |
| **Valuations** | 1,250 rows | 250 consecutive daily valuation records per account |
| **Taxonomies** | 1 taxonomy, 4 categories | Asset Allocation Model (US Equities 60%, Fixed Income 20%, Crypto 10%, FX 10%) |
| **Target Weights** | 4 weights | Target basis points (bps) with drift-band thresholds |
| **Constraints** | 1 constraint | Category upper-bound buying restriction |
| **CSV Statements** | 1,000 rows × 2 | 1,000-row Generic Broker CSV and 1,000-row Interactive Brokers Trade Statement |

---

## 4. Backend Benchmark Results (Criterion)

Measurements were collected using `criterion = "0.5"` over 10–15 samples with 3.0-second warmup periods.

| Workload | Dataset Scope | Measured Mean (`time`) | 95% Confidence Interval | M11-04 Approved Budget | Margin |
|----------|---------------|------------------------|-------------------------|------------------------|--------|
| `holdings_aggregation` | Single Account (250 lots, quotes) | **5.42 ms** | [5.40 ms .. 5.46 ms] | `< 100 ms` | **18.4× faster** |
| `holdings_aggregation` | All Accounts (5 accounts, 50 assets, multi-ccy FX) | **14.06 ms** | [13.84 ms .. 14.46 ms] | `< 250 ms` | **17.8× faster** |
| `valuation_calculation` | Single Account Daily Valuation | **5.38 ms** | [5.33 ms .. 5.41 ms] | `< 100 ms` | **18.6× faster** |
| `valuation_calculation` | All Accounts Daily Rollup (5 accounts) | **14.25 ms** | [13.99 ms .. 14.51 ms] | `< 250 ms` | **17.5× faster** |
| `performance_computation` | XIRR & Geometric TWR (250 valuation periods) | **3.41 ms** | [3.24 ms .. 3.60 ms] | `< 50 ms` | **14.7× faster** |
| `allocation_analysis` | 50 Assets Breakdown across 4 Categories | **18.78 ms** | [16.58 ms .. 22.24 ms] | `< 100 ms` | **5.3× faster** |
| `allocation_analysis` | Constraint Evaluation across All Scope | **33.27 ms** | [31.09 ms .. 37.15 ms] | `< 150 ms` | **4.5× faster** |
| `csv_statement_import` | Generic Broker CSV (1,000 rows, full ledger write) | **61.65 ms** | [54.64 ms .. 78.24 ms] | `< 500 ms` | **8.1× faster** |
| `csv_statement_import` | Interactive Brokers CSV (1,000 rows, multi-section) | **53.91 ms** | [52.50 ms .. 55.65 ms] | `< 500 ms` | **9.3× faster** |

---

## 5. Frontend Client Transformation Results (Vitest)

Measurements were collected using high-resolution monotonic clocks (`performance.now()`) in `apps/desktop/src/features/portfolio/benchmarks/portfolioDashboardBenchmark.test.ts`.

| Frontend Transform | Scope | Measured Latency | Approved Budget |
|--------------------|-------|------------------|-----------------|
| **Valuation Series Downsampling** | 1,250 daily points across 5 accounts aggregated by date | **0.68 ms** | `< 50 ms` |
| **Activity Search, Filter & Sort** | 1,000 transaction rows filtered by date, type, text query | **0.85 ms** | `< 50 ms` |
| **Holdings Grid Formatting** | 50 positions formatted with locale currency, gain/loss styles | **0.42 ms** | `< 50 ms` |
| **Taxonomy Drift & Target Weights** | 50 positions grouped, sum-weighted, drift-tested vs targets | **0.31 ms** | `< 50 ms` |

---

## 6. Approved Budgets & Guardrails for M11-04

For Milestone 11 Task **M11-04** (*Scale and resilience budgets*), the following operational thresholds are approved:

### 6.1 Latency Budgets

1. **Holdings API Response:**
   - Single Account (`get_holdings`): `< 100 ms`
   - Workspace / All Accounts (`get_all_holdings`): `< 250 ms`
2. **Valuation Engine:**
   - Single Account Day (`calculate_day`): `< 100 ms`
   - Multi-Account Day (`calculate_all`): `< 250 ms`
3. **Performance Metrics:**
   - Account Summary (`compute_summary`): `< 50 ms`
4. **Statement Import Pipeline:**
   - Statement Parsing & Ledger Ingestion (up to 1,000 rows): `< 500 ms`
   - Bulk Ingestion (up to 5,000 rows): `< 2,500 ms`
5. **Dashboard Rendering:**
   - Any client-side aggregation / formatting transform: `< 50 ms`

### 6.2 Resilience & Memory Guardrails

- **Memory Usage:** Under normal continuous operations, portfolio calculations must not retain large intermediate clones in memory; peak heap delta per benchmark batch is `< 15 MB`.
- **Concurrency & Cancellation:** Long-running valuation or import batches must respect cancellation tokens and avoid holding write locks across asynchronous I/O boundaries.
- **Fail-Soft Error Propagation:** Missing quotes, malformed CSV records, or unmapped currency pairs must never panic (`unwrap`/`expect`); they must report structured, typed validation diagnostics.

---

## 7. Optimization Candidates for M11-04

While all current paths exceed requirements, M11-04 will incorporate the following proactive hardening items:

1. **Batch Allocation Constraint Queries:** `AllocationService::check_constraints` currently queries constraints per-target iteratively; batching constraint retrieval across all active targets in a single query will lower execution time from 33 ms to < 10 ms.
2. **Pre-Cached Asset Resolvers in CSV Ingestion:** In `ActivityImportService`, maintain an in-memory symbol-to-asset cache to prevent redundant `SELECT` queries per transaction row during bulk file imports.
3. **Holding Table Virtualization:** For portfolios exceeding 200 individual positions, verify TanStack Virtual / table virtualization to guarantee 60 fps DOM scrolling on low-end machines.

---

## 8. Reproduction Instructions

### 8.1 Backend Criterion Benchmarks
```powershell
# Run all scale benchmarks (measures sample statistics)
cargo bench -p alpha-forge --bench portfolio_scale_benchmark

# Run fast execution test (verifies all benchmarks without statistical sampling)
cargo bench -p alpha-forge --bench portfolio_scale_benchmark -- --test
```

### 8.2 Frontend Client Benchmarks
```powershell
pnpm vitest run apps/desktop/src/features/portfolio/benchmarks/portfolioDashboardBenchmark.test.ts
```
