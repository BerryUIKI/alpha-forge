# Appendix: File Index & Glossary

> Part of the Wealthfolio documentation set. This appendix provides a complete
> index of source and configuration files, plus a glossary of abbreviations and
> domain terms used across the codebase.

---

## 1. Source File Index

### 1.1 Rust Files (`apps/` + `crates/`)

#### `apps/server` — Axum web server (web mode)

| File | Purpose |
| --- | --- |
| `build.rs` | Build script (compiles embedded assets / version info). |
| `src/main.rs` | Server entry point: wires modules, router, security headers, device-sync startup. |
| `src/lib.rs` | Library root: re-exports modules and `AppState`, `build_state`, `init_tracing`. |
| `src/main_lib.rs` | Core server bootstrap: shared state, tracing init, app construction. |
| `src/config.rs` | Runtime/server configuration loading. |
| `src/error.rs` | API error types and HTTP error mapping. |
| `src/models.rs` | Shared server-side data models/SQLite-backed types. |
| `src/auth.rs` | HTTP authentication (sessions, guards). |
| `src/oidc.rs` | OpenID Connect integration for login (SSO). |
| `src/features.rs` | Feature-gating helpers. |
| `src/events.rs` | Event emission over websockets/SSE. |
| `src/scheduler.rs` | Background job scheduler (quotes, sync, etc.). |
| `src/ai_environment.rs` | Server-side AI provider environment setup. |
| `src/domain_events/mod.rs` | Domain event system module root. |
| `src/domain_events/planner.rs` | Event planning/aggregation logic. |
| `src/domain_events/queue_worker.rs` | Background event queue worker. |
| `src/domain_events/sink.rs` | Event sink (persistence/dispatch). |
| `src/secrets/mod.rs` | Server-side secret handling. |
| `src/mcp/mod.rs` | MCP server endpoint module root. |
| `src/mcp/auth.rs` | MCP authentication. |
| `src/mcp/audit_sink.rs` | MCP audit logging sink. |
| `src/api.rs` | Axum router assembly + security headers. |
| `src/api/accounts.rs` | `/accounts` HTTP handlers. |
| `src/api/activities.rs` | `/activities` handlers. |
| `src/api/addons.rs` | Addon management handlers. |
| `src/api/addon_network.rs` | Addon network/marketplace handlers. |
| `src/api/agent_access.rs` | Agent access / PAT handlers. |
| `src/api/ai_chat.rs` | AI chat handlers. |
| `src/api/ai_providers.rs` | AI provider config handlers. |
| `src/api/allocation_targets.rs` | Allocation targets/rebalancing handlers. |
| `src/api/alternative_assets.rs` | Alternative assets handlers. |
| `src/api/assets.rs` | Assets handlers. |
| `src/api/connect.rs` | Wealthfolio Connect (broker sync) handlers. |
| `src/api/custom_providers.rs` | Custom market-data provider handlers. |
| `src/api/data_exports.rs` | Data export handlers. |
| `src/api/database_backups.rs` | Database backup/restore handlers. |
| `src/api/device_sync.rs` | Device sync handlers. |
| `src/api/device_sync_engine.rs` | Device sync engine plumbing. |
| `src/api/exchange_rates.rs` | FX exchange-rate handlers. |
| `src/api/goals.rs` | Goals handlers. |
| `src/api/health.rs` | Health-check handlers. |
| `src/api/holdings/mod.rs` | Holdings endpoint module root. |
| `src/api/holdings/dto.rs` | Holdings DTOs. |
| `src/api/holdings/handlers.rs` | Holdings HTTP handlers. |
| `src/api/holdings/mappers.rs` | Holdings DTO mappers. |
| `src/api/limits.rs` | Contribution limits handlers. |
| `src/api/market_data.rs` | Market-data handlers. |
| `src/api/net_worth.rs` | Net-worth handlers. |
| `src/api/performance.rs` | Performance handlers. |
| `src/api/portfolio.rs` | Portfolio handlers. |
| `src/api/portfolios.rs` | Portfolios handlers. |
| `src/api/secrets.rs` | Secrets handlers. |
| `src/api/settings.rs` | Settings handlers. |
| `src/api/shared.rs` | Shared API helpers. |
| `src/api/spending.rs` | Spending handlers. |
| `src/api/sync_crypto.rs` | Sync crypto handlers. |
| `src/api/taxonomies.rs` | Taxonomies handlers. |
| `tests/*.rs` | Server integration tests (auth, oidc, health, agent access, static routes, income API). |

#### `apps/tauri` — Tauri desktop app

| File | Purpose |
| --- | --- |
| `build.rs` | Build script (icon/asset embedding, version info). |
| `src/lib.rs` | Tauri app library root: module wiring, setup, event emission. |
| `src/main.rs` | Desktop binary entry point. |
| `src/error.rs` | Desktop error types. |
| `src/events.rs` | App event emission helpers. |
| `src/listeners.rs` | Tauri event listeners. |
| `src/menu.rs` | Native application menu. |
| `src/scheduler.rs` | Background scheduler for desktop. |
| `src/secret_store.rs` | OS keyring secret store adapter. |
| `src/updater.rs` | Auto-update integration. |
| `src/context/mod.rs` | Service context module root. |
| `src/context/ai_environment.rs` | Desktop AI environment. |
| `src/context/providers.rs` | Dependency providers. |
| `src/context/registry.rs` | Service registry. |
| `src/domain_events/*.rs` | Domain event system (same pattern as server). |
| `src/mcp/*.rs` | Desktop MCP server (mod, server, middleware, lockfile, audit_sink, tests). |
| `src/services/mod.rs` | Desktop services module root. |
| `src/services/connect_service.rs` | Connect broker sync service. |
| `src/commands/mod.rs` | IPC command module root. |
| `src/commands/error.rs` | Command error types. |
| `src/commands/account.rs` | Account IPC commands. |
| `src/commands/activity.rs` | Activity IPC commands. |
| `src/commands/addon.rs` | Addon IPC commands. |
| `src/commands/addon_network.rs` | Addon network IPC commands. |
| `src/commands/ai_chat.rs` | AI chat IPC commands. |
| `src/commands/ai_providers.rs` | AI provider IPC commands. |
| `src/commands/allocation_targets.rs` | Allocation target IPC commands. |
| `src/commands/alternative_assets.rs` | Alternative assets IPC commands. |
| `src/commands/asset.rs` | Asset IPC commands. |
| `src/commands/brokers_sync.rs` | Broker sync IPC commands. |
| `src/commands/custom_provider.rs` | Custom provider IPC commands. |
| `src/commands/device_enroll_service.rs` | Device enrollment. |
| `src/commands/device_sync/mod.rs` | Device sync module root. |
| `src/commands/device_sync/engine.rs` | Device sync engine. |
| `src/commands/device_sync/snapshot.rs` | Device sync snapshot. |
| `src/commands/fire.rs` | FIRE planner IPC commands. |
| `src/commands/goal.rs` | Goal IPC commands. |
| `src/commands/health.rs` | Health IPC commands. |
| `src/commands/limits.rs` | Contribution limits IPC commands. |
| `src/commands/market_data.rs` | Market-data IPC commands. |
| `src/commands/mcp.rs` | MCP IPC commands. |
| `src/commands/platform.rs` | Platform info IPC commands. |
| `src/commands/portfolio.rs` | Portfolio IPC commands. |
| `src/commands/portfolios.rs` | Portfolios IPC commands. |
| `src/commands/providers_settings.rs` | Provider settings IPC commands. |
| `src/commands/secrets.rs` | Secrets IPC commands. |
| `src/commands/settings.rs` | Settings IPC commands. |
| `src/commands/spending.rs` | Spending IPC commands. |
| `src/commands/sync_crypto.rs` | Sync crypto IPC commands. |
| `src/commands/taxonomy.rs` | Taxonomy IPC commands. |
| `src/commands/utilities.rs` | Utility IPC commands. |
| `src/commands/wealthfolio_connect.rs` | Connect IPC commands. |

#### `crates/core` — Business logic, models, services (largest crate)

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Core crate root. |
| `src/constants.rs` | Shared constants. |
| `src/errors.rs` | Domain error types. |
| `src/exports.rs` | Data export logic. |
| `src/accounts/*` | Account model, service, traits, constants. |
| `src/activities/*` | Activity model/service, CSV parser, idempotency, transfers, import runs. |
| `src/addons/*` | Addon traits, models, network, service, storage. |
| `src/assets/*` | Assets, alternative assets, classification, auto-classification. |
| `src/custom_provider/*` | Custom market-data provider model/service/store. |
| `src/events/*` | Domain event model, sink. |
| `src/fx/*` | Currency, FX conversion, rates model/service. |
| `src/goals/*` | Goals model/service. |
| `src/health/*` | Health checks (data consistency, FX integrity, price staleness, etc.) and fixes. |
| `src/limits/*` | Contribution limits model/service. |
| `src/lots/mod.rs` | Tax lots logic. |
| `src/planning/*` | Retirement planning engine, save-up, withdrawal analysis. |
| `src/portfolio/*` | Holdings, allocation, income, net worth, performance, snapshot, valuation, FIRE. |
| `src/portfolios/*` | Multi-portfolio model/service. |
| `src/quotes/*` | Quote fetching, sync, scheduler, provider settings, imports. |
| `src/secrets/mod.rs` | Core secret handling. |
| `src/settings/*` | Settings model/service. |
| `src/sync/*` | App sync model. |
| `src/taxonomies/*` | Taxonomy model/service. |
| `src/utils/*` | CUSIP, ISIN, OCC symbol, decimal serde, time utils. |

#### `crates/storage-sqlite` — Diesel ORM, repositories, migrations

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Storage crate root. |
| `src/errors.rs` | Storage error types. |
| `src/schema.rs` | Diesel DB schema definitions. |
| `src/utils.rs` | Storage utilities. |
| `src/db/mod.rs` | DB connection pool / setup. |
| `src/db/write_actor.rs` | Serialized write actor. |
| `src/accounts/*`, `src/assets/*`, `src/goals/*`, `src/limits/*`, `src/settings/*`, `src/fx/*`, `src/health/*`, `src/taxonomies/*`, `src/portfolios/*`, `src/custom_provider/*`, `src/addons/*`, `src/ai_chat/*`, `src/agent/*`, `src/market_data/*` | Per-domain Diesel models + repositories. |
| `src/portfolio/*` | Allocation targets, snapshot, valuation models/repos. |
| `src/spending/*` | Spending domain persistence (budget, events, splits, rules, etc.). |
| `src/sync/*` | App-sync engine, import runs, platform sync, state persistence. |
| `src/lots.rs` | Tax lots repository. |
| `tests/agent_repos.rs` | Agent repository integration tests. |

#### `crates/market-data` — Market data providers

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Market-data crate root. |
| `src/errors/*` | Error + retry logic. |
| `src/models/*` | Instrument, quote, dividend, profile, search, coverage, provider params. |
| `src/provider/*` | Provider adapters: alpha_vantage, boerse_frankfurt, finnhub, fixture, marketdata_app, metal_price_api, openfigi, us_treasury_calc, yahoo. |
| `src/registry/*` | Provider registry, circuit breaker, rate limiter, validator, skip reason. |
| `src/resolver/*` | Asset/exchange resolvers, exchange metadata, suffix mapping. |

#### `crates/connect` — Wealthfolio Connect (broker integrations)

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Connect crate root. |
| `src/client.rs` | HTTP client. |
| `src/broker/*` | Broker sync orchestrator, phases (activity/holdings), service, progress, traits. |
| `src/broker_ingest/*` | Broker ingest core adapter + models. |
| `src/platform/mod.rs` | Platform abstraction. |
| `src/post_login_bootstrap.rs` | Post-login bootstrap flow. |
| `src/request_metadata.rs` | Request metadata. |
| `src/token_lifecycle.rs` | Access/refresh token lifecycle. |

#### `crates/device-sync` — Device sync + E2EE

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Device-sync crate root. |
| `src/client.rs` | Sync client. |
| `src/crypto.rs` | E2EE crypto primitives. |
| `src/engine/*` | Sync engine, ports, runtime. |
| `src/enroll_service.rs` | Device enrollment. |
| `src/error.rs` | Error types. |
| `src/time.rs` | Clock abstraction. |
| `src/types.rs` | Shared sync types. |

#### `crates/spending` — Spending/budgeting/insights domain

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Spending crate root. |
| `src/error.rs` | Error types. |
| `src/activity_allocations.rs` | Activity allocation helpers. |
| `src/activity_assignments/*`, `src/activity_events/*`, `src/activity_splits/*` | Model/service/traits per subdomain. |
| `src/activity_classification.rs` | Classification logic. |
| `src/analytics/*` | Spending analytics model/service. |
| `src/budget/*` | Budget model/service. |
| `src/cash_activities/*` | Cash activity model/service. |
| `src/categories_seed/*` | Seed categories. |
| `src/categorization_rules/*` | Rule matching, model, presets, service. |
| `src/events/*` | Spending events model/service. |
| `src/insight/*` | Spending insights model/service. |
| `src/settings/*` | Spending settings model/service. |

#### `crates/ai` — AI providers and LLM integration

| File | Purpose |
| --- | --- |
| `src/lib.rs` | AI crate root. |
| `src/bin/eval.rs` | CLI eval binary. |
| `src/chat/*` | Chat: history, streaming, provider clients, working context, attachments. |
| `src/env/*` | Environment config + test env. |
| `src/error.rs` | Error types. |
| `src/eval/*` | Eval harness + scenarios. |
| `src/live_evals/*` | Live eval runner, schema, trace. |
| `src/prompt_template.rs` / `src/prompt_template_service.rs` | Prompt templating. |
| `src/provider_model.rs`, `src/provider_service.rs`, `src/provider_urls.rs`, `src/providers.rs` | Provider model/service/URLs. |
| `src/stream_hook.rs` | Output streaming hook. |
| `src/title_generator.rs` | Chat title generation. |
| `src/tools/*` | AI tool definitions (import_csv, rig_adapter). |
| `src/types.rs` | Shared AI types. |
| `tests/*` | Allowlist, system prompt, tool output parity, tool schema tests. |

#### `crates/agent-tools` — Agent tool definitions

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Agent-tools crate root. |
| `src/catalog.rs` | Tool catalog. |
| `src/constants.rs` | Tool constants. |
| `src/env.rs` | Environment config. |
| `src/scope.rs` | Tool scoping/permissions. |
| `src/tool.rs` | Tool trait. |
| `src/tools/*` | Individual tools: accounts, activities, activity_import, allocation, asset_classification, asset_taxonomies, cash_balances, categorization_context, commit_activity, contribution_limits, create_categorization_rule, goals, health, holdings, income, net_worth, performance, portfolios, propose_categories, record_activities, record_activity, valuation. |

#### `crates/wealthfolio-mcp` — MCP library crate

| File | Purpose |
| --- | --- |
| `src/lib.rs` | MCP crate root. |
| `src/audit.rs` | Audit logging. |
| `src/auth.rs` | MCP auth. |
| `src/handler.rs` | MCP request handler. |
| `src/pat.rs` | Personal access token handling. |
| `src/service.rs` | MCP service. |
| `tests/http_roundtrip.rs` | HTTP round-trip test. |

---

### 1.2 TypeScript/React Files (`apps/frontend/src/`)

#### Root
| File | Purpose |
| --- | --- |
| `main.tsx` | App entry: ReactDOM, providers, i18n init, addon dev mode/lockdown. |
| `App.tsx` | Top-level component: providers, auth gate, routing. |
| `routes.tsx` | Route definitions and page wiring. |
| `lockdown.ts` | Desktop input/lockdown. |
| `use-global-event-listener.ts` | Global event listener hook. |
| `vite-env.d.ts` | Vite env typings. |

#### `adapters/` — Runtime detection (desktop vs web) + command wrappers
| File | Purpose |
| --- | --- |
| `index.ts` | Adapter dispatch entry. |
| `types.ts` | Adapter type contracts. |
| `adapter-command-parity.test.ts` | Parity tests across adapters. |
| `shared/*` | Runtime-agnostic command implementations (accounts, activities, portfolio, goals, etc.). |
| `tauri/*` | Tauri-specific implementations (core, crypto, events, exports, files, settings, ai-streaming, fire-planner, etc.). |
| `web/*` | Web-specific implementations (core, crypto, events, exports, files, settings, etc.). |

#### `addons/` — Addon runtime
| File | Purpose |
| --- | --- |
| `activation-coordinator.ts` | Addon activation coordination. |
| `addon-runtime-loader.tsx` | Loads addon runtimes. |
| `addons-core.ts`, `addons-loader.ts` | Core addon logic + loader. |
| `addons-dev-mode.ts` | Dev-mode helpers. |
| `addons-runtime-context.ts` | Runtime context/provider. |
| `contribution-registry.ts` | Addon contribution registry. |
| `type-bridge.ts` | Type bridge layer. |
| `iframe/*` | Iframe sandbox: manager, route, module rewriter, asset registry, styles, theme, host dependencies, entry, ticker-avatar bridge. |

#### `components/` — Shared UI components
| File | Purpose |
| --- | --- |
| `*.tsx` / `*.ts` | Account/benchmark/currency/symbol selectors, action palette, charts, headers, dialogs, performance charts, theme/language selectors, ticker avatar/search, update dialog, etc. |
| `page/` | Swipable page + routes page. |
| `classification/` | Taxonomy classification sheets. |

#### `context/` — React contexts
| File | Purpose |
| --- | --- |
| `auth-context.tsx` | Auth provider/gate. |
| `portfolio-sync-context.tsx` | Portfolio sync trigger context. |
| `privacy-context.tsx` | Privacy (hide balances) context. |

#### `features/` — Self-contained feature modules
| File | Purpose |
| --- | --- |
| `ai-assistant/*` | AI chat UI: API/stream, components (chat shell, thread, providers, tool UIs), hooks, types. |
| `devices-sync/*` | Device sync UI: pairing flow, E2EE setup, keyring, hooks, services. |
| `goals/*` | Goals UI + retirement planner + save-up. |
| `spending/*` | Spending/budgeting UI: adapters, components, reports/insights, hooks, lib, pages, types. |
| `wealthfolio-connect/*` | Connect UI: broker cards, sync, import runs, auth callback, services, provider. |

#### `hooks/` — Shared custom hooks
| File | Purpose |
| --- | --- |
| `index.ts` | Hook barrel. |
| `use-*.ts` | Data/query hooks (accounts, holdings, portfolios, settings, market data, performance, etc.). |

#### `i18n/` — Internationalization
| File | Purpose |
| --- | --- |
| `i18n.ts` | i18next instance setup. |
| `locales.ts` | Locale registry. |
| `locales/{de,en,es,fr,ja,ko,zh}/*.json` | Translation namespaces per language. |

#### `lib/` — Frontend utilities & types
| File | Purpose |
| --- | --- |
| `*.ts` | Utilities: activity restrictions, asset resolution, auth token, performance, schemas (zod), query keys, cookie/device utils, ISIN/OCC, etc. |
| `types/` | Custom provider, quote import types. |
| `settings-provider.tsx` | Settings provider. |

#### `pages/` — Route pages
| File | Purpose |
| --- | --- |
| `account/` | Account detail: holdings, metrics, contribution limit, performance grid, cash audit. |
| `activity/` | Activity manager, data grid, forms (buy/sell/transfer/etc.), import wizard, mobile forms. |
| `ai-assistant/` | AI assistant page. |
| `allocation-targets/` | Allocation targets, drift, rebalance UI. |
| `asset/` | Assets list, security detail, alternative assets, quote history. |
| `auth/` | Login page. |
| `dashboard/` | Portfolio dashboard, balance, accounts summary, goals, top holdings. |
| `health/` | Data health page. |
| `holdings/` | Holdings tables, charts, drillable donuts. |
| `income/` | Income history UI. |
| `insights/` | Portfolio insights + overview. |
| `layouts/` | App layout, mobile nav, sidebar, onboarding layout. |
| `net-worth/` | Net worth charts, breakdown. |
| `not-found.tsx` | 404 page. |
| `onboarding/` | Onboarding flow (steps, appearance, connect). |
| `performance/` | Performance page + chart series. |
| `settings/` | Settings pages: about, accounts, addons, agent-access, ai-providers, appearance, contribution-limits, exports, general, market-data, portfolios, spending, taxonomies, wealthfolio-connect. |

#### `test/`, `types/`
| File | Purpose |
| --- | --- |
| `test/fake-addon-dom.ts`, `test/setup.ts` | Test helpers/setup. |
| `types/*.d.ts` | Global/custom type declarations. |

#### `packages/` (shared workspaces)
| File | Purpose |
| --- | --- |
| `addon-sdk/src/*` | Addon SDK: data types, host API, permissions, manifest, query keys, utils, version. |
| `ui/src/components/common/*` | Reusable common components (charts, confirm dialogs, searchable select, ticker avatar). |
| `ui/src/components/data-grid/*` | Editable data-grid component. |
| `ui/src/components/financial/*` | Financial displays/inputs (money, currency, quantity, gain, privacy). |
| `ui/src/components/ui/*` | shadcn/Radix UI primitives (button, dialog, select, table, tabs, etc.). |
| `ui/src/hooks/*`, `ui/src/lib/*` | UI hooks and utilities (currencies, ids, utils). |

#### `e2e/` — Playwright end-to-end tests
| File | Purpose |
| --- | --- |
| `*.spec.ts` | End-to-end flows (happy path, activities, CSV import, asset creation, holdings, splits, etc.). |
| `helpers.ts` | E2E helper utilities. |

---

### 1.3 Configuration Files

| File | Purpose |
| --- | --- |
| `Cargo.toml` | Workspace root manifest. |
| `crates/*/Cargo.toml`, `apps/*/Cargo.toml` | Per-crate/app manifests. |
| `rust-toolchain.toml` | Rust toolchain pin. |
| `package.json` | Root npm scripts/deps. |
| `apps/frontend/package.json` | Frontend package. |
| `packages/*/package.json` | Shared package manifests. |
| `pnpm-workspace.yaml` | pnpm workspace config. |
| `tsconfig.json`, `tsconfig.base.json`, `tsconfig.node.json`, `tsconfig.test.json` | TypeScript configs. |
| `apps/frontend/tsconfig*.json` | Frontend TS configs. |
| `vite.config.ts`, `vite.addon-sandbox.config.ts` | Vite build configs. |
| `i18next.config.ts` | i18next configuration. |
| `apps/frontend/index.html` | Frontend HTML shell. |
| `apps/frontend/addon-sandbox.html` | Addon sandbox HTML. |
| `apps/frontend/public/manifest.json` | PWA manifest. |
| `apps/frontend/public/splash.css` | Splash screen styles. |
| `apps/frontend/src/globals.css` | Global theme tokens. |
| `apps/frontend/src/i18n/locales/*/*.json` | Translation namespaces. |
| `apps/tauri/tauri.conf.json` | Tauri app config. |
| `apps/tauri/capabilities/*.json` | Tauri permission capabilities (desktop/ios/mobile). |
| `apps/tauri/gen/apple/project.yml` | Apple project config. |
| `crates/storage-sqlite/diesel.toml` | Diesel config. |
| `crates/storage-sqlite/migrations/*/metadata.toml` | Migration metadata. |
| `crates/spending/seeds/presets/*.json` | Spending category presets. |
| `crates/ai/src/ai_providers.json` | AI provider definitions. |
| `crates/ai/evals/cases/*.toml` | AI eval cases. |
| `crates/market-data/src/resolver/exchanges.json` | Exchange metadata. |
| `packages/ui/components.json` | shadcn config. |
| `packages/ui/src/styles.css` | UI package styles. |
| `compose.yml`, `compose.dev.yml`, `compose.proxy.yml` | Docker Compose configs. |
| `Dockerfile` | Container build. |
| `.github/workflows/*.yml` | CI/CD workflows. |
| `.devcontainer/devcontainer.json` | Dev container config. |
| `.vscode/*.json` | Editor config/settings. |
| `.claude/settings.json` | Claude settings. |
| `dev/keycloak/realm-wealthfolio.json` | Keycloak realm seed. |
| `e2e/fixtures/quotes/instruments.json` | E2E fixture data. |
| `playwright.config.ts`, `playwright.addon-sandbox.config.ts` | Playwright configs. |
| `eslint.config.js`, `eslint.base.config.js` | ESLint configs. |

---

## 2. Glossary & Abbreviations

| Abbreviation | Expansion / Definition |
| --- | --- |
| WF | WealthFolio — project/brand name; also the environment variable prefix (e.g. `WF_*`). |
| MCP | Model Context Protocol — standard for exposing tools/context to LLMs. |
| E2EE | End-to-End Encryption — device sync data is encrypted before leaving the device. |
| TWR | Time-Weighted Return — return measure isolating performance from cash flows. |
| MWR | Money-Weighted Return — return weighted by timing/magnitude of cash flows. |
| IRR | Internal Rate of Return — discount rate at which net present value is zero. |
| FIRE | Financial Independence, Retire Early — retirement planning methodology. |
| OIDC | OpenID Connect — OAuth2-based identity layer for login/SSO. |
| CSP | Content Security Policy — HTTP security header restricting resource loading. |
| Axum | Rust web framework used for the web-mode HTTP server. |
| Diesel | Rust ORM used for SQLite persistence in `storage-sqlite`. |
| Tauri | Desktop app framework (Rust core + web frontend). |
| SQLite | Embedded relational database storing all local data. |
| shadcn | UI component library (copy-paste components built on Radix/Tailwind). |
| Recharts | React charting library used for charts. |
| Zod | TypeScript schema validation library. |
| i18next | Internationalization framework. |
| Zustand | State management library. |
| TanStack Query | Data fetching/caching library (used as React Query). |
| Radix UI | Accessible UI primitives underlying shadcn components. |
| CUSIP | Committee on Uniform Securities Identification Procedures — US security identifier. |
| ISIN | International Securities Identification Number — global security identifier. |
| OCC | Options Clearing Corporation — provider of option symbol scheme. |
| PHC | Password Hash Crypt — Argon2 password hash string format. |
| JWT | JSON Web Token — signed token for auth/session. |
| PKCE | Proof Key for Code Exchange — OAuth2 flow for public clients. |
| SSO | Single Sign-On — one login grants access across services. |
| IPC | Inter-Process Communication — Tauri command bridge. |
| HMR | Hot Module Replacement — dev-time live module reload. |
| CLI | Command Line Interface. |
| SDK | Software Development Kit — e.g. `@wealthfolio/addon-sdk`. |
| ORM | Object-Relational Mapping. |
| API | Application Programming Interface. |
| REST | Representational State Transfer — HTTP API style. |
| CRUD | Create, Read, Update, Delete — basic data operations. |
| CSV | Comma-Separated Values — import/export format. |
| JSON | JavaScript Object Notation — data interchange format. |
| TOML | Tom's Obvious Minimal Language — config format (Rust/Cargo). |
| YAML | YAML Ain't Markup Language — config format (CI, compose). |
| AGPL | GNU Affero General Public License — project license. |