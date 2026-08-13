# Reusable Module Assessment -- Wealthfolio Repository

**Date**: 2026-08-12
**Scope**: All top-level crates, packages, and key frontend modules.

---

## Grading Legend

| Grade | Meaning |
|-------|---------|
| **Fully-Reusable** | Zero or trivial changes to extract; usable as a standalone dependency. |
| **Minor-Modification-Required** | A few trait implementations or model extractions needed; no architectural rewrite. |
| **Non-Detachable** | Tightly coupled to the host application; extraction would require a fork or major refactor. |

---

## Crate Evaluations

### 1. `crates/core` -- Core Business Logic

| Field | Assessment |
|-------|------------|
| **Grade** | **Minor-Modification-Required** |
| **Capability** | Domain entities, repository traits, service traits, error types, event system, FX engine, portfolio calculator, goal planner, settings, secrets abstraction, and import/export logic. |
| **Dependency Prerequisites** | tokio, serde, chrono, diesel, rusqlite, r2d2, rust_decimal, reqwest, uuid, async-trait, `wealthfolio-market-data` (sibling). |
| **Decoupling Suggestion** | The crate is already trait-based. Every domain module (`accounts`, `activities`, `assets`, `goals`, `quotes`, `portfolio`, etc.) defines `*RepositoryTrait` and `*ServiceTrait` interfaces. The concrete storage implementation lives in `storage-sqlite`. To reuse: (a) provide an alternative impl of all repository traits, (b) provide a `SecretStore` impl, (c) provide a market-data provider registry. The `diesel`/`rusqlite`/`r2d2` deps are only needed for the default service implementations that construct Diesel-backed repos -- factor those out or gate behind a feature flag. |
| **Migration Difficulty** | Medium. The trait surface is large (150+ methods across all repos). Any consumer must implement the full contract. |
| **Code Risk** | Low. Traits are well-documented and the separation of concerns is clean. |

---

### 2. `crates/storage-sqlite` -- SQLite Storage Implementation

| Field | Assessment |
|-------|------------|
| **Grade** | **Non-Detachable** |
| **Capability** | Diesel ORM layer, connection pooling, migrations, repository implementations for all core domain entities, SQLite-specific utilities. |
| **Dependency Prerequisites** | diesel, diesel_migrations, rusqlite, r2d2, `wealthfolio-core`. |
| **Decoupling Suggestion** | This crate is the canonical impl of `wealthfolio-core`'s traits. It is not reusable outside Wealthfolio because it implements domain-specific repos. However, the *pattern* is reusable: traits in `core` allow swapping to any other storage backend (Postgres, FoundationDB, in-memory). The `DbPool`, `DbConnection`, `WriteHandle` abstractions could be extracted to a generic SQLite helper crate. |
| **Migration Difficulty** | High to extract standalone. Low to replace (traits make it a drop-in swap). |
| **Code Risk** | Low. The coupling to Diesel is explicit and contained. |

---

### 3. `crates/market-data` -- Market Data Providers

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | Provider-agnostic market data fetching: instrument resolution, quote fetching, asset profile enrichment, circuit breaking, rate limiting. Supports Yahoo Finance, Alpha Vantage, Finnhub, Boerse Frankfurt, OpenFIGI, US Treasury calc, Metal Price API, and a fixture provider. |
| **Dependency Prerequisites** | reqwest, serde, serde_json, chrono, tokio, async-trait, yahoo_finance_api, rust_decimal, thiserror. |
| **Decoupling Suggestion** | Already standalone. No dependency on any other Wealthfolio crate. The `ProviderRegistry` and `ResolverChain` patterns are generic enough for any financial application. Package as `market-data` on crates.io with minimal changes. |
| **Migration Difficulty** | Low. Zero internal dependencies to unwind. |
| **Code Risk** | None. Purely self-contained. |

---

### 4. `crates/connect` -- Broker Sync / Wealthfolio Connect

| Field | Assessment |
|-------|------------|
| **Grade** | **Minor-Modification-Required** |
| **Capability** | Broker account sync, activity ingestion, token lifecycle management, cloud API client, post-login bootstrap, sync orchestration, import run tracking. |
| **Dependency Prerequisites** | `wealthfolio-core`, reqwest, serde, chrono, tokio, async-trait, uuid, rust_decimal, base64. |
| **Decoupling Suggestion** | Depends on `core` for `Account`, `Error`, `Result` types, and `PlatformRepositoryTrait`. Extract shared broker models (e.g., `BrokerAccount`, `BrokerConnection`, `BrokerSyncState`) into a standalone `wealthfolio-broker-models` crate so consumers can reuse the sync protocol without pulling in all of `core`. The `BrokerSyncServiceTrait` and `PlatformRepositoryTrait` are the integration points. |
| **Migration Difficulty** | Low-Medium. Requires extracting a small model crate and defining a slim `Platform` / `SecretStore` adapter trait. |
| **Code Risk** | Low. The broker module is feature-gated behind `broker` and has clear trait boundaries. |

---

### 5. `crates/device-sync` -- E2EE Device Synchronization

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | E2EE crypto engine (XChaCha20-Poly1305, X25519 ECDH, HKDF key derivation, SHA-256), device enrollment/pairing protocol, sync API client, time utilities, sync state machine. |
| **Dependency Prerequisites** | chacha20poly1305, x25519-dalek, hkdf, sha2, hmac, base64, reqwest, serde, chrono, tokio, async-trait, uuid, rand, `wealthfolio-core` (only for `SecretStore` trait). |
| **Decoupling Suggestion** | The only dependency on `core` is the `SecretStore` trait (a 3-method interface). Copy or re-export that trait to make this crate fully standalone. The crypto and protocol logic has no Wealthfolio-specific domain knowledge. The `DeviceSyncClient` and `crypto` module are generic E2EE sync primitives. |
| **Migration Difficulty** | Low. Extract `SecretStore` trait or accept it as a generic parameter. |
| **Code Risk** | Low. Crypto operations are well-isolated and tested. |

---

### 6. `crates/spending` -- Spending Tracking

| Field | Assessment |
|-------|------------|
| **Grade** | **Minor-Modification-Required** |
| **Capability** | Cash-account spending tracking, categorization rules, budget configuration, event management, analytics/insight aggregations, activity splitting/classification. |
| **Dependency Prerequisites** | `wealthfolio-core`, tokio, serde, chrono, async-trait, uuid, rust_decimal, regex. |
| **Decoupling Suggestion** | Depends on `core` for shared types (`Account`, `Activity`, `Taxonomy`). The spending domain models could be extracted to a standalone crate with a small set of trait imports from `core`. The categorization engine (`categorization_rules`, `activity_classification`) is self-contained logic. |
| **Migration Difficulty** | Low-Medium. The spending logic is additive (an optional feature gated by a runtime toggle) and does not deeply entangle with core internals. |
| **Code Risk** | Low. The isolation contract is documented in the crate's lib.rs. |

---

### 7. `crates/ai` -- AI Chat / LLM Orchestration

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | Streaming chat service with tool execution loop, LLM provider abstraction (rig-core), provider catalog/settings management, prompt templates, chat history, title generation, tool registry, stream event types, evaluation framework. |
| **Dependency Prerequisites** | rig-core, `wealthfolio-agent-tools`, `wealthfolio-core`, `wealthfolio-spending`, tokio, serde, reqwest, async-trait, futures, csv. |
| **Decoupling Suggestion** | The `ChatService` and `AiEnvironment` trait provide a clean abstraction boundary. The provider abstraction (`ProviderService`, `AiProviderService`) is provider-agnostic and could power any LLM application. The `tools` module delegates to `wealthfolio-agent-tools` for the tool catalog. To reuse without Wealthfolio domain, swap the tool catalog and `AiEnvironment` impl. The `rig-core` integration pattern is generic. |
| **Migration Difficulty** | Low-Medium. The `AiEnvironment` trait (which provides `SecretStore`, `AgentEnvironment`, and chat repository) is the only integration point. |
| **Code Risk** | Low. The crate is well-structured with clear module boundaries. |

---

### 8. `crates/agent-tools` -- Agent Tool Definitions

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | Runtime-neutral tool definitions for LLM agents. Defines tool catalog, tool execution, scope enforcement, and `AgentEnvironment` trait. Used by both in-app AI and MCP server. |
| **Dependency Prerequisites** | `wealthfolio-core`, `wealthfolio-spending`, serde, async-trait, chrono, rust_decimal, uuid, thiserror. |
| **Decoupling Suggestion** | The `AgentEnvironment` trait is the sole integration point -- it abstracts account/activity/asset/portfolio operations. The tool catalog, scoping, and tool definitions are generic. To reuse outside Wealthfolio, implement `AgentEnvironment` for your domain and provide alternate tool definitions. The crate's own `env.rs` defines the trait contract. |
| **Migration Difficulty** | Low. The `AgentToolCatalog` and `AgentTool` types are clean abstractions. |
| **Code Risk** | Low. Documented as "runtime-neutral" in its own lib.rs. |

---

### 9. `crates/wealthfolio-mcp` -- MCP Server

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | MCP (Model Context Protocol) Streamable HTTP server implementation. Converts `wealthfolio-agent-tools` catalog into MCP tools, handles scope enforcement, audit logging, PAT authentication, and protocol conversion. |
| **Dependency Prerequisites** | rmcp (MCP protocol crate), `wealthfolio-agent-tools`, `wealthfolio-storage-sqlite`, serde, tokio, http, base64, sha2. |
| **Decoupling Suggestion** | The `McpServerBuilder` and `WealthfolioMcpHandler` compose `wealthfolio-agent-tools` into an MCP server. The MCP protocol handling (`rmcp` integration) is generic. To reuse: swap the tool catalog dependency. The `auth` module (PAT, `McpAuthContext`, `ActorKind`) is a reusable auth framework for MCP. |
| **Migration Difficulty** | Low. The MCP protocol layer is separate from the tool implementations. |
| **Code Risk** | Low. Protocol conversion is a thin layer over the tool catalog. |

---

### 10. `packages/addon-sdk` -- Addon SDK

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | TypeScript SDK for building Wealthfolio addons. Provides manifest types, host API bindings, permission system, query keys, utility functions, goal progress helpers. Published as `@wealthfolio/addon-sdk`. |
| **Dependency Prerequisites** | react, react-dom (peer deps). Dev deps: @tanstack/react-query, tsup, typescript. |
| **Decoupling Suggestion** | Already a standalone npm package with MIT license. Published separately. The `host-api.ts` and `host-dependencies.ts` define the contract between addon and host -- reusable by any host application implementing that contract. |
| **Migration Difficulty** | None. Already published as a standalone package. |
| **Code Risk** | None. |

---

### 11. `packages/addon-dev-tools` -- Addon Development CLI

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | CLI tool (`wealthfolio-addon` / `wealthfolio`) for scaffold, dev server with hot reload, and addon development workflow. |
| **Dependency Prerequisites** | express, chokidar, commander, cors. |
| **Decoupling Suggestion** | Already standalone. Published as `@wealthfolio/addon-dev-tools`. The dev server and CLI are generic enough to support any addon system. |
| **Migration Difficulty** | None. |
| **Code Risk** | None. |

---

### 12. `packages/ui` -- Shared UI Component Library

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | shadcn-based UI component library: buttons, dialogs, forms, tables, charts, data grids, financial visualizations, date pickers, etc. Published as `@wealthfolio/ui`. |
| **Dependency Prerequisites** | react, react-dom, i18next, react-i18next (peer deps). Runtime deps: @radix-ui/*, @tanstack/react-query, @tanstack/react-table, recharts, motion, react-hook-form, date-fns, lucide-react, cmdk, sonner, tailwind-merge, class-variance-authority, etc. |
| **Decoupling Suggestion** | Already standalone. The `financial` subdirectory contains Wealthfolio-specific components (portfolio charts, holdings tables) that may need minor renaming, but the `ui` and `common` components are generic. |
| **Migration Difficulty** | None. Already published as a standalone package (MIT license). |
| **Code Risk** | None. |

---

### 13. `apps/frontend/src/adapters/` -- Runtime Adapter Pattern

| Field | Assessment |
|-------|------------|
| **Grade** | **Fully-Reusable** |
| **Capability** | Abstracts the runtime environment (Tauri desktop vs browser web) behind a unified interface. The `index.ts` re-exports from `tauri/` or `web/` based on `BUILD_TARGET`. |
| **Dependency Prerequisites** | None beyond the frontend's own dependencies. |
| **Decoupling Suggestion** | The adapter pattern itself is reusable in any Electron/Tauri/Web hybrid app. The adapter interface defines commands like `getInstalledAddons`, `loadAddon`, `logger`, etc. The `shared/` directory contains common implementations. To reuse: adopt the `adapter-command-parity.test.ts` contract to ensure both backends match. |
| **Migration Difficulty** | Low. The pattern is ~10 files with a clear interface contract. |
| **Code Risk** | Low. The adapter is a thin abstraction layer. |

---

### 14. `apps/frontend/src/addons/` -- Addon Runtime

| Field | Assessment |
|-------|------------|
| **Grade** | **Non-Detachable** |
| **Capability** | Addon lifecycle management: discovery, loading, unloading, activation coordination, contribution registry (nav items, routes), iframe sandboxing, type bridge, dev mode support. |
| **Dependency Prerequisites** | React, @wealthfolio/addon-sdk, sonner, iframe sandboxing infrastructure, frontend routing system. |
| **Decoupling Suggestion** | This module is deeply integrated into the Wealthfolio frontend application. It manages React routes, navigation state, iframe sandboxing, and depends on the app's adapter layer. The activation coordinator, contribution registry, and iframe manager are tightly coupled to the app's React component tree. Extraction would require a significant abstraction layer. |
| **Migration Difficulty** | High. The addon runtime is ~20 files with extensive cross-references to the frontend's routing, state, and UI systems. |
| **Code Risk** | Medium. The `addons-core.ts` loading logic is complex, with activation epochs, epoch-based dedup guards, pinned vs lazy loading, and self-healing retry logic -- all tied to frontend lifecycle. |

---

## Summary Table

| # | Module | Grade | Standalone? | Key Decoupling Step |
|---|--------|-------|-------------|---------------------|
| 1 | `crates/core` | Minor-Modification-Required | No | Implement storage + SecretStore traits |
| 2 | `crates/storage-sqlite` | Non-Detachable | No | Pattern is reusable; impl is Wealthfolio-specific |
| 3 | `crates/market-data` | Fully-Reusable | **Yes** | Already standalone |
| 4 | `crates/connect` | Minor-Modification-Required | No | Extract broker models to standalone crate |
| 5 | `crates/device-sync` | Fully-Reusable | Near-yes | Extract/copy SecretStore trait |
| 6 | `crates/spending` | Minor-Modification-Required | No | Extract shared core types |
| 7 | `crates/ai` | Fully-Reusable | Near-yes | Swap tool catalog + AiEnvironment impl |
| 8 | `crates/agent-tools` | Fully-Reusable | Near-yes | Implement AgentEnvironment for target domain |
| 9 | `crates/wealthfolio-mcp` | Fully-Reusable | Near-yes | Swap tool catalog dependency |
| 10 | `packages/addon-sdk` | Fully-Reusable | **Yes** | Already published on npm |
| 11 | `packages/addon-dev-tools` | Fully-Reusable | **Yes** | Already published on npm |
| 12 | `packages/ui` | Fully-Reusable | **Yes** | Already published on npm |
| 13 | `apps/frontend/src/adapters/` | Fully-Reusable | Near-yes | Adopt the interface contract |
| 14 | `apps/frontend/src/addons/` | Non-Detachable | No | Tightly coupled to frontend React app |

---

## Key Findings

1. **The trait-based architecture in `crates/core` is the single most important design decision for reusability.** Every domain module defines repository and service traits. The `storage-sqlite` crate is merely one implementation.

2. **Three crates are already fully standalone with zero internal dependencies:** `crates/market-data`, `packages/addon-sdk`, `packages/addon-dev-tools`, and `packages/ui`. These could be published independently today.

3. **The `crates/device-sync` crate is fully reusable with a trivial change:** its only dependency on `core` is the `SecretStore` trait (3 methods: `set_secret`, `get_secret`, `delete_secret`). Copying or re-exporting this trait makes it standalone.

4. **The `crates/agent-tools` + `crates/wealthfolio-mcp` + `crates/ai` stack forms a reusable AI agent framework.** The `AgentEnvironment` trait is the sole integration point. Any application could implement it and get a full MCP server + streaming chat interface.

5. **The addon runtime (`apps/frontend/src/addons/`) is the most tightly coupled module.** Its iframe sandboxing, contribution registry, and activation coordinator are deeply embedded in the React application. This is the only module that would require a significant rewrite to extract.

6. **Migration risk is low across all modules.** The codebase consistently uses traits, dependency injection, and clear module boundaries. No module has hidden coupling or implicit dependencies that would cause surprises during extraction.