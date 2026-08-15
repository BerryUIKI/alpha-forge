# Wealthfolio — Architecture & Application Lifecycle

This document describes the layered architecture, end-to-end data flow, module
dependency graph, and full application lifecycle of the Wealthfolio repository.

---

## 1. End-to-End Data Flow

A single request/read path flows through the same logical layers whether running
on desktop (Tauri) or web (Axum). Only the transport between the frontend and the
Rust core differs.

```
Frontend (React + Vite)
   │  typed adapter functions in apps/frontend/src/adapters/
   ▼
Adapter (tauri | web)          ← build-time swap via Vite resolve.alias on BUILD_TARGET
   │  tauri: invoke() → @tauri-apps/api/core
   │  web:    fetch()  → /api/v1/... (COMMANDS map)
   ▼
Command wrapper layer          ← apps/frontend/src/adapters/shared/*.ts (typed API)
   ▼
┌──────────────────────────────TRANSPORT──────────────────────────────┐
│  Desktop                        │  Web                              │
│  Tauri IPC (invoke_handler)     │  Axum HTTP (app_router)           │
│  apps/tauri/src/commands/*.rs   │  apps/server/src/api/*.rs         │
└────────────────────────────────┴────────────────────────────────────┘
   ▼
crates/core (business logic)     ← services, models, traits (DB-agnostic)
   │  e.g. AccountService, QuoteService, ValuationService
   │  calls repository traits (Trait + Send + Sync)
   ▼
crates/storage-sqlite (Diesel ORM) ← only place Diesel exists
   │  repositories implement core traits; Diesel models
   │  writer actor serializes writes; r2d2 pool
   ▼
SQLite DB   (crates/storage-sqlite/migrations/*)
```

Key properties:

- **`crates/core` is database-agnostic.** It defines repository traits
  (`XxxRepositoryTrait`) and service traits (`XxxServiceTrait`); it has no Diesel
  dependency.
- **`crates/storage-sqlite` is the only crate holding Diesel.** It implements the
  core traits with Diesel models and repositories. Verified in
  `crates/storage-sqlite/src/lib.rs`.
- **The frontend never talks to the DB directly.** All access goes through the
  adapter layer, which selects a transport per build target.
- **Writes are serialized through a writer actor**
  (`crates/storage-sqlite/src/db/write_actor.rs`) to avoid SQLite write
  contention; repos are constructed with `(pool.clone(), writer.clone())`.

---

## 2. Layered Structure

### 2.1 Access Layer (entry points)

| Backend | Path | Role |
| --- | --- | --- |
| Tauri commands | `apps/tauri/src/commands/*.rs` | Thin IPC handlers registered via `invoke_handler!` in `lib.rs::run()`. ~30 modules (account, activity, portfolio, spending, goal, market_data, device_sync, ai_chat, mcp, addon, fire…). |
| Axum API handlers | `apps/server/src/api/*.rs` | HTTP handlers mounted by `app_router` in `apps/server/src/api/mod.rs`. Same domain split (accounts, activities, holdings, connect, device_sync_engine, spending…). |

Both layers are intentionally thin: they deserialize requests, call a
`crates/core` service, and serialize the result. Business logic lives
downstream.

### 2.2 Controller Layer (command wrappers)

`apps/frontend/src/adapters/`

- `index.ts` — re-exports `./tauri` (default); Vite `resolve.alias` swaps to
  `web/index.ts` for web builds.
- `types.ts` — shared request/response/Ts types for all platforms.
- `shared/*.ts` — platform-agnostic command wrappers (accounts, activities,
  portfolio, market-data, goals, taxonomies, connect…). They call the platform
  `invoke`/`logger`/`isDesktop` primitives.
- `tauri/*.ts` — desktop implementations (core `invoke` via
  `@tauri-apps/api/core`, events, files, crypto, addons, agent-access, FIRE
  planner).
- `web/*.ts` — browser implementations. `web/core.ts` holds a `COMMANDS` map
  (`{ method, path }`) that translates each command name to an HTTP
  `/api/v1/...` endpoint, plus an SSE `EVENTS_ENDPOINT` and AI chat stream URL.

A parity test (`adapter-command-parity.test.ts`) enforces that desktop and web
adapters expose the same command surface.

### 2.3 Business Layer

`crates/core/src/` — services, models, traits. Modules: `accounts`, `activities`,
`addons`, `assets`, `custom_provider`, `events`, `exports`, `fx`, `goals`,
`health`, `limits`, `lots`, `planning`, `portfolio` (allocation, allocation_targets,
fire, holdings, income, net_worth, performance, snapshot, valuation), `portfolios`,
`quotes`, `secrets`, `settings`, `sync`, `taxonomies`, `utils`.

Services are constructed in `build_state` (`apps/server/src/main_lib.rs`) and
`context::initialize_context` (`apps/tauri/src/context.rs`), wired together with
`Arc` and held on `AppState` / `ServiceContext`.

### 2.4 Data Layer

`crates/storage-sqlite/src/` — repositories + Diesel models. Modules mirror core:
`accounts`, `activities`, `addons`, `agent`, `ai_chat`, `assets`,
`custom_provider`, `fx`, `goals`, `health`, `limits`, `lots`, `market_data`,
`portfolio`, `portfolios`, `settings`, `spending`, `sync`, `taxonomies`, plus
`db` (pool, migrations, writer actor), `schema.rs`, `errors.rs`, `utils.rs`.

- Diesel schema is generated in `schema.rs`; `db::run_migrations` applies
  `crates/storage-sqlite/migrations/*` (30+ timestamped migrations).
- `db::create_pool` returns an r2d2 SQLite pool; a writer actor
  (`db/write_actor.rs`) serializes writes and optionally notifies a sync outbox
  observer.

### 2.5 Utility Layer

| Path | Purpose |
| --- | --- |
| `crates/core/src/utils/` | `time_utils` (timezone/today parsing), `decimal_serde`, `cusip.rs`, `isin.rs`, `occ_symbol.rs` |
| `crates/core/src/fx/` | FX service, currency converter, FX model/traits |
| `crates/market-data/src/resolver/` | Symbol resolution: `asset_resolver`, `rules_resolver`, `chain`, exchange registry/metadata/suffixes |

### 2.6 Domain Events

- **Core types & trait:** `crates/core/src/events/` — `DomainEvent` enum
  (`activities_changed`, `holdings_changed`, `accounts_changed`, `assets_created`,
  `assets_updated`, `asset_classifications_changed`, `tracking_mode_changed`,
  `device_sync_pull_complete`, …) and the `DomainEventSink` trait.
  `emit()` must be fast and non-blocking (best-effort).
- **Tauri sink:** `apps/tauri/src/domain_events/` — `TauriDomainEventSink`
  sends events to an mpsc channel; `start_queue_worker` spawns a debounced queue
  worker that triggers portfolio recalculation, asset enrichment, broker sync.
- **Web sink:** `apps/server/src/domain_events/` — `WebDomainEventSink` uses
  two-phase init (`new()` then `start_worker()`) to break circular dependencies,
  then runs the same debounced `event_queue_worker`.

### 2.7 Scheduled Tasks

| Scheduler | Path | Cadence |
| --- | --- | --- |
| Periodic market data sync | `crates/core/src/quotes/scheduler.rs` — `run_periodic_sync(quote_service, delay, interval)` | 6h, 2min initial delay |
| Broker sync (web) | `apps/server/src/scheduler.rs` — `start_broker_sync_scheduler` | 4h, 60s initial delay |
| Startup broker sync (desktop) | `apps/tauri/src/scheduler.rs` — `run_startup_sync` | once at startup |

`run_periodic_sync` is shared by both runtimes (desktop and web both call it).

### 2.8 Hooks / Event System

- **Tauri listeners:** `apps/tauri/src/listeners.rs` — `setup_event_listeners`
  listens for `PORTFOLIO_TRIGGER_UPDATE` and `PORTFOLIO_TRIGGER_RECALCULATE`,
  then runs market sync + snapshot + valuation recalculation and emits progress
  events back to the frontend.
- **Event emission:** `apps/tauri/src/events.rs` and `apps/server/src/events.rs`
  define event names and payloads (`MARKET_SYNC_START/COMPLETE/ERROR`,
  `PORTFOLIO_UPDATE_START/COMPLETE/ERROR`, `BROKER_SYNC_*`, `navigate-to-route`,
  `deep-link-received`).
- **Server event bus:** `apps/server/src/events.rs` `EventBus`; web pushes events
  to the frontend over SSE (`/api/v1/events/stream`) and emits domain events into
  the same domain-event pipeline.
- **Frontend listeners:** `apps/frontend/src/adapters/{tauri,web}/events.ts` wrap
  Tauri `listen()` or SSE subscriptions.

### 2.9 AI Layer

| Crate | Path | Role |
| --- | --- | --- |
| `crates/ai` | `crates/ai/src/` | LLM providers (`provider_service`, `providers`, `provider_model`), chat service (`chat/`), tool runtime (`tools/`), prompt templates, AI provider catalog (`ai_providers.json`), title generation, stream hooks. |
| `crates/agent-tools` | `crates/agent-tools/src/` | Agent tool catalog (`catalog.rs`), tool definitions (`tool.rs`, `tools/`), scopes (`scope.rs`), `AgentEnvironment` trait (`env.rs`). |
| `crates/wealthfolio-mcp` | `crates/wealthfolio-mcp/src/` | MCP protocol server shared by desktop (embedded, `apps/tauri/src/mcp.rs`) and web (`/mcp` endpoint, `apps/server/src/api/agent_access.rs`): `handler`, `service`, PAT auth (`pat.rs`, `auth.rs`), audit (`audit.rs`). |

`wealthfolio-core` exposes `agent_environment: Arc<dyn AgentEnvironment>` on
`AppState`; the `ServerAiEnvironment` (from `apps/server/src/ai_environment.rs`)
implements it by wiring ~20 core services into agent-accessible tools.

---

## 3. Module Dependency Graph

```
apps/frontend (React)
   │ adapters/{shared,tauri,web}
   ▼
Desktop (Tauri)                          Web (Axum)
apps/tauri/src/commands/*.rs             apps/server/src/api/*.rs  →  app_router
   │                                        │
   │  (IPC)                                 │  (HTTP /api/v1)
   ▼                                        ▼
   ├── apps/tauri/src/context.rs ──────────┤
   │      ServiceContext (Arc services)     │  apps/server/src/main_lib.rs
   │      context::initialize_context       │      AppState::build_state
   │                                        │
   ▼                                        ▼
crates/core (services apply business rules)
   │   calls repository traits (e.g. AccountRepositoryTrait)
   ▼
crates/storage-sqlite (Diesel repositories implement core traits)
   │   db::write_actor (serialized writes) + r2d2 pool
   ▼
SQLite DB (migrations under crates/storage-sqlite/migrations)

Cross-cutting (invoked by services in core):
   crates/core/src/events::DomainEventSink ──► apps/{tauri,server}/src/domain_events
                                                      (queue worker → recalc/enrich/broker sync)
   crates/core/src/quotes/scheduler.rs  ──►  quote_service.sync(...)   (periodic market sync)
   crates/market-data/src/resolver  ◄──  crates/core::quotes (symbol resolution)
   crates/core/src/fx  ◄──  services (currency conversion, FX sync planning)

AI path (built on top of core services):
   frontend → ai_chat/ai_providers commands → crates/ai (ChatService)
        → crates/agent-tools (AgentEnvironment, tools) → crates/core services
        → crates/wealthfolio-mcp (MCP server / agent access)
```

### Which module invokes which

- `apps/frontend/**` invokes `adapters/*` only (never core directly).
- `adapters/{shared,tauri,web}/*` invoke the transport: Tauri `invoke` IPC or
  Axum HTTP endpoints.
- `apps/{tauri,server}/src/{commands,api}/*` invoke `crates/core` services via
  `AppState`/`ServiceContext` handles.
- `crates/core` services invoke each other and repository *traits*; they never
  depend on `storage-sqlite`.
- `crates/storage-sqlite` implements the traits; nothing above core invokes it
  directly for business logic.
- `crates/connect`, `crates/device-sync`, `crates/spending` also depend on
  `storage-sqlite` repositories (per `storage-sqlite/src/lib.rs` doc diagram) and
  are consumed by core-equivalent services.
- Domain events flow core → sink → runtime queue worker → core services again
  (self-referential loop for recalc), plus runtime → frontend via Tauri events /
  SSE.

---

## 4. Full Application Lifecycle

### 4.1 Desktop (Tauri)

```
main.rs → wealthfolio_app_lib::run()
   │  apps/tauri/src/main.rs (thin; calls lib::run())
   ▼
lib.rs::run()
   │  dotenv, tauri::Builder::default()
   │  plugins: single-instance, log, shell, dialog, fs, deep-link, window-state
   │  setup(app):
   │     manage(McpServerState)
   │     desktop::init_plugins / mobile::init_plugins
   │     listeners::setup_event_listeners(handle)      ← hooks
   │     deep_link().on_open_url(...)
   │     desktop::setup(handle, app_data_dir):
   │        context::initialize_context(app_data_dir)  ← setup DB, run migrations,
   │                                                    build ServiceContext (Arc services)
   │        handle.manage(Arc<ServiceContext>)
   │        mcp::remove_stale_lock; mcp::start_if_enabled  ← start MCP server (embedded)
   │        start_sync_outbox_wake_worker (device-sync)
   │        TauriDomainEventSink::start_queue_worker   ← domain event worker
   │        setup_menu
   │        emit_app_ready (frontend kicks off initial update)
   │        portfolio_history_backfill_needed → emit_portfolio_trigger_recalculate
   │        scheduler::run_startup_sync (startup broker sync)
   │        run_periodic_sync(quote_service, 120s, 6h)  ← market data sync scheduler
   │        ensure_background_engine_started (device sync)
   │  invoke_handler![ ~300 commands ]                ← register commands
   │     (account, activity, settings, spending, goal, portfolios, portfolio,
   │      limits, utilities, asset, alternative_assets, market_data, taxonomy,
   │      platform, secrets, providers_settings, ai_providers, ai_chat, mcp,
   │      addon, wealthfolio_connect, brokers_sync, device_sync, sync_crypto,
   │      custom_provider, health, allocation_targets, fire)
   │  .run(): on ExitRequested/Exit → stop MCP server, stop device-sync engine
```

### 4.2 Web (Axum server)

```
main.rs → Config::from_env()
   │  apps/server/src/main.rs
   ▼
init_tracing()                       ← tracing_subscriber (text or JSON via WF_LOG_FORMAT)
   ▼
build_state(&config)                 ← apps/server/src/main_lib.rs
   │   set DATABASE_URL=db_path
   │   db::init → db::run_migrations → db::create_pool
   │   spawn writer actor (with sync-outbox observer)
   │   WebDomainEventSink::new() (phase 1)
   │   construct ~40 services (FX, settings, core, spending, goals, health,
   │                          alternative assets, connect, AI, MCP PAT/audit)
   │   WebDomainEventSink::start_worker(...) (phase 2)
   │   prune sync outbox; portfolio backfill check → trigger_full_portfolio_recalc
   ▼
start device sync engine            ← #device-sync: token warmup → ensure_background_engine_started
   ▼
start broker sync scheduler          ← scheduler::start_broker_sync_scheduler(state) (4h)
   ▼
start market data sync               ← run_periodic_sync(state.quote_service, 120s, 6h)
   ▼
app_router(state, &config)           ← apps/server/src/api/mod.rs
   │   + fallback static ServeDir/ServeFile(index.html)
   │   + security_headers middleware layer
   ▼
TcpListener::bind(config.listen_addr) → axum::serve(...)   ← listen
   │   optionally with auth (AuthManager) / OIDC
```

### 4.3 Frontend (React)

```
index.html → src/main.tsx → <App/>
   ▼
App.tsx
   ├─ QueryClientProvider            ← TanStack Query client (staleTime 5m, no retry)
   ├─ AuthProvider                   ← auth context (web: login/auth gate)
   │   └─ (web only) AuthGate + LoginPage fallback
   ├─ WealthfolioConnectProvider     ← connect/device-sync feature context
   ├─ PrivacyProvider
   ├─ SettingsProvider
   ├─ TooltipProvider                ← @wealthfolio/ui
   ├─ Toaster
   ├─ AddonRuntimeLoader             ← loads enabled addons at startup
   ├─ EventDialogProvider
   └─ AppRoutes                      ← BrowserRouter (desktop) / routes
        ├─ on /onboarding → OnboardingLayout
        ├─ on / → AppLayout (sidebar)
        │    ├─ dashboard, activities(+manage), holdings(+asset profile),
        │    │  import, accounts/:id, income, performance, insights, health,
        │    │  assistant, connect, goals/*, spending/*, allocation-targets
        │    ├─ dynamic addon routes (AddonIframeRoute)
        │    └─ settings/* (accounts, general, market-data, taxonomies, connect,
        │                    ai-providers, agent-access, addons, …)
        └─ * → NotFoundPage
```

---

## 5. Re-usability, Migration Difficulty, Code Risk

### Re-usability Grade

| Layer | Grade | Notes |
| --- | --- | --- |
| `crates/core` | **A** | Cleanly factored, DB-agnostic, trait-driven domain logic. Highly reusable across runtimes. |
| `crates/storage-sqlite` | **A** | Isolated Diesel layer behind core traits; swappable for another DB by implementing the traits. |
| `crates/market-data`, `crates/fx` | **A** | Self-contained providers/resolvers with clean interfaces. |
| `crates/ai`, `crates/agent-tools`, `crates/wealthfolio-mcp` | **B+** | Well separated; some coupling to core service graph via `ServerAiEnvironment`. |
| `apps/{tauri,server}/src/domain_events` | **B** | Duplicated debounced queue workers (Tauri vs Web) — same logic, two implementations. |
| `apps/frontend/src/adapters` | **A** | Shared command surface with parity tests; clean platform swap via build alias. |
| `apps/{tauri,server}/{commands,api}` | **B** | Thin and consistent, but large surface (~300 commands / many handlers) with some duplication between runtimes. |

**Overall: A-.** The core is genuinely re-usable; the main duplication lives in
the per-runtime access/event layers.

### Migration Difficulty

- **Low for swapping storage:** all DB access is behind core-agnostic repo
  traits; only `storage-sqlite` implements them. A new storage backend would
  implement the same traits.
- **Medium for adding a new runtime/transport:** the adapter pattern makes this
  tractable, but you must register every command twice (Tauri handler + Axum
  endpoint + `COMMANDS` map + shared wrapper) and keep the parity test green.
- **Medium for the domain-event pipeline:** the queue-worker logic is duplicated
  between `apps/tauri` and `apps/server`; converging it would require extracting a
  shared worker crate.
- **High for the service-graph construction:** `build_state` /
  `initialize_context` manually wire ~40 services with careful ordering and
  two-phase domain-event init; any refactor of DI is risky.

### Code Risk

| Risk | Level | Detail |
| --- | --- | --- |
| Service-graph circular deps | **High** | Two-phase domain-event sink init exists specifically to break cycles; fragile ordering in `build_state`. |
| Duplicated domain-event logic | **Medium** | Tauri vs Web queue workers drift risk; bug fixes must be applied twice. |
| Huge command/API surface | **Medium** | ~300 Tauri commands + parallel Axum handlers; API/command parity must be maintained. |
| Large hot files | **Medium** | `portfolio.rs` (70KB), `device_sync_engine.rs` (75KB), `quotes/service.rs` (158KB), `quotes/sync.rs` (104KB), `lots.rs` (91KB) — refactor risk concentrated. |
| Writer-actor serialization | **Medium** | All writes funnel through one writer actor; a bottleneck or ordering bug affects all mutations. |
| Best-effort events | **Low** | Domain events are intentionally dropped on channel close/full — acceptable, but silent loss possible under load. |
| Feature-gated sync code | **Medium** | Large `#[cfg(feature = "...")]` blocks (connect-sync, device-sync) complicate reasoning and testing matrix. |
| Massive `invoke_handler!` list | **Low–Medium** | Manual command registration is error-prone; a missed entry silently breaks a feature. |

---

## 6. Key Findings / Summary

- **Clean layering:** core is DB-agnostic and storage-sqlite is the only Diesel
  holder; the direct `core → storage-sqlite → SQLite` arrow is the single
  supported data path.
- **Adapter pattern is the linchpin** of multi-runtime support: one shared
  command surface, swapped transport by build target, guarded by a parity test.
- **Two runtimes share most business logic**; divergence is concentrated in the
  access layer (commands/API), the domain-event workers, and the schedulers.
- **Domain events are the backpressure-free reactive backbone** — services emit
  via a non-blocking sink; runtime workers debounce and drive recalculation,
  asset enrichment, and broker sync.
- **Lifecycle is deterministic and symmetric** across desktop/web: init tracing →
  DB/migrations → build service graph → start schedulers → serve (IPC or HTTP) →
  listen. The web path additionally handles auth/OIDC and TEAM/device sync.
- **Biggest tail-risks** are the manually wired service graph and the duplicated
  domain-event workers, not the storage or business layers.