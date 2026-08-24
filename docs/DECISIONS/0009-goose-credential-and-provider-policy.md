# ADR-0009: Goose Credential Isolation and Provider Policy

## Status

Accepted

## Context

Milestone M10 integrates Goose as a supervised subprocess to assist with read-only investment research analysis. Because external LLM providers (e.g. OpenAI, Anthropic, Ollama) require API credentials, we must establish an unambiguous credential storage and provider governance policy that satisfies AlphaForge's local-first and security-by-design invariants.

### Evaluated Credential Models

1. **Option 1: Goose OS-Keyring Integration (Selected)**
   - Goose reads credentials directly from the host operating system's credential vault (Windows Credential Manager, macOS Keychain, Linux Secret Service / Keyutils) under dedicated namespace `alphaforge-goose`.
   - AlphaForge never receives, prints, or serializes plaintext secrets in command-line arguments, process environments, IPC messages, application logs, recipes, or SQLite tables.
   - React UI never receives raw API keys.

2. **Option 2: Plaintext File / Environment Variable Injection (Rejected)**
   - Passing API keys via `.env` files, config files, or environment variables.
   - **Rejected** because environment variables can be dumped by child processes, logged on crash, or exposed via process inspectors.

3. **Option 3: Full AlphaForge Provider Gateway (Deferred)**
   - Rust hosts a full local proxy server for all LLM HTTP traffic.
   - **Deferred** due to high implementation complexity for MVP; retained as a potential future architecture for enterprise deployments.

---

## Decision

We adopt **Option 1 (OS Keyring Isolation)** with strict provider allowlisting:

1. **Keyring Service Binding**:
   - Keys are stored in the platform-native secure credential store under service name `alphaforge-goose`.
   - Plaintext file fallback is strictly disabled in production builds.

2. **Zero Plaintext Invariant**:
   - Stderr, stdout, and error traces from the Goose sidecar undergo mandatory sanitization via `redact_sensitive` before logging or emitting to the frontend.
   - No API keys are persisted in SQLite, task logs, or research artifacts.

3. **Provider & Model Allowlist**:
   - Only allowlisted providers are supported: `openai`, `anthropic`, `ollama`, `demo`.
   - Model selection is bounded by approved families to prevent unexpected costs or data leakage to untrusted third parties.

4. **Execution Budgets**:
   - Every execution enforces a hard maximum timeout (default 300s), token ceiling (default 100,000 tokens), and cost budget (default \$1.00).

5. **Data Retention & Telemetry**:
   - Zero telemetry: AlphaForge sends no telemetry or analytics.
   - Research prompts sent to external LLMs contain only explicitly allowlisted research excerpts and theses selected by the user.

---

## Consequences

### Positive
- Secrets cannot be extracted from application SQLite files or process memory dumps.
- Strict allowlist prevents unauthorized provider usage.
- Complies with local-first security architecture.

### Negative
- Users must configure their LLM API keys via the OS Keyring or AlphaForge Settings dialog (which writes directly to the secure keyring via Tauri credential commands).
