# Installation

This guide explains how to get AlphaForge running on your machine, either from
source in development mode or as a production build.

## Prerequisites

| Tool | Version | Notes |
|------|---------|-------|
| **Rust** | stable | MSVC toolchain required on Windows (see below) |
| **Node.js** | 22+ | LTS recommended |
| **pnpm** | 9+ | Package manager for the monorepo |

### Windows — Rust MSVC toolchain

AlphaForge is built with Tauri 2, which requires the Microsoft C++ Build Tools
on Windows:

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/).
2. During installation select the **"Desktop development with C++"** workload.
3. Install Rust via [rustup](https://rustup.rs/) and confirm the default MSVC
   toolchain is active:

   ```bash
   rustup default stable-msvc
   rustc --version
   ```

### macOS

Install the [Xcode Command Line Tools](https://developer.apple.com/xcode/):

```bash
xcode-select --install
```

> Note: the MVP does not include macOS notarization, so a Gatekeeper warning
> when opening a downloaded build is a known, expected behavior.

### Linux

Install the [Tauri system dependencies](https://tauri.app/start/prerequisites/)
for your distribution (webkit2gtk, gtk3, and related packages).

## Step 1 — Get the Source Code

```bash
git clone https://github.com/BerryUIKI/alpha-forge.git
cd alpha-forge
```

## Step 2 — Install Dependencies

```bash
pnpm install
```

This installs dependencies for the desktop app, shared packages, and internal
plugins in one pass.

## Step 3 — Run in Development

### Desktop app (full experience)

```bash
pnpm tauri dev
```

This starts the Vite frontend and the Rust backend together, then opens the
native desktop window. The first Rust build takes several minutes.

### Frontend only (browser, no native features)

```bash
pnpm dev:web
```

The web frontend is useful for UI work but native features (filesystem,
credentials, SQLite) are only available in the desktop app.

## Step 4 — Build Production Binaries

```bash
pnpm tauri build
```

Platform installers are produced in `apps/desktop/src-tauri/target/release/`
(e.g. `.msi`/`.exe` on Windows, `.dmg`/`.app` on macOS, `.deb`/`.AppImage` on
Linux).

## Development Quality Commands

```bash
pnpm typecheck        # TypeScript type checking
pnpm lint             # ESLint
pnpm test             # Frontend unit tests (Vitest)

cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Where Your Data Lives

All data is stored locally in a SQLite database inside the app's user data
directory (platform-specific, e.g. `%APPDATA%` on Windows, `~/Library/Application
Support` on macOS). See [Data & Backup](../en/configuration.md#data-and-backup)
for backup and export options.

## Next Steps

- [Configure the application](configuration.md) — language and AI provider.
- [Learn the daily workflow](daily-operations.md).
