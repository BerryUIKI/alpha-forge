# Wealthfolio — Project Overview

## Project Name

**Wealthfolio**

## Business Positioning

Open-source, private portfolio tracker — investments, net worth, spending, and
simulations. Local-first: all data lives on the user's device. No cloud database,
no account required, free forever. AGPL-3.0 licensed (trademarks of Teymz Inc.).

Optional paid add-on: **Wealthfolio Connect** provides automatic brokerage sync
(30+ institutions, read-only) and encrypted multi-device sync. The app never
requires it — manual tracking and CSV import are free.

---

## Complete Technology Stack

| Layer | Technology |
| ----- | ---------- |
| **Frontend** | React 19, TypeScript, Vite 7, Tailwind CSS v4, Radix UI / shadcn, Recharts, React Query (TanStack), React Router v7, Zustand, React Hook Form, Zod, i18next, motion, lucide-react, @phosphor-icons/react, sonner, cmdk, date-fns |
| **Frontend extras** | @assistant-ui/react (AI chat UI), @tanstack/react-table, @tanstack/react-virtual, @number-flow/react, @fontsource fonts, react-day-picker, qrcode.react, @supabase/supabase-js |
| **Desktop** | Tauri v2, Rust 1.95.0 (toolchain pinned), Tauri plugins (fs, dialog, shell, log, deep-link, updater, window-state, single-instance, haptics, barcode-scanner, web-auth, mobile-share) |
| **Backend (Web)** | Axum HTTP server, Tokio async runtime, tower / tower-http middleware, utoipa + Swagger UI, tower_governor rate limiting |
| **Database** | SQLite via Diesel ORM, Diesel migrations, r2d2 connection pooling, rusqlite |
| **Sync** | Device sync engine with E2EE (X25519 + ChaCha20-Poly1305, HKDF, SHA-2, HMAC) |
| **Market Data** | Yahoo Finance, Alpha Vantage, Finnhub, Boerse Frankfurt, MarketData.app, Metal Price API, OpenFIGI, US Treasury |
| **Addon System** | @wealthfolio/addon-sdk, @wealthfolio/addon-dev-tools, @wealthfolio/ui (workspace packages, published to npm) |
| **Dev Tools** | pnpm 10, ESLint 9, Prettier, Playwright, Vitest, Turborepo, i18next-cli |
| **AI** | MCP (Model Context Protocol via `rmcp`), rig agent framework (`rig-core`), AI chat with multiple providers, agent-tool catalog, embedded MCP server |
| **OS / Deploy** | macOS 12+, Windows (WebView2), Linux (WebKitGTK), iOS 16+, Android, Docker / compose |

---

## Directory Structure

```
wealthfolio/
├── apps/                          # Application packages
│   ├── frontend/                  # React frontend (Vite)
│   │   ├── src/
│   │   │   ├── adapters/          # Compile-time env detection (Tauri vs Web)
│   │   │   ├── addons/            # Addon runtime loader, dev mode, sandbox
│   │   │   ├── components/        # Shared React components
│   │   │   ├── features/          # Self-contained feature modules
│   │   │   │   ├── ai-assistant/  # In-app AI chat UI
│   │   │   │   ├── devices-sync/  # Device sync UI
│   │   │   │   ├── goals/         # Goal planning UI
│   │   │   │   ├── spending/      # Spending / cash-account UI
│   │   │   │   └── wealthfolio-connect/  # Connect / brokerage sync UI
│   │   │   ├── pages/             # Route pages
│   │   │   ├── hooks/             # Custom React hooks
│   │   │   ├── lib/               # Utilities, settings provider, schemas
│   │   │   ├── context/           # Auth, privacy providers
│   │   │   ├── i18n/              # i18next config + translations
│   │   │   ├── App.tsx            # Root component (providers + routes)
│   │   │   ├── main.tsx           # React entry (mount point)
│   │   │   ├── routes.tsx         # Route definitions
│   │   │   └── lockdown.ts        # Desktop-only input lockdown
│   │   ├── public/                # Static assets (logo, splash, fonts)
│   │   ├── index.html             # HTML entry point
│   │   └── vite.config.ts         # Vite build config
│   ├── tauri/                     # Tauri desktop/mobile app
│   │   └── src/                   # Rust: main.rs, lib.rs, commands/, IPC
│   └── server/                    # Axum HTTP server for web mode
│       └── src/                   # main.rs, api/, auth/, oidc/, mcp/, config.rs
├── crates/                        # Rust workspace crates (shared backend)
│   ├── core/                      # Business logic, models, services, quotes scheduler
│   ├── storage-sqlite/            # Diesel ORM, repositories, migrations, schema
│   ├── market-data/               # Market data providers (yahoo, alpha_vantage, finnhub,
│   │                              #   boerse_frankfurt, marketdata_app, metal_price_api,
│   │                              #   openfigi, us_treasury_calc)
│   ├── connect/                   # Wealthfolio Connect / brokerage integrations
│   ├── device-sync/               # Device sync + E2EE crypto
│   ├── spending/                  # Cash-account spending, categorization, budget, analytics
│   ├── ai/                        # AI assistant, LLM orchestration (rig-core)
│   ├── agent-tools/               # Runtime-neutral agent tool catalog (shared AI + MCP)
│   └── wealthfolio-mcp/           # MCP protocol layer exposing agent-tool catalog
├── packages/                      # Shared TypeScript packages
│   ├── addon-sdk/                 # Addon SDK (published: @wealthfolio/addon-sdk)
│   ├── addon-dev-tools/           # CLI + dev server for addons (@wealthfolio/addon-dev-tools)
│   └── ui/                        # Shared UI components (@wealthfolio/ui)
├── docs/                          # Documentation
│   ├── addons/                    # Addon dev docs, API reference, migration guides
│   ├── activities/                # Activity types docs
│   ├── architecture/              # Adapter system, AI assistant, market-data docs
│   ├── features/                  # Feature docs
│   └── self-host/                 # Self-hosting guides
├── e2e/                           # Playwright end-to-end tests
│   ├── 01..15-*.spec.ts           # Numbered E2E test suites
│   ├── addon-sandbox/             # Addon sandbox E2E tests
│   └── fixtures/                  # Test fixtures/data
├── scripts/                       # Dev/build/e2e scripts (dev-web.mjs, run-e2e.mjs, ci-check.sh)
├── assets/                        # Brand assets (trademarks)
├── packaging/                     # OS packaging resources
├── dev/                           # Dev resources
├── Cargo.toml                     # Rust workspace config (members: apps/tauri, apps/server, crates/*)
├── package.json                   # Node workspace + scripts
├── pnpm-workspace.yaml            # pnpm workspace (apps/frontend, packages/*)
├── Dockerfile                     # Multi-stage Alpine image build
├── compose.yml                    # Production Docker Compose
├── rust-toolchain.toml            # Rust 1.95.0 pin
└── tsconfig.json / tsconfig.base.json  # TypeScript build configs
```

---

## Application Entry Points

| Target | Entry point | Notes |
| ------ | ----------- | ----- |
| Desktop | `apps/tauri/src/main.rs` | Calls `wealthfolio_app_lib::run()` (lib in `apps/tauri/src/lib.rs`); beforeDevCommand runs Vite at `http://localhost:1420` |
| Web (backend) | `apps/server/src/main.rs` | `#[tokio::main]` — builds state, starts sync schedulers, binds Axum server on configurable `WF_LISTEN_ADDR` (default `0.0.0.0:8088`), serves API + static frontend |
| Frontend | `apps/frontend/index.html` → `apps/frontend/src/main.tsx` → `apps/frontend/src/App.tsx` | React root mounts providers/routes; desktop loads lockdown, addon runtime |

---

## Environment Variables

### Server (`WF_*`) — web / Docker

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `WF_LISTEN_ADDR` | `0.0.0.0:8088` | Server bind address (`0.0.0.0` required in Docker) |
| `WF_DB_PATH` | `./db/app.db` | SQLite path or directory (directory → `app.db` inside) |
| `WF_CORS_ALLOW_ORIGINS` | `*` | Comma-separated allowed origins; wildcard rejected when auth enabled |
| `WF_REQUEST_TIMEOUT_MS` | `30000` | Request timeout (ms) |
| `WF_STATIC_DIR` | `dist` | Static frontend assets dir |
| `WF_SECRET_KEY` | **required** | 32-byte key for secrets encryption + JWT signing (generate `openssl rand -base64 32`) |
| `WF_SECRET_FILE` | `<data-root>/secrets.json` | Encrypted secrets storage file |
| `WF_AUTH_PASSWORD_HASH` | — | Argon2id PHC string enabling password login |
| `WF_AUTH_TOKEN_TTL_MINUTES` | `60` | JWT access token expiry (minutes) |
| `WF_AUTH_REQUIRED` | `true` | Set `false` to allow non-loopback start without auth (reverse proxy) |
| `WF_COOKIE_SECURE` | `auto` | `auto` / `true` / `false` session-cookie `Secure` policy |
| `WF_ADDONS_DIR` | derived from DB path | Addons install/load directory |
| `WF_MCP_ENABLED` | `false` | Expose `/mcp` endpoint for external AI agents |
| `WF_MCP_AUDIT_ENABLED` | `true` | Write agent tool calls to audit log |
| `WF_MCP_ALLOWED_HOSTS` | — | Comma-separated allowed `Host` headers for `/mcp` |
| `WF_PORT` | `8088` | Host port mapping in compose (not a server var) |

### OIDC / SSO (`WF_OIDC_*`) — optional

| Variable | Description |
| -------- | ----------- |
| `WF_OIDC_ISSUER_URL` | IdP base URL (enables OIDC when set with `CLIENT_ID`) |
| `WF_OIDC_CLIENT_ID` | Client id registered with IdP |
| `WF_OIDC_CLIENT_SECRET` | Optional (PKCE always used) |
| `WF_OIDC_REDIRECT_URL` | Required OIDC callback URL |
| `WF_OIDC_SCOPES` | Space-separated scopes (default `openid email profile`) |
| `WF_OIDC_ALLOWED_EMAILS` | Comma-separated email allowlist |
| `WF_OIDC_ALLOWED_SUBS` | Comma-separated `sub` allowlist (recommended) |
| `WF_OIDC_ALLOW_ANY` | `false`; allow any IdP user when no allowlist |
| `WF_OIDC_POST_LOGOUT_REDIRECT_URL` | Post RP-Initiated Logout landing URL |
| `WF_OIDC_RP_LOGOUT` | `true`; force local-only logout via `false` |

### Vite / frontend (`VITE_*`)

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `VITE_API_TARGET` | `http://127.0.0.1:8088` | Backend API URL for Vite proxy |
| `VITE_ENABLE_ADDON_DEV_MODE` | — | Enable addon hot-reload dev mode |

### Wealthfolio Connect (`CONNECT_*`)

| Variable | Description |
| -------- | ----------- |
| `CONNECT_AUTH_URL` | Auth provider URL (baked at build time in Docker) |
| `CONNECT_AUTH_PUBLISHABLE_KEY` | Connect publishable key (build-time) |
| `CONNECT_API_URL` | Connect API base URL |
| `CONNECT_OAUTH_CALLBACK_URL` | OAuth callback for cloud sync |

### Tauri / desktop (`TAURI_*`, misc)

| Variable | Description |
| -------- | ----------- |
| `TAURI_DEBUG` | Enable Tauri debug output (with `pnpm tauri dev`) |
| `DATABASE_URL` | Desktop DB path (`.env.example`: `../db/app.db`) |
| `BUILD_TARGET` | `web` / `tauri` — selects frontend build target (`cross-env`) |

---

## Commands

### Dependencies

```bash
pnpm install        # Node workspace (pnpm 10)
cargo build         # Rust workspace (crates + apps)
```

### Run

```bash
pnpm tauri dev        # Desktop (Tauri + Vite on :1420)
pnpm run dev:web      # Web (Axum + Vite dev server)
cargo run --manifest-path apps/server/Cargo.toml   # Server only
```

### Build

```bash
pnpm tauri build          # Desktop distributable
docker build -t wealthfolio .   # Web Docker image
```

### Debug / Test

```bash
TAURI_DEBUG=1 pnpm tauri dev   # Desktop with debug
pnpm test                      # Vitest unit tests
cargo test                     # Rust tests
pnpm test:e2e                  # Playwright E2E
```

---

## Assessment

### Re-usability Grade

**B — Good.** The architecture is cleanly separated into a reusable Rust core
(`crates/core` business logic, `crates/market-data` providers, `crates/ai` +
`agent-tools` + `wealthfolio-mcp`) decoupled from the two front-ends (Tauri
desktop and Axum web). `@wealthfolio/ui`, `@wealthfolio/addon-sdk`, and
`@wealthfolio/addon-dev-tools` are published, independent npm packages. The
market-data provider registry and agent-tool catalog are genuinely
re-embeddable. Deduction: heavy AGPL coupling, a rich domain model deeply tied
to the finance domain, and tight integration between frontend command wrappers
and the Rust backend reduce drop-in reuse outside this project.

### Migration Difficulty

**High.** Migrating to a different stack or platform is substantial effort:
- Frontend ↔ backend contract is a dense custom command/API surface (scores of
  Tauri IPC commands and Axum endpoints), not a thin REST CRUD layer.
- Diesel + SQLite schema (`crates/storage-sqlite`) is expansive and domain-rich;
  moving to another DB/ORM means porting the whole schema and migrations.
- Addon SDK, sandbox runtime, and MCP layer are purpose-built and tightly coupled
  to the app's data model.
- E2EE device-sync and Connect brokerage integrations add bespoke protocol code.
- Multiple frontend build targets (`BUILD_TARGET=web|tauri`) complicate moving to
  a new bundler.

### Code Risk

**Medium.** Overall well-structured monorepo with strong conventions (workspace
lints, `unsafe_code = "forbid"`, target-gated deps, security-hardened server with
fail-closed auth and CORS checks). Residual risks:
- Rapid feature growth: the `core` and `storage-sqlite` crates are large and
  domain-heavy, raising maintenance surface.
- Secrecy/security-sensitive code (JWT, Argon2id, chacha20poly1305, keyring,
  OIDC allowlists) — misconfiguration is a real exposure vector (server refuses
  to start to enforce safety, which is good but operationally tricky).
- Wide dependency surface (many providers, Tauri plugins, AI/MCP crates) increases
  supply-chain and upgrade churn.
- AGPL-3.0 licensing constrains commercial re-use.