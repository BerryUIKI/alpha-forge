# Contributing to AlphaForge (Investment OS)

First off, thank you for considering contributing to AlphaForge! It's people like you that make AlphaForge a great tool for investment research.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Code Standards](#code-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)
- [Documentation Guidelines](#documentation-guidelines)
- [Communication Channels](#communication-channels)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior.

## Getting Started

### Prerequisites

- **Rust**: Stable toolchain (MSVC on Windows)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Node.js**: Version 22 or higher
  ```bash
  # Using nvm
  nvm install 22
  nvm use 22
  ```

- **pnpm**: Version 9 or higher
  ```bash
  npm install -g pnpm
  ```

### Initial Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/BerryUIKI/alpha-forge.git
   cd alpha-forge
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Verify setup**
   ```bash
   # TypeScript
   pnpm typecheck
   
   # Rust (requires local compilation)
   cargo check --workspace
   
   # Development server
   pnpm dev:web
   ```

4. **Read key documentation**
   - [AGENTS.md](AGENTS.md) - **Required reading for all contributors**
   - [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - System architecture
   - [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) - Development guide

## Development Workflow

### Before Making Changes

1. **Understand before editing**
   - Read relevant sections of `AGENTS.md`
   - Check existing implementations for patterns
   - Review related modules and types

2. **Choose the right branch**
   - `main` - Stable production branch
   - Create feature branches from `main`
   - Use descriptive branch names: `feat/agent-runtime`, `fix/ipc-error`, `docs/api-guide`

### Branch Naming Convention

| Type | Prefix | Example |
|------|--------|---------|
| Feature | `feat/` | `feat/portfolio-tracking` |
| Bug Fix | `fix/` | `fix/memory-leak` |
| Documentation | `docs/` | `docs/api-reference` |
| Refactor | `refactor/` | `refactor/error-handling` |
| Test | `test/` | `test/agent-integration` |
| Chore | `chore/` | `chore/update-dependencies` |

### Development Commands

```bash
# Frontend development
pnpm dev:web          # Start Vite dev server
pnpm typecheck        # TypeScript check
pnpm lint             # ESLint
pnpm test             # Vitest tests

# Backend development (requires local Rust)
pnpm tauri dev        # Full desktop app
cargo check           # Rust check
cargo test            # Rust tests
cargo clippy          # Linter
cargo fmt             # Formatter

# Build
pnpm tauri build      # Production build
```

## Project Structure

```
alpha-forge/
├── apps/                    # Application entry points
│   └── desktop/            # Tauri desktop app
├── crates/                  # Rust crates
│   ├── app/                # Main application logic
│   ├── database/           # SQLite & repositories
│   └── agent/              # Agent runtime
├── packages/                # TypeScript packages
│   ├── ui/                 # React components
│   ├── desktop-api/        # IPC layer
│   └── shared/             # Shared utilities
├── plugins/                 # Internal plugins
├── docs/                    # Documentation
└── scripts/                 # Build scripts
```

### Key Files

| File | Purpose |
|------|---------|
| [AGENTS.md](AGENTS.md) | Agent coding standards (highest priority) |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | System architecture and boundaries |
| [docs/GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Git workflow and conventions |
| [docs/MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | Product milestones |

## Code Standards

### TypeScript

- **Strict mode** is enabled - no `any` types without justification
- **Validate external data** with Zod schemas
- **Centralize shared protocols** - avoid duplication
- **Use explicit discriminated unions** for state machines
- **Prefer small pure functions** for formatting logic

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- **Avoid unnecessary cloning** and shared mutable global state
- **Use explicit error enums** instead of `unwrap()`/`expect()`
- **Make background tasks cancellable**
- **Keep Tauri commands thin** - logic goes in services

### General Principles

1. **One primary responsibility** per component/module
2. **Explicit interfaces** - no hidden magic
3. **Composition over inheritance**
4. **Reuse existing patterns** before creating new ones
5. **Support keyboard navigation** in UI
6. **Handle all states**: loading, success, empty, error, partial, offline

## Commit Guidelines

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `style` | Code style changes (formatting, etc.) |
| `refactor` | Code refactoring |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks |
| `perf` | Performance improvements |

### Examples

```bash
feat(agent): add task cancellation support
fix(ipc): resolve memory leak in event streaming
docs(readme): add installation instructions
test(repository): add workspace persistence tests
chore(deps): update Tauri to 2.0.0
```

### Commit Best Practices

- **Keep changes scoped** - one logical change per commit
- **Do not mix unrelated changes** - refactors separate from features
- **Write clear descriptions** - explain "why" not just "what"
- **Reference issues** - "fixes #123" or "relates to #456"

## Pull Request Process

### Before Submitting

1. **Update from main**
   ```bash
   git checkout main
   git pull origin main
   git checkout your-branch
   git rebase main
   ```

2. **Run all checks**
   ```bash
   pnpm typecheck
   pnpm lint
   pnpm test
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```

3. **Update documentation** if architecture changes

### PR Template Checklist

Your PR should satisfy all items in our [PR template](.github/pull_request_template.md):

- [ ] TypeScript compiles without errors
- [ ] Rust compiles without errors
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes
- [ ] ESLint passes
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No secrets committed

### Review Process

1. **Automated checks** must pass (CI/CD)
2. **Code review** by at least one maintainer
3. **All conversations resolved**
4. **Squash and merge** to maintain clean history

### After Merge

- Delete your feature branch
- Update local main branch
- Start a new branch for your next contribution

## Testing Requirements

### What to Test

Every new behavior must include relevant tests:

- **Rust**: Domain unit tests, repository tests, service tests, command integration tests
- **React**: Component tests, hook tests, schema tests
- **E2E**: Critical user flows (when applicable)

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        // Arrange
        // Act
        // Assert
    }
}
```

### Running Tests

```bash
# TypeScript
pnpm test
pnpm test:watch    # Watch mode
pnpm test:coverage # Coverage report

# Rust
cargo test
cargo test --workspace
cargo test <module_name>
```

### Test Principles

- **Don't fabricate test output** - run actual tests
- **Add regression tests** for bug fixes
- **Test edge cases** and error states
- **Keep tests isolated** - no shared state

## Documentation Guidelines

### When to Update

- **Architecture changes** → update `docs/ARCHITECTURE.md`
- **New features** → update relevant doc + README
- **API changes** → update API documentation
- **Behavior changes** → update user-facing docs
- **Milestone status changes** → synchronously update every relevant status document in the same PR to prevent drift

### Documentation Standards

- **English only** - all project documentation in English
- **Clear structure** - use headers, lists, and code blocks
- **Code examples** - include runnable examples
- **Keep current** - remove outdated information
- **Local validation** - before committing documentation changes, contributors can run local stale-phrase and Markdown-link scans; these scans are planned for future CI but are not yet enforced there

### Agent Documentation (AGENTS.md)

`AGENTS.md` is the highest-priority document. Changes to workflow, architecture, or standards must update it:

```markdown
## New Section

### Context
Why this section exists.

### Rules
1. Specific rules for agents.

### Examples
```rust
// Good example
```
```

## Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **Pull Requests**: Code contributions
- **Discussions**: Questions, ideas, community
- **Security**: See [SECURITY.md](SECURITY.md)

## Recognition

Contributors are recognized in:
- Git history (conventional commits)
- Release notes (significant contributions)
- Project contributors page

## Questions?

1. Check existing [documentation](docs/)
2. Search existing [issues](https://github.com/BerryUIKI/alpha-forge/issues)
3. Start a [discussion](https://github.com/BerryUIKI/alpha-forge/discussions)

---

Thank you for contributing to AlphaForge! 🚀

For AI agents: See [AGENTS.md](AGENTS.md) for complete agent workflow instructions.
