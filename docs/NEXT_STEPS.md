# AlphaForge Next Steps

**Updated:** 2026-08-24
**Active milestone:** S6 Complete (Local MVP Release-Ready); M10 Entry Gate Assessment
**Program decision:** Stabilization program (S0-S6) is completed and accepted; M10 remains planned behind opt-in gate.

Program status is governed by the [Milestone Roadmap](MILESTONE_ROADMAP.md). Corrective sequencing and acceptance gates are governed by the [Stabilization Roadmap](STABILIZATION_ROADMAP.md). Source evidence and the ordered small-PR plan are in the [Frontend-Backend Integration and Functional Completeness Audit](reviews/INTEGRATION_GAP_AUDIT_2026-08-12.md).

## Current status

- The 6-stage stabilization roadmap (S0-S6) has been completed, passing all CI gates and acceptance criteria.
- 100% IPC command parity (176/176) is enforced with camelCase DTOs and Zod runtime validation.
- Agent task runtime, background execution, real-time progress streaming, cancellation, and structured output rendering are fully accepted.
- Artifact window isolation, least-privilege capability boundary (`capabilities/artifact-window.json`), and predefined React renderers are operational.
- Research URL context authority, deep linking, provenance navigation, and 7-standard UI states are verified.
- Portfolio accounts, positions, CSV transactions import, allocations, concentration risks, theme exposure, thesis alignment, and review reports are fully tested.
- Option chain acquisition, contract inspection, strategy persistence, Greeks/pricing engines, and no-trading boundary are verified.
- The local MVP is release-ready.

## Completed stabilization milestones

- [x] **S0 — Baseline truth and build recovery** (#151)
- [x] **S1 — Core Agent loop recovery** (#152)
- [x] **S2 — IPC contract normalization across all 176 commands** (#153, #154, #155, #156)
- [x] **S3 — Artifact and plugin vertical slice** (#157)
- [x] **S4 — Research and portfolio workflow closure** (#158)
- [x] **S5 — Option module re-acceptance** (#159)
- [x] **S6 — Release-readiness re-acceptance**

## Next steps (M10 Entry Gate)

1. Review and verify Goose upstream version, licensing, CLI/API contracts, and checksums.
2. Formulate threat model and Architecture Decision Record for Goose agent integration.
3. Replace MCP mock methods with bounded, workspace-scoped read operations.
4. Implement opt-in configuration for Goose backend service initialization.

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
