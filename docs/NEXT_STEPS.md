# AlphaForge Next Steps

**Updated:** 2026-08-15
**Active milestone:** S0 — Baseline Truth and Build Recovery
**Program decision:** M8 and M9 are reopened for stabilization; M10 remains planned.

Program status is governed by the [Milestone Roadmap](MILESTONE_ROADMAP.md). Corrective sequencing and acceptance gates are governed by the [Stabilization Roadmap](STABILIZATION_ROADMAP.md). Source evidence and the ordered small-PR plan are in the [Frontend-Backend Integration and Functional Completeness Audit](reviews/INTEGRATION_GAP_AUDIT_2026-08-12.md).

## Current status

- The repository contains substantial workspace, research, thesis, portfolio, Artifact, plugin, Option, and Agent implementation.
- The 2026-08-12 code review found release-blocking cross-layer defects.
- Rust module-tree and pnpm baseline repairs are merged (#78, #79).
- OpenAI credential, Agent lifecycle, canonical Option schema, Option IPC, and System IPC repairs are merged (#80, #81, #83, #84, #85) with focused layer-level tests; this does not constitute M8/M9 acceptance.
- The Artifact-window route (#88), Research URL context (#94), controlled Option workflow (#95, #97, #98), and internal-plugin Settings (#99) are merged with focused tests; packaged smoke acceptance and several backend-only APIs remain incomplete.
- Goose has frontend and backend scaffolding, but the UI is unreachable, the service is disabled, and bridge operations remain placeholders.
- Local MVP release acceptance is withdrawn until stabilization evidence is recorded.

## Completed stabilization repairs

- [x] Remove the orphan Rust database module declaration (#78).
- [x] Make pnpm the authoritative workspace tooling and lockfile baseline (#79).
- [x] Align OpenAI credential storage, status, and provider lookup (#80).
- [x] Repair the Agent `created -> queued -> running` flow (#81).
- [x] Establish the canonical Option schema and migration baseline (#83).
- [x] Normalize Option and System IPC DTOs with focused response validation (#84, #85).

## Immediate work order

1. Complete packaged Artifact-window smoke acceptance and retain its route/permission evidence.
2. Review the controlled company-comparison create-to-Artifact workflow.
3. Retain its payload, disabled-state, window-navigation, and renderer evidence.
4. Enforce frontend and Rust quality gates in CI.
5. Re-run release acceptance and update milestone evidence.

Each numbered item should normally be a separate pull request. Branch from `dev`, target `dev`, and never develop directly on `dev` or `main`.

## Required pull-request contents

Every rectification pull request must include:

- One narrowly defined defect or vertical slice.
- A clear frontend/backend contract when IPC is affected.
- Regression tests for the corrected behavior.
- Loading, empty, error, partial, and offline behavior where applicable.
- English documentation updated in the same pull request as behavior.
- The exact verification commands run and their results.
- Remaining risks and deliberately deferred work.

## Verification after code repair

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

Critical workflow and release-impacting pull requests must also run:

```bash
pnpm test:e2e
pnpm tauri build
```

Do not substitute compilation alone for functional acceptance.

## Release acceptance gate

M8 can be accepted again only when:

- All P0 integration-audit findings are closed.
- The core Agent-to-Artifact loop passes E2E.
- The supported primary workflows pass packaged smoke tests.
- Security, dependency, privacy, legal, support, backup, recovery, update, and rollback gates are complete.
- README, architecture, roadmap, milestone, and user documentation agree.
- A release owner records explicit acceptance evidence.

## Option acceptance gate

M9 can be accepted again only when:

- TypeScript and Rust Option DTOs pass shared serialization fixtures.
- Chain acquisition, persistence, contract detail, strategy persistence, calculation, and controlled Artifact rendering work end to end.
- Assumptions, timestamps, source, model, uncertainty, and provenance are visible.
- Numerical, migration, workspace-isolation, accessibility, i18n, E2E, and packaged-build gates pass.
- No trade execution or autonomous investment decision capability exists.

## Goose entry gate

M10 remains planned until stabilization and local MVP acceptance are complete. Goose work must then satisfy the ADR, threat model, version pinning, checksum, opt-in, read-only, workspace-scoping, structured-output, cancellation, budget, packaging, and support requirements in the [Goose Integration Roadmap](goose/INTEGRATION_ROADMAP.md).

## Persistent quality work

- Prefer command-boundary DTOs over leaking database/domain naming into React.
- Keep all IPC calls in the desktop API layer.
- Treat API wrapper presence as infrastructure, not proof of a product feature.
- Keep error codes stable and internal details redacted.
- Preserve local-first data ownership and evidence provenance.
- Maintain the no-trading and mandatory-human-review boundaries.
- Keep branches short-lived and pull requests small.
