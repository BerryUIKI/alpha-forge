# Release Process and Packaging Guide (M11-05)

> **Document:** `docs/RELEASES.md`
> **Status:** ✅ Active Release Protocol
> **Updated:** 2026-10-03
> **Release Coordinator:** `@BerryUIKI`

---

## 1. Overview & Distribution Channels

AlphaForge is distributed as self-contained desktop packages for macOS and Windows.

| Platform | Architecture | Bundle Format | Standard Filename | Support Status |
|----------|--------------|---------------|-------------------|----------------|
| **macOS** | Apple Silicon (`aarch64-apple-darwin`) | Apple Disk Image (`.dmg`) | `AlphaForge_<version>_aarch64.dmg` | Tier 1 (Supported) |
| **Windows** | 64-bit x86_64 (`x86_64-pc-windows-msvc`) | NSIS Installer (`.exe`) | `AlphaForge_<version>_x64-setup.exe` | Tier 1 (Supported) |

---

## 2. Release Candidate Packaging Pipeline

The release packaging procedure is automated via reproducible Node.js scripts in `scripts/`:

### 2.1 Step 1: Pre-Build Quality Gates
Before initiating a release build, all quality gates must pass:
```bash
# Frontend quality checks
pnpm typecheck
pnpm lint
pnpm test

# IPC registration parity
node scripts/check-ipc-registration.mjs

# Rust quality checks
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

### 2.2 Step 2: Build Platform Installers
Run native Tauri bundler for the target platform:
```bash
# On macOS Apple Silicon:
pnpm tauri build --bundles dmg

# On Windows x64:
pnpm tauri build --bundles nsis
```

### 2.3 Step 3: Collect, Normalize, and Generate Checksums
Run the release packaging automation:
```bash
# Collect and rename artifacts to standard naming convention into dist-release/
node scripts/package-release.mjs

# Generate Third-Party Attribution & SBOM notice
node scripts/generate-attribution.mjs
```
This produces:
- `dist-release/AlphaForge_<version>_aarch64.dmg` (or `.exe`)
- `dist-release/SHA256SUMS.txt`
- `dist-release/THIRD_PARTY_NOTICES.md`

### 2.4 Step 4: Verify Artifact Integrity
Verify the generated checksums against the packaged binaries:
```bash
node scripts/package-release.mjs --verify
```

---

## 3. Code Signing & Secret Isolation

Code signing protects binary authenticity and prevents tampering.

1. **Isolation Boundary:**
   - Signing credentials (Apple Developer ID Certificates, Windows Authenticode PFX, private keys) are strictly isolated in secure CI/CD release secrets or the release owner's local hardware keychain.
   - Plaintext credentials and signing keys are **never** committed to Git, logged, or exposed to the frontend/React WebView.
2. **Failure-Safe Handling:**
   - In environments where signing credentials are not configured, builds succeed with an explicit unsigned notification (`⚠️ building unsigned candidate package`).
   - The application does not silently spoof or falsify code signing state.

---

## 4. Software Bill of Materials (SBOM) & Third-Party Attribution

Every release package is accompanied by `THIRD_PARTY_NOTICES.md` documenting:
1. **Goose Agentic Runtime:** Distributed under Apache License 2.0 (Agentic AI Foundation / Linux Foundation).
2. **Wealthfolio Architectural Reference:** Acknowledged under AGPL-3.0 with clean-room SQLite/SQLx implementation.
3. **Bundled Internal Plugins:** Licensed under MIT (`company-comparison`, `financial-analysis`).
4. **Rust & Node.js Dependencies:** Full manifest of all transitive open-source dependencies.

---

## 5. Update & Rollback Policies

1. **Manual Update Checking:**
   - The desktop app checks GitHub Releases only upon explicit user request in **Settings -> About**.
   - It navigates the user to the verified release page; it never downloads or installs executable binaries in the background without consent.
2. **Non-Destructive Rollback:**
   - If an issue is identified in a newly deployed candidate, roll back by installing the prior known-good package.
   - The SQLite database schema supports backward-compatible migrations, preserving user data across installations.
3. **Emergency Disables:**
   - The optional Goose agent runtime can be disabled immediately via **Settings -> Agent Settings** or by launching with `--disable-goose`.

---

## 6. Release Acceptance Gate (M11-07)

Packaging a release candidate (M11-05) produces candidate artifacts for packaged verification (M11-06). Publishing to GitHub Releases or merging to `main` remains governed by the formal acceptance gate (M11-07).
