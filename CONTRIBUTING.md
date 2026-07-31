# Contributing to Investment OS

## Getting Started

1. Clone the repository.
2. Install Rust stable via `rustup`.
3. Install Node 22 and pnpm 9.
4. Run `pnpm install`.
5. Run `pnpm dev` to start the desktop application.

## Development Workflow

- Read `AGENTS.md` before making changes.
- Read `docs/ARCHITECTURE.md` for architectural context.
- Follow the execution workflow in `AGENTS.md` Section 13.
- Do not push directly to `main` — use feature branches.

## Code Standards

- `cargo fmt` and `cargo clippy` must pass on all Rust code.
- ESLint and Prettier must pass on all TypeScript code.
- Write tests for new behavior.
- Update documentation when architecture changes.

## Commit Guidelines

- Keep changes scoped and focused.
- Do not mix unrelated refactors with feature work.
- Write clear commit messages.

## PR Checklist

- [ ] TypeScript compiles without errors.
- [ ] Rust compiles without errors.
- [ ] `cargo fmt` passes.
- [ ] `cargo clippy` passes.
- [ ] ESLint passes.
- [ ] Tests pass.
- [ ] Documentation updated.
- [ ] No secrets committed.
