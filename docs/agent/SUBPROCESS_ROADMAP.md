# Agent Worker Subprocess Implementation Roadmap

## Status and Scope

**Status:** Planned
**Architecture decision:** Accepted in ADR-0010
**Implementation status:** Not started as a generic runtime

This workstream migrates long-running Agent execution from in-process provider
calls to Rust-supervised workers without changing the user-facing task lifecycle.
It does not authorize autonomous investment decisions, unrestricted tools, a
public plugin marketplace, cloud orchestration, or arbitrary local executables.

## Impact Assessment

| Area | Impact |
|---|---|
| Frontend | Mostly unchanged task states; richer startup, recovery, budget, and diagnostic presentation |
| Rust | Shared protocol, supervisor, broker services, backend registry, recovery, platform controls |
| Database | Append-only run/attempt/event/usage/proposal migrations if the current task tables are insufficient |
| Tauri | Packaged worker resolution; no shell capability for React |
| Providers | Host-owned provider broker and normalized usage |
| Artifacts | Validated terminal results and user-reviewed proposals |
| Goose | Migration onto the common supervisor contract without weakening Goose-specific policy |
| Tests | Protocol fixtures, lifecycle, process-tree, broker, security, recovery, packaging, and E2E |
| Documentation | ADR, architecture, protocol, threat model, operations, packaging, and release notes |

## Delivery Principles

- Implement the smallest complete vertical slice before supporting multiple
  worker backends.
- Use a deterministic fixture worker before a real model or tool loop.
- Keep the current in-process path available behind a development rollback flag
  until the worker path passes all acceptance gates.
- Do not ship a partial sandbox claim. Document platform guarantees precisely.
- Each phase ends with a green build, tests, and synchronous documentation.

## AW0: Baseline and Contract Freeze ✅

**Status:** Complete (2026-09-01)

**Goal:** establish measurements and freeze the v1 boundary.

- [x] Inventory current `TaskExecutor`, provider adapters, Goose adapter, task events,
  persistence, credentials, and packaging.
- [x] Record p50/p95 task startup, memory, cancellation, and provider latency targets.
- [x] Define which workloads require a worker and which bounded non-Agent operations
  may remain in-process.
- [x] Freeze protocol v1 envelopes, message types, error codes, limits, and schema
  ownership.
- [x] Reconcile generic worker and Goose documentation so status claims match code.
- [x] Produce the database migration design without editing released migrations.

**Exit gate:** architecture/security review approves the protocol, threat model,
compatibility policy, migration plan, and rollback mechanism.

## AW1: Shared Protocol and Deterministic Fixture Worker ✅

**Status:** Complete (2026-09-01)

**Goal:** prove bidirectional typed communication without real user data.

- [x] Add `crates/agent-protocol` with Serde message types, envelopes, and explicit validators.
- [x] Add a minimal `alphaforge-agent-worker` binary in dedicated `crates/agent-worker`.
- [x] Implement hello/configure/ready/start/progress/result/shutdown messages.
- [x] Add frame (1 MiB default) and aggregate-output limits (16 MiB default), unknown-message rejection, and protocol version negotiation.
- [x] Build a deterministic fixture mode that can complete, fail, hang, emit malformed frames, exceed limits, and request graceful cancellation.
- [x] Add golden protocol fixtures shared by host and worker tests.

**Exit gate:** protocol tests cover valid runs, version mismatch, malformed JSON,
oversized frames, duplicate IDs, cross-run IDs, EOF, and stderr noise.

## AW2: Cross-Platform Worker Supervisor ✅

**Status:** Complete (2026-09-01)

**Goal:** own the full worker lifecycle safely from Rust.

- [x] Add `WorkerSupervisor`, `WorkerRegistry`, `WorkerManifest`, and `LaunchSpec`.
- [x] Resolve only packaged allowlisted binaries and verify integrity metadata (SHA-256).
- [x] Spawn directly with sanitized environment, closed handles, private task directory,
  piped stdin/stdout/stderr, startup timeout, and kill-on-drop fallback.
- [x] Implement graceful cancel, forced process-tree termination, wait/reap, and cleanup.
- [x] Add SupervisorManager with concurrency limits and supervisor shutdown integration.
- [x] Capture bounded redacted stderr and stable diagnostics (`RunDiagnostics`).

**Exit gate:** fixture integration tests prove no orphan worker or descendant after
success, failure, malformed output, timeout, cancellation, and host shutdown on
every supported release platform.

## AW3: Provider and Tool Brokers

**Goal:** keep secrets and privileged capabilities in the Rust host.

- Add `ProviderBroker` over the existing provider adapters.
- Normalize provider capabilities, output, usage, cached/reasoning tokens, latency,
  estimated cost, and stable errors.
- Enforce provider/model allowlists, endpoint policy, request timeout, and budgets.
- Add `ToolBroker` with a deny-by-default registry and typed schemas.
- Attach authoritative task/workspace/entity scope in Rust.
- Start with read-only research tools and bounded opaque content references.
- Add request correlation, backpressure, per-call timeout, cancellation, redaction,
  and provenance.
- Reject generic SQL, shell, arbitrary file, arbitrary URL, credential, and trading
  tools.

**Exit gate:** malicious worker fixtures cannot read secrets, widen workspace
scope, escape path rules, call unknown tools, exceed budgets, or perform writes.

## AW4: Agent Task Vertical Slice

**Goal:** complete one real research run through the worker boundary.

```text
Create task -> queue -> spawn worker -> broker provider request
-> stream progress -> validate structured result -> persist run/events
-> render Artifact -> close/reopen result
```

- Add `AgentOrchestrator` and route one opt-in research profile through it.
- Persist run attempts and reconcile them at startup.
- Map worker messages onto the existing Agent task states and Tauri events.
- Add user input pause/resume with bounded persisted context.
- Validate terminal results and render through an existing controlled Artifact.
- Keep all domain mutations as explicit proposals.
- Add safe user-facing errors for missing worker, integrity failure, version mismatch,
  crash, timeout, protocol error, budget exhaustion, and broker failure.

**Exit gate:** the critical flow passes Rust integration, React component/hook, IPC
schema, restart-recovery, and desktop E2E tests with a deterministic provider.

## AW5: Persistence, Usage, and Human-Approved Proposals

**Goal:** make runs auditable and useful without granting write authority.

- Add append-only migrations for run attempts/events/usage/proposals as approved.
- Persist provider/model/version, source provenance, budgets, usage, and terminal
  error code.
- Add API Usage aggregation and request details from normalized broker metrics.
- Add proposal schemas for notes, evidence candidates, reports, and Artifacts.
- Require field-level user review; revalidate accepted proposals in Rust and call
  existing domain services.
- Explicitly prohibit trade, order, target-position, and autonomous portfolio
  mutation proposals.

**Exit gate:** every persisted change is linked to explicit user acceptance and
every usage figure is provider-reported or visibly marked unknown/estimated.

## AW6: Goose Convergence and Multi-Backend Configuration

**Goal:** reuse one lifecycle without erasing runtime-specific safeguards.

- Adapt Goose to the common `WorkerBackend` and supervisor interfaces where safe.
- Preserve pinned recipes, MCP allowlists, provider policy, structured schemas,
  credential rules, and packaging attribution.
- Add typed Agent profiles that select backend, provider, model, reasoning settings,
  limits, and allowed tool bundles.
- Validate OpenAI/DeepSeek/provider-specific fields through versioned adapters.
- Add connection tests that do not persist or expose secrets.
- Prevent user-supplied executable paths and arbitrary backend configuration.

**Exit gate:** native and Goose backends pass the same lifecycle, cancellation,
budget, diagnostics, and no-orphan contract suite.

## AW7: Platform Hardening, Packaging, and Rollout

**Goal:** ship the worker architecture safely and reversibly.

- Add platform worker artifacts, signatures/checksums, SBOM entries, license notices,
  compatibility matrix, and reproducible packaging steps.
- Complete Windows, macOS, and Linux process/sandbox controls for supported targets.
- Add packaged smoke tests for spawn, handshake, provider fixture, cancellation,
  shutdown, upgrade, and rollback.
- Add a local kill switch and diagnostics page without sensitive data.
- Roll out through developer fixture, internal opt-in, beta, then default-on stages.
- Remove the in-process Agent loop only after default-on acceptance and one stable
  rollback window.

**Exit gate:** signed packaged builds pass security review, upgrade/rollback tests,
and the release verification matrix on every declared platform.

## Verification Matrix

| Gate | Required evidence |
|---|---|
| Functional | Protocol, supervisor, broker, task, persistence, Artifact, and E2E tests |
| Safety | No shell/SQLite/key/unauthorized path or write capability; trade guardrails |
| Reliability | Crash, EOF, timeout, cancellation, restart, backpressure, and orphan tests |
| Security | Integrity, scope, redaction, injection, malformed frame, and process-tree tests |
| Performance | Startup, first-progress, CPU, memory, throughput, cancellation latency |
| Release | Signed worker, SBOM, license, platform matrix, packaged smoke, rollback |

Standard checks for every phase:

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
node scripts/check-ipc-registration.mjs
```

Packaging phases additionally run:

```bash
pnpm test:e2e
pnpm tauri build
```

## Definition of Done

- Long-running Agent tasks execute outside the privileged Tauri process.
- Rust remains the only owner of credentials, provider network access, SQLite,
  files, domain writes, policy, budgets, and audit state.
- React has no shell or subprocess permission.
- The complete worker process tree terminates reliably on every terminal path.
- Protocol and output are versioned, bounded, validated, and fuzz-tested.
- Runs recover safely after host or worker crashes.
- Provider usage and estimated cost are attributable and auditable.
- Domain writes require explicit user acceptance.
- Supported installers contain verified workers and pass packaged smoke tests.
- Documentation, threat models, support procedures, and rollback instructions are
  current in the same changeset.
