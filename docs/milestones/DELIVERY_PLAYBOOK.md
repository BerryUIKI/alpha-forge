# Milestone Delivery Playbook

This playbook turns [MILESTONE_ROADMAP.md](../MILESTONE_ROADMAP.md) into an execution contract for development agents. Workstream-specific plans may add stricter requirements but cannot weaken repository rules.

## 1. Confirm the milestone is active

Before implementation:

1. Read the milestone overview, entry gate, supporting documents, deliverables, and acceptance criteria.
2. Confirm prerequisite milestones are complete and the milestone is explicitly active.
3. Resolve product-owner decisions that change scope, providers, credentials, permissions, data, or release behavior.
4. Reinspect current `dev`; plans describe intent, while the repository is the current implementation baseline.
5. Record drift between the plan and code before editing.

Planning documentation does not activate a milestone. Candidate code on another branch does not mark a deliverable complete.

## 2. Select one vertical slice

Choose the smallest work package that completes a user-observable loop:

```text
validated input
  -> Rust service or pure domain operation
  -> persistence/background execution as needed
  -> typed IPC and frontend validation
  -> loading/success/empty/error/partial/offline UI
  -> tests and documentation
```

Do not create only interfaces, placeholder modules, or a broad refactor. Reuse current types, repositories, services, commands, hooks, components, errors, and test utilities.

## 3. State the impact before editing

Every work package identifies impact on:

| Area              | Questions                                                                     |
| ----------------- | ----------------------------------------------------------------------------- |
| Frontend          | Routes, components, state, schemas, accessibility, locales, async states?     |
| Rust              | Domain, service, provider, task runtime, cancellation, errors, logs?          |
| Database          | Append-only migration, rows, repositories, upgrade paths, isolation?          |
| Tauri             | Commands, windows, capabilities, lifecycle, packaging?                        |
| Artifacts/plugins | Schema, renderer, permissions, untrusted output?                              |
| Tests             | Unit, repository, service, command, component, E2E, migration, package smoke? |
| Documentation     | Milestone evidence, contracts, decisions, user/operator docs?                 |

Any new privileged operation documents why it is needed, its recipient, accepted input, validation, and abuse prevention before implementation.

## 4. Prepare Git safely

- Follow [GIT_WORKFLOW.md](../GIT_WORKFLOW.md).
- Never implement directly on `main` or `dev`.
- Start from current `dev` unless the milestone explicitly defines an approved staging branch.
- Preserve unrelated and uncommitted user changes.
- Keep one milestone slice per PR.
- Never edit applied migrations or force-push shared history.
- Commit, push, merge, or delete branches only when explicitly authorized.

## 5. Implement by contract

For each slice, define before coding:

- Input and output schema.
- Domain invariants and stable error codes.
- Persistence ownership and transaction boundary.
- Task lifecycle, timeout, cancellation, retry, concurrency, token, and cost limits.
- Provenance and uncertainty fields.
- Permission and credential boundaries.
- UI states and keyboard/accessibility behavior.
- Test fixtures and acceptance scenario.

External and Agent output is `unknown` until validated. Rust owns SQLite, network, filesystem, credentials, and background processes. React owns presentation and calls only the unified desktop API.

## 6. Verify in layers

Run focused tests while developing, then the repository gates appropriate to the final scope:

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Also run `pnpm test:e2e` for critical product flows and `pnpm tauri build` for packaging, capability, sidecar, or release changes. Benchmarks and cross-platform smoke tests run where the workstream plan requires them.

Never report a command as passed without running it. If a command cannot run, record the exact blocker and residual risk.

## 7. Record acceptance evidence

The implementation PR contains an evidence table:

| Gate          | Evidence                                                   |
| ------------- | ---------------------------------------------------------- |
| Scope         | Changed paths and explicitly excluded work                 |
| Behavior      | User flow demonstrated or tested                           |
| Data          | Migration/upgrade/restart evidence                         |
| Safety        | Validation, permissions, credential and redaction evidence |
| Quality       | Exact commands and observed outcomes                       |
| Documentation | Updated milestone/workstream links                         |
| Risk          | Known limitations, rollback or disable path                |

Checklists are assertions, not evidence. Link tests, logs, screenshots, benchmark output, or review records where practical.

## 8. Update milestone status

A work package becomes complete only when:

- Required code is accepted on `dev`.
- Required checks and reviews pass.
- Documentation matches the integrated behavior.
- Remaining risks are recorded.

Update the detailed plan first, then the milestone summary. Use these status meanings consistently:

| Status      | Meaning                                                |
| ----------- | ------------------------------------------------------ |
| Planned     | Scope exists; implementation is not active or accepted |
| In progress | An approved work package is being implemented          |
| Blocked     | A named dependency prevents progress                   |
| Complete    | Integrated on `dev` with acceptance evidence           |
| Deferred    | Explicitly outside the active milestone                |

## 9. Handoff

The final implementation report states:

1. What user-visible loop was completed.
2. Key files changed.
3. Verification actually run.
4. Remaining risks, deferred items, and next work package.

The next agent should be able to start from the milestone link without reconstructing decisions from branch history or chat.
