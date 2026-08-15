# Wealthfolio — Configuration Reference

Repository: `F:\dev\wealthfolio` · Generated: 2026-08-12

This document interprets every parameter in the repository's YAML, JSON, env,
and INI configuration files. For each entry: parameter name, default value,
data type, affected scope, description, required/optional, and security
implications. Files ending with a `## Grades` section carry re-usability,
migration difficulty, and code-risk grades.

---

## Table of Contents

1. [Scope & Methodology](#scope--methodology)
2. [.env.example — Desktop App Environment](#1-envexample--desktop-app-environment)
3. [.env.web.example — Web Mode Environment](#2-envwebexample--web-mode-environment)
4. [apps/tauri/tauri.conf.json — Tauri App Configuration](#3-appstauritauriconfjson--tauri-app-configuration)
5. [compose.yml — Docker Compose Production](#4-composeyml--docker-compose-production)
6. [compose.dev.yml — Docker Compose Dev Overrides](#5-composedevyml--docker-compose-dev-overrides)
7. [compose.proxy.yml — Docker Compose Proxy Overrides](#6-composeproxyyml--docker-compose-proxy-overrides)
8. [Dockerfile — Docker Build Args](#7-dockerfile--docker-build-args)
9. [apps/server/src/config.rs — Server Config Struct](#8-appsserversrcconfigrs--server-config-struct)
10. [rust-toolchain.toml — Rust Toolchain](#9-rust-toolchaintoml--rust-toolchain)
11. [tsconfig.json — TypeScript Project References](#10-tsconfigjson--typescript-project-references)
12. [tsconfig.base.json — Base TypeScript Config](#11-tsconfigbasejson--base-typescript-config)
13. [.prettierrc.cjs — Prettier Config](#12-prettierrccjs--prettier-config)
14. [eslint.config.js — ESLint Flat Config](#13-eslintconfigjs--eslint-flat-config)
15. [pnpm-workspace.yaml — pnpm Workspace](#14-pnpm-workspaceyaml--pnpm-workspace)
16. [apps/frontend/vite.config.ts — Vite Build Config](#15-appsfrontendviteconfigts--vite-build-config)
17. [apps/tauri/capabilities/*.json — Tauri Capabilities](#16-appstauricapabilitiesjson--tauri-capabilities)
18. [apps/frontend/public/manifest.json — PWA Manifest](#17-appsfrontendpublicmanifestjson--pwa-manifest)
19. [.devcontainer/devcontainer.json — Dev Container](#18-devcontainerdevcontainerjson--dev-container)
20. [.vscode/settings.json — VSCode Settings](#19-vscodesettingsjson--vscode-settings)
21. [crates/storage-sqlite/diesel.toml — Diesel Config](#20-cratesstorage-sqlitedieseltoml--diesel-config)
22. [Grading Summary](#22-grading-summary)

---

## Scope & Methodology

- **Scope**: 20 requested files plus 3 supporting files that the requested
  paths pointed at implicitly (`.prettierrc.cjs` was requested as
  `prettierrc.cjs`; the capabilities directory holds `desktop.json`,
  `ios.json`, and `mobile.json`; `eslint.base.config.js` backs
  `eslint.config.js`).
- **Defaults verified against source**: `apps/server/src/config.rs`
  (server env vars), `apps/server/src/oidc.rs` (WF_OIDC_*), and
  `apps/server/src/auth.rs` (AuthConfig, CookieSecurePolicy).
- **Flag parse convention** (server): `true`/`1`/`yes` are truthy;
  `false`/`0`/`no` are falsy, case-insensitive.
- **Grade legend**: Re-usability (how portable the config is across projects),
  Migration difficulty (how hard to change/remove), Code risk (how likely a
  wrong value breaks the app or leaks data).

---

## 1. `.env.example` — Desktop App Environment

Path: `F:\dev\wealthfolio\.env.example` · Format: dotenv · Scope: Desktop app
(Tauri) + Wealthfolio Connect cloud sync

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `DATABASE_URL` | `../db/app.db` | String | Desktop (Tauri/SQLite) | SQLite database file location relative to the Tauri app data dir | Required | Low — local file |
| `CONNECT_AUTH_URL` | *(empty)* | String (URL) | Desktop + server (Connect SDK) | Auth provider base URL for Wealthfolio Connect cloud sync | Optional | Medium — provider identity; must be HTTPS |
| `CONNECT_AUTH_PUBLISHABLE_KEY` | *(empty)* | String | Desktop + server (Connect SDK) | Publishable (non-secret) key for the auth provider | Optional | Low — publishable by design; never use a secret key |
| `CONNECT_API_URL` | `https://api.wealthfolio.app` | String (URL) | Desktop + server (Connect SDK) | Wealthfolio Connect cloud API endpoint | Optional | Medium — points at the sync backend |
| `CONNECT_OAUTH_CALLBACK_URL` | `https://connect.wealthfolio.app/auth/callback` | String (URL) | Desktop (Connect OAuth) | OAuth redirect callback registered with the provider | Optional | Medium — must match provider registration |

Notes: In the Dockerfile, `CONNECT_AUTH_URL` and
`CONNECT_AUTH_PUBLISHABLE_KEY` are baked into the web bundle/server binary at
build time; `CONNECT_API_URL` is baked into the final image but overridable at
runtime. The values in this file matter for Connect cloud sync only — the app
works without them (sync simply disabled).

### Grades
- Re-usability: **High** — generic OAuth/DB env pattern.
- Migration difficulty: **Low** — renaming keys only touches Connect wiring.
- Code risk: **Low** — all optional; no startup failure if unset.

---
## 2. `.env.web.example` — Web Mode Environment

Path: `F:\dev\wealthfolio\.env.web.example` · Format: dotenv · Scope: Axum web
server + Vite dev proxy. This is the most comprehensive env file. Defaults
verified against `apps/server/src/config.rs`.

### Server (Axum Backend)

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `WF_LISTEN_ADDR` | `0.0.0.0:8088` | String (SocketAddr) | Server | Bind address/port. `127.0.0.1` local-only; `0.0.0.0` for Docker/network | Optional | **High** — non-loopback without auth refuses to start (fail-closed) |
| `WF_DB_PATH` | `./db/app.db` | String (path) | Server + storage | SQLite DB path. File path or directory (app.db created inside a dir) | Optional | Medium — choose file vs dir changes layout |
| `WF_CORS_ALLOW_ORIGINS` | `*` | Comma-separated String | Server | Allowed CORS origins. **Cannot be `*` when auth is enabled** (panic) | Optional | **High** — wildcard + credentials = startup panic; restrict in prod |
| `WF_REQUEST_TIMEOUT_MS` | `30000` (note: config.rs code default **300000**) | Int (ms) | Server | Max request time. Comment says 30s; code default is 300000 ms (300s) | Optional | Medium — mismatch between doc and code |
| `WF_STATIC_DIR` | `dist` | String (path) | Server | Directory of built frontend assets (production static serving) | Optional | Low |

### Secrets & Security

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `WF_SECRET_KEY` | *(none — panics)* | String (Base64/ASCII 32-byte) | Server (all) | Master key. Used for secrets-at-rest encryption (HKDF-derived) and JWT session signing. 32-byte key | **Required** | **Critical** — must be random, never committed; decode failure = startup panic |
| `WF_AUTH_PASSWORD_HASH` | *(none)* | String (Argon2id PHC) | Server (auth) | Argon2id hash enabling password login; enables session signer | Optional (one of auth methods) | **Critical** — hash of password; in Compose double `$` |
| `WF_AUTH_TOKEN_TTL_MINUTES` | `60` | Int (minutes) | Server (auth) | JWT session cookie lifetime | Optional | Medium — shorter = more secure |
| `WF_AUTH_REQUIRED` | `true` | Boolean | Server (startup) | When `false`, allows non-loopback without auth (reverse proxy handles auth). **No MCP escape hatch** | Optional | **Critical** — `false` disables fail-closed guard |

### OIDC / SSO (optional)

Enabled only when BOTH issuer URL and client id are set; partial config panics.

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `WF_OIDC_ISSUER_URL` | *(none)* | String (URL) | Server (OIDC) | IdP base URL; discovery at `<issuer>/.well-known/openid-configuration` | One-half of enable pair | Medium |
| `WF_OIDC_CLIENT_ID` | *(none)* | String | Server (OIDC) | Client credential registered with IdP | One-half of enable pair | Low |
| `WF_OIDC_CLIENT_SECRET` | *(none)* | String | Server (OIDC) | Optional (PKCE always used); for confidential clients | Optional | **High** — secret; keep out of VCS |
| `WF_OIDC_REDIRECT_URL` | *(none — panics if OIDC on)* | String (URL) | Server (OIDC) | Callback URL, must be registered with IdP | **Required if OIDC enabled** | Medium |
| `WF_OIDC_SCOPES` | `openid email profile` | Space-separated String | Server (OIDC) | Requested OIDC scopes | Optional | Low |
| `WF_OIDC_ALLOWED_EMAILS` | *(none)* | Comma-separated String | Server (OIDC) | Email allowlist; **only honored when `email_verified=true`** | Optional (see below) | **High** — fail-closed allowlist |
| `WF_OIDC_ALLOWED_SUBS` | *(none)* | Comma-separated String | Server (OIDC) | Subject (`sub`) allowlist; stronger/stabler control, recommended | Optional (see below) | **High** — fail-closed allowlist |
| `WF_OIDC_ALLOW_ANY` | `false` | Boolean | Server (OIDC) | Allow any IdP user when no allowlist. **Required to start OIDC without allowlist** | Optional | **Critical** — `true` grants every IdP user access |
| `WF_OIDC_POST_LOGOUT_REDIRECT_URL` | *(none)* | String (URL) | Server (OIDC) | Return URL after RP-Initiated Logout; must be registered | Optional | Low |
| `WF_OIDC_RP_LOGOUT` | `true` | Boolean | Server (OIDC) | `false` forces local-only logout even if IdP supports RP-Initiated | Optional | Low |
| `WF_SECRET_FILE` | `<data-root>/secrets.json` | String (path) | Server | Encrypted secrets storage file; data root derived from DB path | Optional | **High** — holds encrypted secrets |
| `WF_ADDONS_DIR` | `<data-root>/addons` | String (path) | Server (addons) | Addon install/load directory; derived from DB path | Optional | **High** — addons execute code |

### MCP

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `WF_MCP_ENABLED` | `false` | Boolean | Server (MCP) | Expose `/mcp` for external AI agents. Auth required on non-loopback | Optional | **Critical** — off-host without auth = startup panic (no WF_AUTH_REQUIRED bypass) |

### Vite Dev Server

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `VITE_API_TARGET` | `http://127.0.0.1:8088` | String (URL) | Vite proxy | Backend API target for the Vite dev proxy; should match `WF_LISTEN_ADDR` | Optional | Low |

### Grades
- Re-usability: **High** — the `WF_*` prefix is clean and self-describing.
- Migration difficulty: **Medium** — OIDC + auth flags are intertwined; the
  `WF_SECRET_KEY` requirement is a hard dependency.
- Code risk: **High** — panics on missing key, wildcard-with-auth, partial
  OIDC, and unauthenticated network binds make misconfiguration loud but
  fail-closed design can surprise first-time deployers.

---
## 3. `apps/tauri/tauri.conf.json` — Tauri App Configuration

Path: `F:\dev\wealthfolio\apps\tauri\tauri.conf.json` · Format: JSON · Scope:
Desktop (Tauri) app build, bundling, security

### build

| Parameter | Value | Type | Description |
|---|---|---|---|
| `build.beforeDevCommand` | `node ./apps/tauri/scripts/sync-ios-composer-icon.mjs && pnpm --filter frontend dev:tauri` | String (cmd) | Runs before `tauri dev`; syncs iOS composer icon, starts Vite |
| `build.beforeBuildCommand` | `node .../sync-ios-composer-icon.mjs && pnpm --filter frontend build:tauri` | String (cmd) | Runs before `tauri build` |
| `build.frontendDist` | `../../dist` | String (path) | Built frontend assets embedded into the app (relative to tauri.conf.json) |
| `build.devUrl` | `http://localhost:1420` | String (URL) | Vite dev server URL used in dev mode |

### bundle

| Parameter | Value | Type | Description |
|---|---|---|---|
| `bundle.active` | `true` | Boolean | Enable bundling into installers |
| `bundle.targets` | `all` | String | Bundle targets (`all` = deb/rpm/nsis/dmg/msi/app) |
| `bundle.icon` | array | Array (paths) | App icons for all platforms/sizes |
| `bundle.copyright` | `2026 Teymz Inc.` | String | Copyright string |
| `bundle.category` | `public.app-category.finance` | String (macOS) | App category |
| `bundle.createUpdaterArtifacts` | `v1Compatible` | String | Generate updater artifacts (v1 compatible) |
| `bundle.iOS.developmentTeam` | `DYDJ2RNL5H` | String | Apple dev team ID (signing) |
| `bundle.iOS.infoPlist` | `Info.ios.plist` | String | iOS Info.plist template |
| `bundle.macOS.bundleVersion` | `20260301.1` | String | macOS bundle version |
| `bundle.macOS.signingIdentity` | `Apple Distribution: Teymz Inc (DYDJ2RNL5H)` | String | macOS signing identity |
| `bundle.macOS.minimumSystemVersion` | `12.0` | String | Minimum macOS version |
| `bundle.linux.deb.desktopTemplate` / `rpm.desktopTemplate` | `linux/wealthfolio.desktop` | String | Desktop entry template for deb/rpm |

### Top-level app identity

| Parameter | Value | Type | Description |
|---|---|---|---|
| `productName` | `Wealthfolio` | String | Product name (installer/app name) |
| `mainBinaryName` | `Wealthfolio` | String | Main binary name |
| `version` | `3.7.0` | String | App version |
| `identifier` | `com.teymz.wealthfolio` | String | Bundle identifier (reverse-DNS) — changing it breaks updater/installer identity |

### plugins

| Parameter | Value | Type | Description | Security |
|---|---|---|---|---|
| `plugins.updater.pubkey` | *(base64 minisign key)* | String | Updater public key — verifies update signatures | **High** — wrong key rejects/fails updates |
| `plugins.updater.endpoints` | `https://wealthfolio.app/releases/{{target}}/{{arch}}/{{current_version}}` | Array (URL) | Update manifest endpoints | **High** — man-in-the-middle if not HTTPS |
| `plugins.updater.windows.installMode` | `passive` | String | Windows installer runs silently | Low |
| `plugins.deep-link.mobile` | array | Array | Deep-link hosts/path-prefixes (Connect mobile) | Medium |
| `plugins.deep-link.desktop.schemes` | `["wealthfolio"]` | Array (scheme) | Desktop deep-link URI scheme | Low |
| `plugins.deep-link.schemes` | `["wealthfolio"]` | Array (scheme) | Global deep-link schemes | Low |

### app

| Parameter | Value | Type | Description |
|---|---|---|---|
| `app.withGlobalTauri` | `false` | Boolean | Expose Tauri API on `window.__TAURI__` (kept off = safer) |
| `app.windows[0].label` | `main` | String | Window label |
| `app.windows[0].create` | `true` | Boolean | Create window at startup |
| `app.windows[0].dragDropEnabled` | `false` | Boolean | HTML5 drag-drop disabled |
| `app.windows[0].resizable` | `true` | Boolean | Resizable window |
| `app.windows[0].theme` | `Light` | String | Window theme |
| `app.windows[0].titleBarStyle` | `Overlay` | String | Overlay title bar |
| `app.windows[0].hiddenTitle` | `true` | Boolean | Hide native title |
| `app.windows[0].title` | `Wealthfolio` | String | Window title |
| `app.windows[0].width` / `height` | `1440` / `960` | Int | Initial window size |

### security (CSP)

| Parameter | Description | Security |
|---|---|---|
| `app.security.csp` | Production Content-Security-Policy. `default-src 'self'`; script-src allows `'wasm-unsafe-eval'`, `blob:`, tauri:; style-src allows `'unsafe-inline'`; connect-src allows wealthfolio/app/connect domains; frame/object-src `'none'` | **Critical** — hardens the WebView; `'unsafe-inline'`/`'wasm-unsafe-eval'` are necessary for the framework but reduce strictness |
| `app.security.devCsp` | Relaxed dev CSP (adds `localhost:1420`, `ws:`, `'unsafe-inline'`/`'unsafe-eval'` for HMR) | **High** — dev-only relaxation |

### Grades
- Re-usability: **Medium** — heavily Wealthfolio-specific (identifiers, teams,
  Connect domains, updater keys).
- Migration difficulty: **High** — `identifier`/`version`/updater pubkey
  changes break installer and auto-update identity.
- Code risk: **Medium** — CSP and updater config are security-sensitive; CSP
  must stay in sync with any new connect-src domains.

---

## 4. `compose.yml` — Docker Compose Production

Path: `F:\dev\wealthfolio\compose.yml` · Format: YAML · Scope: Docker
production deployment

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `services.wealthfolio.image` | `wealthfolio/wealthfolio:latest` | String (image) | Docker | Image to pull | Required | Medium — supply chain |
| `services.wealthfolio.container_name` | `wealthfolio` | String | Docker | Container name | Optional | Low |
| `services.wealthfolio.restart` | `unless-stopped` | String | Docker | Restart policy | Optional | Low |
| `services.wealthfolio.ports` | `"${WF_PORT:-8088}:8088"` | String (port map) | Docker | Host->container port mapping | Required | **High** — port 8088 published to host |
| `services.wealthfolio.volumes` | `wealthfolio-data:/data` | Volume map | Docker | Persist DB and secrets | **Required** | **Critical** — without volume, all data lost on restart |
| `services.wealthfolio.environment.WF_LISTEN_ADDR` | `0.0.0.0:8088` | String | Server | Hardcoded container listen address | Hardcoded | Medium |
| `services.wealthfolio.environment.WF_DB_PATH` | `/data/wealthfolio.db` | String | Server | DB path inside volume | Hardcoded | Medium |
| `services.wealthfolio.environment.WF_SECRET_KEY` | `${WF_SECRET_KEY:?}` | Env ref | Server | **Must be set** — mandatory env var | **Required** | **Critical** |
| `services.wealthfolio.environment.WF_AUTH_PASSWORD_HASH` | `${WF_AUTH_PASSWORD_HASH:-}` | Env ref | Server | Argon2id hash; `$` must be doubled in YAML | Optional | **Critical** |
| `services.wealthfolio.environment.WF_AUTH_TOKEN_TTL_MINUTES` | `${WF_AUTH_TOKEN_TTL_MINUTES:-60}` | Env ref | Server | Session TTL | Optional | Medium |
| `services.wealthfolio.environment.WF_AUTH_REQUIRED` | *(commented out)* | Env ref | Server | `"true"` default; `"false"` for reverse proxy auth | Optional | **Critical** |
| `services.wealthfolio.environment.WF_OIDC_*` | `${WF_OIDC_*:-}` | Env refs | Server | All OIDC vars pass through from host env | Optional | Varies |
| `services.wealthfolio.environment.WF_CORS_ALLOW_ORIGINS` | `${WF_CORS_ALLOW_ORIGINS:-}` | Env ref | Server | Must be set when auth is enabled | **Required if auth** | **Critical** |
| `services.wealthfolio.environment.WF_REQUEST_TIMEOUT_MS` | `${WF_REQUEST_TIMEOUT_MS:-30000}` | Env ref | Server | Request timeout | Optional | Medium |
| `services.wealthfolio.environment.WF_MCP_ENABLED` | `${WF_MCP_ENABLED:-false}` | Env ref | Server | MCP endpoint | Optional | **Critical** |
| `services.wealthfolio.healthcheck` | see YAML | Composite | Docker | `wget` to `/api/v1/healthz` every 30s, 3 retries, 15s start period | Optional | Low |
| `services.wealthfolio.deploy.resources.limits.memory` | `512M` | String | Docker | Memory limit | Optional | Low |
| `services.wealthfolio.deploy.resources.reservations.memory` | `128M` | String | Docker | Memory reservation | Optional | Low |
| `services.wealthfolio.security_opt` | `no-new-privileges:true` | Array | Docker | Drop privilege escalation | Optional | **High** — defense in depth |
| `services.wealthfolio.read_only` | `true` | Boolean | Docker | Read-only root filesystem | Optional | **High** — prevents container tampering |
| `services.wealthfolio.tmpfs` | `/tmp:size=64M` | Array | Docker | Writable tmpfs for /tmp | Optional | Low |

### Grades
- Re-usability: **High** — standard Compose pattern; env-var passthrough.
- Migration difficulty: **Low** — port/volume/secret-key are the only host-specifics.
- Code risk: **Medium** — `read_only: true` + `no-new-privileges` + `tmpfs` are
  hardening best-practices; wrong `$` escaping in `WF_AUTH_PASSWORD_HASH` is
  a common deployer mistake.

---

## 5. `compose.dev.yml` — Docker Compose Dev Overrides

Path: `F:\dev\wealthfolio\compose.dev.yml` · Format: YAML · Scope: Local
development overlay (applied on top of `compose.yml`)

| Parameter | Value | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `services.wealthfolio.build.context` / `dockerfile` | `.` / `Dockerfile` | String | Docker | Build from source instead of pulling image | Optional | Low |
| `services.wealthfolio.ports` | `"${WF_PORT:-8088}:8088"` | String | Docker | Publish port for direct browser access | Optional | High — port exposed |
| `services.wealthfolio.expose` | `!reset []` | YAML tag | Docker | Reset inherited expose (Compose `!reset` override) | Optional | Low |
| `services.wealthfolio.read_only` | `false` | Boolean | Docker | **Override** production hardening: writable rootfs | Override | **High** — dev relaxation |
| `services.wealthfolio.security_opt` | `!reset []` | YAML tag | Docker | Remove `no-new-privileges` | Override | **High** — dev relaxation |
| `services.wealthfolio.deploy` | `!reset {}` | YAML tag | Docker | Remove memory limits | Override | Low |
| `services.wealthfolio.environment.WF_CORS_ALLOW_ORIGINS` | `http://localhost:1420,http://localhost:3000` | String | Server | Dev CORS origins | Override | Medium |
| `services.wealthfolio.environment.WF_AUTH_PASSWORD_HASH` | `""` | String | Server | Auth disabled in dev | Override | **High** — dev only |
| `services.wealthfolio.environment.WF_AUTH_REQUIRED` | `"false"` | String | Server | Skip auth requirement | Override | **Critical** — dev only |

Notes: Uses Compose `!reset` YAML tags to null out production hardening. Never
use this overlay in production — it disables auth, privilege-dropping, and
read-only filesystem.

### Grades
- Re-usability: **Medium** — `!reset` overlay pattern is clever but Compose-version-specific.
- Migration difficulty: **Low** — dev-only, isolated overlay.
- Code risk: **High** — if accidentally merged into prod deploy, disables all security hardening.

## 6. `compose.proxy.yml` — Docker Compose Proxy Overrides

Path: `F:\dev\wealthfolio\compose.proxy.yml` · Format: YAML · Scope: Reverse
proxy deployment (Coolify, Caddy, Traefik, Nginx Proxy Manager)

| Parameter | Value | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `services.wealthfolio.ports` | `!reset []` | YAML tag | Docker | Remove published host port (proxy on same network instead) | Override | **High** — avoids exposing app port directly to host |
| `services.wealthfolio.expose` | `"8088"` | Array | Docker | Expose port only on internal Docker network for the proxy | Override | **High** — keeps app reachable only via proxy |

Notes: The proxy routes to `http://wealthfolio:8088`. This is the recommended
production topology — the app port is never published to the host, so only the
proxy (which terminates TLS) is directly reachable.

### Grades
- Re-usability: **High** — generic reverse-proxy overlay pattern.
- Migration difficulty: **Low** — tiny, self-contained.
- Code risk: **Low** — removing the published port is strictly safer.

---

## 7. `Dockerfile` — Docker Build Args

Path: `F:\dev\wealthfolio\Dockerfile` · Format: Dockerfile · Scope: Multi-stage
Docker image build

| Parameter | Default | Type | Scope | Description | Req | Security |
|---|---|---|---|---|---|---|
| `RUST_IMAGE` | `rust:1.91-alpine` | ARG (image tag) | Backend build stage | Rust base image for server build | Optional | Medium — supply chain |
| `CONNECT_AUTH_URL` | *(empty)* | ARG/ENV | Frontend bundle + server binary | Auth provider URL baked at build time; omit to build without Connect | Optional | Medium |
| `CONNECT_AUTH_PUBLISHABLE_KEY` | *(empty)* | ARG/ENV | Frontend bundle + server binary | Publishable key baked at build time | Optional | Low |
| `CONNECT_API_URL` | *(empty)* | ARG/ENV | Final image | Connect API URL, baked but overridable at runtime | Optional | Medium |
| `TARGETPLATFORM` | *(auto)* | ARG (buildx) | Backend build stage | Target platform for cross-compilation | Implicit | Low |
| `BUILD_TARGET` | `web` | ENV | Frontend build | Build target for Vite (`web` in this image) | Implicit | Low |
| `CI` | `1` | ENV | pnpm install | CI mode: frozen lockfile, no interactive prompts | Implicit | Low |
| `CARGO_REGISTRIES_CRATES_IO_PROTOCOL` | `sparse` | ENV | Cargo | Use sparse crates.io index (faster) | Implicit | Low |
| `OPENSSL_STATIC` | `1` | ENV | Server build | Statically link OpenSSL | Implicit | Low |
| `WF_DB_PATH` | `/data/wealthfolio.db` | ENV | Final runtime | Default DB path in the image | Implicit | Medium |

Runtime defaults in the final image: `USER 1000:1000` (non-root), `VOLUME
["/data"]`, `EXPOSE 8088`, `CMD ["/usr/local/bin/wealthfolio-server"]`.

### Grades
- Re-usability: **Medium** — multi-stage build is exemplary; the Connect ARGs
  and workspace stubbing are project-specific.
- Migration difficulty: **Medium** — adding a new Connect-like build-time var
  requires touching multiple stages.
- Code risk: **Medium** — build args are baked into images; a leaked
  `CONNECT_AUTH_PUBLISHABLE_KEY` would be embedded in every image layer.

---

## 8. `apps/server/src/config.rs` — Server Config Struct

Path: `F:\dev\wealthfolio\apps\server\src\config.rs` · Format: Rust source ·
Scope: Axum web server (authoritative defaults for all `WF_*` env vars)

This is the authoritative source for server-side defaults. Notable entries
(env var → default → semantics):

| Env var | Struct field | Default | Notes |
|---|---|---|---|
| `WF_LISTEN_ADDR` | `listen_addr: SocketAddr` | `0.0.0.0:8088` | Parse failure panics |
| `WF_DB_PATH` | `db_path: String` | `./db/app.db` | Also derives `addons_root` and data root |
| `WF_CORS_ALLOW_ORIGINS` | `cors_allow: Vec<String>` | `*` | `*` + auth enabled → **panic** |
| `WF_REQUEST_TIMEOUT_MS` | `request_timeout: Duration` | `300000` (ms) | **Code default is 300s**, not the 30s the .env comment claims |
| `WF_STATIC_DIR` | `static_dir: String` | `dist` | Production static assets dir |
| `WF_SECRET_KEY` | `raw_secret_key` + `secrets_encryption_key` | *(panic if unset)* | Must be 32-byte; HKDF-derives JWT sign key + encryption key |
| `WF_ADDONS_DIR` | `addons_root: String` | DB path parent | Addon install root |
| `WF_AUTH_PASSWORD_HASH` | `auth.password_hash` | *(none)* | Argon2id PHC; enables auth |
| `WF_AUTH_TOKEN_TTL_MINUTES` | `access_token_ttl` | `60` | Must be > 0 |
| `WF_COOKIE_SECURE` | `cookie_secure` | `auto` | `auto`/`true`/`false`; `Secure` cookie policy |
| `WF_AUTH_REQUIRED` | *(startup guard)* | `true` | `false` allows non-loopback without auth |
| `WF_MCP_ENABLED` | `mcp_enabled` | `false` | Non-loopback + MCP + no auth → **panic** |
| `WF_MCP_AUDIT_ENABLED` | `mcp_audit_enabled` | `true` | Log agent tool calls |
| `WF_MCP_ALLOWED_HOSTS` | `mcp_allowed_hosts: Option<Vec>` | `None` | `None` disables Host validation (loopback default would break proxies) |

Fail-closed startup guards (all panic to stderr):
1. `WF_SECRET_KEY` missing/empty/not 32-byte.
2. `WF_CORS_ALLOW_ORIGINS == "*"` while auth is enabled.
3. Listening on a non-loopback address without any auth method, unless
   `WF_AUTH_REQUIRED=false`.
4. `WF_MCP_ENABLED=true` on a non-loopback address without auth — **no
   `WF_AUTH_REQUIRED` bypass exists for MCP**.

### Grades
- Re-usability: **High** — clean single-struct env parsing; centralizes all defaults.
- Migration difficulty: **Medium** — renames ripple through `config.rs`, `auth.rs`, `oidc.rs`.
- Code risk: **Medium** — plenty of panics make misconfig loud, but the
  `WF_REQUEST_TIMEOUT_MS` doc/code default mismatch (30s vs 300s) is a latent footgun.

---

## 9. `rust-toolchain.toml` — Rust Toolchain

Path: `F:\dev\wealthfolio\rust-toolchain.toml` · Format: TOML · Scope: Rust
workspace (all `crates/*`, `apps/tauri`, `apps/server`)

| Parameter | Default | Type | Description | Req |
|---|---|---|---|---|
| `toolchain.channel` | `1.95.0` | String | Pinned Rust toolchain version — all builds use this exact version | Required |
| `toolchain.components` | `["rustfmt", "clippy"]` | Array | Auto-install rustfmt and clippy with the toolchain | Optional |

Notes: Pinning to `1.95.0` ensures reproducible builds across CI and local dev.
Note the Dockerfile builds with `rust:1.91-alpine` (older) — a version skew
between the image and `rust-toolchain.toml`.

### Grades
- Re-usability: **High** — standard, minimal Rust toolchain pin.
- Migration difficulty: **Low** — bumping the channel is one line.
- Code risk: **Low** — version skew with the Docker Rust image is the only concern.

---

## 10. `tsconfig.json` — TypeScript Project References

Path: `F:\dev\wealthfolio\tsconfig.json` · Format: JSON · Scope: TypeScript
monorepo root (project references)

| Parameter | Value | Type | Description |
|---|---|---|---|
| `extends` | `./tsconfig.base.json` | String (path) | Inherit base compiler options |
| `references` | `frontend`, `ui`, `addon-sdk` | Array (paths) | Project references for composite builds |
| `files` | `[]` | Array | No root source files (all in referenced projects) |

### Grades
- Re-usability: **High** — standard TS workspace reference pattern.
- Migration difficulty: **Low** — reference list mirrors the workspace.
- Code risk: **Low**.

---

## 11. `tsconfig.base.json` — Base TypeScript Config

Path: `F:\dev\wealthfolio\tsconfig.base.json` · Format: JSON · Scope: All TS
workspaces (inherited)

| Parameter | Value | Type | Description |
|---|---|---|---|
| `compilerOptions.target` | `ES2022` | String | ECMAScript target |
| `compilerOptions.useDefineForClassFields` | `true` | Boolean | Native class-field semantics |
| `compilerOptions.lib` | `["ES2022","DOM","DOM.Iterable"]` | Array | Library types |
| `compilerOptions.module` | `ESNext` | String | Module system |
| `compilerOptions.skipLibCheck` | `true` | Boolean | Skip .d.ts checking |
| `compilerOptions.esModuleInterop` | `true` | Boolean | ESM/CJS interop |
| `compilerOptions.allowSyntheticDefaultImports` | `true` | Boolean | Default-import interop |
| `compilerOptions.moduleResolution` | `bundler` | String | Bundler-style resolution |
| `compilerOptions.allowImportingTsExtensions` | `true` | Boolean | Import `.ts` extensions |
| `compilerOptions.resolveJsonModule` | `true` | Boolean | JSON imports |
| `compilerOptions.isolatedModules` | `true` | Boolean | Per-file transpile |
| `compilerOptions.jsx` | `react-jsx` | String | React JSX transform |
| `compilerOptions.strict` | `true` | Boolean | **Strict mode** |
| `compilerOptions.noUnusedLocals` / `noUnusedParameters` | `true` / `true` | Boolean | Error on unused code |
| `compilerOptions.noFallthroughCasesInSwitch` | `true` | Boolean | No fallthrough |
| `compilerOptions.noImplicitReturns` | `false` | Boolean | Explicitly disabled |
| `compilerOptions.noImplicitOverride` | `true` | Boolean | Require `override` keyword |
| `compilerOptions.noImplicitAny` / `noImplicitThis` | `true` / `true` | Boolean | Implicit any/this errors |
| `compilerOptions.strictNullChecks` | `true` | Boolean | Null safety |
| `compilerOptions.strictFunctionTypes` | `true` | Boolean | Function variance checks |
| `compilerOptions.strictBindCallApply` | `true` | Boolean | Strict bind/call/apply |
| `compilerOptions.strictPropertyInitialization` | `true` | Boolean | Strict property init |
| `compilerOptions.alwaysStrict` | `true` | Boolean | Emit `"use strict"` |
| `compilerOptions.baseUrl` | `.` | String | Alias base (overridden per workspace) |
| `compilerOptions.paths` | `{}` | Object | Path aliases (overridden per workspace) |
| `compilerOptions.composite` | `true` | Boolean | Enable project references |
| `exclude` | `["node_modules","dist","build","coverage"]` | Array | Excluded paths |

### Grades
- Re-usability: **High** — clean, strict-but-pragmatic base config.
- Migration difficulty: **Medium** — `strict: true` + type-checked ESLint means
  changes can surface many errors at once.
- Code risk: **Low**.

---

## 12. `.prettierrc.cjs` — Prettier Config

Path: `F:\dev\wealthfolio\.prettierrc.cjs` (requested as `prettierrc.cjs`)
· Format: CommonJS · Scope: All formatted JS/TS/JSON/MD/YAML

| Parameter | Value | Type | Description |
|---|---|---|---|
| `printWidth` | `100` | Int | Line width |
| `tabWidth` | `2` | Int | Indent width |
| `useTabs` | `false` | Boolean | Spaces, not tabs |
| `semi` | `true` | Boolean | Semicolons |
| `singleQuote` | `false` | Boolean | Double quotes |
| `quoteProps` | `as-needed` | String | Quote object props only when needed |
| `trailingComma` | `all` | String | Trailing commas everywhere |
| `jsxSingleQuote` | `false` | Boolean | Double quotes in JSX |
| `bracketSpacing` | `true` | Boolean | Spaces in object braces |
| `bracketSameLine` | `false` | Boolean | Closing bracket on new line |
| `singleAttributePerLine` | `false` | Boolean | Multiple attributes per line |
| `arrowParens` | `always` | String | Always parenthesize arrow params |
| `endOfLine` | `lf` | String | LF line endings |
| `insertPragma` / `requirePragma` | `false` / `false` | Boolean | No pragma gating |
| `proseWrap` | `always` | String | Wrap prose |
| `htmlWhitespaceSensitivity` | `css` | String | Whitespace sensitivity |
| `embeddedLanguageFormatting` | `auto` | String | Format embedded languages |
| `overrides` | JSON/MD/YAML blocks | Array | Per-format overrides (MD printWidth 80) |
| `plugins` | `["prettier-plugin-tailwindcss"]` | Array | Tailwind class sorting |

### Grades
- Re-usability: **High** — standard, widely-compatible Prettier config.
- Migration difficulty: **Low**.
- Code risk: **Low**.

---

## 13. `eslint.config.js` — ESLint Flat Config

Path: `F:\dev\wealthfolio\eslint.config.js` (+ `eslint.base.config.js`)
· Format: ESM · Scope: Root workspace linting

| Parameter | Value | Type | Description |
|---|---|---|---|
| `ignores` | *(long list)* | Array | Skip dist, node_modules, apps, packages, generated, db, test artifacts |
| `createBaseConfig({ includeReact, includeTanstackQuery, includeReactRefresh, tsconfigPath })` | React/TanStack/Refresh off; `./tsconfig.json` | Function | Root applies base config to root-level JS/TS only |

Base config (`eslint.base.config.js`) key rules: `prefer-const`/`no-var` =
error; most `@typescript-eslint/no-unsafe-*` = warning; `react-hooks` rules =
warning; TanStack pending-query rules = warning; `no-console` = warn (allow
warn/error); `eslint-config-prettier` last to disable formatting conflicts.
Uses `projectService: true` for typed linting.

### Grades
- Re-usability: **High** — flat config with a reusable `createBaseConfig` factory.
- Migration difficulty: **Medium** — flat config + projectService is modern but
  the per-workspace split is subtle.
- Code risk: **Low** — mostly warnings; low breakage risk.

---

## 14. `pnpm-workspace.yaml` — pnpm Workspace

Path: `F:\dev\wealthfolio\pnpm-workspace.yaml` · Format: YAML · Scope: Monorepo
package management

| Parameter | Value | Type | Description |
|---|---|---|---|
| `packages` | `["apps/frontend","packages/*"]` | Array (globs) | Workspace packages (frontend + shared packages) |
| `ignoredBuiltDependencies` | `["@tailwindcss/oxide","esbuild","msw"]` | Array | Skip postinstall build scripts for these deps (pnpm 10: `onlyBuiltDependencies` counterpart) |

### Grades
- Re-usability: **High** — minimal standard workspace config.
- Migration difficulty: **Low**.
- Code risk: **Low** — ignoring native builds (esbuild, tailwind oxide) is
  intentional; re-adding them could slow installs.

---

## 15. `apps/frontend/vite.config.ts` — Vite Build Config

Path: `F:\dev\wealthfolio\apps\frontend\vite.config.ts` · Format: TS ·
Scope: Frontend build + dev server

### Env-driven behavior

| Env var | Default | Type | Scope | Description |
|---|---|---|---|---|
| `TAURI_DEV_HOST` | *(unset)* | String | Dev server | When set, binds host `0.0.0.0` and enables ws HMR |
| `VITE_API_TARGET` / `WF_API_TARGET` | `http://127.0.0.1:8088` | String (URL) | Dev proxy | Backend proxy target |
| `WF_ENABLE_VITE_PROXY` | *(unset)* | Boolean | Dev proxy | `"true"` enables `/api` + `/docs` proxy |
| `VITE_DEV_PORT` | `1420` | Int | Dev server | Dev server port (strict) |
| `BUILD_TARGET` | `tauri` | String | Build | `tauri` or `web`; selects adapter alias + `__BUILD_TARGET__` define |
| `TAURI_DEBUG` | *(unset)* | Boolean | Build | When set: skip minify, emit sourcemaps |

### Static config

| Parameter | Value | Type | Description |
|---|---|---|---|
| `envDir` | `../..` | String | Load env from repo root (`.env`, `.env.web`) |
| `plugins` | `react()`, `tailwindcss()` | Array | Vite plugins |
| `publicDir` | `public` | String | Static assets dir |
| `optimizeDeps.include` | `["lucide-react","recharts"]` | Array | Pre-bundle deps |
| `define.__BUILD_TARGET__` | from `BUILD_TARGET` | String | Compile-time build target constant |
| `resolve.alias` | `@wealthfolio/*`, `@/adapters`, `#platform`, `@` | Object | Path aliases; adapter/platform aliases switch on build target |
| `resolve.extensions` | `[.js,.ts,.jsx,.tsx,.json]` | Array | Resolvable extensions |
| `server.port` / `strictPort` | `1420` / `true` | Int/Bool | Fixed port, fail if busy |
| `server.headers` | `Access-Control-Allow-Origin: *` | Object | Dev CORS (dev only) |
| `server.watch.ignored` | `**/apps/tauri/**` | Array | Don't watch Tauri dir |
| `envPrefix` | `["VITE_","TAURI_","CONNECT_"]` | Array | Which env vars are exposed to client |
| `build.target` | `chrome107,edge107,firefox104,safari16` | Array | Browser targets |
| `build.outDir` | `../../dist` | String | Root dist (embedded by Tauri) |
| `build.emptyOutDir` | `true` | Boolean | Clean dist each build (prevents stale bundles) |
| `build.rollupOptions.input` | `main`, `addon-sandbox` | Object | Two entry HTML files |
| `test` | Vitest config | Object | globals, jsdom, setup, include |

### Grades
- Re-usability: **Medium** — build-target adapter switching is a clever but
  project-specific pattern.
- Migration difficulty: **High** — the `tauri`/`web` adapter alias split and
  `envDir: ../..` coupling are non-trivial to restructure.
- Code risk: **Medium** — `envPrefix` includes `CONNECT_` (so Connect keys leak
  into the client bundle by design); `BUILD_TARGET` mismatch silently selects
  the wrong adapter.

---

## 16. `apps/tauri/capabilities/*.json` — Tauri Capabilities

Path: `F:\dev\wealthfolio\apps\tauri\capabilities\` (desktop.json, ios.json,
mobile.json) · Format: JSON · Scope: Tauri permission grants per platform

### desktop.json (platforms: macOS, windows, linux; window: `main`)

| Permission | Description | Security |
|---|---|---|
| `core:default` | Base core permissions | Low |
| `fs:allow-read-file` / `copy-file` / `remove` / `exists` | File ops | Medium |
| `fs:scope` (`$APPDATA/pending-exports/**`) | FS scope restricted to exports dir | **High** — scoping limits file access |
| `core:window:allow-start-dragging` / `set-fullscreen` / `is-fullscreen` | Window controls | Low |
| `dialog:allow-open` / `save` | File dialogs | Low |
| `fs:default`, `dialog:default`, `deep-link:default`, `updater:default`, `log:default`, `window-state:default` | Default permission sets | Low |
| `core:app:allow-set-app-theme`, `core:window:allow-set-theme` / `theme` | Theme control | Low |

### ios.json (platforms: iOS) — `mobile-share:default` only.

### mobile.json (platforms: iOS, android)

Adds: `fs:*` read/write/read-dir/mkdir/rename, `shell:allow-open`,
`haptics:*`, `web-auth:default` + `web-auth:allow-authenticate`,
`barcode-scanner:*` (scan/cancel/check/request permissions). FS scope widened
to `$APPDATA/**`.

Security note: the desktop capability scopes FS to
`$APPDATA/pending-exports/**` while mobile widens to `$APPDATA/**`; the
difference reflects export-only vs broader mobile needs.

### Grades
- Re-usability: **Medium** — capability model is modern Tauri; scopes are app-specific.
- Migration difficulty: **Medium** — permission lists must stay in sync with new features.
- Code risk: **High** — overly broad FS/shell permissions are an escalation
  surface; `shell:allow-open` and `$APPDATA/**` on mobile deserve scrutiny.

---

## 17. `apps/frontend/public/manifest.json` — PWA Manifest

Path: `F:\dev\wealthfolio\apps\frontend\public\manifest.json` · Format: JSON ·
Scope: Web/PWA install metadata

| Parameter | Value | Type | Description |
|---|---|---|---|
| `name` / `short_name` | `Wealthfolio` | String | App display name |
| `start_url` | `/` | String | Launch URL |
| `display` | `standalone` | String | No browser chrome (app-like) |
| `background_color` | `#09090b` | String | Splash background |
| `theme_color` | `#09090b` | String | Browser theme |
| `icons` | 4 entries | Array | apple-touch-icon, 192, 512, SVG |

### Grades
- Re-usability: **Low** — trivial, app-specific.
- Migration difficulty: **Low**.
- Code risk: **Low**.

---

## 18. `.devcontainer/devcontainer.json` — Dev Container

Path: `F:\dev\wealthfolio\.devcontainer\devcontainer.json` · Format: JSON
(with comments) · Scope: Local dev container environment

### Core

| Parameter | Value | Type | Description |
|---|---|---|---|
| `name` | `Wealthflolio Dev Environment` | String | Container display name |
| `image` | `ivangabriele/tauri:debian-bookworm-22` | String | Base image (Tauri + Rust deps) |
| `containerEnv.DISPLAY` | `:99` | String | Virtual X display |
| `containerEnv.XVFB` / `XVFBARGS` | `/usr/bin/Xvfb` / `-ac +extension RANDR` | String | Xvfb path/args |
| `containerEnv.X11VNC` / `X11VNCARGS` | `/usr/bin/x11vnc` / `-forever -rfbport 5900` | String | VNC server path/args |
| `containerEnv.LC_ALL` / `LANG` / `LANGUAGE` | `en_US.UTF-8` | String | Locale |
| `containerEnv.WEBKIT_DISABLE_DMABUF_RENDERER` | `1` | String | Fix WebKit rendering |
| `runArgs` | `--gpus=all`, `--name=wealthflolio-dev`, `--hostname=...`, `--net=host` | Array | GPU, naming, host network |
| `workspaceMount` / `workspaceFolder` | bind / `/app/dev/workspace/...` | String | Mount workspace |
| `mounts` | cargo cache + appdata volumes | Array | Persist cargo cache + app data |
| `features` | shell-history, apt-packages (xvfb, x11vnc, net-tools, locales) | Object | Devcontainer features |
| `forwardPorts` | `[1420, 1421, 5900]` | Array | Forward app + VNC ports |
| `postCreateCommand` | `yes\|pnpm install` | Object | Install deps |
| `postStartCommand` | startXVFB, startX11VNC | Object | Start virtual display + VNC |
| `remoteUser` | `root` | String | Run as root |

### Security notes
- `--net=host` shares the host network namespace (needed for websockets) — a
  dev-only convenience.
- `--gpus=all` grants full GPU access.
- `X11VNC_PASSWORD` is only set via `remoteEnv` (commented out by default) —
  **VNC is unauthenticated unless set**.
- `remoteUser: root` runs the container as root.

### Grades
- Re-usability: **Low** — purpose-built for Tauri GUI dev over VNC.
- Migration difficulty: **Medium** — tangled Xvfb/VNC/GPU/host-network config.
- Code risk: **High** — `--net=host`, default-root, and unauthenticated VNC
  are real security concerns if this container is used beyond local dev.

---

## 19. `.vscode/settings.json` — VSCode Settings

Path: `F:\dev\wealthfolio\.vscode\settings.json` · Format: JSON (with comments)
· Scope: Editor behavior for contributors

| Parameter | Value | Type | Description |
|---|---|---|---|
| `editor.formatOnSave` | `true` | Boolean | Format on save |
| `editor.defaultFormatter` | `esbenp.prettier-vscode` | String | Prettier as default formatter |
| `editor.codeActionsOnSave` | eslint fix + organize imports | Object | Fix lint + organize imports on save |
| `eslint.useFlatConfig` | `true` | Boolean | ESLint v9 flat config |
| `eslint.validate` | js/ts/jsx/tsx | Array | Files linted |
| `eslint.workingDirectories` | `["./"]` | Array | Lint root |
| `prettier.requireConfig` | `true` | Boolean | Only format if config present |
| `prettier.useEditorConfig` | `false` | Boolean | Ignore .editorconfig |
| `typescript.*` | various | Misc | TS preferences/inlay hints |
| language-specific formatters | Prettier for all; rust-analyzer for Rust | Object | Per-language formatters |
| `files.trimTrailingWhitespace` / `insertFinalNewline` / `trimFinalNewlines` | true | Boolean | Whitespace hygiene |
| `kiroAgent.configureMCP` | `Disabled` | String | Disable kiro MCP |
| `i18n-ally.*` | locales/en | Misc | i18n ally settings |

### Grades
- Re-usability: **High** — standard, portable editor settings.
- Migration difficulty: **Low**.
- Code risk: **Low**.

---

## 20. `crates/storage-sqlite/diesel.toml` — Diesel Config

Path: `F:\dev\wealthfolio\crates\storage-sqlite\diesel.toml` · Format: TOML ·
Scope: Diesel ORM schema/migrations for the SQLite storage crate

| Parameter | Value | Type | Description |
|---|---|---|---|
| `print_schema.file` | `src/schema.rs` | String (path) | Generated schema output file |
| `print_schema.custom_type_derives` | `["diesel::query_builder::QueryId"]` | Array | Derives applied to custom types |
| `migrations_directory.dir` | `migrations` | String (path) | Diesel migration directory |

### Grades
- Re-usability: **High** — standard Diesel config.
- Migration difficulty: **Low**.
- Code risk: **Low** — `custom_type_derives` is slightly unusual but harmless.

---
## 22. Grading Summary

| # | File | Re-usability | Migration Difficulty | Code Risk |
|---|---|---|---|---|
| 1 | `.env.example` | High | Low | Low |
| 2 | `.env.web.example` | High | Medium | High |
| 3 | `apps/tauri/tauri.conf.json` | Medium | High | Medium |
| 4 | `compose.yml` | High | Low | Medium |
| 5 | `compose.dev.yml` | Medium | Low | High |
| 6 | `compose.proxy.yml` | High | Low | Low |
| 7 | `Dockerfile` | Medium | Medium | Medium |
| 8 | `apps/server/src/config.rs` | High | Medium | Medium |
| 9 | `rust-toolchain.toml` | High | Low | Low |
| 10 | `tsconfig.json` | High | Low | Low |
| 11 | `tsconfig.base.json` | High | Medium | Low |
| 12 | `.prettierrc.cjs` | High | Low | Low |
| 13 | `eslint.config.js` | High | Medium | Low |
| 14 | `pnpm-workspace.yaml` | High | Low | Low |
| 15 | `apps/frontend/vite.config.ts` | Medium | High | Medium |
| 16 | `apps/tauri/capabilities/*.json` | Medium | Medium | High |
| 17 | `apps/frontend/public/manifest.json` | Low | Low | Low |
| 18 | `.devcontainer/devcontainer.json` | Low | Medium | High |
| 19 | `.vscode/settings.json` | High | Low | Low |
| 20 | `crates/storage-sqlite/diesel.toml` | High | Low | Low |

### Notable findings

1. **Security fail-closed**: The server (`config.rs`) panics on several
   misconfiguration scenarios (missing key, wildcard+auth, non-loopback without
   auth, MCP without auth). This is a strong safety design.
2. **Timeout mismatch**: `.env.web.example` documents `WF_REQUEST_TIMEOUT_MS`
   default as 30000 (30s), but `config.rs` code defaults to `300000` (300s).
   The comment may be correct and the code stale, or vice versa.
3. **Docker $ escaping**: `WF_AUTH_PASSWORD_HASH` uses `$argon2id$...` and
   Docker Compose interpolates `$` — users must double every `$` or use
   single-quoted `.env` files. This is a recurring deployer pain point.
4. **Dev overlay risk**: `compose.dev.yml` disables auth, read-only fs, and
   privilege-dropping. If used in production by mistake, it opens the server
   completely.
5. **Capability scope**: Desktop scopes FS to `$APPDATA/pending-exports/**`;
   mobile widens to `$APPDATA/**` plus `shell:allow-open`. Mobile has higher
   escalation risk.
6. **Dev container risk**: `--net=host`, `remoteUser: root`, and
   unauthenticated VNC (no password set by default) are significant security
   concerns for a container intended for GUI development.
7. **Version skew**: `rust-toolchain.toml` pins 1.95.0, but the Dockerfile
   uses `rust:1.91-alpine` — a mismatch that could cause subtle build
   differences.
