# Managed Agent Worker Subprocess Architecture

## 1. Purpose

This document defines the target runtime architecture for AlphaForge Agent
workloads. It separates untrusted or failure-prone Agent execution from the
privileged desktop backend while keeping product state and authority in Rust.

The subprocess is a worker, not a database service, credential service, plugin
host with unrestricted capabilities, or autonomous investment decision-maker.

## 2. Scope

Use a managed worker for workloads that meet any of these conditions:

- multi-turn reasoning or tool loops
- long-running or cancellation-sensitive execution
- third-party Agent runtimes such as Goose
- native libraries with a meaningful crash or memory-risk profile
- workload-specific dependencies that should not be linked into Tauri
- per-run CPU, memory, output, or process-tree enforcement

An ordinary bounded provider request used by a non-Agent service may remain in
the Rust host. It must still use provider adapters, budgets, timeouts, structured
output, and redacted errors.

## 3. Target Topology

```text
React UI
  -> desktopApi (typed + Zod)
  -> Tauri command
  -> AgentService / TaskExecutor
  -> AgentOrchestrator (trusted Rust host)
       |-- TaskRepository / EventRepository -> SQLite
       |-- CredentialService -> OS keychain
       |-- ProviderBroker -> approved external provider
       |-- ToolBroker -> allowlisted domain services
       |-- ArtifactService -> validated renderer payload
       `-- WorkerSupervisor
             -> anonymous stdin/stdout pipes
             -> ephemeral alphaforge-agent-worker
                  - reasoning loop
                  - protocol state machine
                  - no SQLite handle
                  - no plaintext provider key
                  - no arbitrary filesystem tool
                  - no domain write authority
```

Goose is another `WorkerBackend` implementation supervised directly by the host.
The native worker must not launch Goose, and Goose must not launch the native
worker. This prevents nested, untracked process trees.

## 4. Component Responsibilities

### 4.1 AgentOrchestrator

- validates the requested Agent profile and task scope
- creates the persisted run record before process creation
- resolves the worker backend from an allowlisted registry
- reserves concurrency, token, cost, and time budgets
- starts the supervisor and maps worker events into Agent task events
- validates the final structured result
- sends approved results to Artifact or proposal services
- records terminal state and recoverability

### 4.2 WorkerSupervisor

- resolves only packaged, registered worker binaries
- verifies worker identity, version, and integrity metadata
- creates a task-owned temporary directory
- starts the child without a shell and with a sanitized environment
- owns stdin, stdout, stderr, process-group/job handles, and cancellation
- enforces startup, idle, task, shutdown, and output deadlines
- bounds frame size, total bytes, stderr, and in-flight requests
- terminates and reaps the complete process tree
- emits diagnostics using stable error codes and redacted context

### 4.3 ProviderBroker

- owns provider selection and capability validation
- reads credentials from the OS keychain inside the Rust host
- performs network requests with endpoint allowlists and timeouts
- normalizes token usage, reasoning tokens, latency, and estimated cost
- enforces run budgets before and after every request
- returns only the provider response fields required by the worker

The worker never receives a raw API key. Local providers that require a loopback
endpoint must still pass endpoint policy and cannot be selected by arbitrary URL.

### 4.4 ToolBroker

- exposes a versioned allowlist of typed tools
- attaches authoritative `workspace_id`, `task_id`, and entity scope in Rust
- rejects worker-supplied attempts to widen scope
- validates inputs and outputs and applies pagination/size limits
- routes reads through domain services, never direct SQL
- returns provenance with every research datum
- represents mutations as proposals requiring explicit user acceptance

### 4.5 WorkerBackend

The backend abstraction hides runtime-specific launch details while preserving a
common policy:

```rust
pub trait WorkerBackend {
    fn manifest(&self) -> &WorkerManifest;
    fn build_launch_spec(&self, run: &AuthorizedRun) -> Result<LaunchSpec, WorkerError>;
    fn validate_result(&self, value: serde_json::Value) -> Result<AgentResult, WorkerError>;
}
```

Implementations may include:

- `NativeAgentWorkerBackend`
- `GooseWorkerBackend`
- a deterministic fixture backend used only in tests

No implementation may bypass the Supervisor, ProviderBroker, ToolBroker, or
result validation.

## 5. Worker Protocol v1

### 5.1 Transport

- anonymous child stdin/stdout pipes
- newline-delimited UTF-8 JSON
- stdout reserved exclusively for protocol frames
- stderr reserved for bounded diagnostics
- no loopback port and no globally discoverable named pipe in v1
- default maximum frame: 1 MiB
- default maximum aggregate worker output: 16 MiB per run
- configurable values have hard application ceilings

### 5.2 Envelope

```json
{
  "protocolVersion": 1,
  "runId": "uuid",
  "messageId": "uuid",
  "replyTo": null,
  "type": "worker.ready",
  "payload": {}
}
```

Unknown protocol versions, message types, duplicate IDs, invalid reply chains,
oversized frames, and messages for another run fail closed.

### 5.3 Startup Handshake

```text
Host spawns worker
  -> worker.hello {workerId, workerVersion, protocolVersions}
  -> host.configure {selectedVersion, runScope, limits, capabilities, nonce}
  -> worker.ready {nonceProof, supportedFeatures}
  -> host.start {taskInputRef, outputSchemaId}
```

The nonce is written through stdin after process creation; it is never placed in
arguments or environment variables. It prevents accidental attachment to the
wrong process but is not treated as a replacement for OS process ownership.

### 5.4 Runtime Messages

Worker to host:

- `run.progress`
- `run.waitingForInput`
- `provider.request`
- `tool.request`
- `proposal.created`
- `run.result`
- `run.failure`
- `worker.heartbeat`

Host to worker:

- `provider.response`
- `tool.response`
- `input.response`
- `budget.updated`
- `run.cancel`
- `worker.shutdown`

Every request has one correlated response or an explicit timeout/cancellation.
The host caps outstanding requests to prevent memory exhaustion.

### 5.5 Structured Results

A terminal result contains a schema identifier and version. The host validates
it against a Rust-owned schema before task completion. Recommended research
fields include:

- summary
- claims
- evidence and source IDs
- contradictions
- risks and unknowns
- portfolio impact statements that are descriptive, not trade instructions
- confidence
- provider/model/run provenance
- usage metrics

Free-form Markdown can be an optional presentation field but cannot be the only
machine-readable result.

## 6. Lifecycle and State Mapping

```text
created -> queued -> running -> completed
                       |  |-> waiting_for_input -> running
                       |  |-> failed
                       `----> cancelled
```

Process details do not add UI task states in v1. Worker startup, handshake,
provider calls, tool calls, graceful shutdown, and forced termination are stored
as typed run events and diagnostics.

### 6.1 Start

1. Validate task and Agent profile.
2. Persist the run and queue event.
3. Reserve concurrency and budgets.
4. Create a private task directory.
5. Resolve and verify the worker binary.
6. Spawn and attach process-tree controls.
7. Complete the handshake within the startup timeout.
8. Mark the task running and send `host.start`.

### 6.2 Cancellation

1. Mark cancellation requested and stop accepting new broker calls.
2. Send `run.cancel` and wait for a short grace period.
3. Terminate the complete process tree if it does not exit.
4. Drain bounded diagnostics, wait/reap the process, and delete task temp data.
5. Persist `cancelled` exactly once and emit the Tauri event.

### 6.3 Crash and Restart Recovery

- A worker exit without a valid terminal frame becomes a recoverable
  `AGENT_WORKER_EXITED` failure.
- A protocol violation becomes `AGENT_WORKER_PROTOCOL_ERROR` and is not retried
  automatically.
- A host restart never attaches to a PID from a previous session.
- Runs left in `running` are reconciled at startup and become a recoverable
  interrupted failure or return to `queued` only when their retry policy permits.
- Retrying creates a new run attempt linked to the original task; it does not
  overwrite the prior attempt's audit events.

## 7. Security Model

Process isolation reduces blast radius but is not a complete sandbox. Security
depends on layered controls.

### 7.1 Mandatory Controls

- fixed, packaged executable path
- direct spawn; no shell
- sanitized environment with a minimal allowlist
- private task working directory with no user-controlled path
- closed or explicitly piped standard handles
- worker network denied where platform packaging supports it
- no raw secrets, SQLite handles, or arbitrary file paths
- host-brokered providers and tools
- workspace/task scope attached by Rust
- frame, output, log, time, turn, token, cost, CPU, and memory limits
- process-tree cleanup and child reaping
- worker and recipe integrity verification
- structured output validation and human-reviewed mutations
- stable redacted logs with `run_id`, not prompts or credentials

### 7.2 Platform Controls

| Platform | Minimum process control | Hardened release target |
|---|---|---|
| Windows | Job Object, kill-on-close, no breakaway | CPU/memory limits and restricted token/AppContainer feasibility review |
| Linux | process group, parent-death signal, rlimits | seccomp/Landlock or distribution sandbox profile |
| macOS | signed bundled helper, inherited app container | App Sandbox inheritance or XPC after packaging review |

Platform hardening is a release gate. Unsupported guarantees must be documented;
the application must not claim a sandbox that it does not enforce.

## 8. Data and Persistence

Recommended append-only schema additions, subject to a migration design review:

- `agent_runs`: task, attempt, backend, worker/protocol version, status, budgets,
  timestamps, terminal error code
- `agent_run_events`: ordered typed events with bounded JSON payloads
- `agent_usage_events`: provider/model token and estimated-cost metrics
- `agent_proposals`: validated proposal, review state, reviewer timestamp, result ID

Do not persist:

- provider secrets
- full inherited environment
- raw unbounded stderr
- arbitrary worker paths
- hidden chain-of-thought or provider-internal reasoning text

## 9. Observability

Each run uses a correlation ID across task events, broker requests, worker logs,
provider usage, and artifacts. Diagnostics include:

- backend and worker version
- protocol version
- lifecycle timings
- exit code or signal mapped to a stable error code
- current budget counters
- bytes and frames exchanged
- graceful versus forced shutdown

Prompts, document bodies, credentials, cookies, local absolute paths, and raw
provider errors are redacted by default.

## 10. Performance Strategy

Start with ephemeral workers. Measure:

- p50/p95 startup and handshake latency
- first-progress latency
- steady-state CPU and memory
- protocol throughput and backpressure
- cancellation-to-exit time
- orphan-process rate

Consider a warm pool only if p95 startup materially harms the user experience.
A pool design requires worker reset proof, cross-workspace data-leak tests, idle
resource limits, crash replacement, and the same version/integrity gates.

## 11. Compatibility and Rollout

- Protocol versions are negotiated during handshake.
- Host and worker compatibility is declared in the worker manifest.
- A worker update requires contract, security, packaging, and rollback tests.
- The existing in-process provider path remains behind a feature flag during
  migration, then becomes a controlled fallback only if policy permits.
- Goose retains its specialized recipes and MCP rules but adopts the common
  supervisor lifecycle and diagnostics where practical.

## 12. References

- [ADR-0010](../DECISIONS/0010-managed-agent-worker-subprocess.md)
- [Agent Protocol](../AGENT_PROTOCOL.md)
- [Goose topology ADR](../DECISIONS/0004-goose-integration-topology.md)
- [Tauri sidecars](https://v2.tauri.app/develop/sidecar/)
- [Tauri shell permissions](https://v2.tauri.app/plugin/shell/)
- [Tokio process lifecycle](https://docs.rs/tokio/latest/tokio/process/)
- [Windows Job Objects](https://learn.microsoft.com/windows/win32/procthread/job-objects)
- [Apple App Sandbox](https://developer.apple.com/documentation/security/app-sandbox)
