# External Dependency Inventory

> Generated: 2026-08-12
> Repository: Wealthfolio v3.7.0

---

## 1. Dependency Packages

### 1.1 Rust Workspace Dependencies (Cargo.toml)

Resolved at workspace level (`F:\dev\wealthfolio\Cargo.toml`) with version constraints. Lock status: **semver-compatible range** (no exact pin except `yahoo_finance_api`).

| Dependency | Version | Lock | Function |
|---|---|---|---|
| tokio | 1 | Range | Async runtime (multi-thread, macros, sync) |
| serde | 1 | Range | Serialization framework with derive |
| serde_json | 1 | Range | JSON serialization/deserialization |
| diesel | 2.2 | Range | ORM with SQLite, chrono, r2d2, numeric, returning clauses |
| diesel_migrations | 2.2 | Range | Embedded SQLite migrations |
| rusqlite | 0.34 | Range | Raw SQLite bindings (bundled) |
| r2d2 | 0.8 | Range | Connection pool |
| chrono | 0.4 | Range | Date/time library with serde feature |
| reqwest | 0.12 | Range | HTTP client (JSON, rustls-tls) |
| thiserror | 1 | Range | Derive macro for error types |
| anyhow | 1 | Range | Flexible error handling |
| uuid | 1 | Range | UUID generation (v4, v7) with serde |
| rust_decimal | 1.39 | Range | Decimal arithmetic with serde-float |
| rust_decimal_macros | 1.39 | Range | Decimal literal macros |
| num-traits | 0.2 | Range | Numeric trait abstractions |
| async-trait | 0.1 | Range | Async trait methods |
| futures | 0.3 | Range | Async primitives |
| rmcp | 1.7 | Range | Model Context Protocol (server, streamable HTTP) |
| log | 0.4 | Range | Logging facade |
| chacha20poly1305 | 0.10 | Range | AEAD encryption for secrets |
| x25519-dalek | 2 | Range | ECDH key exchange (device sync) |
| hkdf | 0.12 | Range | HKDF key derivation |
| sha2 | 0.10 | Range | SHA-256 hashing |
| rand | 0.8 | Range | Random number generation |

### 1.2 Crate-Level Dependencies

#### wealthfolio-core (crates/core/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| yahoo_finance_api | 4.1 | Range | Yahoo Finance API client |
| regex | 1.10 | Range | Regular expressions |
| rand_distr | 0.4 | Range | Random distributions |
| rayon | 1 | Range | Data parallelism |
| dashmap | 6.1 | Range | Concurrent hash map |
| serde_with | 3 | Range | Serde deserialization helpers |
| urlencoding | 2 | Range | URL percent-encoding |
| csv | 1.4.0 | Range | CSV parsing |
| chardetng | 0.1 | Range | Character encoding detection |
| encoding_rs | 0.8 | Range | Encoding conversion |
| zip | 2.2.0 | Range | ZIP archive handling (addons) |
| jsonpath-rust | 0.7 | Range | JSONPath queries (custom providers) |
| scraper | 0.22 | Range | HTML parsing/scraping |
| sha2 | 0.10 | Range | SHA-256 (also workspace) |
| hex | 0.4 | Range | Hex encoding |
| chrono-tz | 0.10 | Range | Timezone database |
| url | 2 | Range | URL parsing |
| reqwest | 0.12 (workspace) | Range | (with cookies feature) |

#### wealthfolio-storage-sqlite (crates/storage-sqlite/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| diesel, diesel_migrations, rusqlite, r2d2 | workspace | Range | SQLite ORM, migrations, pool |
| All sibling crates | workspace | Path | Internal crate dependencies |

#### wealthfolio-market-data (crates/market-data/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| yahoo_finance_api | =4.1.0 | **Pinned** | Yahoo Finance API (exact version) |
| reqwest | workspace | Range | HTTP client |
| serde, serde_json | 1 | Range | JSON serialization |
| thiserror | 2 | Range | Error derive |
| time | 0.3 | Range | Alternative time library |
| tokio | 1 | Range | Async runtime |
| tracing | 0.1 | Range | Structured logging |
| urlencoding | 2 | Range | URL encoding |
| lazy_static | 1.4 | Range | Lazy initialization |

#### wealthfolio-connect (crates/connect/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| reqwest | workspace | Range | HTTP client |
| base64 | 0.22 | Range | Base64 encoding |

#### wealthfolio-device-sync (crates/device-sync/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| reqwest | workspace | Range | HTTP client |
| urlencoding | 2 | Range | URL encoding |
| base64 | 0.22 | Range | Base64 encoding |
| chacha20poly1305 | 0.10 | Range | E2EE encryption |
| hmac | 0.12 | Range | HMAC authentication |
| hkdf | 0.12 | Range | Key derivation |
| sha2 | 0.10 | Range | Hashing |
| x25519-dalek | 2 | Range | ECDH key exchange |
| rand | 0.8 | Range | Randomness |
| uuid | 1 | Range | Device ID generation |

#### wealthfolio-ai (crates/ai/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| rig-core | 0.30 | Range | LLM orchestration (reqwest-rustls) |
| once_cell | 1 | Range | Lazy static init |
| csv | 1.3 | Range | CSV parsing |
| tokio-stream | 0.1 | Range | Async streaming |
| toml | 0.8 | Optional | Config parsing (eval binary) |
| regex | 1 | Optional | Regex (eval binary) |
| env_logger | 0.11 | Optional | Logging (eval binary) |

#### wealthfolio-agent-tools (crates/agent-tools/Cargo.toml)

No third-party dependencies beyond workspace crates and internal siblings.

#### wealthfolio-spending (crates/spending/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| regex | 1 | Range | Pattern matching |

#### wealthfolio-mcp (crates/wealthfolio-mcp/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| rmcp | workspace | Range | MCP protocol (streamable HTTP) |
| http | 1 | Range | HTTP types |
| base64 | 0.22 | Range | Base64 encoding |
| subtle | 2 | Range | Constant-time comparison (PAT auth) |

#### wealthfolio-server (apps/server/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| axum | 0.8 | Range | Web framework (JSON, macros, multipart) |
| tower | 0.5 | Range | Middleware layer |
| tower-http | 0.6 | Range | CORS, tracing, compression, timeout, request-id, fs |
| tracing | 0.1 | Range | Structured logging |
| tracing-subscriber | 0.3 | Range | Log output (fmt, env-filter, json) |
| dotenvy | 0.15 | Range | .env file loading |
| utoipa | 4 | Range | OpenAPI documentation |
| utoipa-swagger-ui | 4 | Range | Swagger UI |
| serde_with | 3 | Range | Serde helpers |
| serde_urlencoded | 0.7 | Range | URL-encoded form data |
| base64 | 0.21 | Range | Base64 encoding |
| argon2 | 0.5 | Range | Password hashing |
| jsonwebtoken | 10 | Range | JWT creation/validation |
| chacha20poly1305 | 0.10 | Range | Encryption |
| hkdf | 0.12 | Range | Key derivation |
| sha2 | 0.10 | Range | Hashing |
| rand | 0.8 | Range | Randomness |
| urlencoding | 2 | Range | URL encoding |
| tower_governor | 0.8 | Range | Rate limiting |
| tokio-stream | 0.1 | Range | Async streaming |
| futures | 0.3 | Range | Async primitives |
| futures-core | 0.3 | Range | Core async traits |
| semver | 1 | Range | Semantic versioning |
| hyper | 0.14 | Range | HTTP library (full) |
| openidconnect | 4 | Range | OIDC SSO (reqwest, rustls-tls) |
| rmcp | workspace | Range | MCP protocol |
| subtle | 2 | Range | Constant-time comparison |

#### wealthfolio-app (Tauri) (apps/tauri/Cargo.toml)

| Dependency | Version | Lock | Function |
|---|---|---|---|
| tauri | 2.10 | Range | Desktop application framework |
| tauri-plugin-fs | 2.5 | Range | File system access |
| tauri-plugin-dialog | 2 | Range | Native dialogs |
| tauri-plugin-shell | 2 | Range | Shell access |
| tauri-plugin-log | 2 | Range | Logging |
| tauri-plugin-deep-link | 2 | Range | Deep link handling |
| tauri-plugin-updater | 2 | Range | App updates (desktop only) |
| tauri-plugin-window-state | 2 | Range | Window state persistence |
| tauri-plugin-single-instance | 2 | Range | Single instance enforcement |
| tauri-plugin-haptics | 2 | Range | Haptic feedback (mobile) |
| tauri-plugin-barcode-scanner | git | Branch | QR/barcode scanning (mobile) |
| tauri-plugin-web-auth | 1 | Range | Web auth (iOS) |
| tauri-plugin-mobile-share | 0.1.2 | Range | Share sheet (iOS) |
| keyring | 2.0 | Range | OS keychain access |
| local-ip-address | 0.6 | Range | Local IP detection |
| hostname | 0.3 | Range | Hostname resolution |
| base64 | 0.22 | Range | Base64 encoding |
| urlencoding | 2.1.3 | Range | URL encoding |
| axum | 0.8 | Range | Embedded MCP HTTP server (desktop) |
| subtle | 2 | Range | Constant-time comparison |
| tokio-util | 0.7 | Range | Tokio utilities |

### 1.3 Frontend npm Dependencies (apps/frontend/package.json)

Lock status: `^` (caret range, compatible updates) and `~` (tilde range, patch-only).

| Package | Version | Lock | Function |
|---|---|---|---|
| @assistant-ui/react | ^0.11.58 | Caret | AI chat UI components |
| @assistant-ui/react-markdown | ^0.11.10 | Caret | Markdown rendering for chat |
| @fontsource-variable/inter | ^5.2.8 | Caret | Inter variable font |
| @fontsource/ibm-plex-mono | ^5.2.7 | Caret | IBM Plex Mono font |
| @fontsource/jetbrains-mono | ^5.2.8 | Caret | JetBrains Mono font |
| @fontsource/merriweather | ^5.2.11 | Caret | Merriweather font |
| @hookform/resolvers | ^5.2.2 | Caret | React Hook Form schema resolvers |
| @internationalized/date | ^3.11.0 | Caret | i18n date utilities |
| @number-flow/react | ^0.6.1 | Caret | Animated number display |
| @phosphor-icons/react | ^2.1.10 | Caret | Icon library |
| @radix-ui/react-slot | ^1.2.4 | Caret | Radix UI slot component |
| @supabase/supabase-js | ^2.95.3 | Caret | Supabase client (auth only) |
| @tailwindcss/typography | ^0.5.19 | Caret | Tailwind typography plugin |
| @tailwindcss/vite | ^4.1.18 | Caret | Tailwind CSS Vite plugin |
| @tanstack/react-query | ^5.90.20 | Caret | Server state management |
| @tanstack/react-table | ^8.21.3 | Caret | Table component |
| @tanstack/react-virtual | ^3.13.18 | Caret | Virtual scrolling |
| @tauri-apps/api | ^2.11.0 | Caret | Tauri IPC bridge |
| @tauri-apps/plugin-barcode-scanner | ^2.4.4 | Caret | Barcode scanner (mobile) |
| @tauri-apps/plugin-deep-link | 2.4.9 | Exact | Deep link handling |
| @tauri-apps/plugin-dialog | ~2.7.1 | Tilde | Native dialogs |
| @tauri-apps/plugin-fs | ~2.5.1 | Tilde | File system access |
| @tauri-apps/plugin-haptics | ~2.3.2 | Tilde | Haptic feedback |
| @tauri-apps/plugin-log | ~2.8.0 | Tilde | Logging |
| @tauri-apps/plugin-shell | ~2.3.5 | Tilde | Shell access |
| @tauri-apps/plugin-updater | ~2.10.1 | Tilde | App updates |
| @tauri-apps/plugin-window-state | ~2.4.1 | Tilde | Window state persistence |
| clsx | ^2.1.1 | Caret | Class name utility |
| cmdk | ^1.1.1 | Caret | Command palette |
| css-tree | ^3.2.1 | Caret | CSS parsing (addon sandbox) |
| date-fns | ^4.1.0 | Caret | Date utilities |
| es-module-lexer | ^2.1.0 | Caret | ES module analysis (addon sandbox) |
| i18next | ^25.6.0 | Caret | Internationalization framework |
| i18next-resources-to-backend | ^1.2.1 | Caret | i18n resource loading |
| lucide-react | ^0.561.0 | Caret | Icon library |
| motion | ^12.34.0 | Caret | Animation library |
| nanoid | ^5.1.6 | Caret | Unique ID generation |
| qrcode.react | ^4.2.0 | Caret | QR code rendering |
| react | ^19.2.8 | Caret | UI framework |
| react-day-picker | ^9.13.2 | Caret | Date picker |
| react-dom | ^19.2.8 | Caret | React DOM renderer |
| react-hook-form | ^7.71.1 | Caret | Form management |
| react-i18next | ^16.0.0 | Caret | React i18n bindings |
| react-router-dom | ^7.18.2 | Caret | Client-side routing |
| recharts | ^3.7.0 | Caret | Charting library |
| remark-gfm | ^4.0.1 | Caret | GitHub Flavored Markdown |
| sonner | ^2.0.7 | Caret | Toast notifications |
| tailwind-merge | ^3.4.0 | Caret | Tailwind class merging |
| tauri-plugin-mobile-share | ^0.1.2 | Caret | Mobile share sheet |
| tauri-plugin-web-auth-api | ^1.0.0 | Caret | Web auth API |
| zod | ^3.25.76 | Caret | Schema validation |
| zustand | ^5.0.11 | Caret | State management |

**Frontend devDependencies:**

| Package | Version | Lock | Function |
|---|---|---|---|
| @tanstack/eslint-plugin-query | ^5.91.4 | Caret | ESLint plugin for TanStack Query |
| @testing-library/jest-dom | ^6.9.1 | Caret | DOM testing matchers |
| @testing-library/react | ^16.3.2 | Caret | React testing utilities |
| @testing-library/user-event | ^14.6.1 | Caret | User event simulation |
| @types/css-tree | ^3.2.0 | Caret | CSS Tree type definitions |
| @types/node | ^24.10.13 | Caret | Node.js type definitions |
| @types/react | ^19.2.13 | Caret | React type definitions |
| @types/react-dom | ^19.2.3 | Caret | React DOM type definitions |
| @vitejs/plugin-react | ^5.1.4 | Caret | Vite React plugin |
| @vitest/coverage-v8 | ^4.1.0 | Caret | Vitest coverage reporter |
| autoprefixer | ^10.4.24 | Caret | CSS vendor prefixes |
| cross-env | ^10.1.0 | Caret | Cross-platform env vars |
| i18next-cli | ^1.14.0 | Caret | i18n CLI tools |
| jsdom | ^28.0.0 | Caret | DOM environment for tests |
| tailwindcss | ^4.1.18 | Caret | CSS utility framework |
| tw-animate-css | ^1.4.0 | Caret | Tailwind animation CSS |
| vite | ^7.3.6 | Caret | Build tool |
| vitest | ^4.1.0 | Caret | Test runner |

### 1.4 Root DevDependencies (package.json)

| Package | Version | Lock | Function |
|---|---|---|---|
| @eslint/compat | ^1.4.1 | Caret | ESLint compatibility layer |
| @eslint/js | ^9.39.2 | Caret | ESLint JS rules |
| @playwright/test | ^1.58.2 | Caret | E2E testing framework |
| @tanstack/eslint-plugin-query | ^5.91.4 | Caret | TanStack Query ESLint |
| @tauri-apps/cli | ^2.11.1 | Caret | Tauri CLI |
| eslint | ^9.39.2 | Caret | Linter |
| eslint-config-prettier | ^10.1.8 | Caret | Prettier ESLint config |
| eslint-plugin-react | ^7.37.5 | Caret | React ESLint rules |
| eslint-plugin-react-hooks | ^5.2.0 | Caret | React Hooks ESLint rules |
| eslint-plugin-react-refresh | ^0.4.26 | Caret | React Refresh ESLint |
| globals | ^16.5.0 | Caret | Global variables definitions |
| prettier | ^3.8.1 | Caret | Code formatter |
| prettier-plugin-tailwindcss | ^0.6.14 | Caret | Tailwind CSS Prettier plugin |
| typescript | ^5.9.3 | Caret | TypeScript compiler |
| typescript-eslint | ^8.55.0 | Caret | TypeScript ESLint rules |

**pnpm overrides (security patches):**
- qs@<=6.15.1 -> 6.15.2
- @babel/core@<=7.29.0 -> ^7.29.6
- esbuild@>=0.27.3 <0.28.1 -> ^0.28.1

---

## 2. Outbound Network Requests

### 2.1 Market Data Providers

| Provider | Protocol | Base URL | Purpose | Auth |
|---|---|---|---|---|
| Yahoo Finance | HTTPS | `https://fc.yahoo.com` | Cookie acquisition | None |
| | | `https://query1.finance.yahoo.com` | Chart data, quotes, search | Cookie + crumb |
| | | `https://query2.finance.yahoo.com` | Search (fallback) | Crumb |
| Alpha Vantage | HTTPS | `https://www.alphavantage.co/query` | Stock quotes, FX rates | API key (query param) |
| Finnhub | HTTPS | `https://finnhub.io/api/v1` | Stock quotes, company profiles | API key (query param) |
| Boerse Frankfurt | HTTPS | `https://api.live.deutsche-boerse.com/v1` | European stock data | API key |
| MarketData.app | HTTPS | `https://api.marketdata.app/v1` | Stock prices, candles | API key |
| Metal Price API | HTTPS | `https://api.metalpriceapi.com/v1` | Precious metal prices | API key |
| OpenFIGI | HTTPS | `https://api.openfigi.com/v3/mapping` | FIGI identifier mapping | API key (optional) |
| | | `https://api.openfigi.com/v3/search` | FIGI search | API key (optional) |

**Payload details:**
- Yahoo: HTTP GET with cookie `A3` from `fc.yahoo.com`, crumb token from `/v1/test/getcrumb`, then signed requests to `/v8/finance/chart/{symbol}`, `/v10/finance/quoteSummary/{symbol}`, `/v1/finance/search`
- Alpha Vantage: GET with `function`, `symbol`, `apikey` query params
- Finnhub: GET with `symbol`, `token` query params
- Boerse Frankfurt: GET with `symbol` path, `mic` params
- MarketData.app: GET with symbol path, date range, adjust splits/dividends params
- Metal Price API: GET with `base`, `currencies`, `start_date`, `end_date` params
- OpenFIGI: POST JSON mapping requests (idType, idValue, securityType, exchCode)

### 2.2 Wealthfolio Cloud Services

| Service | Protocol | Base URL | Purpose | Auth |
|---|---|---|---|---|
| Connect API | HTTPS | `https://api.wealthfolio.app` | Broker sync, subscription management | Bearer token |
| Auth API | HTTPS | `https://auth.wealthfolio.app` | Authentication flows | Publishable key |
| Connect Portal | HTTPS | `https://connect.wealthfolio.app` | Deep link callback, device management | OAuth code |
| | | `https://connect-staging.wealthfolio.app` | Staging deep link | OAuth code |
| Device Sync API | HTTPS | `https://api.wealthfolio.app` | E2EE device sync, snapshots | Bearer token |
| Addon Store | HTTPS | `https://wealthfolio.app/api/addons` | Addon listing, download | None (public) |
| App Updates | HTTPS | `https://wealthfolio.app/releases/...` | Update manifest download | Public key (ed25519) |
| Website | HTTPS | `https://wealthfolio.app` | Documentation, terms, privacy | None |

**API paths (device-sync):**
- `POST /api/v1/sync/snapshots/` - Upload encrypted snapshot
- `GET /api/v1/sync/snapshots/` - Download snapshots
- `POST /api/v1/sync/team/devices` - Register device
- `GET /api/v1/sync/team/devices/{id}` - Get device info
- `PATCH /api/v1/sync/team/devices/{id}` - Update device
- `DELETE /api/v1/sync/team/devices/{id}` - Remove device
- `POST /api/v1/sync/team/devices/{id}/revoke` - Revoke device
- `POST /api/v1/sync/team/keys/initialize` - Initialize encryption keys

### 2.3 OIDC Providers (Configurable)

| Protocol | Address | Payload | Purpose |
|---|---|---|---|
| HTTPS | Configurable via `WF_OIDC_ISSUER_URL` | Authorization Code + PKCE exchange | SSO login |
| | | `GET {issuer}/.well-known/openid-configuration` | Provider metadata discovery |
| | | `POST {issuer}/token` | Token exchange (code -> tokens) |
| | | `GET {issuer}/authorization` | Browser redirect for auth |
| | | `GET {issuer}/end_session` | RP-Initiated Logout |

Supports any OpenID Connect provider (Google, Microsoft, Okta, Auth0, etc.).

### 2.4 MCP Transport

| Protocol | Transport | Purpose |
|---|---|---|
| HTTPS | Streamable HTTP (via rmcp) | AI agent tool calls over MCP protocol |
| | | Endpoint: `/mcp` (when `WF_MCP_ENABLED=true`) |
| | | Auth: Personal Access Token (SHA-256 hashed, constant-time compared) |

### 2.5 AI Provider Endpoints

Configured via `WF_AI_PROVIDER` and `WF_AI_API_KEY` or stored in secret store. Embedded provider catalog (`crates/ai/src/ai_providers.json`) includes:

- Ollama: `http://localhost:11434` (local)
- OpenAI: configurable `baseUrl` + `apiKey`
- Anthropic: configurable `baseUrl` + `apiKey`
- Google AI: configurable `baseUrl` + `apiKey`
- Custom: configurable `baseUrl` + `apiKey`

---

## 3. File Read-Write Locations

### 3.1 SQLite Database

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | (none) | Diesel ORM connection string (set by server at startup) |
| `WF_DB_PATH` | `./db/app.db` | Database file path (web mode) |
| Tauri | Platform app data dir | Tauri manages the database path internally |

The database contains all user financial data: accounts, holdings, transactions, settings, AI chat history, MCP audit logs, device sync outbox.

### 3.2 Secrets Storage

| Variable | Default | Description |
|---|---|---|
| `WF_SECRET_FILE` | `<data-root>/secrets.json` | Encrypted JSON file with ChaCha20Poly1305 |
| `WF_SECRET_KEY` | (required) | 32-byte key for encryption/decryption |

**Tauri (desktop)**: Uses OS keyring via `keyring` crate (`KeyringSecretStore`). Service names prefixed with `wealthfolio_`.

**Server (web)**: Uses `FileSecretStore` -- encrypted JSON at `<data-root>/secrets.json`. Encrypted with ChaCha20Poly1305 using derived key from `WF_SECRET_KEY`.

### 3.3 Addons Directory

| Variable | Default | Description |
|---|---|---|
| `WF_ADDONS_DIR` | `<db-path-parent>` | Root directory for addon installations |
| | | Each addon gets: `<addons_root>/<addon_id>/` |
| | | Temp files: `<addons_root>/.<addon_id>.tmp-<nonce>/` |
| | | Backups: `<addons_root>/.<addon_id>.backup-<nonce>/` |

### 3.4 Logs

| Variable | Default | Description |
|---|---|---|
| `WF_LOG_FORMAT` | `text` | Output format: `text` or `json` |
| `RUST_LOG` | `info` | Log level filter (EnvFilter) |
| | | Output: stderr (via `tracing-subscriber`), configurable |

Tauri desktop uses `tauri-plugin-log` for native logging integration.

### 3.5 Temporary Files

| Location | Pattern | Purpose |
|---|---|---|
| System temp dir | `wealthfolio-fixture-provider-{ts}-{id}` | Fixture provider cache (market data tests) |
| Addons dir | `.<addon_id>.tmp-{nonce}` | Atomic addon install/update |
| Addons dir | `.<addon_id>.backup-{nonce}` | Addon rollback snapshots |
| `WF_ADDONS_DIR` | `<addon_id>/` | Production addon files |

---

## 4. Listening Ports and System Permissions

### 4.1 Network Ports

| Mode | Port | Variable | Default | Description |
|---|---|---|---|---|
| Web server | HTTP | `WF_LISTEN_ADDR` | `0.0.0.0:8088` | Axum HTTP server (API + static files) |
| Vite dev | HTTP | `VITE_DEV_PORT` | `1420` | Vite dev server (strict port) |
| Vite HMR | WS | -- | `1421` | Hot module replacement WebSocket |
| Tauri desktop | None | -- | -- | No network ports (native app) |

**Web server path restrictions:**
- Root path `/` serves the built frontend SPA
- `/api/v1/*` serves the REST API
- `/mcp` serves the MCP Streamable HTTP endpoint (when enabled)
- `/docs` serves Swagger UI (when enabled)
- Auth required for non-loopback addresses by default (fail-closed)

### 4.2 System Permission Requirements

| Permission | Scope | Component | Purpose |
|---|---|---|---|
| OS keyring | Desktop | `keyring` crate | Store API keys, tokens, credentials |
| File system | Desktop | `tauri-plugin-fs` | Read/write addon files, logs |
| Dialog | Desktop | `tauri-plugin-dialog` | Native file open/save dialogs |
| Shell | Desktop | `tauri-plugin-shell` | External process execution (addons) |
| Deep link | Desktop | `tauri-plugin-deep-link` | `wealthfolio://` scheme handling |
| Deep link | Mobile | Tauri deep-link | `connect.wealthfolio.app` deeplink domain |
| Haptics | Mobile | `tauri-plugin-haptics` | Haptic feedback |
| Barcode scanner | Mobile | `tauri-plugin-barcode-scanner` | QR code scanning (broker sync) |
| Camera | iOS | `tauri-plugin-barcode-scanner` | Camera access for scanning |
| Web auth | iOS | `tauri-plugin-web-auth` | ASWebAuthenticationSession |
| Share sheet | iOS | `tauri-plugin-mobile-share` | Native share functionality |
| Single instance | Desktop | `tauri-plugin-single-instance` | Prevent duplicate app instances |
| Window state | Desktop | `tauri-plugin-window-state` | Persist window position/size |
| App updates | Desktop | `tauri-plugin-updater` | Download and install updates |

---

## 5. Assessment Scores

### Re-usability Grade: A

The dependency selection is well-structured with clear separation of concerns. The workspace-level dependency management in Cargo.toml with centralized version ranges makes upgrades straightforward. The `rmcp` crate for MCP and `rig-core` for AI orchestration are modern, well-maintained choices. The `yaoo_finance_api` pinned version is the only concern but is isolated to the market-data crate.

### Migration Difficulty: Medium

- **Rust side**: The tight coupling to `diesel` (SQLite via ORM), `tower-http`/`axum` (web framework), and `tauri` (desktop) would require significant effort to swap. The `SecretStore` trait abstraction is good for platform portability. The `MarketDataProvider` trait allows swapping providers independently.
- **Frontend side**: Heavy dependency on `@tanstack/react-query`, `react-router-dom`, `zod`, `react-hook-form` -- these are standard and well-supported but would be a full rewrite to switch frameworks.
- **pnpm overrides** for security patches show proactive maintenance.

### Code Risk: Low

- All dependencies use semver ranges (not pinned), which is standard practice but carries minor breakage risk on `pnpm update`/`cargo update`.
- One exact pin (`yahoo_finance_api = 4.1.0`) in `market-data` -- this blocks automatic patch updates but is a deliberate choice.
- One git dependency (`tauri-plugin-barcode-scanner` from branch) -- this is a minor risk as it tracks a moving target, but it's mobile-only and gated behind `cfg(target_os)`.
- No deprecated or unmaintained libraries detected. The dependency tree is modern (2024-2025 versions).
- `hyper 0.14` (vs. 1.x) is the only older major version -- used by the server alongside axum 0.8 which requires hyper 0.14 for compatibility.
- Security-sensitive crates (`chacha20poly1305`, `x25519-dalek`, `sha2`, `argon2`, `jsonwebtoken`, `subtle`) are all well-audited, actively maintained choices.