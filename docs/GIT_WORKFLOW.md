# Git Workflow

## Branch Strategy

### Main Branch

`main` is the stable branch.

Rules:

- Never develop directly on `main`.
- Never commit feature work directly to `main`.
- `main` should always represent a buildable and stable state.
- Only merge completed, reviewed, and verified changes into `main`.

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

When a feature is complete:

1. Verify the branch.
2. Run all relevant checks.
3. Review changed files.
4. Update documentation if architecture changed.
5. Create a Pull Request.

PR description should include:

```markdown
## Summary

What changed.

## Implementation

Important technical decisions.

## Testing

Commands executed and results.

## Risks

Known limitations.
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
