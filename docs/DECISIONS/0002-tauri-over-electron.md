# ADR 0002: Tauri over Electron

**Date:** 2026-07-31

**Status:** Accepted

## Context

Investment OS needs a desktop application shell that hosts a React frontend and a Rust backend. The two dominant options for cross-platform desktop apps with web UIs are Electron and Tauri.

## Decision

**Choose Tauri 2.**

## Evaluation

| Dimension | Tauri 2 | Electron |
|-----------|---------|----------|
| Binary size | ~5-10 MB | ~150 MB |
| Memory usage | ~50-100 MB baseline | ~200-400 MB baseline |
| Backend language | Rust (native) | JavaScript (Node.js) |
| Security model | Capability-based permissions | Full Node.js access by default |
| IPC performance | Native (Rust ↔ JS via Serde) | Serialized over process boundary |
| Ecosystem maturity | Growing, fewer plugins | Very mature, extensive plugins |
| Learning curve | Moderate (Rust + Tauri concepts) | Low (JavaScript everywhere) |

### Why Rust Matters

The backend of Investment OS is not a thin proxy to cloud services. It runs:

- An agent runtime with async task execution.
- A local SQLite database with migrations.
- Document parsing and processing.
- Credential management via OS keychain.
- Sandboxed artifact windows.

Rust's type system, ownership model, and async runtime (Tokio) are well-suited for this workload. Node.js would require more defensive coding around concurrency, error handling, and resource management.

### Why Binary Size Matters

Investment OS is a focused research tool. A 150 MB download for what is essentially a single-purpose application is difficult to justify. Tauri's 5-10 MB binary respects the user's disk and bandwidth.

### Why Security Matters

Electron apps have full Node.js access by default. This creates a large attack surface — any dependency vulnerability can compromise the application. Tauri's capability-based permission model means each window receives only the permissions it needs. Artifact windows, in particular, run with `core:default` only — no filesystem, no shell, no network.

## Trade-offs Accepted

- **Smaller ecosystem.** Fewer community plugins, less documentation, fewer Stack Overflow answers. Mitigated by Tauri's active development and the ability to write custom Rust plugins.
- **Rust onboarding.** Contributors need Rust knowledge. Mitigated by clear documentation and a well-structured codebase.
- **WebView dependency.** Tauri uses the system WebView (WebKit on macOS, WebView2 on Windows). This means rendering behavior may vary slightly across platforms. Mitigated by testing on all target platforms.

## Consequences

- **Positive:** Small, fast, secure desktop application with a native Rust backend.
- **Negative:** Smaller ecosystem requires more custom implementation. Rust knowledge required.
- **Neutral:** System WebView dependency — consistent but platform-specific rendering.
