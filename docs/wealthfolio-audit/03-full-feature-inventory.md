# Full Feature Inventory — Wealthfolio

> Scope: every existing feature in the repository, classified into four
> categories. Each entry records purpose, trigger, entry function, source
> path, inputs, return value, and known exception scenarios, plus a
> re-usability grade, migration difficulty, and code-risk rating.
>
> Legend — Re-usability: **A** drop-in reusable, **B** reusable with
> adaptation, **C** tightly coupled / copy-paste only. Migration difficulty:
> **1** trivial, **2** moderate, **3** hard. Code risk: **Low** / **Med** /
> **High** (risk of breakage or data loss when touched).

---

## Category 1 — Public-Exposed Features

### 1.1 HTTP API (web mode, Axum)

Root router: `apps/server/src/api.rs` — all routes are nested under
`/api/v1`, composed from ~30 per-domain modules. Auth middleware wraps the
whole subtree; `/healthz`, `/readyz`, `/auth/oidc/callback` are exempt.
`/mcp` is mounted **outside** `/api/v1` (see 1.4).

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| API-01 | Health check | Liveness probe for orchestration | GET `/healthz` | `healthz` | `apps/server/src/api.rs` | — | 200 always | — | A | 1 | Low |
| API-02 | Readiness check | DB + runtime readiness probe | GET `/readyz` | `readyz` | `apps/server/src/api.rs` | — | 200/503 | DB down → 503 | A | 1 | Low |
| API-03 | Auth status | Current session state | GET `/auth/status` | `auth_status` | `apps/server/src/api/auth.rs` | — | session JSON | unauthenticated → 401 | B | 2 | Low |
| API-04 | Login (password) | Password authentication, session cookie set | POST `/auth/login` | `login` | `apps/server/src/api/auth.rs` | email, password | session set / 204 | wrong creds → 401; rate-limited (tower_governor 5/60s) | B | 2 | Med |
| API-05 | Logout | Session teardown | POST `/auth/logout` | `logout` | `apps/server/src/api/auth.rs` | — | session cleared | — | A | 1 | Low |
| API-06 | Current user | Authenticated user profile | GET `/auth/me` | `me` | `apps/server/src/api/auth.rs` | — | user JSON | unauthenticated → 401 | A | 1 | Low |
| API-07 | OIDC login | SSO redirect start | GET `/auth/oidc/login` | `oidc_login` | `apps/server/src/api/auth.rs` | — | redirect to IdP | OIDC not configured → 404 | B | 2 | Med |
| API-08 | OIDC callback | SSO token exchange + session create | GET `/auth/oidc/callback` (exempt from auth) | `oidc_callback` | `apps/server/src/api/auth.rs` | code, state | redirect + session | state mismatch → 400 | B | 2 | Med |
| API-09 | Accounts CRUD | List/create/update/delete accounts | GET/POST `/accounts`, PUT/DELETE `/accounts/{id}` | `list_accounts`, `create_account`, `update_account`, `delete_account` | `apps/server/src/api/accounts.rs` | account fields, id | account JSON / 204 | not found → 404 | B | 2 | Low |
| API-10 | Activity search | Paginated/filtered activity query | GET `/activities` | `search_activities` | `apps/server/src/api/activities.rs` | filters, pagination | activity page | invalid filter → 400 | B | 2 | Low |
| API-11 | Activity CRUD | Create/update/bulk/delete activities | POST/PUT/DELETE `/activities*` | `create_activity`, `update_activity`, `bulk_*`, `delete_activity` | `apps/server/src/api/activities.rs` | activity records | activity JSON / 204 | not found → 404 | B | 2 | Med |
| API-12 | Transfer pairing | Link/unlink transfer pairs | POST `/activities/transfers/{id}/pair`, `/unlink` | `pair_transfer`, `unlink_transfer` | `apps/server/src/api/activities.rs` | transfer ids | 204 | already paired → 409 | B | 2 | Med |
| API-13 | Activity import (check/parse/preview/import) | CSV import pipeline | POST `/activities/import/check`, `/parse`, `/preview`, `/import` | `check_activities_import`, `parse_activities`, `preview_import`, `import_activities` | `apps/server/src/api/activities.rs` | file/rows, mapping | check report / parsed rows / result | malformed CSV → 400 | B | 2 | Med |
| API-14 | Import mapping | Get/save column mapping | GET/PUT `/activities/import/mapping` | `get_import_mapping`, `save_import_mapping` | `apps/server/src/api/activities.rs` | mapping JSON | mapping | — | A | 1 | Low |
| API-15 | Import templates CRUD | Template management | GET/POST/PUT/DELETE `/activities/import/templates` | template handlers | `apps/server/src/api/activities.rs` | template fields | template JSON | not found → 404 | A | 1 | Low |
| API-16 | Duplicate check | Find duplicate activities | POST `/activities/check-duplicates` | `check_duplicates` | `apps/server/src/api/activities.rs` | activity batch | duplicates list | — | B | 2 | Low |
| API-17 | Portfolio update | Refresh holdings from market data | POST `/portfolio/update` | `update_portfolio` | `apps/server/src/api/portfolio.rs` | token/acct filter | updated holdings | provider down → partial/error | B | 2 | Med |
| API-18 | Portfolio recalculate | Recompute derived values | POST `/portfolio/recalculate` | `recalculate_portfolio` | `apps/server/src/api/portfolio.rs` | — | recalc report | totals mismatch → warning | B | 2 | Med |
| API-19 | Event stream (SSE) | Live portfolio events to web UI | GET `/events/stream` | `stream_events` | `apps/server/src/api/portfolio.rs` | — | SSE stream | channel closed → end | B | 2 | Med |
| API-20 | Holdings & valuations | Holdings/valuations/allocations/snapshots CRUD+query | `/holdings*` (~20 routes) | holdings handlers | `apps/server/src/api/holdings/mod.rs` | ids, filters, ranges | holdings/valuations JSON | not found → 404 | B | 2 | Med |
| API-21 | MCP endpoint (server) | Remote MCP over Streamable HTTP with PAT auth | POST `/mcp` (outside `/api/v1`, gated by `WF_MCP_ENABLED`) | `router()` | `apps/server/src/mcp/mod.rs`, `apps/server/src/mcp/auth.rs`, `apps/server/src/mcp/audit_sink.rs` | JSON-RPC over HTTP, `Authorization: Bearer <PAT>` | JSON-RPC responses / SSE | bad PAT → 401 (fail-closed); audit on by default | B | 2 | High |

### 1.2 Tauri IPC (desktop mode)

Registry: `apps/tauri/src/commands/mod.rs` (~30 modules). All commands are
thin wrappers that delegate to `crates/core`; they accept struct args
(flat structs used to dodge Tauri v2 serde internal-tagged enum issues —
see `AccountScopeInput` in `apps/tauri/src/commands/portfolio.rs`).

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| IPC-01 | Account commands | List/create/update/delete accounts | IPC invoke (frontend `commands/account.ts`) | `get_accounts`, `create_account`, `update_account`, `delete_account` | `apps/tauri/src/commands/account.rs` | account fields | account JSON | not found → error | B | 2 | Low |
| IPC-02 | Activity commands | Search/create/update/delete/import/transfer (~20) | IPC invoke (frontend `commands/activity.ts`) | activity command fns | `apps/tauri/src/commands/activity.rs` | activity records, filters | activity JSON | validation → Err | B | 2 | Med |
| IPC-03 | Portfolio commands | Recalculate/update portfolio, holdings, valuations, allocations, snapshots, performance | IPC invoke (frontend `commands/portfolio.ts`) | `recalculate_portfolio`, `update_portfolio`, `get_holdings`, `get_holdings_list`, `get_holding`, `get_asset_holdings`, `get_asset_lots`, `get_portfolio_allocations`, `get_holdings_by_allocation`, `get_historical_valuations`, `get_latest_valuations`, `get_current_valuation`, `get_income_summary`, `calculate_accounts_simple_performance`, `calculate_performance_history`, `calculate_performance_summary`, `get_performance_summaries`, `save_manual_holdings`, `check_holdings_import`, `import_holdings_csv`, `import_single_snapshot`, `get_snapshots`, `get_snapshot_by_date`, `delete_snapshot` | `apps/tauri/src/commands/portfolio.rs` (1926 lines) | `AccountScopeInput` (flat struct), ids, dates, CSV | holdings/valuations/performance JSON | date not found → error; CSV parse fail → Err | B | 2 | High (large, central) |
| IPC-04 | Holdings import (CSV) | Check + import holdings snapshots | IPC invoke | `check_holdings_import`, `import_holdings_csv` | `apps/tauri/src/commands/portfolio.rs` | CSV content, mapping | report / rows | malformed CSV → Err | B | 2 | Med |

### 1.3 Frontend pages (React)

Route map: `apps/frontend/src/routes.tsx` (BrowserRouter). Pages live in
`apps/frontend/src/pages/<domain>/` with custom entry filenames (no standard
`page.tsx`).

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| UI-01 | Dashboard | Portfolio overview / net worth snapshot | Route `/` | `PortfolioPage`/`DashboardContent` | `apps/frontend/src/pages/dashboard/portfolio-page.tsx`, `dashboard-content.tsx` | account scope | rendered view | backend down → error state | C | 3 | Med |
| UI-02 | Account pages | Account list + detail, balances | Route `/accounts*` | account page components | `apps/frontend/src/pages/account/` | account id | rendered view | — | C | 3 | Low |
| UI-03 | Activity pages | Activity list, detail, import wizard, transfers | Route `/activities*` | activity page components | `apps/frontend/src/pages/activity/` | filters, id | rendered view | — | C | 3 | Med |
| UI-04 | Holdings pages | Holdings table, snapshot compare | Route `/holdings*` | holdings page components | `apps/frontend/src/pages/holdings/` | account scope | rendered view | — | C | 3 | Med |
| UI-05 | Performance | Performance charts/history | Route `/performance` | performance page components | `apps/frontend/src/pages/performance/` | date range | rendered view | — | C | 3 | Med |
| UI-06 | Net worth | Net worth timeline | Route `/net-worth` | net-worth components | `apps/frontend/src/pages/net-worth/` | — | rendered view | — | C | 3 | Low |
| UI-07 | Income | Income summary | Route `/income` | income components | `apps/frontend/src/pages/income/` | — | rendered view | — | C | 3 | Low |
| UI-08 | Goals | Goal tracking | Route `/goals*` | goals components | `apps/frontend/src/pages/goals/` | — | rendered view | — | C | 3 | Low |
| UI-09 | Spending | Spending insights | Route `/spending*` | spending components | `apps/frontend/src/pages/spending/` | — | rendered view | — | C | 3 | Low |
| UI-10 | Insights | Analytics insights | Route `/insights` | insights components | `apps/frontend/src/pages/insights/` | — | rendered view | — | C | 3 | Low |
| UI-11 | Allocation targets | Allocation target management | Route `/allocation-targets` | allocation-targets components | `apps/frontend/src/pages/allocation-targets/` | — | rendered view | — | C | 3 | Low |
| UI-12 | AI assistant | Chat with agent assistant | Route `/ai-assistant` | assistant components | `apps/frontend/src/pages/ai-assistant/` | messages | chat UI | LLM down → error state | C | 3 | Med |
| UI-13 | Settings | Settings incl. addons, MCP, sync | Route `/settings*` | settings components | `apps/frontend/src/pages/settings/` | — | rendered view | — | C | 3 | Med |
| UI-14 | Auth pages | Login / OIDC handling | Route `/auth*` | auth components | `apps/frontend/src/pages/auth/` | creds | redirect | wrong creds → inline error | C | 3 | Med |
| UI-15 | Onboarding | First-run setup | Route `/onboarding` | onboarding components | `apps/frontend/src/pages/onboarding/` | answers | redirect to dashboard | — | C | 3 | Med |
| UI-16 | Health pages | App health info | Route `/health` | health components | `apps/frontend/src/pages/health/` | — | rendered view | — | C | 3 | Low |
| UI-17 | Asset page | Asset detail | Route `/assets/:id` | asset components | `apps/frontend/src/pages/asset/` | asset id | rendered view | not found → 404 | C | 3 | Low |
| UI-18 | Layouts / not-found | App shell, 404 | Route fallback | layout components | `apps/frontend/src/pages/layouts/`, `not-found/` | — | shell | — | C | 3 | Low |

### 1.4 MCP tools (agent + server + desktop)

Tool registry: `crates/agent-tools/src/tools/mod.rs`; catalog + scoping in
`crates/agent-tools/src/catalog.rs` (assistant_catalog hides commit tools;
mcp_catalog exposes all). Protocol layer: `crates/wealthfolio-mcp/src/`
(`lib.rs` handler/service/auth). Three deployment surfaces: server `/mcp`
(1.1 API-21), desktop embedded loopback server (Category 2, BG-05), and the
frontend AI assistant.

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| MCP-01 | Read tools (16) | Query accounts/holdings/valuations/activities/income/goals/health/classifications | Agent tool call | tool fns | `crates/agent-tools/src/tools/mod.rs` | scoped args | JSON rows | truncated at MAX rows | A | 1 | Low |
| MCP-02 | Draft/suggest tools (5) | Draft activities, classification suggestions | Agent tool call | draft fns | `crates/agent-tools/src/tools/mod.rs` | context args | drafts | scope denied → fail-closed | A | 1 | Low |
| MCP-03 | Commit tools (3) | Commit drafted activities/classifications | Agent tool call (excluded from assistant_catalog) | commit fns | `crates/agent-tools/src/tools/mod.rs` | draft ids | commit result | scope denied → fail-closed | A | 1 | Med |
| MCP-04 | Import tools (3) | Activity CSV import via agent | Agent tool call | import fns | `crates/agent-tools/src/tools/mod.rs` | file/rows | import report | malformed CSV → error | B | 2 | Med |
| MCP-05 | MCP protocol layer | Streamable HTTP transport, JSON-RPC, fingerprint | HTTP request | `WealthfolioMcpHandler` / `McpServerBuilder` | `crates/wealthfolio-mcp/src/lib.rs`, `service.rs`, `handler.rs` | JSON-RPC payloads | JSON-RPC responses | bad auth → 401 fail-closed | A | 1 | Med |
| MCP-06 | PAT auth context | Personal-access-token verification | Every MCP call | `McpAuthContext` (`ActorKind::Pat` only) | `crates/wealthfolio-mcp/src/auth.rs` | bearer token | actor context | invalid/expired PAT → 401 | A | 1 | Med |

---

## Category 2 — Background Resident Tasks

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| BG-01 | Quote sync scheduler | Periodic incremental quote refresh loop | Startup + timer interval | `run_periodic_sync` | `crates/core/src/quotes/scheduler.rs` | — | sync outcome | provider down → retry/backoff | B | 2 | Med |
| BG-02 | Broker sync scheduler (server) | Pull broker/connect data every 4 h | `cfg(connect-sync)` + timer (4 h) | `start_broker_sync_scheduler`, `run_scheduled_sync` | `apps/server/src/scheduler.rs` | — | sync result | connect feature flag off → not started; provider down → skip | B | 2 | Med |
| BG-03 | Broker sync on startup (desktop) | One-shot sync at app launch | Startup | `run_startup_sync` | `apps/tauri/src/scheduler.rs` | — | sync result | network fail → logged, non-fatal | B | 2 | Med |
| BG-04 | Domain event queue worker | Debounce domain events (portfolio recalc, asset enrichment) then process | Events pushed to sink, flush after 1 s debounce | `WebDomainEventSink` / `TauriDomainEventSink` (+ queue worker) | `apps/server/src/domain_events/queue_worker.rs` + `mod.rs`; `apps/tauri/src/domain_events/queue_worker.rs` + `mod.rs` | domain events | processed/acked | worker crash → events dropped/redelivered | B | 2 | Med |
| BG-05 | Embedded MCP server (desktop) | Loopback MCP service for local agents | Settings toggle `mcp_server_enabled` / auto-start; binds 127.0.0.1:8639 | `start_server` / `start_if_enabled` / `set_enabled` (orchestration `McpServerState`) | `apps/tauri/src/mcp/server.rs`, `apps/tauri/src/mcp/mod.rs` | HTTP requests | JSON-RPC / health | port busy → 8639 conflict; disabled → not bound | B | 2 | Med |
| BG-06 | Device sync engine | E2EE device pairing + sync across devices | `cfg(device-sync)` + pairing flow / timer | device sync services | `crates/device-sync/src/` (crypto in `crypto.rs`) | root key, pairing code | synced state | key mismatch → SAS mismatch; feature flag off → inactive | B | 2 | High |

---

## Category 3 — Private Utility Functions

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| UTIL-01 | E2EE crypto suite | Root key gen, HKDF DEK derivation, X25519 ECDH, XChaCha20-Poly1305 AEAD, pairing code verify | Direct call | `generate_root_key`, `derive_dek`, ECDH/AEAD fns | `crates/device-sync/src/crypto.rs` | key material, version | key/encrypted payload | wrong key size → Err; bad root key → Err | A | 1 | High |
| UTIL-02 | CSV parser | Strict CSV parsing for imports | Import flows | `ParseConfig`-driven parser | `crates/core/src/activities/csv_parser.rs` | CSV text, config | parsed rows | malformed/quoted-newline → parse error | A | 1 | Med |
| UTIL-03 | Decimal serde helpers | rust_decimal (de)serialization + rounding display | Every monetary struct | `serialize`, `deserialize`, `decimal_serde_option`, `decimal_serde_round_display` | `crates/core/src/utils/decimal_serde.rs` | Decimal | JSON | — | A | 1 | Low |
| UTIL-04 | CUSIP parser | Validate/normalize CUSIP | Asset resolution | cusip fns | `crates/core/src/utils/cusip.rs` | symbol | parsed CUSIP | invalid → error enum | A | 1 | Low |
| UTIL-05 | ISIN parser | Validate/normalize ISIN incl. check digit | Asset resolution | `compute_isin_check_digit` + parse fns | `crates/core/src/utils/isin.rs` | symbol | parsed ISIN | bad check digit → error | A | 1 | Low |
| UTIL-06 | OCC symbol parser | Parse option contract symbols | Asset resolution | occ fns | `crates/core/src/utils/occ_symbol.rs` | symbol | parsed OCC | malformed → error enum | A | 1 | Low |
| UTIL-07 | Asset resolver | Resolve asset → market data provider chain (first-match on provider_overrides) | Quote/valuation flow | `resolve_asset` (first resolver) | `crates/market-data/src/resolver/asset_resolver.rs` | asset | provider + instrument | no provider → no resolution | B | 2 | Med |
| UTIL-08 | Token-bucket rate limiter | Per-provider request throttling (60/min default, cap 10) | Market data calls | rate limiter fns | `crates/market-data/src/registry/rate_limiter.rs` | provider key | permit | bucket empty → wait/drop | A | 1 | Med |
| UTIL-09 | Circuit breaker | 3-state breaker (closed/open/half-open): 5 failures, 60 s timeout, 2 half-open successes | Market data calls | breaker fns | `crates/market-data/src/registry/circuit_breaker.rs` | call result | state/allow | repeated failures → open → fast-fail | A | 1 | Med |
| UTIL-10 | Time utils | Date/range helpers for reporting | Performance/reporting | time_utils fns | `crates/core/src/utils/time_utils.rs` | dates | formatted/range | — | A | 1 | Low |

---

## Category 4 — Debug-Only Functions / Hidden Switches

| Feature-ID | Feature-name | Purpose | Trigger | Entry function | Source file | Inputs | Return | Known exceptions | Reuse | Migr. | Risk |
|---|---|---|---|---|---|---|---|---|---|---|---|
| DBG-01 | `TAURI_DEBUG` | Disable minification + enable sourcemaps in frontend build | Env var read by Vite config | vite config | `apps/frontend/vite.config.ts` | env | dev build | — | A | 1 | Low |
| DBG-02 | `VITE_ENABLE_ADDON_DEV_MODE` | Load addons from live dev servers instead of installed bundles | Env var + `MODE === "addon-dev"` | `isAddonDevModeEnabled`, `loadAllAddons` | `apps/frontend/src/addons/addons-loader.ts` | env | loaded addon list | dev server down → fallback to installed | A | 1 | Med |
| DBG-03 | `WF_MCP_ENABLED` | Expose `/mcp` endpoint on the server (default false) | Env var → `Config::mcp_enabled` | `router()` gating | `apps/server/src/config.rs`, `apps/server/src/api.rs` | env | router mounted / not | disabled → 404 | A | 1 | Low |
| DBG-04 | `WF_MCP_AUDIT_ENABLED` | Write agent tool calls to audit log (default true) | Env var | `RepoAuditSink` wiring | `apps/server/src/config.rs`, `apps/server/src/mcp/audit_sink.rs` | env | audit writes | disabled → no audit rows | A | 1 | Low |
| DBG-05 | `WF_MCP_ALLOWED_HOSTS` | Host-header allowlist for `/mcp` (None disables validation) | Env var | `router()` config branch | `apps/server/src/config.rs`, `apps/server/src/mcp/mod.rs` | env | host allowlist | misconfig → 400 wrong host | A | 1 | Low |
| DBG-06 | Golden-scenario eval harness | Assert agent tool-call behavior offline | `cfg(eval)` + manual run; **no runner wired** | `assert_valid_event_ordering` + scenario helpers | `crates/ai/src/eval/mod.rs`, `harness.rs`, `scenarios.rs` | scenario events | pass/fail | no runner → used as library only | B | 2 | Low |
| DBG-07 | Live evals (real LLM) | Drive chat turns through `ChatService`, assert traces | `cfg(test-utils)`; `WF_EVAL_PROVIDER` (default `ollama`), `WF_EVAL_MODEL` (default `gemma4:e4b`) | runner (`MAX_ATTEMPTS=3`, backoff 750 ms) | `crates/ai/src/live_evals/mod.rs`, `runner.rs`, `schema.rs`, `trace.rs` | prompt, rubric | `CaseResult` | provider down → retry then fail; spending stubs `unimplemented!` → panic if touched | B | 2 | Med |
| DBG-08 | Feature-flag helpers | Route gating for connect-sync / device-sync / cloud | Compile-time `cfg` + env | `connect_sync_enabled()`, `device_sync_enabled()`, `cloud_sync_enabled()`, `cloud_api_base_url()` | `apps/server/src/features.rs` | env/cfg | bool/url | — | B | 1 | Low |

---

## Cross-Cutting Notes

- **Central risk**: `apps/tauri/src/commands/portfolio.rs` (1926 lines, ~25
  commands) is the largest single surface and the highest-risk file (IPC-03).
- **MCP duplicated across three surfaces** (server `/mcp`, desktop loopback,
  assistant catalog) — all share `crates/wealthfolio-mcp` + `crates/agent-tools`
  (A-grade reuse); the server surface adds PAT auth + audit (API-21, MCP-06).
- **Evals are split**: golden scenarios are library-only (no runner, DBG-06);
  the live LLM harness runs only under `test-utils` and panics on spending
  stubs (DBG-07).
- **Hidden switches are all env/cfg-gated** (DBG-01..05, DBG-08); none are
  user-facing UI toggles except the desktop MCP server settings (BG-05).
- All routes/commands are thin delegates into `crates/core` — the core
  services are the primary migration surface; transport layers swap out.