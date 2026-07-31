# Security

## Principles

1. **Least privilege.** Every component receives only the permissions it needs — nothing more.
2. **Defense in depth.** Multiple isolation layers: Tauri capabilities, window-level permissions, sandboxed artifact windows.
3. **Credentials never in plaintext.** API keys live in the OS keychain, never in files, env vars, or React state.
4. **Untrusted by default.** All external input — agent output, plugin manifests, remote JSON, file paths — is treated as hostile until validated.
5. **Auditability.** Permission grants, plugin registrations, and credential access are logged.

## Credential Storage

- API keys (OpenAI, market data providers) are stored in the operating system keychain:
  - macOS: Keychain
  - Windows: Credential Manager
  - Linux: libsecret
- Keys are never:
  - Stored in plaintext files or `.env`.
  - Passed to React components.
  - Included in logs or error messages.
  - Committed to version control.
- React receives only a status indicator: "OpenAI: configured" / "OpenAI: not configured".

## Window-Level Permissions

| Window | Tauri Capabilities |
|--------|--------------------|
| **Main** | `core:default`, `opener:default`, `shell:allow-open`, `store:default` |
| **Artifact** | `core:default` only |

Artifact windows cannot:
- Access the filesystem.
- Open URLs or execute shell commands.
- Access the Tauri store (key-value persistence).
- Make HTTP requests.
- Access the main window's state or DOM.

## Plugin Trust Boundaries

Plugins are internal-only in the MVP. Even so, every plugin is untrusted:

- Plugin manifests are validated against a schema.
- Plugin input is validated before rendering.
- Plugins cannot access Tauri commands beyond `core:default`.
- Plugin errors are isolated — a plugin crash cannot take down the main application.
- Plugin code is statically reviewed before inclusion (no dynamic code loading).

## File Access Rules

- The application may read and write only within its designated app data directory.
- User file access (opening documents, importing CSVs) uses OS-native file dialogs — the application never has unrestricted filesystem access.
- All file paths are normalized and validated to prevent directory traversal attacks.
- Artifact windows have no filesystem access whatsoever.

## Network Permissions

- The main window may make network requests through the Tauri HTTP plugin.
- External URLs are validated against a configurable allowlist before requests are dispatched.
- Artifact windows have no network access.
- Agent tasks may fetch web content through the Rust backend, which enforces the allowlist and rate limiting.

## Agent Output Safety

- Agent output is validated against Zod schemas before reaching the UI.
- Agent output is never rendered as raw HTML in the main window.
- Agent-generated artifacts are validated against their input schema before rendering.
- The agent cannot generate arbitrary code for execution.

## Input Validation

All external data is validated at the boundary:

- **TypeScript (frontend):** Zod schemas validate Tauri command responses and user input.
- **Rust (backend):** Serde with explicit validation validates API responses, file contents, and plugin manifests.
- **Agent output:** Validated against structured output schemas before being treated as data.

## Logging and Telemetry

- Structured logs via `tracing` — never `println!` or `console.log` for production paths.
- Logs must redact:
  - API keys and access tokens.
  - Cookies and session identifiers.
  - Sensitive user data (portfolio values, positions).
  - Internal file paths beyond the app data directory.
- No telemetry or analytics are sent externally unless explicitly enabled by the user.

## Development Safeguards

- `AppError` enforces typed, recoverable error handling — no `unwrap()` or `expect()` in production paths.
- `.env` files are in `.gitignore` and never committed.
- Pre-commit hooks (future) will scan for secrets, hardcoded credentials, and `unwrap()` calls.
- CI will run `cargo clippy` with `-D warnings` and security-focused lint rules.
