# Option Module Git Workflow

This document consolidates the completed historical Option Git plan and reconciles it with the repository's current state. The repository-wide [Git Workflow](../GIT_WORKFLOW.md) remains authoritative when rules conflict.

## Branch history and current policy

The historical strategy used:

```text
docs/option-* and feature/option/*
  -> integration/option
  -> one final integration PR to dev
```

That history is preserved in the remote branches. However, `integration/option` now contains a large, stale candidate diff spanning Option code and unrelated shared runtime files. It must not be merged wholesale merely because its commit message says all phases are complete.

For M9, the default integration strategy is:

```text
current dev
  -> feature/option/<vertical-slice>
  -> review and verify that slice
  -> PR to dev
```

If the product owner explicitly requests a staging branch for coordinated M9 work, create a new branch from the then-current `dev` (for example `integration/option-m9`). Do not repurpose or force-update the historical `integration/option` branch.

## Required branch rules

- Never develop directly on `main` or `dev`.
- Start every M9 branch from an updated `dev` unless an approved M9 staging branch exists.
- Use `feature/option/<description>`, `fix/option/<description>`, `test/option/<description>`, or `docs/option-<description>`.
- Do not mix i18n, Goose, release infrastructure, or unrelated runtime refactors into an Option PR.
- Never force-push protected or shared integration branches.
- Never edit or renumber a migration that may already have been applied; add a new append-only migration.
- Commit, push, merge, tag, or delete branches only with the authority required by the repository workflow.

## Candidate-code reuse

Existing code on `origin/integration/option` is reference material, not a patch to apply blindly.

For each slice:

1. Compare the candidate paths with current `dev`.
2. Identify shared files changed for reasons unrelated to the slice.
3. Reuse the smallest valid implementation in a fresh branch from `dev`.
4. Reconcile it with current error types, task lifecycle, repositories, UI components, and security policies.
5. Add or repair tests before claiming the behavior works.

Useful read-only audit commands:

```bash
git fetch origin dev integration/option
git diff --stat origin/dev...origin/integration/option
git diff --name-status origin/dev...origin/integration/option
git log --oneline origin/dev..origin/integration/option
```

Do not stage the entire candidate diff. Stage explicit files belonging to the current slice.

## Recommended PR sequence

| Order | Branch example                  | Scope                                                                 |
| ----- | ------------------------------- | --------------------------------------------------------------------- |
| 1     | `fix/option-schema-runtime`     | Append-only schema reconciliation, migration runner, repository tests |
| 2     | `feature/option-pricing-core`   | `option-core`, pure models, numerical tests and benchmarks            |
| 3     | `feature/option-chain-slice`    | Provider, service, IPC, desktop API, chain UI, persistence            |
| 4     | `feature/option-strategy-slice` | Strategy service, payoff output, builder UI, Artifact renderer        |
| 5     | `feature/option-risk-slice`     | Scenario and portfolio risk integration with provenance               |
| 6     | `test/option-release-gates`     | Security, performance, packaged smoke, E2E, documentation evidence    |

Each PR should leave `dev` usable and should complete an input-to-persistence vertical slice where practical.

## Before opening a PR

```bash
git status -sb
git diff --check
git diff --name-only origin/dev...HEAD
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Run `pnpm test:e2e`, benchmarks, and `pnpm tauri build` when the slice changes critical UI, calculations, or packaging. Record commands that cannot run and why.

## PR description requirements

- The milestone work package and user-visible outcome.
- Included and explicitly excluded scope.
- Candidate code reused, rewritten, or rejected, with rationale.
- Database and permission impact.
- Test and benchmark commands with observed results.
- Known limitations, financial-model assumptions, and rollback path.
- Documentation updated.

Use Conventional Commits. A documentation-only Option PR uses `docs:`, while functional slices use the most accurate `feat:`, `fix:`, or `test:` prefix.

## Completion rule

A Roadmap checkbox changes to complete only after the relevant code is on `dev`, required checks have passed, and the PR or milestone evidence is linked. Existence on `integration/option` is not completion.
