# AlphaForge Next Steps

**Updated:** 2026-09-20
**Active milestone:** M11 — Portfolio Hardening & Release Candidate
**Authoritative task queue:** [M11 Execution Plan](milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md)

Program status is governed by the [Milestone Roadmap](MILESTONE_ROADMAP.md). The [Delivery Playbook](milestones/DELIVERY_PLAYBOOK.md) defines execution and evidence rules. The older [Sequential Task Breakdown](milestones/SEQUENTIAL_TASK_BREAKDOWN.md) is retained as M8-M10 delivery history and is not the active queue.

## Current baseline

- M0 through M10 and stabilization S0 through S6 are complete on `dev`.
- AW0 through AW7 managed Agent Worker isolation is complete.
- The native macOS GUI overhaul is complete.
- Portfolio storage, services, financial CRUD, dashboard UI, and thesis-to-asset linkage are integrated.
- On-demand market-data refresh and quote caching are integrated through PR #211.
- Generic and IBKR activity-statement file import is integrated through PR #213.
- PR #199 contains the changes associated with issues #194 through #198; the tracker still needs evidence-based reconciliation.
- No pull requests were open when the 2026-09-20 planning baseline was reviewed.

The baseline was reviewed at `origin/dev` commit `9e21553`. Agents must fetch and inspect the latest remote state before starting work.

## Completed Milestones

- [x] **M0 — Project Foundation**
- [x] **M1 — Desktop Runtime Foundation**
- [x] **M1.5 — Application Foundation**
- [x] **M2 — Agent Runtime** (Stabilized in S1)
- [x] **M3 — Artifact Intelligence System** (Stabilized in S3)
- [x] **M4 — Research Workspace** (Stabilized in S4)
- [x] **M5 — Investment Knowledge System** (Stabilized in S4)
- [x] **M6 — Portfolio Intelligence** (Stabilized in S4)
- [x] **M7 — Internal Plugin Ecosystem** (Stabilized in S3)
- [x] **M8 — Local MVP Completion & Release Readiness** (Stabilized in S6)
- [x] **M9 — Option Module Integration** (Stabilized in S5)
- [x] **M10 — Goose Agent Integration** (M10-G0 through M10-G6, #161-#166)

## Active sequence

Execute one task at a time. Do not begin a task until its predecessor is merged into `dev`.

1. **M11-01 — Canonical Portfolio surface audit**
2. **M11-02 — Legacy Portfolio path retirement**
3. **M11-03 — Large-portfolio benchmark and approved budgets**
4. **M11-04 — Portfolio scale and resilience hardening**
5. **M11-05 — Reproducible macOS and Windows release packages**
6. **M11-06 — Cross-platform packaged smoke and upgrade evidence**
7. **M11-07 — Release-candidate acceptance gate**

Every task has its objective, acceptance criteria, involved files, branch type, and verification checklist in the [M11 Execution Plan](milestones/M11_PORTFOLIO_RELEASE_EXECUTION_PLAN.md).

## Explicitly deferred

- Live broker synchronization and broker credential storage.
- FIRE and retirement planning.
- Authentication, billing, license enforcement, cloud sync, and telemetry.
- New locales beyond `en` and `zh-CN`.
- A public plugin marketplace or arbitrary plugin code execution.
- Trading, brokerage order routing, and autonomous investment decisions.

These items require a separate approved milestone. File-based Generic and IBKR import is already present and is not authorization for live broker connectivity.

## Required quality gates

Run focused checks during implementation and the full applicable matrix before acceptance:

```bash
node scripts/check-ipc-registration.mjs
node scripts/check-option-ipc-registration.mjs
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
pnpm test:e2e
pnpm tauri build
```

Package and smoke tasks must additionally retain platform, architecture, package hash, build command, install/upgrade result, and residual-risk evidence.

## Assignment rules

- Branch from the latest `origin/dev`; never develop or commit directly on `dev` or `main`.
- Use one focused branch and one small PR per task, targeting `dev`.
- Prefer one cohesive Conventional Commit.
- Update related English documentation in the same PR as implementation.
- Record commands actually run; never claim an unrun check passed.
- Stop after the assigned task and wait for its PR to merge.
