# Wealthfolio Agent Development Manual

> Generated from repository scan at `F:\dev\wealthfolio`
> Revision: based on workspace v3.7.0

---

## 1. Directory Rules for Newly Added Functions

### 1.1 Frontend (TypeScript/React)

| Purpose | Directory | Convention |
|---|---|---|
| New route pages | `apps/frontend/src/pages/<kebab-case>/` | Each page gets its own directory; register in `routes.tsx` |
| New feature modules | `apps/frontend/src/features/<kebab-case>/` | Self-contained feature logic, hooks, sub-components |
| Shared components | `apps/frontend/src/components/` | Reusable UI primitives; flat or grouped by domain |
| Backend command wrappers | `apps/frontend/src/commands/` | Typed wrapper functions following the `RUN_ENV` adapter pattern |

**Frontend layer flow:**

```
Frontend component
  -> imports command from apps/frontend/src/commands/<domain>.ts
    -> command calls adapter (apps/frontend/src/adapters/)
      -> adapter resolves to Tauri IPC or Axum HTTP at build time
```

### 1.2 Backend (Rust)

| Layer | Directory | Responsibility |
|---|---|---|
| Business logic | `crates/core/src/` | Domain entities, services, repository traits (database-agnostic) |
| Database implementation | `crates/storage-sqlite/src/` | Diesel ORM models, repository impls, connection management |
| Tauri IPC commands | `apps/tauri/src/commands/` | Thin command handlers; delegate to `crates/core` |
| Axum HTTP handlers | `apps/server/src/api/` | REST endpoints; delegate to `crates/core` |

**Data flow for new backend features:**

```
Frontend (typed command)
  -> Tauri IPC / Axum HTTP
    -> apps/tauri/src/commands/*.rs  or  apps/server/src/api/*.rs
      -> crates/core/src/ (service/repo trait)
        -> crates/storage-sqlite/src/ (Diesel impl)
          -> SQLite database
```

### 1.3 Addon System

| Package | Directory | Purpose |
|---|---|---|
| Addon SDK | `packages/addon-sdk/src/` | Public types (`AddonContext`, `HostAPI`, manifest types, permission types) |
| Dev tools | `packages/addon-dev-tools/` | CLI, dev server, scaffold templates |

**Addon development flow:**

```
packages/addon-sdk/src/  -- types consumed by addon developers
packages/addon-dev-tools/ -- local dev server (port 3001), hot reload, scaffolding
```

---

## 2. Protected Core Modules (Never Modify)

These modules are protected by policy. Any modification requires explicit approval and justification.

| Module | Path | Reason |
|---|---|---|
| Core business logic | `crates/core/src/` | Heart of the application; all domain entities, services, and traits |
| Database migrations | `crates/storage-sqlite/migrations/` | **Append-only.** Existing migration files must never be modified -- only new timestamped migration directories may be added |
| Diesel schema | `crates/storage-sqlite/src/schema.rs` | **Auto-generated** by `diesel print-schema`. Manual edits are overwritten on regeneration |
| E2EE cryptography | `crates/device-sync/src/crypto.rs` | Security-critical: E2EE key exchange, encryption/decryption |
| Broker sync | `crates/connect/src/broker/` | Critical path for external financial data synchronization |
| Addon SDK public API | `packages/addon-sdk/src/` | Public contract; breaking changes affect all published addons |

**Additional protected items:**

- `crates/device-sync/src/crypto.rs` -- 12,277 bytes of security-critical E2EE code
- `crates/connect/src/broker/orchestrator.rs` -- sync orchestration logic
- `crates/connect/src/broker/service.rs` -- 90,756 bytes of broker service logic

---

## 3. Debug Workflow and Log Inspection Guide

### 3.1 Frontend (React + Vite)

| Tool / Method | How to Use |
|---|---|
| React DevTools | Browser extension for component tree, props, state inspection |
| Vite HMR | Hot module replacement active in dev mode (port 1420) |
| Browser console | `console.log` / `console.error` output; network tab for API calls |
| Build target | `BUILD_TARGET=tauri` (default) or `BUILD_TARGET=web` -- Vite resolves adapter aliases at compile time |
| Debug builds | `TAURI_DEBUG=true` disables minification, enables sourcemaps |

### 3.2 Desktop (Tauri + Rust)

| Tool / Method | How to Use |
|---|---|
| Tauri console | `pnpm tauri dev` -- shows Rust stdout/stderr and webview console |
| Rust debug logs | Uses the `tracing` crate -- structured logging throughout backend |
| `TAURI_DEBUG=true` | Enables verbose Tauri logging, dev sourcemaps, disables JS minification |
| `cargo check` | Compile-time validation only (no runtime) |
| `cargo test` | Rust unit/integration tests |

### 3.3 Web Mode (Axum Server)

| Tool / Method | How to Use |
|---|---|
| Axum server logs | HTTP request/response logging via `tracing` |
| `tracing` output | Structured log events from all `crates/` modules |
| Server port | Axum server listens on `http://127.0.0.1:8088` (default) |

### 3.4 Database (SQLite)

| Tool / Method | How to Use |
|---|---|
| SQLite file inspection | Direct `.db` file inspection via `sqlite3` CLI or DB browser |
| Diesel logging | `diesel` crate emits SQL query logs when `log` feature is enabled |
| Migration status | Run `diesel migration run` or check `__diesel_schema_migrations` table |
| Connection pool | Managed via `r2d2` pool in `crates/storage-sqlite/src/db.rs` |

### 3.5 AI System

| Component | Path | Purpose |
|---|---|---|
| Eval harness | `crates/ai/src/eval/` | Assertion helpers (`harness.rs`, `scenarios.rs`) for stream-event ordering and guardrail compliance. No runner wired yet -- code-flow regressions covered by integration tests |
| Live evals | `crates/ai/src/live_evals/` | Real LLM eval framework (`schema.rs`, `trace.rs`, `runner.rs`). Catches real model drift. Gated behind `test-utils` and `eval` features |
| Eval binary | `crates/ai/src/bin/eval.rs` | Runner binary for live evals |
| Eval config | `crates/ai/evals/README.md` | Run instructions and TOML schema |

**Eval commands:**

```bash
# Helper tests (no LLM)
cargo test -p wealthfolio-ai eval:: -- --nocapture

# Live evals (requires LLM provider)
cargo test -p wealthfolio-ai --features live-evals
```

### 3.6 MCP (Model Context Protocol)

| Tool / Method | How to Use |
|---|---|
| MCP crate | `crates/wealthfolio-mcp/` -- server implementation using `rmcp` |
| Protocol debugging | Inspect MCP tool calls and responses via `crates/wealthfolio-mcp/src/handler.rs` |
| Transport | Streamable HTTP server transport |
| Audit | MCP requests logged via `crates/wealthfolio-mcp/src/audit.rs` |

---

## 4. Blacklist of Dangerous Modification Operations

The following operations are strictly prohibited without explicit authorization:

| Operation | Details | Risk |
|---|---|---|
| **Modify existing migrations** | `crates/storage-sqlite/migrations/` -- existing migration files are immutable; only new timestamped directories may be added | Data loss, irreversible schema drift |
| **Edit `schema.rs` manually** | `crates/storage-sqlite/src/schema.rs` -- auto-generated by Diesel; manual edits are overwritten | Silent schema mismatch |
| **Commit secrets** | `.env`, `.env.web`, `.env.docker` -- must remain in `.gitignore` | Credential leak |
| **Change CSP without review** | `apps/tauri/tauri.conf.json` `app.security.csp` -- security boundary | XSS, data exfiltration |
| **Remove `unsafe_code = "forbid"`** | `Cargo.toml` workspace lints -- forbids `unsafe` Rust code | Undefined behavior, memory safety |
| **Skip the adapter pattern** | Frontend code must go through `apps/frontend/src/adapters/` -- never call Tauri or fetch directly | Broken web/desktop support |
| **Hardcode API keys/secrets** | Secrets must use OS keyring, never disk or localStorage | Credential leak |
| **Modify addon SDK public API** | `packages/addon-sdk/src/` -- breaking changes require version bump | Breaks all published addons |

---

## 5. Re-usability, Migration Difficulty, and Code Risk Assessment

### 5.1 Re-usability Grade

| Module | Grade | Notes |
|---|---|---|
| `crates/core/` | A | Fully database-agnostic via traits; can be reused with any backend |
| `crates/storage-sqlite/` | B | Diesel-specific; migration to another ORM requires rewriting all repositories |
| `crates/market-data/` | B | Provider pattern is reusable; provider implementations are app-specific |
| `crates/connect/` | B | Broker sync logic is specific to financial data; orchestration pattern is reusable |
| `crates/device-sync/` | C | Tightly coupled to Wealthfolio's data model and E2EE scheme |
| `crates/ai/` | B | Provider abstraction is reusable; tool definitions are app-specific |
| `crates/wealthfolio-mcp/` | B | MCP server pattern is standard; tool set is app-specific |
| `packages/addon-sdk/` | A | Clean public API; could be published as standalone SDK |
| `apps/frontend/src/adapters/` | A | Pattern is reusable; implementations are app-specific |
| `apps/frontend/src/components/` | B | Some are generic (selectors, charts); many are domain-specific |

### 5.2 Migration Difficulty

| Scenario | Difficulty | Key Considerations |
|---|---|---|
| SQLite to PostgreSQL | High | Diesel schema differences, SQLite-specific SQL, migration files are SQLite-only |
| Tauri to Electron | Medium | Adapter pattern isolates Tauri calls; need new adapter impl |
| Axum to Actix-web | Low | Thin HTTP handlers delegate to `crates/core`; swap framework only |
| Addon SDK version bump | Medium | Public API changes require coordinated addon updates |
| E2EE algorithm change | Very High | `crates/device-sync/src/crypto.rs` -- affects all enrolled devices, key exchange protocol |

### 5.3 Code Risk

| Module | Risk Level | Rationale |
|---|---|---|
| `crates/connect/src/broker/service.rs` (90 KB) | **High** | Largest single file in the project; orchestrates financial data sync; critical path |
| `crates/device-sync/src/crypto.rs` | **High** | Security-critical E2EE; any bug breaks confidentiality guarantees |
| `crates/core/src/exports.rs` (19 KB) | **Medium** | Central export/import logic; data integrity risk |
| `apps/server/src/api/connect.rs` (58 KB) | **High** | Large HTTP handler surface for broker connections |
| `apps/tauri/src/commands/portfolio.rs` (70 KB) | **High** | Largest Tauri command file; complex portfolio logic |
| `apps/frontend/src/components/` | **Low** | UI components; errors are visual, not data-corrupting |
| `crates/storage-sqlite/migrations/` | **Medium** | Append-only; errors only from new migrations, not existing ones |
| `packages/addon-sdk/src/` | **Low** | Type definitions only; no runtime logic |

---

## 6. Build and Validation Commands

```bash
pnpm tauri dev          # Desktop development
pnpm run dev:web        # Web development
pnpm test               # TypeScript tests (Vitest)
cargo test              # Rust tests
pnpm type-check         # TypeScript type checking
pnpm lint               # ESLint + Prettier
pnpm check              # All checks (type-check + lint + test)
```

---

## 7. Key File Paths Reference

| File | Role |
|---|---|
| `F:\dev\wealthfolio\AGENTS.md` | Agent behavioral guidelines and architecture overview |
| `F:\dev\wealthfolio\CONTRIBUTING.md` | Community contribution guide |
| `F:\dev\wealthfolio\Cargo.toml` | Workspace configuration, lints, dependencies |
| `F:\dev\wealthfolio\apps\frontend\vite.config.ts` | Vite config, adapter aliases, build targets |
| `F:\dev\wealthfolio\apps\frontend\src\adapters\index.ts` | Adapter re-export entry point |
| `F:\dev\wealthfolio\apps\tauri\tauri.conf.json` | Tauri app configuration, CSP, window settings |
| `F:\dev\wealthfolio\docs\addons\addon-architecture.md` | Addon system architecture and development guide |
| `F:\dev\wealthfolio\docs\architecture\adapters.md` | Adapter pattern documentation |
| `F:\dev\wealthfolio\crates\storage-sqlite\src\lib.rs` | SQLite storage crate entry, repository exports |
| `F:\dev\wealthfolio\crates\core\src\lib.rs` | Core business logic crate entry, module exports |
| `F:\dev\wealthfolio\crates\ai\src\eval\mod.rs` | Eval harness (assertion helpers, golden scenarios) |
| `F:\dev\wealthfolio\crates\ai\src\live_evals\mod.rs` | Live-model eval framework (real LLM, tool-call tracing) |
| `F:\dev\wealthfolio\crates\wealthfolio-mcp\src\handler.rs` | MCP protocol handler |
| `F:\dev\wealthfolio\crates\device-sync\src\crypto.rs` | E2EE cryptography implementation |
| `F:\dev\wealthfolio\crates\storage-sqlite\src\schema.rs` | Diesel auto-generated schema (do not edit manually) |