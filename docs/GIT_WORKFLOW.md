# Git Workflow

## Branch Strategy

AlphaForge uses a **two-tier branching model**. `dev` is the default branch, the integration branch, and the release line; `main` is a frozen historical snapshot.

```
dev (default; integration and release line)
  ↑
  │ PR (requires review + CI)
  │
feature/* (development)

main (frozen historical snapshot; no merges, PRs, or releases)
```

### Branch Types

#### 1. `dev` - Default, Integration, and Release Branch

`dev` is the repository **default branch**, the integration branch, and the release line.

**Purpose**:

- Integrates completed work from all contributors
- Serves as the base branch for every feature branch
- Is the source of every release tag and package
- Aggregates changes for the next release

**Branch Rules** (configured in GitHub):

- 🔒 **Direct pushes are BLOCKED** — All changes must go through a Pull Request
- ✅ **Pull Request required** — At least 1 approval needed
- 🔄 **Linear history** — Squash or rebase preferred; not enforced on `dev`
- 🧪 **Status checks** — Quality Gates and CodeQL should pass before merge; not required rules on `dev`
- 💬 **Conversation resolution** — Resolve discussions before merge; not enforced on `dev`
- ⚠️ **Administrators can bypass** — For emergencies only

**Workflow Rules**:

1. **All feature branches must branch from `dev`**
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b feature/my-new-feature
   ```

2. **Feature branches PR back to `dev`**
   - Never directly to `main`
   - Requires code review
   - Must pass CI checks

3. **`dev` is the release line**
   - Release candidates are cut and tagged from `dev`
   - Publishing remains a separate release-owner action

**What Goes in `dev`**:

- ✅ Completed features
- ✅ Bug fixes
- ✅ Documentation updates
- ✅ Refactoring
- ✅ Test improvements

**What Does NOT Go in `dev`**:

- ❌ Broken code
- ❌ Half-finished features
- ❌ Unreviewed changes
- ❌ Experimental work (use separate branches)

---

#### 2. `main` - Frozen Historical Branch

`main` was the original production branch. On 2026-11-13 the repository default branch moved to `dev`; `main` is now frozen at commit `f3baf81` as a historical snapshot.

**Rules**:

- No new merges, pull requests, or releases target `main`.
- Do not create feature branches from `main`; use `origin/dev`.
- Branch protection remains in place so the frozen history cannot be rewritten.
- Release packages and tags are produced from `dev`, not `main`.

**Why the change**:

- `dev` contains the complete product history and every completed milestone.
- `main` had diverged into an older M1.5 line; keeping two release branches would duplicate code and confuse releases.

See [ADR-0011](DECISIONS/0011-dev-default-release-line.md) for the full decision record.

---

#### 3. Feature Branches

All development work happens on **dedicated feature branches**.

**Branch Naming Convention**:

```
feature/<short-description>
fix/<short-description>
docs/<short-description>
refactor/<short-description>
test/<short-description>
chore/<short-description>
```

**Examples**:

```
feature/agent-runtime
feature/artifact-system
feature/research-workspace
fix/ipc-connection-error
fix/sqlite-migration-issue
docs/update-architecture
refactor/rust-error-handling
test/add-agent-coverage
```

**Branch Creation Rules**:

1. **Always branch from `dev`**:
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b feature/my-feature
   ```

2. **Keep branches focused and short-lived**:
   - One feature per branch
   - Complete within 1-2 weeks
   - Delete after merging

3. **Rebase regularly on `dev`**:
   ```bash
   git checkout feature/my-feature
   git fetch origin
   git rebase origin/dev
   ```

---

## Complete Workflow Example

### Step 1: Start New Feature

```bash
# Ensure dev is up-to-date
git checkout dev
git pull origin dev

# Create feature branch
git checkout -b feature/agent-runtime
```

### Step 2: Develop and Commit

```bash
# Make changes
git add .
git commit -m "feat: add agent task domain models"

# Run checks before push
pnpm lint && pnpm typecheck && pnpm test
cargo fmt --check && cargo clippy && cargo test

# Push to remote
git push origin feature/agent-runtime
```

### Step 3: Create Pull Request to `dev`

1. Create PR: `feature/agent-runtime` → `dev`
2. Fill in PR template:
   - Summary of changes
   - Testing performed
   - Checklist completed
3. Request review
4. Address feedback
5. Ensure CI passes

### Step 4: Merge to `dev`

After approval:

```bash
# Squash and merge through GitHub (recommended)
gh pr merge <number> --squash

# Or use the GitHub UI's "Squash and merge" button
```

Do not push directly to `dev`; branch protection rejects it.

### Step 5: Delete Feature Branch

```bash
# Delete local branch
git branch -d feature/agent-runtime

# Delete remote branch
git push origin --delete feature/agent-runtime
```

### Step 6: Release from `dev`

When the release owner accepts a candidate:

```bash
# Tag the accepted commit on dev
git checkout dev
git pull origin dev
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

`main` is not updated by the release process.

---

## Branch Protection Matrix

| Branch    | Direct Push | PR Required | Approvals | CI Required | Target    |
|-----------|-------------|-------------|-----------|-------------|-----------|
| `dev` (default) | ❌ Blocked | ✅ Required | 1+ | ⚠️ Recommended | N/A |
| `main` (frozen) | ❌ Blocked | ✅ Required | 1+ | ✅ Required | No merges |
| `feature/*` | ✅ Allowed | ❌ Optional | Optional  | ⚠️ Recommended | `dev`   |
| `fix/*`     | ✅ Allowed | ❌ Optional | Optional  | ⚠️ Recommended | `dev`   |
| `docs/*`    | ✅ Allowed | ❌ Optional | Optional  | ❌ Optional    | `dev`   |

`main` is frozen; its protection rules are dormant because no PRs or merges should target it.

---

## Sub-Branch Strategy

For **large features** that need multiple PRs, use sub-branches:

```
dev
 ↑
 │
feature/agent-runtime (Parent)
 ├── feature/agent-runtime/domain   → PR to parent
 ├── feature/agent-runtime/runtime  → PR to parent
 └── feature/agent-runtime/tests    → PR to parent
```

**Sub-Branch Rules**:

- Sub-branches PR to the parent branch (not directly to `dev`)
- Sub-branches still require code review
- After all sub-branches merge, parent PRs to `dev`
- Maintains clean history on the parent branch

**Example Workflow**:

```bash
# Create parent branch from dev
git checkout dev
git checkout -b feature/agent-runtime

# Create sub-branch
git checkout -b feature/agent-runtime/domain

# Work on domain models...
git commit -m "feat: add agent task domain models"
git push origin feature/agent-runtime/domain

# PR: feature/agent-runtime/domain → feature/agent-runtime
# After merge, continue with next sub-branch...

git checkout feature/agent-runtime
git checkout -b feature/agent-runtime/runtime
# ... and so on
```

---

## Commit Rules

### Commit Messages

Use **Conventional Commits**:

```
<type>: <description>
```

**Allowed Types**:

```
feat      New feature
fix       Bug fix
docs      Documentation only
refactor  Code change that neither fixes a bug nor adds a feature
test      Adding or updating tests
chore     Build process, tooling, or auxiliary changes
perf      Performance improvement
build     Build system or external dependencies
ci        CI configuration
```

**Good Examples**:

```
feat: add agent runtime foundation
feat: implement artifact system
fix: resolve IPC connection timeout
docs: update agent protocol documentation
test: add integration tests for agent service
chore: update dependencies
refactor: simplify agent state machine
```

**Avoid**:

```
update files
changes
fix stuff
work in progress
asdf
```

### Pre-Commit Checks

Before every commit, run relevant checks:

**Frontend Changes**:

```bash
pnpm lint
pnpm typecheck
pnpm test
```

**Rust Changes**:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

**Do not commit known broken code** unless explicitly requested.

### Commit Timing

Create a commit when:

1. **A complete milestone is finished** — after verifying the application works
2. **A logical subsystem is complete** — frontend API layer, Rust IPC layer, SQLite setup, etc.
3. **Before risky changes** — large refactors, dependency upgrades, architecture changes

**Preferred Frequency**:

- Several meaningful commits per feature
- Each commit should leave the repository buildable whenever practical

**Avoid**:

- One giant commit after many unrelated changes
- Hundreds of tiny commits for trivial edits

---

## Pull Request Rules

### Minimal PR Principle

Every PR should be **minimal, focused, and reviewable**.

**Key Rules**:

1. **Single Purpose**: Each PR solves ONE problem
2. **Under 800 Lines**: PR should be reviewable in < 60 minutes
3. **Related Changes**: All changes should belong together
4. **Clear Scope**: Document what's included and excluded

See [PR_BEST_PRACTICES.md](PR_BEST_PRACTICES.md) for detailed guidelines.

### PR Creation Process

When a feature or fix is complete:

1. Verify the branch works locally
2. Run all relevant checks (`pnpm lint`, `pnpm typecheck`, `pnpm test`, `cargo fmt --check`, `cargo clippy`, `cargo test`, `node scripts/check-ipc-registration.mjs`)
3. Review changed files
4. **Synchronously update documentation** (architecture, roadmaps, specs, README) in the same branch/commit
5. Create a Pull Request

**PR Description Template**:

```markdown
## Summary

What changed (2-3 sentences).

## Scope

**Included:**
- What's in this PR

**NOT Included:**
- What's in separate PRs

## Testing

Commands executed and results.

## Checklist

- [ ] All checks pass
- [ ] Tests written & passing
- [ ] Documentation synchronously updated in this PR (Architecture, Roadmaps, README)
```

### PR Review Guidelines

**For Reviewers**:

- Review within 24 hours when possible
- Focus on correctness, maintainability, and architecture alignment
- Run the code locally for significant changes
- Be constructive and specific in feedback

**For Authors**:

- Respond to all comments
- Make changes in new commits (not amend) for clarity
- Keep PR updated with the target branch

---

## Forbidden Actions

Never execute without explicit user permission:

```bash
git reset --hard
git clean -fd
git push --force
git push --force-with-lease
git branch -D
git rebase main
git merge main
```

**Never**:

- Delete user changes
- Overwrite uncommitted work
- Push directly to `main` or `dev`
- Skip code review
- Merge the frozen `main` branch into `dev`

---

## Sync Strategy

### Regular Sync (Recommended Daily)

Keep your feature branch up-to-date:

```bash
# On feature branch
git fetch origin
git rebase origin/dev

# Resolve conflicts if any
git add .
git rebase --continue

# Force push (safe after rebase)
git push --force-with-lease origin feature/my-feature
```

### Hotfixes After a Release

`main` is frozen and never receives hotfixes. Fix released issues on a branch from `dev`:

```bash
git checkout dev
git pull origin dev
git checkout -b fix/critical-issue
# Fix, commit, and open a PR to dev.

# After the fix merges, tag a patch release on dev
git checkout dev
git pull origin dev
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

Then rebase any open feature branch on `origin/dev`.

---

## Summary Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        WORKFLOW                              │
└─────────────────────────────────────────────────────────────┘

1. Branch from dev
   ┌──────┐
   │ dev  │ ← git checkout -b feature/my-feature
   └──────┘
      ↓
2. Develop & Commit
   ┌────────────────────┐
   │ feature/my-feature │ ← Multiple commits
   └────────────────────┘   ← Run tests before each commit
      ↓
3. Push & Create PR
   ┌────────────────────┐
   │ feature/my-feature │ → PR to dev
   └────────────────────┘   ← Requires review
      ↓
4. Merge to dev
   ┌──────┐
   │ dev  │ ← Squash merge
   └──────┘   ← Delete feature branch
      ↓
5. Test & Validate
   ┌──────┐
   │ dev  │ ← Run full test suite
   └──────┘   ← Validate integration
      ↓
6. Release from dev
   ┌──────┐
   │ dev  │ ← Release-owner acceptance
   └──────┘
      ↓
7. Tag Release
   ┌──────┐
   │ dev  │ → git tag v1.0.0
   └──────┘
```

---

## See Also

- [PR_BEST_PRACTICES.md](PR_BEST_PRACTICES.md) — Detailed PR guidelines
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Contribution workflow
- [README.md](../README.md) — Project overview