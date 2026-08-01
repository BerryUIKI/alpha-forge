# Option Analysis Platform - Git Workflow

## Overview

The Option Analysis Platform follows an **isolated development workflow** to develop independently from the main AlphaForge project until fully complete. This document defines the branching strategy, PR flow, and integration rules specific to the Option platform.

---

## Key Principle: Two-Stage Integration

```text
Phase 1-7 Development:
  Feature branches → integration/option
                              ↓
                    Integration testing
                              ↓
                    All phases complete
                              ↓
Final Integration (ONE-TIME):
  integration/option → dev (main AlphaForge branch)
```

**Critical Rules**:
- ✅ During phases 1-7: PRs go to `integration/option`
- ❌ During phases 1-7: NEVER PR directly to `dev` or `main`
- ✅ Only ONCE all phases complete: Create final PR `integration/option` → `dev`

---

## Branch Strategy

### Integration Branch: `integration/option`

**Purpose**: Stable integration branch for all Option platform work.

**Naming Note**: We use `integration/option` instead of `dev/option` due to a Git technical constraint:
- Git cannot have both `refs/heads/dev` (file) and `refs/heads/dev/option` (directory) in the same repository
- `integration/option` avoids this conflict while clearly signaling "integration branch"

**Protection Level**: Protected (requires review, cannot force-push)

### Development Branches

All development happens on feature branches following this naming convention:

```text
docs/option-<short-description>        # Documentation
feature/option/<short-description>     # Implementation
fix/option/<short-description>         # Bug fixes
test/option/<short-description>        # Testing
```

**Examples**:
```bash
docs/option-product-spec          # Documentation
feature/option/domain             # Domain models
feature/option/database           # Database schema
feature/option/backend-api        # IPC commands
test/option/integration           # Integration tests
```

### Branch Hierarchy

```text
main (production, never touched)
  └── dev (AlphaForge main development)
        └── integration/option (Option platform integration)
              ├── docs/option-*
              ├── feature/option/*
              ├── fix/option/*
              └── test/option/*
```

---

## Workflow: Development Phase

### Step 1: Start New Work

```bash
# Ensure integration branch is up-to-date
git checkout integration/option
git pull origin integration/option

# Create feature branch
git checkout -b feature/option/pricing-engine
```

### Step 2: Develop and Commit

Follow conventional commits:

```bash
git add .
git commit -m "feat: add Black-Scholes pricing model

- Implement analytical solution for European options
- Add Greeks calculations (Delta, Gamma, Theta, Vega, Rho)
- Include unit tests with known values from literature"

# Run checks before push
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pnpm lint && pnpm typecheck && pnpm test
```

### Step 3: Create Pull Request to `integration/option`

1. Push to remote:
   ```bash
   git push -u origin feature/option/pricing-engine
   ```

2. Create PR using GitHub CLI or web:
   ```bash
   gh pr create \
     --base integration/option \
     --head feature/option/pricing-engine \
     --title "feat: add Black-Scholes pricing model" \
     --body "## Summary
   Implements core pricing engine for option analysis.
   
   ## Scope
   **Included:**
   - Black-Scholes pricing formula
   - Greeks calculations
   - Unit tests
   
   **NOT Included:**
   - Binomial model (separate PR)
   - Live data provider integration
   
   ## Testing
   - Unit tests: 15/15 passing
   - Accuracy validated against literature values"
   ```

3. **CRITICAL**: Always verify base branch is `integration/option`, NOT `dev` or `main`

### Step 4: Review and Merge

**Requirements**:
- At least 1 approval required
- All CI checks passing
- All conversations resolved

**Merge Strategy**:
- Squash merge (maintains clean history)
- Delete branch after merge

```bash
# Via GitHub UI: "Squash and merge"
# Or after approval, merge locally:
git checkout integration/option
git merge --squash feature/option/pricing-engine
git push origin integration/option

# Delete feature branch
git branch -d feature/option/pricing-engine
git push origin --delete feature/option/pricing-engine
```

---

## Workflow: Documentation Phase

For documentation-only changes:

```bash
# Start from integration/option
git checkout integration/option
git pull origin integration/option

# Create docs branch
git checkout -b docs/option-api-spec

# Write documentation
vim docs/option/API_SPEC.md

# Commit and push
git add docs/option/API_SPEC.md
git commit -m "docs: add option platform API specification"
git push -u origin docs/option-api-spec

# Create PR to integration/option
gh pr create --base integration/option --head docs/option-api-spec
```

---

## Sub-Branch Strategy (Large Features)

For large features requiring multiple PRs, use sub-branches:

```text
feature/option/pricing-engine        (Parent)
├── feature/option/pricing-engine/black-scholes
├── feature/option/pricing-engine/binomial
└── feature/option/pricing-engine/tests
```

**Process**:
1. Create parent branch from `integration/option`
2. Create sub-branches from parent
3. PR sub-branches to parent (NOT to `integration/option`)
4. After all sub-branches merged, PR parent to `integration/option`

**Example**:
```bash
# Create parent
git checkout integration/option
git checkout -b feature/option/pricing-engine

# Push parent
git push -u origin feature/option/pricing-engine

# Create sub-branch
git checkout -b feature/option/pricing-engine/black-scholes

# Work and commit...

# PR sub-branch to parent
gh pr create --base feature/option/pricing-engine --head feature/option/pricing-engine/black-scholes

# After all sub-branches merged, PR parent to integration/option
gh pr create --base integration/option --head feature/option/pricing-engine
```

---

## Forbidden Actions

### During Development (Phases 1-7)

❌ **NEVER**:
- Create PR directly to `dev` or `main`
- Push to `dev` or `main` directly
- Force-push to `integration/option`
- Merge `dev` into `integration/option` (creates conflicts)
- Delete `integration/option` branch

### Always

❌ **NEVER**:
- Commit to `main` directly
- Use `git reset --hard` without explicit permission
- Use `git push --force` on protected branches
- Delete uncommitted user changes
- Mix unrelated refactors with feature work

---

## Keeping Up-to-Date with Main Project

### Periodic Rebases (Optional)

If AlphaForge's `dev` branch has significant changes:

```bash
# On integration/option
git checkout integration/option
git pull origin integration/option

# Fetch latest from main project
git fetch origin dev

# Rebase (creates merge commit, preserves history)
git merge origin/dev

# Or rebase (linear history, requires force-push)
# git rebase origin/dev
# git push --force-with-lease origin integration/option

# Resolve conflicts if any
# Push updates
git push origin integration/option
```

**Warning**: Rebasing `integration/option` requires coordination with all developers. Announce beforehand.

---

## CI/CD Integration

### Required Checks

Every PR to `integration/option` must pass:

**Backend (Rust)**:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

**Frontend (TypeScript)**:
```bash
pnpm lint
pnpm typecheck
pnpm test
```

### Branch Protection Rules

`integration/option` branch protection:
- Require PR before merging
- Require at least 1 approval
- Require status checks to pass
- Require linear history
- Include administrators

---

## Integration Testing

Before final integration to `dev`, run comprehensive tests:

```bash
# On integration/option branch
cargo test --all
pnpm test
pnpm test:e2e

# Build production
pnpm tauri build

# Verify no regressions in main AlphaForge features
```

Document test results in the final PR description.

---

## Troubleshooting

### Problem: Push rejected (branch name conflict)

**Error**: 
```
remote: cannot lock ref 'refs/heads/dev/option': 'refs/heads/dev' exists
```

**Solution**: This is why we use `integration/option`. Never try to create `dev/option` on remote.

### Problem: PR created to wrong base

**Error**: Accidentally created PR to `dev` instead of `integration/option`

**Solution**:
```bash
# Close the wrong PR
gh pr close <number>

# Recreate with correct base
gh pr create --base integration/option --head <your-branch>
```

### Problem: Merge conflicts with `dev`

**Solution**:
```bash
# Fetch latest
git fetch origin dev

# Merge into your branch
git merge origin/dev

# Resolve conflicts manually
# Test after resolving
cargo test && pnpm test

# Commit resolution
git add .
git commit -m "merge: resolve conflicts with dev"
git push
```

---

## Summary

| What | Where |
|------|-------|
| **Develop** | Feature branches (`feature/option/*`) |
| **Integrate** | `integration/option` (NOT `dev`) |
| **Final Integration** | `integration/option` → `dev` (once, after all phases) |
| **Production** | `main` (never touch) |

**Golden Rule**: During development, **all** PRs go to `integration/option`. Only create a PR to `dev` after **all 7 phases** are complete and tested.

---

## References

- [ROADMAP.md](./ROADMAP.md) - Phase-by-phase development plan
- [INTEGRATION_PLAN.md](./INTEGRATION_PLAN.md) - Final integration procedure
- [../GIT_WORKFLOW.md](../GIT_WORKFLOW.md) - Main AlphaForge workflow
- [../PR_BEST_PRACTICES.md](../PR_BEST_PRACTICES.md) - PR guidelines