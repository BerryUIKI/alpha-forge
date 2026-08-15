# Wealthfolio Optimization & TODO Inventory

Generated: 2026-08-12

---

## 1. TODO / FIXME / Known Gaps

### Rust Code

| # | File | Line | Content | Category | Risk | Reusability | Migration |
|---|------|------|---------|----------|------|-------------|-----------|
| 1 | `crates/ai/src/live_evals/runner.rs` | 501 | `// Response rubric — TODO once we have a judge model. For now log a warn.` | Missing feature | Medium | Low (eval-specific) | Low |
| 2 | `crates/core/src/health/service.rs` | 1918 | `// TODO: Call quote sync service to refresh prices` | Stub action | Medium | Medium | Low |
| 3 | `apps/server/src/api/ai_chat.rs` | 233 | `// TODO: Add tag support to ChatService` | Stub endpoint | Low | High (pattern applies to all CRUD) | Low |
| 4 | `apps/server/src/api/ai_chat.rs` | 244 | `// TODO: Add tag support to ChatService` | Stub endpoint | Low | High | Low |
| 5 | `apps/tauri/src/commands/ai_chat.rs` | 154 | `// TODO: Add tag support to ChatService` | Stub command | Low | High | Low |
| 6 | `apps/tauri/src/commands/ai_chat.rs` | 165 | `// TODO: Add tag support to ChatService` | Stub command | Low | High | Low |

### TypeScript Code

| # | File | Line | Content | Category | Risk | Reusability | Migration |
|---|------|------|---------|----------|------|-------------|-----------|
| 7 | `apps/frontend/src/pages/activity/import/components/symbol-resolution-panel.tsx` | 19 | `// TODO: Same non-nullable values as in create-custom-asset-dialog.tsx. Maybe it makes sense to unify this logic in one place?` | Duplication | Low | Medium | Low |

---

## 2. Known Performance Bottlenecks

### 2.1 SQLite Write Contention (WriteActor Pattern)

**Location:** `crates/storage-sqlite/src/db/write_actor.rs`

The write actor is a single-threaded serial executor using an MPSC channel (capacity 1024). All database writes funnel through this actor, processed one at a time via `immediate_transaction`. This is a deliberate design choice to avoid SQLite's well-known concurrency issues, but it creates a single point of contention:

- **Impact:** Heavy concurrent operations (e.g., large CSV import, broker sync, portfolio recalculation) queue behind each other.
- **Key detail:** `immediate_transaction` is used, which acquires a write lock at the start, preventing deadlocks but serializing all writes.
- **Risk:** Medium. Channel capacity of 1024 with backpressure. If the channel fills, callers block on `send()`.
- **Mitigation:** The `exec_projected` and `exec_tx` APIs batch outbox writes within the same transaction, reducing per-operation overhead.

### 2.2 Market Data Rate Limiting

**Location:** `crates/market-data/src/registry/rate_limiter.rs`

Token bucket rate limiter with per-provider buckets. Defaults: 60 requests/min, burst capacity 10.

- **Impact:** Providers like Yahoo Finance, Alpha Vantage, Finnhub each have their own rate limits that must be respected. Backpressure is handled via `tokio::time::sleep` in the `acquire()` method.
- **Risk:** Low. Well-implemented token bucket. The two separate `Mutex` locks (buckets + configs) are released in order to avoid deadlock, and poison recovery is implemented.
- **Note:** The `try_acquire()` synchronous variant still uses `Mutex` internally, which could block async tasks if held too long. The async `acquire()` drops the lock before sleeping.

### 2.3 Portfolio Snapshot Calculation Complexity

**Location:** `crates/core/src/portfolio/snapshot/snapshot_service.rs`

The snapshot service supports three recalculation modes:
- `Full` -- wipe all snapshots, recalculate from earliest activity
- `IncrementalFromLast` -- resume from latest existing snapshot
- `SinceDate(NaiveDate)` -- wipe from a date forward and recalculate

- **Complexity:** The holdings calculator (`HoldingsCalculator` + `ProjectionRun`) processes activity-by-activity across potentially thousands of activities per account, maintaining a lot book, and computing cost basis using the configured method (FIFO, etc.).
- **Optimization:** The incremental mode (`IncrementalFromLast`) avoids full recalculation for most operations. The service auto-escalates to `Full` when lot operations are involved (line 405: `Upgrade to Full in that case`).
- **Risk:** Medium. Full recalculation for accounts with years of daily activity can be computationally expensive. The service uses `VecDeque` for snapshot storage and `BTreeMap` for date-indexed lookups.

### 2.4 Large CSV Import Performance

**Location:** `crates/core/src/activities/activities_service.rs` (notably `import_activities` at line 4788, `parse_csv` at line 1610)

- **Impact:** CSV parsing uses a custom parser (`crates/core/src/activities/csv_parser.rs`). Symbol resolution operates in batches (`resolve_symbols_batch`), but activity import processes activities one-by-one within a batch, with deduplication checks against existing data.
- **Risk:** Medium. Import of thousands of rows involves multiple DB round-trips through the write actor, each creating activities and potentially triggering domain events.
- **Related:** `crates/core/src/activities/import_run_model.rs` tracks import runs but does not appear to support streaming/chunked import.

### 2.5 Recharts Rendering with Large Datasets

**Location (key files):**
- `apps/frontend/src/components/performance-chart.tsx`
- `apps/frontend/src/components/history-chart.tsx`
- `apps/frontend/src/components/performance-chart-mobile.tsx`
- `apps/frontend/src/components/renderable-chart-container.tsx`
- `apps/frontend/src/features/goals/retirement-planner/components/retirement-coverage-chart.tsx`

- **Impact:** Recharts renders all data points into the DOM. Datasets with thousands of daily valuation points over many years can cause significant DOM bloat and slow re-renders.
- **Risk:** Medium. No evidence of data downsampling or windowing in the chart components. The performance chart component uses `LineChart` with `ResponsiveContainer`, which recalculates on every resize.
- **Mitigation opportunities:** Use `recharts` `data` prop with downsampled data, virtualize X-axis labels, or use a canvas-based chart library for large datasets.

---

## 3. Extension / Plugin Hook Points

### 3.1 Addon System

**Location:** `crates/core/src/addons/` (models, traits, service, network, storage_repository)

The addon system is a full-featured plugin system with:

- **Manifest-driven installation:** Addons declare `id`, `version`, `main`, `permissions`, `network`, `contributes` (routes + links), `minWealthfolioVersion`, `hostDependencies`
- **Permission model:** Function-level permissions with static analysis detection (`detect_addon_permissions`). Categories: portfolio, activities, accounts, market-data, assets, quotes, performance, financial-planning, contribution-limits, currency, settings, files, secrets, snapshots, events, query, network, ui
- **Iframe sandbox:** Addons run in sandboxed iframes (`apps/frontend/src/addons/iframe/`). Communication via `MessageChannel` (CHANNEL = "wealthfolio:addon-sandbox:v1"). The iframe uses `sandbox-entry.tsx` and has CSS isolation (`addon-sandbox.css`).
- **Contribution system:** Routes and links declared in `contributes` field are registered without loading addon code (lazy activation). See `contribution-registry.ts` and `activation-coordinator.ts`.
- **Brokered network:** Addon network requests go through the host via `addon_network_request` with host allowlist approval.
- **Persistent storage:** Per-addon key-value storage (`get_addon_storage_item`, `set_addon_storage_item`, `delete_addon_storage_item`, `clear_addon_storage`). Sync payload limit: 250,000 bytes.
- **Dev mode:** `apps/frontend/src/addons/addons-dev-mode.ts` supports hot-reload development.
- **Limit constants:** Max 256 archive entries, 5 MB per file, 25 MB total, 50 MB compressed.

### 3.2 MCP / Agent Tools

**Location:** `crates/agent-tools/` and `crates/wealthfolio-mcp/`

Multiple tool tiers with scope-gated execution:

- **v1 Read-only tools** (`AgentToolCatalog::v1_read_tools`): accounts, activities, activity_import, allocation, asset_classification, asset_taxonomies, cash_balances, categorization_context, contribution_limits, goals, health, holdings, income, net_worth, performance, portfolios, propose_categories, valuation
- **Draft/suggest tools** (`AgentToolCatalog::assistant_catalog`): Adds draft operations for the in-app assistant
- **Commit tools** (`AgentToolCatalog::mcp_catalog`): Full read + write catalog for MCP (Streamable HTTP) protocol
- **MCP protocol layer** (`crates/wealthfolio-mcp/`): Converts agent tools to MCP server. Includes audit logging (`audit/`), auth (`auth/`), PAT management (`pat/`), and handlers (`handler/`).
- **Scope enforcement:** `AgentScopeSet` in `scope.rs` gates each tool at the boundary. The `McpAuthContext` carries the authenticated scope.
- **AI integration:** `crates/ai/src/tools/` adapter layer connects agent tools to LLM rig framework.

### 3.3 Domain Events System

**Location:** `crates/core/src/events/`

Events emitted by core services after successful mutations:

| Event | Purpose | Consumers |
|-------|---------|-----------|
| `ActivitiesChanged` | Activities CRUD | Portfolio recalc, broker sync, FX sync |
| `AssetSplitActivitiesChanged` | Split adjustments | Portfolio recalc |
| `HoldingsChanged` | Snapshot changes | Portfolio recalc |
| `AccountsChanged` | Account CRUD | FX asset sync planning |
| `AssetsCreated` | New assets | Quote sync, enrichment |
| `AssetsUpdated` | Asset changes | Quote sync, recalculation |
| `AssetClassificationsChanged` | Taxonomy changes | Portfolio recalc |
| `AssetsMerged` | Unknown asset resolution | Portfolio recalc, activity migration |
| `TrackingModeChanged` | Account mode change | Broker sync, portfolio recalc |
| `DeviceSyncPullComplete` | Cross-device sync | Full portfolio recalc |

- **Sink pattern:** `DomainEventSink` trait with `emit()` and `emit_batch()`. Must be fast and non-blocking. `NoOpDomainEventSink` for tests, `MockDomainEventSink` for test assertions.
- **Risk:** No TODO/FIXME markers found in the events module.

### 3.4 Custom Market Data Providers

**Location:** `crates/market-data/src/provider/` (individual providers) and `crates/market-data/src/registry/provider_registry.rs`

Registered providers:
- Yahoo Finance (`yahoo/`)
- Alpha Vantage (`alpha_vantage/`)
- Finnhub (`finnhub/`)
- Boerse Frankfurt (`boerse_frankfurt/`)
- marketdata.app (`marketdata_app/`)
- Metal Price API (`metal_price_api/`)
- OpenFIGI (`openfigi/`)
- US Treasury Calculator (`us_treasury_calc/`)
- Fixture/test provider (`fixture/`)
- Custom scraper provider (`crates/core/src/quotes/custom_scraper_provider.rs`)

The `ProviderRegistry` manages:
- Provider registration with priority ordering
- Rate limiting (per provider via `RateLimiter`)
- Circuit breaking (`CircuitBreaker` for fault tolerance)
- Quote validation (`QuoteValidator`)
- Fallback to alternative providers on failure
- Diagnostic tracking (`FetchDiagnostics`, `SkipReason`, `ProviderAttempt`)

### 3.5 Custom Provider Sources

**Location:** `crates/core/src/custom_provider/`

Custom provider system allows users to define their own data sources:

- `CustomProviderRepository` trait for persistence
- `NewCustomProvider` / `UpdateCustomProvider` / `CustomProviderWithSources` models
- Assets reference custom providers via `custom_provider_code` in their config
- Asset resolution (`crates/core/src/assets/asset_resolution.rs`) resolves symbols through custom providers

### 3.6 Import Templates

**Location:** `crates/core/src/activities/activities_model.rs` (ImportTemplate, ImportTemplateData, ImportTemplateScope)

- `ImportTemplate` models with `scope` (User/Account/Global), `context_kind`, `config`
- Agent tools surface import templates via `activity_import.rs` tool
- `ImportTemplateScope` enum: User, Account, Global
- `ImportTemplateData` DTO for frontend/backend exchange
- Categorization context tools (`categorization_context.rs`) assist with template-based import decisions

### 3.7 Categorization Rules (Presets)

**Location:** `crates/spending/src/categorization_rules/`

Full categorization rule system:

- **Service:** `CategorizationRulesService` with `rerun_lock` (Mutex) to prevent race conditions during background reruns
- **Repository:** `CategorizationRulesRepositoryTrait` with CRUD + `import_preset_rules` + `remove_preset`
- **Presets:** Country-specific JSON bundles embedded at compile time via `include_str!`:
  - US (`us.json`), Canada (`ca.json`), UK (`gb.json`), Spain (`es.json`)
  - Each preset has stable `presetId`, `presetVersion`, rules with stable `key`
  - Preset import/upgrade/removal with user-modification detection
- **Matcher:** `matcher.rs` compiles regex patterns, `match_compiled` for matching
- **Agent tools:** `propose_categories.rs`, `create_categorization_rule.rs`, `categorization_context.rs` expose categorization to AI
- **Risk:** Low. Well-structured with thread safety via `rerun_lock`.

---

## 4. Additional Observations

### 4.1 Unused/Dead Code

- `crates/spending/src/categorization_rules/model.rs` has `allow(dead_code)` on `get_declared_functions`, `get_detected_functions`, `get_undeclared_detected_functions`, `has_undeclared_detected_functions` -- these are utility functions exposed for external use but not yet consumed.

### 4.2 Test Coverage

- **Snapshot service:** `crates/core/src/portfolio/snapshot/snapshot_service_tests.rs` -- comprehensive
- **Rate limiter:** Unit tests for token bucket, custom config, per-provider isolation, reset, async acquire
- **Domain events:** Serialization round-trip tests for all event variants
- **Write actor:** Tests for outbox observer notification, cancellation handling
- **Categorization rules:** Preset validation (unique keys, valid regexes, match types)
- **Agent tools:** `crates/ai/tests/tool_outputs_parity.rs` and `crates/ai/tests/tool_schemas.rs`
- **Addons:** `activation-coordinator.test.ts`, `addons-core.test.ts`, `addon-iframe-manager.test.ts`, `addon-sandbox-asset-registry.test.ts`, `addon-sandbox-assets.test.ts`, `addon-sandbox-styles.test.ts`, `addon-sandbox-theme.test.ts`, `addon-module-rewriter.test.ts`, `addon-iframe-route.test.tsx`, `addon-iframe-error.test.ts`

### 4.3 Security-Relevant Extension Points

- **Addon sandbox:** iframe sandbox attribute not set in the `addon-sandbox-entry.tsx` -- sandboxing is assumed to be at the transport/iframe level, but the exact sandbox attributes are not visible in the entry point. The `CHANNEL` message channel isolates communication.
- **Addon permissions:** Static analysis `detect_addon_permissions` scans addon JavaScript for API function usage patterns. This is a heuristic, not a hard guarantee.
- **Addon network brokering:** All external network requests go through the host, which enforces the declared `allowedHosts` list.
- **MCP auth:** `crates/wealthfolio-mcp/src/auth/` handles token authentication. Tokens are fingerprinted via `sha256:<hex>` for audit logging.
- **Device sync E2EE:** `crates/device-sync/` uses `XChaCha20-Poly1305` for encryption. Addon storage sync respects the 250 KB payload limit to avoid server dead-lettering.

### 4.4 Summary of Risk Levels

| Area | Risk | Priority |
|------|------|----------|
| AI chat tag stubs (4 endpoints) | Low | Low |
| Health service price sync stub | Medium | Medium |
| LLM judge model for eval | Medium | Low |
| Symbol resolution code duplication | Low | Low |
| SQLite write contention | Medium | Medium |
| Recharts rendering with large data | Medium | Medium |
| CSV import performance | Medium | Medium |
| Portfolio full recalc cost | Medium | Low |
| Addon permission detection heuristic | Low | Low |
| Addon iframe sandbox attributes | Low | Low |