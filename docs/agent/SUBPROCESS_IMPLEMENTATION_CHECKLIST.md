# Agent Worker Subprocess Implementation Checklist

This checklist translates the accepted architecture into reviewable work packages.
Check an item only when its implementation, tests, and documentation are merged.

## 1. Architecture and Contracts

- [x] Confirm workload routing policy: Agent worker versus bounded in-process call.
- [x] Freeze worker protocol v1 and publish golden fixtures.
- [x] Define stable worker/supervisor/broker error codes.
- [x] Define hard ceilings for frames, output, stderr, turns, time, tokens, cost,
      concurrency, CPU, and memory.
- [x] Approve the append-only persistence migration design.
- [x] Update the Agent threat model for generic workers and write proposals.
- [x] Define feature flag, kill switch, fallback, and rollback behavior.

## 2. Proposed Rust Modules

- [x] Create `crates/agent-protocol/` for shared Serde contracts and validation.
- [x] Create `crates/agent-worker/` or an equivalent dedicated worker binary crate.
- [x] Add `apps/desktop/src-tauri/src/agent/orchestrator.rs`.
- [x] Add `crates/agent-core/src/supervisor.rs`.
- [x] Add `crates/agent-core/src/manifest.rs` (WorkerRegistry).
- [x] Add `crates/agent-core/src/broker/provider_broker.rs`.
- [x] Add `crates/agent-core/src/broker/tool_broker.rs`.
- [ ] Add `apps/desktop/src-tauri/src/agent/recovery.rs`.
- [ ] Replace the placeholder sandbox module with enforced platform adapters.
- [x] Keep Tauri commands thin: validate, call service, map result.

## 3. Worker Protocol

- [x] Implement bounded line reader before JSON deserialization.
- [x] Implement strict envelope and payload validation.
- [x] Implement hello/configure/ready/start handshake and timeout.
- [x] Implement progress, heartbeat, waiting-for-input, and terminal messages.
- [x] Implement correlated provider and tool request/response messages.
- [x] Reject unknown versions/types, duplicate IDs, invalid replies, and cross-run IDs.
- [x] Implement stdout protocol exclusivity and bounded redacted stderr capture.
- [x] Add backpressure and a cap on in-flight broker requests.
- [x] Add protocol property tests and parser fuzz targets.

## 4. Process Lifecycle

- [x] Resolve only packaged allowlisted workers.
- [x] Verify worker manifest, version compatibility, and integrity metadata.
- [x] Spawn directly without a shell or user-controlled executable path.
- [x] Sanitize environment and close unrelated inherited handles.
- [x] Create and validate a task-owned private working directory.
- [x] Enforce startup, idle, run, broker-call, graceful-exit, and hard-kill timeouts.
- [x] Terminate and reap the entire process tree on every terminal path.
- [x] Clean temporary data after success, failure, cancellation, and host restart.
- [x] Prove no orphan processes with platform integration tests.

## 5. Platform Controls

### Windows

- [ ] Assign workers to a Job Object before meaningful execution.
- [ ] Enable kill-on-job-close and disallow breakaway.
- [ ] Apply and test memory/CPU limits.
- [ ] Document restricted token/AppContainer feasibility and final guarantee.

### Linux

- [ ] Use a dedicated process group and parent-death behavior.
- [ ] Apply and test resource limits.
- [ ] Define seccomp/Landlock or packaging-specific sandbox policy.

### macOS

- [ ] Bundle and sign the worker helper.
- [ ] Validate App Sandbox inheritance requirements.
- [ ] Decide whether hardened releases require XPC for stronger separation.

## 6. Provider Broker

- [x] Keep all provider keys in the Rust host and OS keychain.
- [x] Enforce typed provider/model capabilities and endpoint policy.
- [x] Add request timeout, cancellation, retry classification, and concurrency limits.
- [x] Normalize input, cached, output, reasoning, and total tokens.
- [x] Capture latency, request status, price version, and estimated cost.
- [x] Record `unknown` when the provider does not report usage.
- [x] Redact provider errors before worker, UI, persistence, or logs receive them.
- [x] Test OpenAI, DeepSeek, local provider, failure, and cancellation adapters.

## 7. Tool Broker

- [x] Register tools through a deny-by-default typed registry.
- [x] Attach authoritative workspace/task/entity scope in Rust.
- [x] Start with read-only research and provenance tools.
- [x] Use IDs and bounded opaque references instead of arbitrary paths.
- [x] Validate and size-limit every tool request and response.
- [x] Reject SQL, shell, credential, arbitrary URL, and trading capabilities.
- [x] Convert every mutation request into a user-reviewed proposal.
- [x] Add prompt-injection, traversal, cross-workspace, oversized, and unknown-tool tests.

## 8. Persistence and Recovery

- [ ] Add append-only `agent_runs` migration if approved.
- [ ] Add append-only ordered run-event storage if approved.
- [ ] Add usage-event storage and aggregation if approved.
- [ ] Add proposal review and resulting-entity linkage if approved.
- [ ] Separate database rows from domain models.
- [ ] Add repository/service/command/schema tests for every persisted field.
- [ ] Make terminal transitions idempotent and monotonic.
- [ ] Preserve prior attempt history when retrying.
- [ ] Never persist secrets, raw environment, chain-of-thought, or unbounded logs.

## 9. Frontend and IPC

- [ ] Route all commands through `desktopApi` with Zod validation.
- [ ] Preserve initial/loading/running/waiting/completed/failed/cancelled states.
- [ ] Add worker startup, recovery, budget, and diagnostic explanations.
- [ ] Add explicit data scope and tool scope review before a run.
- [ ] Add field-level proposal acceptance/rejection UI.
- [ ] Add API usage charts and request details from real normalized metrics.
- [ ] Keep React free of shell, filesystem, keychain, and worker protocol access.
- [ ] Add component, hook, schema, empty, offline, and error tests.

## 10. Goose Convergence

- [ ] Map Goose lifecycle onto the common supervisor without weakening its policy.
- [ ] Retain pinned binary, recipe, extension allowlist, and output schema checks.
- [ ] Retain Goose credential/provider and MCP threat-model controls.
- [ ] Run the common no-orphan, timeout, cancellation, and diagnostics suite.
- [ ] Update Goose status documents to match the actual implementation evidence.

## 11. Packaging and Operations

- [ ] Add worker binaries to platform package configuration.
- [ ] Pin source, version, hashes/signatures, license, and SBOM data.
- [ ] Add worker/host compatibility matrix and update procedure.
- [ ] Add packaged spawn/handshake/cancel/shutdown smoke tests.
- [ ] Add non-sensitive diagnostics and a local kill switch.
- [ ] Document upgrade, rollback, missing/corrupt worker, and support procedures.
- [ ] Verify no runtime auto-download or user-supplied production worker path exists.

## 12. Final Acceptance

- [ ] Frontend typecheck, lint, and tests pass.
- [ ] Rust formatting, Clippy, workspace tests, and protocol fuzz suite pass.
- [ ] IPC registration parity passes.
- [ ] Desktop E2E critical flow passes.
- [ ] Packaged builds pass on every declared platform.
- [ ] Security review signs off on process, protocol, broker, and proposal boundaries.
- [ ] Performance targets are met or documented with an accepted follow-up.
- [ ] Rollback is tested.
- [ ] Documentation and release notes are complete in the same pull request.
