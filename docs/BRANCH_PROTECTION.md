# Branch Protection Rules Update

**Date**: 2026-08-01
**Status**: Active
**Author**: Berry Wahlberg

---

## Summary

Branch protection rules have been configured for Investment OS to ensure code quality and production stability.

---

## Protected Branches

### `main` Branch - Production

**Strict Protection** - All rules are enforced

| Rule | Status | Details |
|------|--------|---------|
| 🔒 **No Direct Pushes** | ✅ Enforced | All changes must go through Pull Request |
| 🧪 **Required Status Checks** | ✅ Required | `CI` and `Code Quality` must pass |
| 👥 **Required Reviews** | ✅ Required | Minimum 1 approval required |
| 💬 **Conversation Resolution** | ✅ Required | All discussions must be resolved |
| 📏 **Linear History** | ✅ Required | Squash merge or rebase only |
| 🔐 **Admin Enforcement** | ✅ Enforced | Even admins must follow PR process |
| 🚫 **No Force Pushes** | ✅ Blocked | Cannot overwrite history |
| 🗑️ **No Deletions** | ✅ Blocked | Branch cannot be deleted |

### `dev` Branch - Integration

**Recommended Protection** - Most rules enforced

| Rule | Status | Details |
|------|--------|---------|
| 🔒 **No Direct Pushes** | ⚠️ Recommended | Should use PR, but admins can bypass |
| 🧪 **Status Checks** | ⚠️ Recommended | `CI` check expected, but not required |
| 👥 **Reviews** | ⚠️ Recommended | 1 approval suggested |
| 💬 **Conversation Resolution** | ✅ Required | All discussions must be resolved |
| 📏 **Linear History** | ✅ Required | Squash merge or rebase only |
| 🔐 **Admin Enforcement** | ❌ Optional | Admins can push directly in emergencies |
| 🚫 **No Force Pushes** | ✅ Blocked | Cannot overwrite history |
| 🗑️ **No Deletions** | ✅ Blocked | Branch cannot be deleted |

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

# 7. When ready for release, create PR: dev → main
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

### Required for `main`

- **CI**: Build, test, and verify application
- **Code Quality**: Lint, format check, and static analysis

### Expected for `dev`

- **CI**: Recommended but not strictly required

---

## Key Differences

| Aspect | `main` | `dev` |
|--------|--------|-------|
| Purpose | Production releases | Development integration |
| Protection Level | Strict | Recommended |
| Direct Push | Blocked | Blocked (bypassable by admins) |
| Status Checks | Required (2) | Expected (1) |
| Review Required | Yes | Recommended |
| Admin Override | No | Yes (emergency only) |

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

### Hotfix to `main`

```bash
# Create hotfix branch from main
git checkout main
git pull origin main
git checkout -b fix/critical-issue

# Fix and commit
git commit -m "fix: resolve critical issue"

# Create PR to main
# Get expedited review
# Merge and release

# Sync back to dev
git checkout dev
git pull origin main
git push origin dev
```

### Emergency Push to `dev`

Administrators can bypass `dev` protection in emergencies:

```bash
# Only for critical situations!
git push origin dev --force-with-lease
```

**Note**: This will still trigger warnings. Use sparingly.

---

## Policy Updates

| Date | Change | Reason |
|------|--------|--------|
| 2026-08-01 | Initial branch protection setup | Establish code quality standards |

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