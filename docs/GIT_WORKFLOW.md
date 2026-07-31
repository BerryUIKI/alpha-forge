# Git Workflow

## Branch Strategy

### Main Branch

`main` is the stable branch.

**Protected Branch Rules** (enforced by GitHub):

- 🔒 **Direct pushes are BLOCKED** — All changes must go through Pull Request
- ✅ **Pull Request required** — At least 1 approval needed
- 🔄 **Linear history** — No merge commits, use squash or rebase
- 💬 **Conversation resolution** — All discussions must be resolved before merge
- ⚠️ **Administrators included** — Even admins must follow PR process

**Additional Rules**:

- Never commit feature work directly to `main`.
- `main` should always represent a buildable and stable state.
- Only merge completed, reviewed, and verified changes into `main`.

**Why This Matters**:

- Ensures all changes are reviewed
- Maintains clean commit history
- Protects against accidental pushes
- Creates audit trail for all modifications

### Development Branches

All work must happen on dedicated branches.

Branch naming convention:

```
feature/<short-description>
fix/<short-description>
docs/<short-description>
refactor/<short-description>
```

Examples:

```
feature/project-foundation
feature/tauri-foundation
feature/agent-runtime
feature/artifact-system
feature/research-workspace
fix/ipc-connection-error
fix/sqlite-migration-issue
docs/update-architecture
refactor/rust-error-handling
```

---

## Commit Rules

### Commit Messages

Use Conventional Commits:

```
<type>: <description>
```

Allowed types:

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

Good examples:

```
feat: add react application shell
feat: add rust ipc commands
chore: initialize tauri desktop application
chore: initialize sqlite migration system
docs: update architecture documentation
test: add ipc integration tests
fix: resolve sqlite migration startup failure
```

Avoid:

```
update files
changes
fix stuff
work in progress
asdf
```

### Pre-Commit Checks

Before every commit, run relevant checks:

Frontend changes:

```bash
pnpm lint
pnpm typecheck
pnpm test
```

Rust changes:

```bash
cargo fmt
cargo clippy
cargo test
```

Do not commit known broken code unless explicitly requested.

### Commit Timing

Create a commit when:

1. **A complete milestone is finished** — after verifying the application works.
2. **A logical subsystem is complete** — frontend API layer, Rust IPC layer, SQLite setup, etc.
3. **Before risky changes** — large refactors, dependency upgrades, architecture changes.

Preferred frequency:

- Several meaningful commits per phase.
- Each commit should leave the repository buildable whenever practical.

Avoid:

- One giant commit after many unrelated changes.
- Hundreds of tiny commits for trivial edits.

---

## Pull Request Rules

### Minimal PR Principle

Every PR should be **minimal, focused, and reviewable**.

See [PR_BEST_PRACTICES.md](PR_BEST_PRACTICES.md) for detailed guidelines.

**Key Rules**:

1. **Single Purpose**: Each PR solves ONE problem
2. **Under 800 Lines**: PR should be reviewable in < 60 minutes
3. **Related Changes**: All changes should belong together
4. **Clear Scope**: Document what's included and excluded

### Sub-Branch Strategy

For large features, use sub-branches:

```text
feature/agent-runtime              (Parent)
├── feature/agent-runtime/domain   (Sub-branch 1)
├── feature/agent-runtime/runtime  (Sub-branch 2)
└── feature/agent-runtime/tests    (Sub-branch 3)
```

**Sub-Branch Rules**:
- Sub-branch PRs merge to parent branch (not directly to main)
- Sub-branch PRs still require review
- After all sub-branches merged, parent PR merges to main
- Maintains linear history on parent

### PR Creation Process

When a feature is complete:

1. Verify the branch.
2. Run all relevant checks.
3. Review changed files.
4. Update documentation if architecture changed.
5. Create a Pull Request.

PR description should include:

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
- [ ] Tests written
- [ ] Docs updated
```

---

## Forbidden Actions

Never execute without explicit user permission:

```bash
git reset --hard
git clean -fd
git push --force
git branch -D
git rebase main
```

Never delete user changes.
Never overwrite uncommitted work.

---

## See Also

- [PR_BEST_PRACTICES.md](PR_BEST_PRACTICES.md) — Detailed PR guidelines
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Contribution workflow
