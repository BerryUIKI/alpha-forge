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

## Installing Prebuilt Releases (macOS & Windows)

If you downloaded an official release package from the repository releases:

### Windows (`AlphaForge_<version>_x64-setup.exe`)
1. Download the installer file.
2. Run the `.exe` setup file.
3. **Privileges:** The installer installs to your local user directory (`%LOCALAPPDATA%\Programs\AlphaForge`) and requires **no Administrator privileges**.
4. **SmartScreen Warning:** For early or unsigned release candidate builds, Windows SmartScreen may show an untrusted warning. Click **"More info"** followed by **"Run anyway"**.

### macOS (`AlphaForge_<version>_aarch64.dmg`)
1. Download the disk image file.
2. Double-click to mount the `.dmg`, then drag `AlphaForge.app` into your `Applications` folder.
3. **Gatekeeper Notice:** For unsigned release candidates, macOS Gatekeeper may alert that the developer cannot be verified. Right-click the app in Finder and choose **Open**, or go to **System Settings → Privacy & Security** and click **"Open Anyway"**.

---

## Step 4 — Build Production Binaries from Source

```bash
# Package standard installers for the current host:
pnpm release:package

# Or run Tauri bundler directly:
pnpm tauri build
```

Platform installers are placed in `dist-release/` and `apps/desktop/src-tauri/target/release/bundle/`.

## Development Quality Commands

```bash
pnpm typecheck        # TypeScript type checking
pnpm lint             # ESLint
pnpm test             # Frontend unit tests (Vitest)
pnpm test:smoke       # Packaged flows smoke verification

cargo fmt --check     # Rust code formatting check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Where Your Data Lives & Uninstallation

All data is stored locally in a SQLite database inside the user data directory:
- **Windows:** `%APPDATA%\com.berry.alphaforge\alphaforge.db`
- **macOS:** `~/Library/Application Support/com.berry.alphaforge/alphaforge.db`

**Uninstallation Policy:**
When uninstalling AlphaForge, the application binaries are removed cleanly, but your SQLite database and workspaces are **intentionally preserved** to prevent accidental data loss. If you wish to purge all local data completely, manually delete the `com.berry.alphaforge` directory after uninstalling.

See [User Data Export and Recovery Guide](../../DATA_EXPORT_RECOVERY.md) for manual backup instructions.

## Next Steps

- [Configure the application](configuration.md) — language and AI provider.
- [Learn the daily workflow](daily-operations.md).
