# Branch Protection Rules

**Date**: 2026-08-01
**Updated**: 2026-11-13
**Status**: Active
**Author**: Berry Wahlberg

---

## Summary

Branch protection rules have been configured for AlphaForge to ensure code quality and production stability.

---

## Protected Branches

### `dev` Branch - Default, Integration, and Release

**Active protection** - enforced by GitHub

| Rule | Status | Details |
|------|--------|---------|
| 🔒 **No Direct Pushes** | ✅ Enforced | All changes must go through Pull Request |
| 🧪 **Status Checks** | ⚠️ Recommended | Quality Gates and CodeQL run on every PR; not configured as required checks |
| 👥 **Required Reviews** | ✅ Required | Minimum 1 approval required |
| 💬 **Conversation Resolution** | ⚠️ Recommended | Resolve discussions before merge; not enforced on `dev` |
| 📏 **Linear History** | ⚠️ Recommended | Squash or rebase preferred; not enforced on `dev` |
| 🔐 **Admin Enforcement** | ❌ Optional | Admins can push directly in emergencies |
| 🚫 **No Force Pushes** | ✅ Blocked | Cannot overwrite history |
| 🗑️ **No Deletions** | ✅ Blocked | Branch cannot be deleted |

### `main` Branch - Frozen Historical Snapshot

`main` is frozen since 2026-11-13. It is not a development or release target. Protection remains so the historical snapshot cannot be rewritten.

| Rule | Status | Details |
|------|--------|---------|
| 🔒 **No Direct Pushes** | ✅ Enforced | Branch is frozen |
| 🧪 **Required Status Checks** | ✅ Required | `Analyze (javascript-typescript)` and `Analyze (rust)` |
| 👥 **Required Reviews** | ✅ Required | Minimum 1 approval required |
| 💬 **Conversation Resolution** | ✅ Required | All discussions must be resolved |
| 📏 **Linear History** | ✅ Required | Squash merge or rebase only |
| 🔐 **Admin Enforcement** | ✅ Enforced | Even admins must follow PR process |
| 🚫 **No Force Pushes** | ✅ Blocked | Cannot overwrite history |
| 🗑️ **No Deletions** | ✅ Blocked | Branch cannot be deleted |

No pull requests, merges, or releases target `main`. The protection rules above remain configured to prevent history rewrites; they are otherwise dormant.

---

## Workflow

### Standard Development Process

```bash
# 1. Start from dev
git checkout dev
git pull origin dev

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Develop and commit
git add .
git commit -m "feat: add new feature"

# 4. Push and create PR to dev
git push origin feature/my-feature
# Create PR: feature/my-feature → dev

# 5. After approval and CI passes, merge to dev
# Use squash merge on GitHub

# 6. Delete feature branch
git branch -d feature/my-feature
git push origin --delete feature/my-feature

# 7. When ready for release, tag the accepted commit on dev
#    git tag -a v0.1.0 -m "Release v0.1.0"
#    git push origin v0.1.0
```

### Branch Naming Convention

```
feature/<description>   - New features
fix/<description>       - Bug fixes
docs/<description>      - Documentation updates
refactor/<description>  - Code refactoring
test/<description>      - Test additions/updates
chore/<description>     - Maintenance tasks
```

### Examples

```
✅ feature/agent-runtime
✅ fix/ipc-timeout
✅ docs/api-reference
✅ refactor/error-handling
✅ test/agent-coverage

❌ my-feature
❌ fix-bug
❌ updates
```

---

## CI Status Checks

### Required for `main` (frozen)

- `Analyze (javascript-typescript)`
- `Analyze (rust)`

### Expected for `dev` (default)

- **Quality Gates**: frontend and Rust quality gates
- **CodeQL**: `Analyze (javascript-typescript)` and `Analyze (rust)`

These checks run on every push and pull request but are not configured as required rules on `dev`; they should still pass before merge.

---

## Key Differences

| Aspect | `dev` | `main` |
|--------|-------|--------|
| Purpose | Default branch, integration, and releases | Frozen historical snapshot |
| Protection Level | Active | Strict |
| Direct Push | Blocked (admin bypass for emergencies) | Blocked |
| Status Checks | Expected | Required (2) |
| Review Required | Yes (1) | Yes (1) |
| Admin Override | Yes (emergency only) | No |
| Releases | Yes | No |

---

## Team Guidelines

### Before Creating a PR

1. ✅ Ensure all tests pass locally
2. ✅ Run linting and formatting
3. ✅ Update documentation if needed
4. ✅ Write/update tests for new code
5. ✅ Keep PR focused and minimal (< 800 lines)

### During PR Review

1. ✅ Respond to all comments
2. ✅ Make changes in new commits
3. ✅ Keep PR updated with target branch
4. ✅ Resolve all conversations

### After Merge

1. ✅ Delete the feature branch
2. ✅ Update local repository
3. ✅ Start next task from updated `dev`

---

## Emergency Procedures

### Hotfix After a Release

```bash
# Create a fix branch from dev
git checkout dev
git pull origin dev
git checkout -b fix/critical-issue

# Fix, commit, and open a PR to dev
# After approval and merge, tag a patch release on dev
git checkout dev
git pull origin dev
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

### Emergency Bypass on `dev`

Administrators can bypass the review requirement for a critical fix:

```bash
# Only for critical situations, with release-owner approval
gh pr merge <number> --squash --admin
```

Never rewrite `dev` history or force-push; open a PR and use the admin merge override instead.

---

## Policy Updates

| Date | Change | Reason |
|------|--------|--------|
| 2026-08-01 | Initial branch protection setup | Establish code quality standards |
| 2026-11-13 | Default branch moved from `main` to `dev`; `main` frozen as a historical snapshot ([ADR-0011](DECISIONS/0011-dev-default-release-line.md)) | `dev` is the complete product line; avoid two divergent release branches |

---

## Resources

- [Git Workflow Documentation](GIT_WORKFLOW.md)
- [Pull Request Best Practices](PR_BEST_PRACTICES.md)
- [Contributing Guide](../CONTRIBUTING.md)

---

## Questions?

If you have questions about these rules:

1. Check the [Git Workflow](GIT_WORKFLOW.md) documentation
2. Ask in team chat or create a Discussion
3. Reach out to maintainers

---

**Remember**: These rules exist to protect production stability and ensure code quality. They help us maintain a high-quality codebase that's safe for everyone to work with.