# Code Debt and Risk Report

**Repository:** Wealthfolio  
**Generated:** 2026-08-12  
**Scope:** Crate dependencies, hard-coded risks, dead code, exception handling, concurrency safety

---

## 1. High-Coupled Module List

### 1.1 `crates/core` -- Central Dependency Hub

`crates/core` is the single most-coupled module in the workspace. Every other crate depends on it:

| Crate | Depends on `core` |
|---|---|
| `crates/storage-sqlite` | Yes |
| `crates/market-data` | Yes |
| `crates/connect` | Yes |
| `crates/device-sync` | Yes |
| `crates/ai` | Yes |
| `crates/spending` | Yes |
| `crates/agent-tools` | Yes |
| `crates/wealthfolio-mcp` | Yes |
| `apps/tauri` | Yes |
| `apps/server` | Yes |

**Decoupling difficulty: HIGH.** Any change to core's error types (`Error`, `DatabaseError`), domain models, or service traits ripples across the entire workspace. Extracting a subset of core for a different storage backend would require untangling the dependency graph first. The `Error` type alone aggregates eight sub-error types (`Database`, `Asset`, `MarketData`, `Activity`, `Calculator`, `Fx`, `Validation`, `Secret`) -- all of which must remain stable to avoid breaking every consumer.

**Migration difficulty: HIGH.** If you wanted to replace the storage layer, `crates/core` defines `DatabaseError` that storage-sqlite converts into, but core itself has no storage abstraction trait -- it calls storage directly through concrete service types. A true migration would need trait-based service interfaces in core, which does not currently exist.

### 1.2 `crates/storage-sqlite` -- Tight Diesel ORM Coupling

This crate is deeply coupled to Diesel:

- `WriteHandle::exec()` takes `FnOnce(&mut SqliteConnection)` -- the Diesel connection type is baked into the public API.
- `DbWriteTx` wraps `&mut SqliteConnection` directly.
- `spawn_writer_inner()` calls `pool.get_timeout()` (r2d2 pooled connection) and `conn.immediate_transaction()` (Diesel method).
- `StorageError` explicitly maps `DieselError` variants to `DatabaseError` variants via `From<StorageError> for Error`.
- All repository implementations use Diesel query DSL, table macros, and schema inference.

**Decoupling difficulty: VERY HIGH.** The entire storage layer would need to be rewritten to swap out Diesel. The write-actor pattern (single-threaded writer via MPSC channel) is the one architectural element that would survive a migration to a different SQL library or database.

**Migration difficulty: VERY HIGH.** Diesel's query builder, type system, and migration runner are pervasive. Every repository function uses Diesel-specific constructs.

### 1.3 `apps/tauri` and `apps/server` -- Shared Business Logic via `crates/core`

Both applications share the same `crates/core` business logic, but each has its own surface layer:

- `apps/tauri` -- 130+ Tauri IPC commands registered in `lib.rs` (lines 417-862), all thin wrappers calling `crates/core` services. Desktop-specific: menu, updater, deep-link, MCP embedded server.
- `apps/server` -- Axum HTTP router aggregating 20+ sub-routers, all calling `crates/core` services. Web-specific: auth/JWT, OIDC SSO, CSP headers, CORS, rate limiting.

**Decoupling difficulty: LOW for the shared core, HIGH for the surface layers.** The frontend adapter pattern (`apps/frontend/src/adapters/`) cleanly separates Tauri IPC calls from HTTP calls behind shared interfaces (`adapters/shared/`). Adding a third runtime (e.g., Electron) would mean writing new adapter implementations but reusing all shared interfaces.

### 1.4 Frontend Adapter Pattern

`apps/frontend/src/adapters/`:
- `shared/` -- 19 shared interface files (interface definitions only)
- `tauri/` -- 14 adapter implementations (Tauri `invoke()` calls)
- `web/` -- 13 adapter implementations (HTTP fetch calls)

**Re-usability grade: A.** This is the strongest architectural pattern in the codebase. Adding a new runtime requires only new adapter implementations, no changes to shared interfaces.

---

## 2. Hard-Coded Secret Risk, Unsafe File Path, Missing Parameter Validation

### 2.1 Hardcoded URLs

The following production URLs are hardcoded in multiple locations:

| URL | Files | Risk |
|---|---|---|
| `https://wealthfolio.app` | `tauri.conf.json` (updater endpoints, CSP), `apps/server/src/api.rs` (CSP), frontend `.tsx` files (navigation, docs links) | **Low.** These are public-facing URLs for navigation, documentation, and the updater. They are not secrets. |
| `https://auth.wealthfolio.app` | `tauri.conf.json` (CSP), `apps/server/src/api.rs` (CSP), `apps/frontend/src/features/wealthfolio-connect/providers/wealthfolio-connect-provider.tsx:33` (default AUTH_URL fallback) | **Medium.** Production auth URL is the default fallback. If the env var `CONNECT_AUTH_URL` is not set, it falls back to production, which could cause issues in staging/dev environments. |
| `https://connect.wealthfolio.app` | `tauri.conf.json` (deep-link, CSP), `apps/server/src/api.rs` (CSP, connect.rs:138 fallback), frontend `.tsx` files | **Medium.** Multiple fallback patterns. The server's `connect.rs` at line 138 has `.or_else(|| Some("https://auth.wealthfolio.app".to_string()))` -- a hardcoded production fallback with no env override. |
| `https://connect-staging.wealthfolio.app` | `tauri.conf.json` (deep-link, CSP), `apps/server/src/api.rs` (CSP) | **Low.** Staging URL in CSP is acceptable. |
| `support@wealthfolio.app` | Frontend subscription-plans component, about-page | **Low.** Public contact email. |
| `https://assets.wealthfolio.app` | `apps/frontend/public/mock-update.json` | **Low.** Mock data only. |

### 2.2 Content Security Policy

`tauri.conf.json` CSP (line 82):
```
connect-src 'self' https://wealthfolio.app https://auth.wealthfolio.app
  https://connect.wealthfolio.app https://connect-staging.wealthfolio.app
```

`apps/server/src/api.rs` line 87 (SERVER_CSP):
```
connect-src 'self' https://wealthfolio.app https://auth.wealthfolio.app
  https://connect.wealthfolio.app https://connect-staging.wealthfolio.app
```

**Assessment:** The CSP allows four specific external domains for `connect-src`. This is acceptable for a finance app that needs to reach its own backend services. The `devCsp` is more permissive (adds `localhost:1420`, `ws://localhost:1420`, `http://localhost:3001`) which is expected for development.

### 2.3 Debug Endpoints Without Auth

From `apps/server/src/api.rs`:
- `GET /healthz` -- **No auth required.** Returns basic health check. Acceptable for liveness probes.
- `GET /readyz` -- **No auth required.** Returns readiness check. Acceptable for readiness probes.
- `GET /auth/status` -- **No auth required.** Returns whether password auth and OIDC are configured. Acceptable (this is needed before login).
- `GET /api/v1/openapi.json` -- **Behind auth middleware.** Protected by `require_jwt` when auth is enabled.

**Assessment:** No unprotected debug endpoints that leak sensitive data. The health/readiness endpoints are minimal and safe.

### 2.4 Hardcoded File Paths and Config Panics

In `apps/server/src/config.rs`:
- Line 43: `std::env::var("WF_DB_PATH").unwrap_or_else(|_| "./db/app.db".into())` -- relative path, resolved from process working directory. Could point to unexpected location.
- Line 56: `std::env::var("WF_STATIC_DIR").unwrap_or_else(|_| "dist".into())` -- same issue.
- Line 58: `panic!("WF_SECRET_KEY must be set...")` -- startup panic on missing config. Deliberate fail-closed behavior, but panics in library code are less recoverable than `Result` returns.
- Lines 125-130, 134-154, 160-170: `panic!()` for fail-closed validation (CORS+auth, non-loopback+no-auth, MCP+no-auth). These are deliberate security safeguards.

**Assessment:** The panics in `Config::from_env()` are intentional fail-closed security measures. The relative path defaults are a minor concern.

---

## 3. Dead Code, Unused Imports, Commented-Out Legacy Code

### 3.1 `#[allow(dead_code)]` in Production Code

The following locations suppress dead-code warnings in non-test code:

| File | Line(s) | Notes |
|---|---|---|
| `crates/ai/src/chat/mod.rs` | 113 | `#[allow(dead_code)]` on a struct field |
| `crates/ai/src/chat/history.rs` | 95 | `#[allow(dead_code)]` on a variant |
| `crates/ai/src/live_evals/runner.rs` | 584, 592 | Dead code in live eval runner |
| `crates/connect/src/client.rs` | 44, 115 | Dead code in connect client |
| `crates/core/src/addons/models.rs` | 227, 238, 249, 260 | Four consecutive `#[allow(dead_code)]` on enum variants |
| `crates/core/src/portfolio/performance/performance_service.rs` | 402 | Dead code in performance service |
| `crates/market-data/src/provider/alpha_vantage/mod.rs` | 197, 258, 261, 264, 326, 359 | Extensive dead code (6 locations) -- deprecated provider? |
| `crates/market-data/src/provider/finnhub/mod.rs` | 93 | Dead code in Finnhub provider |
| `crates/market-data/src/provider/metal_price_api/mod.rs` | 45, 48 | Dead code in metal price provider |

**Assessment:** The `alpha_vantage` provider has the most dead code (6 annotations), which may indicate it is partially deprecated or has stub fields for future use. The `addons/models.rs` dead enum variants suggest incomplete feature work. **Total: 19 production dead-code suppressions.**

### 3.2 Unused Imports

Not systematically checked for all files, but `apps/server/src/main.rs` has:
- Lines 22-24: `use tracing::{info, warn}` and `use wealthfolio_device_sync::SyncState` behind `#[cfg(feature = "device-sync")]` -- these are used conditionally.
- No obviously unused imports detected in the read files.

### 3.3 Commented-Out / Legacy Code

No large blocks of commented-out code were found in the examined files. The codebase appears actively maintained. The `crates/storage-sqlite/migrations/` directory contains 38 migration files dating back to 2023, which is normal for an active project. No orphaned migration files were detected.

---

## 4. Incomplete Exception Handling, Timeout-Missing Risk, Memory-Leak Hazard

### 4.1 `unwrap()` Calls in Production Code

**`crates/core/src` production code** (non-test `unwrap()` calls):
- `crates/core/src/exports.rs` line 493: `NaiveDate::from_ymd_opt(2026, 6, 25).unwrap()` -- hardcoded date, safe but fragile.
- Total in `crates/core/src`: approximately 40 `unwrap()` calls, but the vast majority are in `#[cfg(test)]` modules. Production unwraps are limited to hardcoded/known-safe values.

**`apps/server/src` production code:**
- `apps/server/src/api.rs` lines 128, 284, 287, 291, 293, 310, 312, 317, 319: Multiple `unwrap()` calls in routing and test setup.
- `apps/server/src/api.rs:128`: `o.parse().unwrap()` on CORS origin strings -- **panics on invalid input**.
- `apps/server/src/ai_environment.rs:109`: `self.base_currency.read().unwrap().clone()` -- **potential panic if RwLock is poisoned**.
- `apps/server/src/api/activities.rs:95`: `state.timezone.read().unwrap().clone()` -- **potential panic if RwLock is poisoned**.
- `apps/server/src/api/data_exports.rs:63, 94`: `state.base_currency.read().unwrap().clone()` -- same issue.
- `apps/server/src/oidc.rs:686-714`: `unwrap()` in tests only.

**`apps/tauri/src` production code:**
- `apps/tauri/src/commands/limits.rs:109`: `state.base_currency.read().unwrap()` -- **RwLock poison risk**.
- `apps/tauri/src/context/registry.rs:93-105`: Multiple `read().unwrap()` / `write().unwrap()` -- **RwLock poison risk**.
- `apps/tauri/src/domain_events/sink.rs:93`: `received.unwrap()` -- **potential panic on closed channel**.
- `apps/tauri/src/listeners.rs:382-410`: `unwrap()` in tests only.

**`crates/storage-sqlite/src/db/write_actor.rs`:**
- Line 69: `.expect("Writer actor's receiving channel was closed...")` -- panics if the actor stops unexpectedly. This is a **reliability risk**: if the write actor crashes, every subsequent write panics the caller.
- Line 73: `.expect("Writer actor dropped the reply sender...")` -- same pattern.
- Line 77: `.unwrap_or_else(|_| panic!("Failed to downcast writer actor result."))` -- panics on type mismatch.

### 4.2 `expect()` Calls in Production Code

| File | Line | Risk |
|---|---|---|
| `crates/agent-tools/src/tools/commit_activity.rs` | 418 | `expect("security draft should carry an asset")` |
| `crates/agent-tools/src/tools/categorization_context.rs` | 810 | `expect("valid timestamp literal")` |
| `crates/agent-tools/src/tools/allocation.rs` | 187, 231 | `expect("single-account branch checked")` |
| `crates/ai/src/providers.rs` | 28 | `expect("Failed to parse ai_providers.json")` -- startup panic |
| `crates/ai/src/chat/provider_clients.rs` | 237, 245, 253 | `expect("openai/groq/openrouter client")` |
| `crates/connect/src/token_lifecycle.rs` | 388, 398 | `expect("current time after epoch")` |
| `crates/storage-sqlite/src/db/write_actor.rs` | 69, 73, 77 | Actor channel panics (see above) |
| `apps/server/src/api.rs` | 203, 210, 215 | `expect("valid governor config")` -- hardcoded values, safe |

### 4.3 Timeout-Missing Risk

**Rate limiter `acquire()` loop** (`crates/market-data/src/registry/rate_limiter.rs` lines 168-192):
```rust
pub async fn acquire(&self, provider: &ProviderId) {
    loop {
        let wait_time = { ... bucket.time_until_available() ... };
        if wait_time > Duration::ZERO {
            tokio::time::sleep(wait_time).await;
        }
    }
}
```
**Risk: HIGH.** This is an unbounded spin-loop with no timeout or cancellation. If a provider's rate limit is set to a very low value (e.g., 1 request per hour), the caller blocks indefinitely. There is no `tokio::time::timeout` wrapper, no `select!` with a deadline, and no cancellation token. This could cause resource leaks in the async runtime (blocked tasks holding references).

**Circuit breaker** (`crates/market-data/src/registry/circuit_breaker.rs`) -- no timeout issues. The recovery timeout is enforced correctly.

**Write actor connection acquisition** (`crates/storage-sqlite/src/db/write_actor.rs` lines 238-268) -- has proper timeout handling: `MAX_TOTAL_WAIT = 8 seconds`, `PER_ATTEMPT_TIMEOUT = 800ms`, with retry logic. **Good pattern.**

### 4.4 Memory-Leak Hazard

**`Vec::with_capacity` in hot paths:**
- `crates/core/src/planning/save_up.rs:295-298`: `Vec::with_capacity((months + 1) as usize)` -- pre-allocated vectors in a financial planning computation. Acceptable for non-streaming computations.

**Global static collections with Mutex:**
- `crates/core/src/quotes/sync.rs:55`: `static SYNC_LOCKS: LazyLock<Mutex<HashSet<String>>>` -- grows unbounded with unique symbol strings. **Potential memory leak** if sync operations are called with ever-increasing unique symbols.
- `crates/device-sync/src/client.rs:30`: `static SNAPSHOT_UPLOAD_IN_FLIGHT: OnceLock<Mutex<HashSet<String>>>` -- bounded by active uploads, expected to be small.
- `crates/storage-sqlite/src/sync/broker_activity_patch.rs:388`: `&'static Mutex<HashMap<String, PendingBrokerActivityUserPatch>>` -- grows unbounded with broker activity patches. **Potential memory leak** if patches are created but never cleaned up.
- `crates/storage-sqlite/src/sync/app_sync/repository.rs:79`: `fn payload_column_catalog_cache() -> &'static Mutex<HashMap<String, PayloadColumnCatalog>>` -- bounded by schema type count, likely stable.

**Assessment:** `SYNC_LOCKS` and `PENDING_BROKER_ACTIVITY_PATCHES` both use unbounded collections behind `Mutex` with no eviction or cleanup mechanism. These are low-to-medium risk memory leaks.

---

## 5. Multi-Thread / Concurrent-Safety Risk

### 5.1 `unsafe` Code

The workspace `Cargo.toml` forbids unsafe code:
```toml
[workspace.lints.rust]
unsafe_code = "forbid"
```

No `unsafe` blocks were found in production code. The grep for `unsafe` returned only comments and the lint config. **Excellent safety practice.**

### 5.2 `RefCell` Usage

No `RefCell` usage was found in any crate. The codebase exclusively uses `Mutex` and `RwLock` for interior mutability, which is appropriate for multithreaded async Rust.

### 5.3 Global Mutable State Patterns

The codebase uses several patterns for shared mutable state:

| Pattern | Location | Risk |
|---|---|---|
| `LazyLock<Mutex<HashSet<String>>>` | `crates/core/src/quotes/sync.rs:55` | Medium -- unbounded growth, mutex contention |
| `OnceLock<Mutex<HashSet<String>>>` | `crates/device-sync/src/client.rs:30` | Low -- bounded by active uploads |
| `OnceLock<Mutex<()>>` | `crates/device-sync/src/enroll_service.rs:26` | Low -- unit lock, no contention |
| `lazy_static!` with `Mutex` | `crates/market-data/src/provider/yahoo/mod.rs:70`, `crates/market-data/src/resolver/exchange_registry.rs:110` | Low -- initialized once, read-heavy |
| `&'static Mutex<HashMap<...>>` | `crates/storage-sqlite/src/sync/broker_activity_patch.rs:388` | Medium -- unbounded growth |
| `Arc<RwLock<String>>` | `apps/server/src/main_lib.rs:80-81`, `apps/tauri/src/context/registry.rs` | Low -- protects base_currency and timezone, small critical sections |

### 5.4 Write Actor Pattern (storage-sqlite)

The write actor (`crates/storage-sqlite/src/db/write_actor.rs`) is a **well-designed concurrency pattern**:

- Single writer thread processes serialized jobs via MPSC channel.
- Channel capacity is bounded at 1024.
- Jobs are closures that receive `&mut SqliteConnection`.
- Results are returned via `oneshot` channels.
- `immediate_transaction()` wraps each job for proper SQLite transaction semantics.

**Risk: LOW.** The write actor correctly serializes database writes, preventing SQLite's single-writer limitation from causing concurrency issues. The only concern is that `expect()` panics on channel failure (see section 4.1), which would crash the entire application.

### 5.5 RwLock Poison Risks

Both `apps/server` and `apps/tauri` use `RwLock<String>` for `base_currency` and `timezone`:
- Server: `apps/server/src/main_lib.rs:80-81`
- Tauri: `apps/tauri/src/context/registry.rs:93-105`

The `read().unwrap()` and `write().unwrap()` calls will panic if the lock is poisoned (e.g., if a writer panics while holding the lock). In practice, these are simple string assignments that are unlikely to panic, but the `unwrap()` pattern is still a code smell.

---

## 6. Summary Grades

| Category | Grade | Key Issues |
|---|---|---|
| **Re-usability** | B | Adapter pattern is strong (A-grade). Core crate is over-coupled (C-grade). |
| **Migration Difficulty** | C | Diesel ORM coupling makes storage migration very hard. Core dependency graph is tightly coupled. |
| **Code Risk** | B | No unsafe code is excellent. Unbounded `Mutex<HashSet>` growth, unwrap/expect panics in production paths, and the unbounded rate-limiter spin-loop are the top risks. |

### Top 5 Risks to Address

1. **Rate-limiter infinite loop** (`crates/market-data/src/registry/rate_limiter.rs:168`): The `acquire()` method has no timeout or cancellation mechanism. Add `tokio::time::timeout` or a `select!` with a deadline.

2. **Write actor panic cascades** (`crates/storage-sqlite/src/db/write_actor.rs:69,73,77`): `expect()` panics in the write handle propagate to every caller. Consider converting to `Result` returns with `mpsc::error::SendError`.

3. **Unbounded global collections** (`crates/core/src/quotes/sync.rs:55`, `crates/storage-sqlite/src/sync/broker_activity_patch.rs:388`): `HashSet` and `HashMap` behind `Mutex` with no eviction policy. Add LRU capping or periodic cleanup.

4. **RwLock unwrap poison risk** (`apps/server/src/ai_environment.rs:109`, `apps/tauri/src/context/registry.rs:93-105`): `read().unwrap()` on `RwLock` will panic if poisoned. Use `.lock().unwrap_or_else(|e| e.into_inner())` pattern to recover.

5. **CORS origin parse panic** (`apps/server/src/api.rs:128`): `o.parse().unwrap()` on user-provided origin strings will panic on invalid input. Use `.ok()` or `.unwrap_or_default()`.