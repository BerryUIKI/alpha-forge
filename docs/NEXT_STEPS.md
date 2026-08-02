# AlphaForge Next Steps

**Updated**: 2026-08-03

**Current completed milestone**: M7 Plugin Ecosystem

**Next planned milestone**: M8 Local MVP Completion & Release Readiness

Program status and milestone acceptance criteria are governed by [MILESTONE_ROADMAP.md](MILESTONE_ROADMAP.md). Implementation agents follow the [Milestone Delivery Playbook](milestones/DELIVERY_PLAYBOOK.md).

## Current status

- M0 through M7 are recorded complete in the milestone roadmap.
- No M8 implementation is marked complete by this documentation update.
- The M8 decision record still requires product-owner approval before release choices are authoritative on `dev`.
- The Option documentation is consolidated, but the Option runtime is not integrated on `dev`.
- Goose implementation is explicitly post-MVP and is not part of M8.

## Immediate actions: activate M8

1. Complete the [M8 Decision Record](M8_DECISION_RECORD.md): launch locale, platforms, privacy, export, update, signing, legal, security, and support owners.
2. Execute [i18n Implementation Plan](i18n/IMPLEMENTATION_PLAN.md), beginning with the decision and string inventory package.
3. Plan local export, installer, update, privacy, security, legal, and support work as separate vertical slices.
4. Record test, packaged-smoke, and review evidence before declaring the MVP complete.

## After MVP: M9 Option

Start with the [Option Index](option/README.md), then run the baseline audit in the [Option Integration Plan](option/INTEGRATION_PLAN.md).

The first implementation package must repair Option schema application through a new append-only migration and tests. Do not merge the historical `integration/option` branch wholesale; extract and revalidate scoped candidate code against current `dev`.

## After MVP and M9: M10 Goose

Start with the [Goose Integration Roadmap](goose/INTEGRATION_ROADMAP.md). Reverify the current AAIF Goose source and APIs, approve an ADR and threat model, then perform a synthetic-data spike. The first shipped mode is read-only and opt-in. Goose receives no direct database, shell, unrestricted filesystem, credential, trade, or privileged Tauri capability.

## Persistent quality work

- Increase regression coverage for migrations, task cancellation/restart, Artifact isolation, and critical UI states.
- Keep error codes stable and logs redacted.
- Preserve local-first data ownership and source provenance.
- Maintain loading, success, empty, error, partial, and offline behavior for asynchronous features.
- Keep documentation aligned with integrated behavior; unmerged branch code is not completion evidence.

## Standard verification

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Run `pnpm test:e2e` and `pnpm tauri build` for critical flows and release-impacting work. Record commands that cannot run and the remaining risk.

## Next decision point

The next decision is whether M8 product and release inputs are sufficiently complete to activate the first implementation work package. M9 and M10 remain planned until their entry gates pass.
