# ADR-0011: `dev` as the Default Branch and Release Line

## Status

Accepted on 2026-11-13.

## Context

The repository contained two divergent lines that shared only commit `a000606` (2026-08-02):

- `dev` — 311 commits containing the complete product: milestones M0-M11, the Portfolio and Agent runtime work, the macOS GUI, and 24 migration files.
- `main` — 16 commits merged in November from the same old base, describing a Phase 1.5 application with 5 migration files.

`main` was the GitHub default branch but no longer represented the product. Keeping both lines as release candidates would require merging unrelated histories, reconciling colliding migration numbers, and maintaining duplicate implementations of the same features.

## Decision

1. `dev` is the repository default branch, the integration branch, and the release line.
2. All feature branches are cut from `origin/dev`; all pull requests target `dev`.
3. Release tags (`vX.Y.Z`) are created on `dev` after the release-owner acceptance gate.
4. `main` is frozen at `f3baf81` as a historical snapshot. It receives no merges, pull requests, or releases, and its protection remains to prevent history rewrites.
5. The CLI that existed only on `main` (`apps/cli`) is ported to `dev` as a separate change (PR #261).

## Consequences

- One canonical product line and one release path.
- Contributors use `dev`; the default branch, PR base, issue-template documentation link, and CI pull-request triggers reflect that.
- `main` retains its old content and history; it is not deleted.
- Any future change to this policy requires a new decision record.

## References

- [`docs/GIT_WORKFLOW.md`](../GIT_WORKFLOW.md)
- [`docs/BRANCH_PROTECTION.md`](../BRANCH_PROTECTION.md)
- PR #260 (open-issue fixes on `dev`)
- PR #261 (CLI port on `dev`)
