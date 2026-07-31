# CI/CD Configuration

This document describes the Continuous Integration and Continuous Deployment (CI/CD) setup for AlphaForge (Investment OS).

## 📋 Table of Contents

- [Overview](#overview)
- [Workflows](#workflows)
- [Branch Protection](#branch-protection)
- [PR Automation](#pr-automation)
- [Dependabot](#dependabot)
- [Required Setup](#required-setup)

---

## Overview

AlphaForge uses **GitHub Actions** for CI/CD with comprehensive automation for:

- ✅ Code quality checks (lint, typecheck, format)
- ✅ Test execution (frontend + backend)
- ✅ Build verification
- ✅ Security auditing
- ✅ PR validation and automation
- ✅ Automated dependency updates
- ✅ Release management

---

## Workflows

### 1. CI Workflow (`ci.yml`)

**Triggers:** Push to `main`, Pull Requests to `main`

**Jobs:**

| Job | Description | Steps |
|-----|-------------|-------|
| `frontend` | TypeScript/React checks | typecheck, lint, test |
| `rust` | Rust checks | fmt, clippy, build, test |
| `build` | Build verification | frontend build, artifact check |
| `security` | Security audit | npm audit, cargo audit |
| `labeler` | Auto-label PRs | file-based labeling |

**Concurrency:** Cancels in-progress runs on new commits

---

### 2. PR Check Workflow (`pr-check.yml`)

**Triggers:** PR opened, synchronized, reopened, ready_for_review

**Jobs:**

| Job | Description |
|-----|-------------|
| `validate` | PR title format (Conventional Commits), large PR warning, forbidden patterns |
| `branch-check` | Branch naming convention check |
| `size-label` | Automatic size label (XS/S/M/L/XL) |
| `welcome` | Welcome message on new PRs |

---

### 3. PR Automation Workflow (`pr-automation.yml`)

**Triggers:** PR events, reviews

**Jobs:**

| Job | Description |
|-----|-------------|
| `review-reminder` | Remind after 24h waiting for review |
| `merge-check` | Create merge readiness status check |
| `auto-assign` | Auto-assign reviewers on new PRs |
| `stale-check` | Label stale PRs after 30 days |

---

### 4. Code Quality Workflow (`code-quality.yml`)

**Triggers:** Pull Requests to `main`

**Jobs:**

| Job | Description |
|-----|-------------|
| `coverage` | Test coverage report |
| `bundle-size` | Bundle size check (< 5MB) |
| `complexity` | File size and complexity warnings |
| `todos` | TODO/FIXME tracking |

---

### 5. Release Workflow (`release.yml`)

**Triggers:** Push of version tags (`v*.*.*`)

**Jobs:**

| Job | Description |
|-----|-------------|
| `release` | Create GitHub Release with changelog |
| `build` | Build cross-platform binaries (Linux, macOS, Windows) |

---

## Branch Protection

### `main` Branch Rules

The `main` branch is protected with the following rules:

1. **Direct pushes blocked** - All changes via PR
2. **Required status checks:**
   - Frontend CI (typecheck, lint)
   - Rust CI (fmt, clippy, build)
   - Build verification
3. **Required approvals:** At least 1
4. **Linear history:** Required (squash merge recommended)
5. **Conversation resolution:** Required

### Setup in GitHub UI

Go to: **Settings → Branches → Branch protection rules → main**

Configure:
- ✅ Require a pull request before merging
  - ✅ Require approvals: 1
  - ✅ Dismiss stale pull request approvals when new commits are pushed
  - ✅ Require review from Code Owners
- ✅ Require status checks to pass before merging
  - Select: `frontend`, `rust`, `build`
- ✅ Require branches to be up to date before merging
- ✅ Require linear history
- ✅ Require conversation resolution before merging

---

## PR Automation

### Auto-Labeling

Labels are automatically applied based on changed files:

| Label | Trigger Files |
|-------|---------------|
| `frontend` | `*.ts`, `*.tsx`, `packages/` |
| `backend` | `*.rs`, `Cargo.toml` |
| `rust` | `*.rs` |
| `typescript` | `*.ts`, `*.tsx` |
| `documentation` | `docs/`, `README*.md` |
| `database` | `migrations/`, `database/` |
| `ci` | `.github/workflows/`, `scripts/` |
| `plugins` | `plugins/` |
| `tests` | `*.test.*`, `*.spec.*` |
| `security` | `security/`, `capabilities/` |
| `ui` | `styles/`, `components/` |
| `config` | `package.json`, `*.config.*` |

### Size Labels

| Label | Lines Changed |
|-------|---------------|
| `size/XS` | < 100 |
| `size/S` | 100-299 |
| `size/M` | 300-799 |
| `size/L` | 800-1499 |
| `size/XL` | ≥ 1500 |

### PR Title Format

PR titles must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new feature
fix(scope): bug description
docs: update documentation
chore: routine maintenance
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`, `build`, `ci`, `revert`

---

## Dependabot

### Configuration

Dependabot checks for updates weekly on Mondays at 09:00 Asia/Shanghai:

| Ecosystem | Directory | Limit |
|-----------|-----------|-------|
| npm/pnpm | `/` | 10 PRs |
| cargo | `/` | 5 PRs |
| GitHub Actions | `/` | 5 PRs |

### Grouping

Dependencies are grouped:
- **Production dependencies** - grouped together
- **Development dependencies** - grouped together

### Review

All Dependabot PRs are:
- Assigned to: @BerryUIKI
- Reviewed by: @BerryUIKI
- Labeled: `dependencies`, `frontend`/`rust`/`ci`

---

## Required Setup

### 1. Enable GitHub Actions

Go to: **Settings → Actions → General**

- ✅ Allow all actions and reusable workflows
- ✅ Allow actions created by GitHub
- ✅ Allow actions by Marketplace verified creators

### 2. Branch Protection

Configure branch protection as described above.

### 3. Secrets

No secrets are required for basic CI. For releases:

- `GITHUB_TOKEN` - automatically provided by GitHub Actions

### 4. CODEOWNERS

The `.github/CODEOWNERS` file assigns ownership:

- Default owner: @BerryUIKI
- Specific paths have designated owners

### 5. Enable Dependabot

Go to: **Settings → Security → Code security and analysis**

- ✅ Enable Dependabot security updates
- ✅ Enable Dependabot version updates

### 6. Merge Settings

Go to: **Settings → Pull Requests**

- ✅ Allow squash merging (recommended)
- ✅ Allow merge commits (optional)
- ❌ Allow rebase merging (not recommended for linear history)

**Squash merge commit message:**
- ✅ Default to PR title

---

## Monitoring

### Workflow Status

Monitor workflow runs at: `Actions` tab in GitHub

### Badges

Add to README.md:

```markdown
[![CI](https://github.com/BerryUIKI/alpha-forge/workflows/CI/badge.svg)](https://github.com/BerryUIKI/alpha-forge/actions/workflows/ci.yml)
[![Release](https://github.com/BerryUIKI/alpha-forge/workflows/Release/badge.svg)](https://github.com/BerryUIKI/alpha-forge/actions/workflows/release.yml)
```

---

## Troubleshooting

### CI Fails on Empty Tests

The CI is configured with `continue-on-error: true` for tests since no tests exist yet. Remove this once tests are added.

### Build Fails on Tauri

Tauri builds require platform-specific tooling:
- **Linux**: `libwebkit2gtk-4.0-dev`, `libssl-dev`, etc.
- **macOS**: Xcode Command Line Tools
- **Windows**: Microsoft Visual Studio C++ Build Tools

See: https://tauri.app/v1/guides/getting-started/prerequisites

### Dependabot Not Creating PRs

Check that:
1. Dependabot is enabled in repository settings
2. `dependabot.yml` is in `.github/` directory
3. Repository is not archived

---

## Future Improvements

- [ ] Add code coverage reporting (Codecov/Coveralls)
- [ ] Add performance benchmarks
- [ ] Add visual regression testing
- [ ] Add dependency license checking
- [ ] Add automated changelog generation
- [ ] Add Slack/Discord notifications
- [ ] Add artifact signing for releases

---

## Related Documentation

- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guide
- [PR_BEST_PRACTICES.md](PR_BEST_PRACTICES.md) - PR guidelines
- [GIT_WORKFLOW.md](GIT_WORKFLOW.md) - Git workflow