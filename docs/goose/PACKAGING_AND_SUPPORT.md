# Goose Packaging, Diagnostics, Attribution, and Support (M10-G6)

## 1. Overview

This document defines the release packaging protocols, software bill of materials (SBOM), upstream attribution, platform compatibility matrix, runtime diagnostics, and emergency operator kill switch / rollback procedures for the Goose agent integration in AlphaForge (AlphaForge).

---

## 2. Platform & Architecture Compatibility Matrix

| Platform | Architecture | Target Triplet | Binary Distribution | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Windows** | x86_64 | `x86_64-pc-windows-msvc` | Bundled sidecar / Local PATH | Tier 1 (Supported) |
| **Windows** | ARM64 | `aarch64-pc-windows-msvc` | Bundled sidecar / Local PATH | Tier 2 (Experimental) |
| **macOS** | Apple Silicon | `aarch64-apple-darwin` | Bundled sidecar / Universal | Tier 1 (Supported) |
| **macOS** | Intel x86_64 | `x86_64-apple-darwin` | Bundled sidecar / Universal | Tier 1 (Supported) |
| **Linux** | x86_64 | `x86_64-unknown-linux-gnu` | Bundled sidecar / AppImage | Tier 1 (Supported) |

---

## 3. Binary Acquisition & Integrity Verification

1. **Exact Version Pinning**:
   - Upstream Release: `v1.0.0` (or pinned commit tag).
   - Expected SHA-256 digests are pinned in the release configuration per platform.
2. **Fail-Closed Verification**:
   - Prior to execution, the Rust supervisor (`GooseAdapter`) computes the SHA-256 digest of the binary.
   - If the binary is missing, corrupted, or does not match the pinned digest, execution is immediately aborted with `GooseError::ChecksumMismatch` or `GooseError::BinaryNotFound`.
3. **No Automatic Runtime Downloads**:
   - The application will NEVER automatically download, extract, or execute unverified third-party binaries at runtime.

---

## 4. Software Bill of Materials (SBOM) & Upstream Attribution

### Upstream Project
- **Project Name**: Goose (aaif-goose)
- **Repository**: `https://github.com/aaif-goose/goose`
- **Governing Body**: Agentic AI Foundation (Linux Foundation)
- **License**: Apache License 2.0

### Attribution Notice
```text
Copyright 2024-2026 Agentic AI Foundation (AAIF), a Linux Foundation project.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

---

## 5. Diagnostics & Health Monitoring

The application exposes a sanitized diagnostics endpoint (`desktopApi.goose.getDiagnostics()`):
- Reports:
  - Integration version & engine identifier
  - Binary availability & integrity status
  - Active policy profile (`read_only_shadow_mode`)
  - Supported OS platform & architecture
  - Maximum & active concurrent subprocess counts
- Sanitization:
  - Never leaks user credentials, API keys, full filesystem paths, or research notes.

---

## 6. Operator Kill Switch & Rollback Procedures

If the Goose runtime experiences instability or security anomalies:

### 6.1 Instant Runtime Disable (Kill Switch)
1. Navigate to **Settings -> Agent Settings -> Goose Integration**.
2. Toggle **Enable Shadow Mode** to `OFF`.
3. Alternatively, launch the app with `--disable-goose` flag.

### 6.2 Data Integrity During Rollback
- Disabling or uninstalling Goose **never alters or deletes** previously saved investment theses, research notes, evidence links, or portfolio records.
- All historical proposals and artifacts remain fully viewable and searchable in SQLite.
