# ADR-0010: Managed Subprocesses for Agent Execution

## Status

Accepted on 2026-08-25. Implementation is planned.

## Context

AlphaForge currently executes the primary research provider path in background
Tokio tasks inside the Tauri Rust process. Goose uses a separate supervised
sidecar adapter. As Agent capabilities grow to include longer reasoning loops,
multiple providers, document processing, tool calls, and third-party runtimes,
running every workload in the privileged desktop process would increase failure
coupling and make resource enforcement more difficult.

The architecture must preserve the following constraints:

- React has no process or shell authority.
- Agent code has no direct SQLite, keychain, unrestricted filesystem, or domain
  write access.
- Cancellation, timeouts, concurrency, token budgets, cost budgets, and audit
  events remain authoritative in Rust.
- Agent output is untrusted and schema-validated before rendering or persistence.
- No Agent may autonomously make or execute a real investment decision.

## Options Considered

### A. Keep all Agent execution in the Tauri process

This has the lowest IPC overhead and simplest packaging, but a runtime crash,
deadlock, memory leak, or unsafe dependency can affect the entire desktop app.
It also provides a weak resource-isolation boundary.

### B. Run a persistent local Agent service

A loopback HTTP or gRPC service can be reused across tasks, but creates a
long-lived local attack surface, requires transport authentication, complicates
startup and shutdown, and consumes resources while idle.

### C. Run managed Agent worker subprocesses

Rust starts a version-pinned worker directly, supervises its complete process
tree, exchanges bounded typed messages over inherited pipes, and terminates the
worker on completion, cancellation, timeout, protocol failure, or application
shutdown.

## Decision

Select **Option C: managed Agent worker subprocesses**.

The initial production topology is one ephemeral worker per Agent run. A warm
pool may be considered later only after measurements prove startup latency is a
material problem and the same security and cleanup guarantees are retained.

The subprocess boundary applies to long-running, tool-using, or third-party
Agent workflows. Small non-Agent provider operations may remain in-process when
they do not execute an Agent loop and the owning service documents that choice.

## Required Boundaries

1. Rust launches an approved packaged binary by fixed path without a shell.
2. User input never becomes an executable path, command line, shell fragment, or
   environment-variable name.
3. The worker receives no provider secret. Provider calls are brokered by the
   Rust host, which reads credentials from the OS keychain.
4. The worker receives no database handle or arbitrary file path. Tool calls are
   brokered through typed, allowlisted Rust services with task and workspace
   scope attached by the host.
5. The frontend never receives Tauri shell permissions and cannot spawn, kill,
   or write to a subprocess directly.
6. Standard input/output carry only the versioned worker protocol. Standard
   error is bounded, redacted diagnostic output and never a protocol channel.
7. The host validates every protocol message, output schema, identifier, byte
   limit, budget, and state transition.
8. The process tree is terminated on cancellation, timeout, host shutdown, or
   supervisor loss. Killing only the immediate child is insufficient.
9. Domain mutations remain proposals until the user explicitly accepts them;
   the host revalidates accepted data and calls normal domain services.
10. The worker binary and built-in recipes are version-pinned, integrity-checked,
    included in the SBOM, and never downloaded automatically at runtime.

## Protocol Decision

Version 1 uses newline-delimited JSON over anonymous stdin/stdout pipes:

- one JSON object per line
- UTF-8 only
- maximum frame size enforced before deserialization
- `protocolVersion`, `runId`, `messageId`, `type`, and typed `payload`
- request/response correlation for provider and tool calls
- large content passed by bounded opaque host references rather than raw frames

The protocol types live in a shared Rust crate and use Serde with explicit
validation. A transport change to length-prefixed CBOR or another encoding would
require a new protocol version; it must not change the security model.

## Process Ownership

The Tauri Rust host owns the supervisor. Tauri's JavaScript shell API is not used.
The packaged-worker mechanism may use Tauri sidecar/resource resolution, but
process creation is performed only by trusted Rust code.

Platform requirements:

- **Windows**: assign the worker to a Job Object with kill-on-close semantics and
  no breakaway, then apply memory/CPU limits where supported.
- **Linux**: use a dedicated process group, parent-death handling where available,
  resource limits, and a release-specific sandbox profile.
- **macOS**: use a signed helper with App Sandbox inheritance or an approved XPC
  design when release constraints require stronger privilege separation.

Tokio process handles must use explicit wait/reap behavior and kill-on-drop as a
last-resort safety net. Kill-on-drop alone does not guarantee complete descendant
cleanup on every platform.

## Consequences

### Benefits

- Agent crashes and most leaks do not crash the desktop application.
- Cancellation and resource limits can terminate the complete workload.
- Third-party runtimes are hidden behind one typed adapter boundary.
- Provider credentials and domain persistence remain in the trusted host.
- Worker updates and compatibility can be reviewed and tested independently.

### Costs

- Additional packaging and platform-specific process-management work.
- A versioned bidirectional protocol and compatibility test suite are required.
- Streaming and provider/tool brokering introduce IPC overhead.
- Debugging spans host and worker logs, so correlation IDs and diagnostics are
  mandatory.

## Rejected Shortcuts

- Exposing `@tauri-apps/plugin-shell` to React.
- Starting workers through `cmd.exe`, PowerShell, `/bin/sh`, or command strings.
- Passing API keys in arguments, environment variables, recipes, or temporary
  plaintext files.
- Accepting a user-supplied worker executable path in production.
- Giving a worker direct SQLite access for performance.
- Treating process isolation as a complete security sandbox.
- Starting with a persistent worker pool before lifecycle correctness is proven.

## References

- [Detailed subprocess architecture](../agent/SUBPROCESS_ARCHITECTURE.md)
- [Implementation roadmap](../agent/SUBPROCESS_ROADMAP.md)
- [Tauri: Embedding External Binaries](https://v2.tauri.app/develop/sidecar/)
- [Tauri Shell Permissions](https://v2.tauri.app/plugin/shell/)
- [Tokio process documentation](https://docs.rs/tokio/latest/tokio/process/)
- [Microsoft: Job Objects](https://learn.microsoft.com/windows/win32/procthread/job-objects)
- [Apple: Enabling App Sandbox](https://developer.apple.com/documentation/security/app-sandbox)
