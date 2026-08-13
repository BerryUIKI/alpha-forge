# Wealthfolio Technology Stack Migration Guide

## 1. Existing Pain-Points of Current Technology Stack

### 1.1 Diesel ORM with SQLite

- **Migration complexity**: 48 Diesel-managed SQL migrations, each with hand-written SQL. Diesel's `embed_migrations!` macro compiles migrations into the binary, which is convenient but ties the migration system tightly to the build pipeline. Switching to a new ORM requires extracting and re-applying all 48 migration SQL files.
- **No async support natively**: Diesel 2.x is synchronous. The project works around this via a `WriteHandle` actor (tokio `mpsc` channel + background writer thread) that serializes writes through a single `SqliteConnection`. This is an architectural workaround, not native async, and adds complexity (oneshot reply channels, job type erasure, outbox observers).
- **SQLite-specific SQL patterns**: `chunk_for_sqlite()`, `SQLITE_MAX_PARAMS_CHUNK`, `PRAGMA` statements (`journal_mode=WAL`, `busy_timeout`, `synchronous=NORMAL`), `VACUUM INTO` for backups, and `rusqlite` for direct connection access during backup/restore. Any migration to a different database requires rewriting all repository implementations.
- **R2D2 connection pooling**: Limited to synchronous pool operations. Pool size of 8, with connection customizers that set SQLite-specific PRAGMAs on every checkout.

### 1.2 Tauri v2

- **Still maturing ecosystem**: Tauri v2 plugin ecosystem is smaller than Electron's. Some plugins used (barcode-scanner, haptics, mobile-share) are niche and may have limited community support.
- **Plugin API churn**: Several plugins use version ranges (`~2.7.1`, `~2.3.2`, `~2.5.1`), suggesting API instability. The `@tauri-apps/api` v2 API surface differs significantly from v1.
- **Mobile support is nascent**: iOS/Android targets add complexity (sandbox paths, permission models, mobile-specific plugins).
- **Limited debugging**: Tauri's Rust + WebView hybrid debugging is less mature than Electron's Chrome DevTools for the backend.

### 1.3 Monorepo with Mixed Rust/TypeScript

- **Tooling complexity**: Requires both `cargo` (Rust) and `pnpm` (TypeScript) toolchains. Two build systems, two test frameworks, two linters, two formatters.
- **Cross-language boundaries**: Tauri IPC commands (Rust) communicate with frontend (TypeScript) via serialized JSON. This requires maintaining type definitions in both languages (adapters/types.ts with 470+ lines of shared types).
- **Build coordination**: Tauri builds require the Vite frontend to build first, then Rust compiles. Web builds omit the Rust layer entirely, requiring separate build configurations.
- **CI/CD overhead**: Compiling Rust for multiple targets (Windows, macOS, Linux, iOS, Android) is significantly slower than equivalent JavaScript/TypeScript builds.

### 1.4 SQLite — No Concurrent Write Scalability

- **Single-writer model**: The entire application serializes writes through a single `WriteHandle` actor. This is by design (SQLite allows only one writer at a time), but limits throughput for any future multi-user or server-side bulk operations.
- **WAL mode helps readers, not writers**: WAL journal mode allows concurrent reads during writes, but writes remain serialized. The device sync engine's oplog and outbox patterns compensate but add complexity.
- **No horizontal scaling**: SQLite is file-based, so it cannot be sharded, replicated, or distributed. The device sync mechanism provides multi-device access but with eventual consistency and E2EE overhead.

---

## 2. Migration Difficulty Rating per Module

| Module | Lines / Files | Language | Difficulty | Rationale |
|---|---|---|---|---|
| **Frontend (React)** | ~935 TS/TSX files | TypeScript | **Hard** | Large codebase with complex routing, state management (zustand + react-query), i18n with 16+ languages, addon runtime, animations (motion), charting (recharts), and a custom adapter pattern for desktop vs web. Rewriting in a different framework means rebuilding all component trees, hooks, and context providers. |
| **Core (Rust)** | ~468 .rs files | Rust | **Hard** | Domain logic tightly coupled to Rust's type system (enums, pattern matching, Result/Option). Traits define the storage abstraction, but many services (portfolio calculation, lots, tax handling, spending rules) use Rust-specific patterns. Porting to Go/Java means rewriting all domain logic. |
| **Storage (SQLite → PostgreSQL)** | 48 migrations, ~20 repository modules | Rust + SQL | **Medium** | The `core` crate defines repository traits that `storage-sqlite` implements, providing a clean abstraction boundary. However, SQLite-specific features (WAL PRAGMAs, `chunk_for_sqlite`, `VACUUM INTO` backups, `rusqlite` for raw connections, the write-actor pattern) must be replaced. PostgreSQL would need async drivers (sqlx or diesel-async), connection pooling (deadpool or bb8), and reworked migrations. |
| **Desktop (Tauri → Electron)** | Tauri commands + adapter layer | Rust + TS | **Medium** | The frontend is already web-compatible via the adapter pattern (`apps/frontend/src/adapters/tauri/` vs `apps/frontend/src/adapters/web/`). Replacing Tauri with Electron means rewriting the IPC layer (Tauri commands → Electron IPC/main process), but the frontend itself stays mostly unchanged. The Rust-backed Tauri commands (69+ exported functions) would need to be reimplemented as Node.js or Rust-via-napi. |
| **Market Data Providers** | ~10 source files | Rust | **Easy** | Pure HTTP API calls with protocol-agnostic provider abstraction. The `market-data` crate fetches from Yahoo Finance and other providers via `reqwest`. Porting to any language is straightforward — it's REST/JSON over HTTP. |
| **Device Sync** | ~15 source files | Rust | **Hard** | Core E2EE crypto (X25519, ChaCha20Poly1305, HKDF, SHA-256, HMAC) is implemented in Rust using `x25519-dalek`, `chacha20poly1305`, `hkdf`, `sha2`, `hmac` crates. The protocol includes pairing codes, key derivation, encrypted oplog, and snapshot-based bootstrap. Rewriting this in another language requires careful audit of cryptographic primitives and protocol compatibility. |
| **AI / MCP** | ~30 source files | Rust | **Medium** | The AI crate uses `rig-core` for LLM orchestration and `rmcp` for the MCP protocol. Both are Rust-specific. The `agent-tools` crate defines a runtime-neutral tool catalog. Porting MCP to another language is feasible (MCP is a protocol), but the existing integration with the storage layer and the streaming implementation would need to be rebuilt. |
| **Addon System** | ~20 source files | TypeScript | **Hard** | The addon runtime (`addons-core.ts`, `addon-runtime-loader.tsx`, `activation-coordinator.ts`, `contribution-registry.ts`, `type-bridge.ts`, `addons-runtime-context.tsx`) is deeply integrated with the React frontend. It uses iframe-based sandboxing, an SDK (`@wealthfolio/addon-sdk`), dev tools, and a contribution registry that hooks into the UI. Porting to a different framework means rebuilding the entire runtime and SDK. |
| **Connect (Broker Integration)** | ~5 source files | Rust | **Medium** | HTTP-based broker API integrations. Protocol-agnostic, but the Rust implementation uses `reqwest` and serde for deserialization. Straightforward to port, but requires rebuilding the sync orchestration logic. |
| **Spending Module** | ~10 source files | Rust | **Medium** | Cash-account spending, categorization, rules, budget calculations. Domain logic is Rust-specific (enums, pattern matching), but the module is self-contained with clearly defined trait boundaries. |

---

## 3. Migration Priority per Module

| Priority | Module | Rationale |
|---|---|---|
| **P0 — Immediate** | Storage (SQLite → PostgreSQL) | Most impactful for scalability and async support. Enables concurrent writes, richer query capabilities, and removes the single-writer bottleneck. |
| **P1 — High** | Desktop (Tauri → Electron) | Tauri v2 ecosystem risk. If Tauri plugins become unmaintained or the API shifts again, migration effort increases. Electron has a mature ecosystem and larger talent pool. |
| **P1 — High** | Core (Rust → Go) | If the project moves to a unified language (Go for backend logic) to reduce the Rust/TypeScript tooling burden. Go offers simpler concurrency, faster compile times, and easier hiring. |
| **P2 — Medium** | Frontend (React → Svelte) | Lower priority because React is stable and well-supported. Migration to Svelte/Vue/Angular only makes sense if the team decides to optimize bundle size or developer experience. |
| **P2 — Medium** | Addon System | Tied to the frontend framework. If the frontend is migrated, the addon system must be migrated simultaneously. |
| **P2 — Medium** | Device Sync | High risk but low urgency. The current implementation works. Only migrate if the underlying language/framework changes. |
| **P3 — Low** | AI / MCP | Protocol-based; can be kept as a Rust service or migrated independently. MCP servers can run as separate processes regardless of the main application language. |
| **P3 — Low** | Market Data Providers | Trivial to migrate at any point. HTTP-only, no state. |
| **P3 — Low** | Connect (Broker Integration) | Low priority. HTTP-based, can be migrated alongside Core. |
| **P3 — Low** | Spending Module | Low priority. Self-contained, can be migrated alongside Core. |

---

## 4. Functions to Discard vs. Functions Requiring Full Rewrite

### 4.1 When Switching Language (Rust → Go)

**Discard (replace with native equivalents):**
- `thiserror` derive macros → Go's `fmt.Errorf` / `errors.New` / `errors.Is` / `errors.As`
- `serde` derive macros → Go `encoding/json` struct tags
- `diesel` ORM query DSL → Go `sqlx` raw queries or `gorm`/`ent` ORM
- `async-trait` → Go interfaces (natively async)
- `rayon` parallel iterators → Go goroutines + channels
- `dashmap` → Go `sync.Map`
- `r2d2` connection pool → Go `sql.Open` with `database/sql` pool
- `uuid` crate → Go `github.com/google/uuid`
- `rust_decimal` → Go `github.com/shopspring/decimal`
- `chrono` → Go `time.Time`
- `reqwest` → Go `net/http` or `resty`
- `tokio` runtime → Go's native goroutine scheduler
- `once_cell` → Go `sync.OnceValue` / `sync.OnceFunc`
- `lazy_static` → Go `sync.Once` with init functions

**Full rewrite required:**
- `WriteHandle` actor → Go channel-based worker pool
- Backup/restore with `VACUUM INTO` → PostgreSQL `pg_dump` / `pg_restore`
- `chunk_for_sqlite()` → PostgreSQL batch insert with `COPY` or `INSERT ... ON CONFLICT`
- Device sync E2EE (X25519, ChaCha20Poly1305, HKDF, SHA-256) → Go `crypto/...` stdlib equivalents
- `rig-core` LLM orchestration → Go `langchaingo` or custom HTTP client to LLM APIs
- `rmcp` MCP server → Go `github.com/mark3labs/mcp-go` or custom MCP implementation
- `scraper` HTML parsing → Go `goquery` or `colly`
- `yahoo_finance_api` → Direct HTTP calls to Yahoo Finance API

### 4.2 When Switching Database (SQLite → PostgreSQL)

**Discard (SQLite-specific):**
- `PRAGMA journal_mode = WAL` → PostgreSQL's native MVCC (no manual config needed)
- `PRAGMA foreign_keys = ON` → PostgreSQL enforces FKs by default
- `PRAGMA busy_timeout` → PostgreSQL handles connection waits via `statement_timeout` / `lock_timeout`
- `PRAGMA synchronous = NORMAL` → PostgreSQL `synchronous_commit`
- `PRAGMA wal_checkpoint(TRUNCATE)` → Not needed in PostgreSQL
- `VACUUM INTO` for backups → `pg_dump` / `pg_dumpall`
- `chunk_for_sqlite()` / `SQLITE_MAX_PARAMS_CHUNK` → PostgreSQL has no parameter limit issue
- `RusqliteConnection` direct access → PostgreSQL `sqlx::PgConnection` or `tokio-postgres`
- `r2d2` pool → `deadpool-postgres` or `sqlx::PgPool`
- `AUTOINCREMENT` → `SERIAL` / `IDENTITY` columns
- `INSERT OR REPLACE` → `INSERT ... ON CONFLICT ... DO UPDATE` / `DO NOTHING`
- Write-actor serialized writes → Native async concurrent writes

**Keep (with adaptation):**
- Repository trait interfaces (same abstraction, different implementation)
- Migration SQL files (syntax changes: `TEXT` → `VARCHAR`/`TIMESTAMPTZ`, `INTEGER` → `BIGINT`/`BOOLEAN`, index syntax)
- Domain models (same business logic, different type mappings)

### 4.3 When Switching Desktop Framework (Tauri → Electron)

**Discard:**
- `tauri` crate and all Tauri plugins
- Tauri IPC command handlers (`apps/tauri/src/commands/`)
- `@tauri-apps/api` npm package and all `@tauri-apps/plugin-*` packages
- Tauri-specific adapters (`apps/frontend/src/adapters/tauri/`)
- Tauri event listeners (`listenFileDropHover`, `listenDeepLink`, etc.)
- Tauri file dialogs, shell commands, updater

**Full rewrite required:**
- Rust Tauri commands → Electron main process (Node.js or Rust napi-rs)
- Tauri IPC → Electron `ipcMain` / `ipcRenderer`
- File system operations → Node.js `fs` module
- OS dialogs → Electron `dialog` module
- Auto-updater → Electron `autoUpdater` (electron-updater package)
- Window state management → `electron-window-state` or custom implementation
- Deep link handling → Electron `app.setAsDefaultProtocolClient()`
- Barcode scanning → Electron native module or web API fallback
- Mobile targets → Separate React Native or Flutter project (Tauri's mobile support is unique)

**Keep (unchanged):**
- `apps/frontend/src/adapters/shared/` (19 shared modules — identical logic)
- `apps/frontend/src/adapters/web/` (web adapter — already platform-independent)
- All React components, pages, features, state management
- Vite build configuration (mostly)
- Addon runtime (iframe-based, framework-agnostic)
- `apps/frontend/src/adapters/types.ts` (shared types, may need IPC adjustments)

### 4.4 When Switching Frontend Framework (React → Svelte/Vue/Angular)

**Full rewrite required:**
- All `.tsx` components in `apps/frontend/src/components/` (50+ files)
- All page components in `apps/frontend/src/pages/` (17 page directories)
- All feature modules in `apps/frontend/src/features/` (5 feature directories)
- Routing: `react-router-dom` → `svelte-router` / `vue-router` / `@angular/router`
- State management: `zustand` + `@tanstack/react-query` → native stores + fetch lib
- Form handling: `react-hook-form` + `zod` → native forms + validation
- i18n: `react-i18next` → `svelte-i18n` / `vue-i18n` / `@angular/localize`
- Animation: `motion` → `svelte/motion` / `vueuse/motion` / Angular animations
- Charts: `recharts` → charting library for target framework
- UI primitives: `@radix-ui/react-slot`, `cmdk` → native equivalents
- AI assistant UI: `@assistant-ui/react` → framework-specific AI chat components

**Keep (mostly unchanged):**
- `apps/frontend/src/adapters/` (adapter types and shared modules — only import paths change)
- `apps/frontend/src/addons/` (react-specific runtime; would need framework port)
- `apps/frontend/src/lib/schemas.ts` (zod schemas — portable across frameworks)
- `apps/frontend/src/i18n/` (translation JSON files — format-agnostic)
- `apps/frontend/src/globals.css` (Tailwind v4 styles — framework-agnostic)
- `apps/frontend/vite.config.ts` (mostly portable, plugin changes needed)

---

## 5. Compatibility Plan for Hybrid Old-and-New Tech-Stack Runtime

### 5.1 Recommended Migration Strategy: Strangler Fig Pattern

Phase out the old stack incrementally by routing new functionality to the new stack while the old stack continues to serve existing features. The key is a clean **adaptation layer** between old and new systems.

### 5.2 Adaptation Layer Design

```
                    +--------------------------+
                    |   Frontend (React/TS)    |
                    +-----------+--------------+
                                |
                    +-----------+--------------+
                    |   Adapter Layer          |
                    | (desktop/web runtime     |
                    |  detection + routing)    |
                    +-----------+--------------+
                                |
            +-------------------+-------------------+
            |                   |                   |
    +-------v-------+   +------v--------+   +------v--------+
    | Tauri IPC     |   | Axum HTTP     |   | New Backend   |
    | (Rust, old)   |   | (Rust, old)   |   | (Go, new)     |
    +-------+-------+   +-------+-------+   +-------+-------+
            |                   |                   |
    +-------v-------------------+-------------------v-------+
    |              CQRS-style Bus/Queue                     |
    |  (old storage-sqlite <-> new PostgreSQL bridge)       |
    +-------------------------------------------------------+
            |                                   |
    +-------v-------+                   +-------v-------+
    | SQLite (old)  |                   | PostgreSQL    |
    |               |                   | (new)         |
    +---------------+                   +---------------+
```

### 5.3 Phase Plan

**Phase 1: Storage Dual-Write (Duration: 2-3 months)**

- Introduce a new `storage-postgres` crate alongside `storage-sqlite`.
- Both implement the same `core` repository traits.
- Deploy a **dual-write** mode: every write operation goes to both SQLite and PostgreSQL.
- Read from PostgreSQL first, fall back to SQLite.
- Monitor consistency and latency.
- This phase is zero-risk: SQLite remains the source of truth.

**Phase 2: Core Migration (Duration: 3-4 months)**

- Port `crates/core` domain logic from Rust to Go.
- The Go service exposes a gRPC or HTTP API that mirrors the repository trait interface.
- The Rust codebase calls the Go service for domain operations via the adapter layer.
- The Rust core crate is gradually replaced by the Go implementation, service by service.
- Device sync, being the most sensitive module, is migrated last.

**Phase 3: Desktop Framework Migration (Duration: 2-3 months)**

- Build an Electron shell alongside the existing Tauri app.
- The adapter layer (`apps/frontend/src/adapters/`) already supports both `tauri` and `web` targets. Add an `electron` target.
- Shared modules (19 files) remain unchanged. Port the 14 Tauri-specific adapter files to Electron equivalents.
- Run both Tauri and Electron builds in CI to ensure parity.
- Once Electron is stable, deprecate Tauri.

**Phase 4: Frontend Framework Migration (Duration: 4-6 months, optional)**

- Only if the team decides to switch from React (e.g., to Svelte for smaller bundles).
- Use micro-frontend techniques: embed the new framework's components inside the existing React app via Web Components or iframes.
- The addon runtime is the biggest blocker — it must be ported first.
- Given the complexity, this is the lowest priority migration.

### 5.4 Key Compatibility Constraints

| Constraint | Solution |
|---|---|
| **SQLite → PostgreSQL data sync** | Implement a CDC (Change Data Capture) layer. The existing outbox pattern in the write-actor can be repurposed to emit events consumed by a PostgreSQL sync worker. |
| **Crypto protocol compatibility** | Device sync E2EE must produce identical ciphertexts regardless of the implementation language. Use well-defined primitives (X25519, ChaCha20Poly1305 with 12-byte nonce, HKDF-SHA256) and test vectors across implementations. |
| **MCP server migration** | MCP is a protocol (JSON-RPC over HTTP/SSE). The Rust `rmcp` server can run alongside a new Go MCP server. Clients connect to either. The agent-tools catalog is runtime-neutral. |
| **Addon SDK compatibility** | The addon SDK is TypeScript and runs in the browser. It is framework-agnostic. The addon runtime (`addons-core.ts`, etc.) is React-specific and would need to be ported if the frontend framework changes. |
| **Backup/restore format** | SQLite backups are `.db` files. PostgreSQL backups are SQL dumps. The adaptation layer must handle both formats during the transition period. |
| **Build system** | During migration, the monorepo will have three build systems: `pnpm` (TS/JS), `cargo` (Rust), and `go build` (Go). CI pipelines must handle all three. The `Makefile` or `package.json` scripts coordinate the build. |

### 5.5 Risk Assessment

| Risk | Severity | Mitigation |
|---|---|---|
| Dual-write inconsistency | High | Use idempotent writes, conflict resolution with timestamps, and monitoring dashboards for drift detection. |
| Crypto non-portability | Critical | Write cross-language test vectors (known plaintext → known ciphertext) for every crypto operation. Test in CI. |
| Performance regression | Medium | Benchmark both stacks in parallel. The Go backend should match or exceed Rust for I/O-bound workloads. |
| Team cognitive load | High | Dedicate specific engineers per migration track. Avoid having the same person work on Rust and Go simultaneously. |
| Feature stagnation | High | Ship new features in the new stack first. Only maintain the old stack, don't extend it. |
| Addon ecosystem breakage | Medium | Version the addon API. Old addons target `v1` (React), new addons target `v2` (new framework). The runtime loads both. |

---

## 6. Re-usability, Migration Difficulty, and Code Risk Summary

| Module | Re-usability | Migration Difficulty | Code Risk |
|---|---|---|---|
| Frontend (React) | Medium — adapter pattern helps, but components are framework-specific | Hard | High — 935 files, deeply interconnected |
| Core (Rust) | Low — domain logic is Rust-idiomatic | Hard | High — 468 files, financial calculation correctness critical |
| Storage (SQLite) | Medium — trait abstraction exists, but SQLite-specific code runs deep | Medium | High — data integrity, 48 migrations |
| Desktop (Tauri) | Medium — adapter pattern isolates platform code | Medium | Medium — 14 adapter files, but Tauri plugins are many |
| Market Data | High — HTTP-only, protocol-agnostic | Easy | Low — stateless, no data persistence |
| Device Sync | Low — E2EE crypto in Rust, custom protocol | Hard | Critical — security, data loss risk |
| AI / MCP | Medium — protocol-based, tool catalog is runtime-neutral | Medium | Low — no user data stored |
| Addon System | Low — tightly coupled to React frontend runtime | Hard | Medium — addon compatibility is a product feature |
| Connect (Broker) | High — HTTP API calls | Medium | Low — only affects sync, not core data |
| Spending Module | Medium — self-contained domain logic | Medium | Medium — financial rule correctness critical |

### Re-usability Grade Definitions

- **High**: Can be lifted to another language/framework with minimal changes (HTTP clients, data models).
- **Medium**: Core logic is reusable but the surrounding infrastructure is language/framework-specific.
- **Low**: Implementation is tightly coupled to Rust's type system, Tauri's IPC, or React's component model.

### Migration Difficulty Definitions

- **Easy**: < 1 week, straightforward port, no specialist knowledge required.
- **Medium**: 1-4 weeks, requires domain knowledge but well-defined boundaries.
- **Hard**: 1-3 months, requires deep understanding of the module and cross-cutting concerns.

### Code Risk Definitions

- **Critical**: Data loss, financial calculation errors, or security vulnerabilities possible.
- **High**: Functional correctness depends on correct migration; testing is essential.
- **Medium**: Functional issues possible but recoverable without data loss.
- **Low**: Low impact; failures are cosmetic or affect non-critical features.