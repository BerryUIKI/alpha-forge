# AlphaForge Next Steps

**Updated:** 2026-08-24
**Active milestone:** Production Release & Distribution Packaging (M10 Complete)
**Program decision:** All core milestones (M0-M10) and stabilization streams (S0-S6) are 100% completed, tested, and accepted.

Program status is governed by the [Milestone Roadmap](MILESTONE_ROADMAP.md). Corrective sequencing and acceptance gates are governed by the [Stabilization Roadmap](STABILIZATION_ROADMAP.md) and [Goose Integration Roadmap](goose/INTEGRATION_ROADMAP.md).

## Current status

- The 6-stage stabilization roadmap (S0-S6) has been completed, passing all CI gates and acceptance criteria.
- Milestone M10 (Goose Agent Integration, M10-G0 through M10-G6) is 100% complete and merged into `dev` (#161-#166).
- 100% IPC command parity (183/183) is enforced with camelCase DTOs and Zod runtime validation.
- Agent task runtime, background execution, real-time progress streaming, cancellation, and structured output rendering are fully operational.
- Artifact window isolation, least-privilege capability boundary (`capabilities/artifact-window.json`), and predefined React renderers are active.
- Research URL context authority, deep linking, provenance navigation, and 7-standard UI states are verified.
- Portfolio accounts, positions, CSV transactions import, allocations, concentration risks, theme exposure, thesis alignment, and review reports are fully tested.
- Option chain acquisition, contract inspection, strategy persistence, Greeks/pricing engines, and no-trading boundary are verified.
- Supervised Goose sidecar with read-only MCP bridge, `AuthorizedScope` validation, OS Keyring credential protection, zero-trading guardrails, and diagnostics API is fully integrated.
- The local MVP is release-ready.

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

## Active Next Steps (Release & Packaging)

1. Synchronously update roadmap and release status documentation.
2. Execute production binary packaging (`pnpm tauri build`) for Windows distribution.
3. Validate packaged installer bundle, SHA-256 digests, and smoke test installation.
4. Prepare version release notes and operator distribution guide.

## Quality Gates

```bash
node scripts/check-ipc-registration.mjs
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
pnpm tauri build
```
