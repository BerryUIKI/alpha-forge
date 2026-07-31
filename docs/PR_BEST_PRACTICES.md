# Pull Request Best Practices

## Minimal PR Principle

Every PR should be **minimal, focused, and reviewable**.

### What is a Minimal PR?

A minimal PR:
- Solves **one problem** or implements **one feature**
- Has **clear scope** (not a mix of unrelated changes)
- Is **small enough** to review in 15-30 minutes
- Contains **related changes** that belong together

### PR Size Guidelines

| Size | Lines Changed | Files Changed | Review Time |
|------|---------------|---------------|-------------|
| **Ideal** | < 400 | < 10 | 15-30 min |
| **Acceptable** | 400-800 | 10-20 | 30-60 min |
| **Too Large** | > 800 | > 20 | > 60 min |

**Rule**: If review takes > 60 minutes, split the PR.

---

## Forbidden PR Patterns

### ❌ Don't: Kitchen Sink PR

```text
PR Title: "Add agent runtime and fix bugs and update docs"

Changes:
- Agent runtime implementation (500 lines)
- Fix typo in README (1 line)
- Refactor error handling (200 lines)
- Update ROADMAP.md (50 lines)
- Add .gitignore entry (2 lines)
```

**Problem**: Mixed concerns, hard to review, hard to revert.

### ✅ Do: Focused PRs

```text
PR #1: "feat: implement agent task domain model"
- Task struct and state machine
- TaskRepository trait
- Unit tests for task transitions

PR #2: "fix: correct typo in README"
- Fix typo in installation section

PR #3: "refactor: improve error handling in runtime"
- Refactor runtime error handling
- Add error tests
```

---

## Sub-Branch PR Strategy

### When to Use Sub-Branches

Use sub-branches when:
1. Feature is **too large** for single PR (> 800 lines)
2. Feature has **distinct phases** (design, implementation, tests)
3. Multiple developers work on **different aspects** of same feature

### Sub-Branch Naming

```text
feature/agent-runtime              (Parent branch)
├── feature/agent-runtime/domain   (Sub-branch: domain models)
├── feature/agent-runtime/runtime  (Sub-branch: runtime implementation)
└── feature/agent-runtime/tests    (Sub-branch: integration tests)
```

### Sub-Branch PR Workflow

```text
Step 1: Create parent branch
    git checkout main
    git checkout -b feature/agent-runtime

Step 2: Create sub-branch from parent
    git checkout feature/agent-runtime
    git checkout -b feature/agent-runtime/domain

Step 3: Work on sub-branch
    git add .
    git commit -m "feat: add task domain model"

Step 4: Create PR from sub-branch to parent
    gh pr create --base feature/agent-runtime
    
Step 5: After approval, merge sub-branch to parent
    (Maintains linear history on parent)

Step 6: When all sub-branches merged, create PR from parent to main
    gh pr create --base main
```

### Sub-Branch PR Rules

**Must Have**:
1. Clear scope definition (what this sub-branch implements)
2. Reference to parent branch in PR description
3. All checks pass before merge to parent
4. Squash merge to parent (to keep clean history)

**Should Have**:
1. Link to tracking issue (if exists)
2. Tests for new functionality
3. Documentation updates (if needed)

**Must NOT**:
1. Mix unrelated changes across sub-branches
2. Skip review for sub-branch PRs (still need review)
3. Merge to parent without passing checks

---

## PR Template Compliance

### Required Sections

Every PR must have:

#### 1. Summary

**What** this PR does in 2-3 sentences.

```markdown
## Summary

Implement Agent Task domain model with state machine, repository trait, 
and unit tests for all state transitions.

- Add Task struct with status field
- Implement TaskRepository trait
- Add comprehensive unit tests
```

#### 2. Scope

**What** is included and **what is NOT** included.

```markdown
## Scope

**Included:**
- Task domain model
- TaskRepository trait definition
- Unit tests for Task state machine

**NOT Included:**
- Runtime implementation (separate PR)
- Provider integration (separate PR)
- Persistence layer (separate PR)
```

#### 3. Testing

**How** the changes were tested.

```markdown
## Testing

```bash
# Ran unit tests
cargo test task::tests
# Result: 12 tests passed

# Ran type check
cargo check
# Result: No errors
```

#### 4. Checklist

From `.github/pull_request_template.md`:

```markdown
## Checklist

- [x] TypeScript compiles without errors.
- [x] Rust compiles without errors.
- [x] `cargo fmt` passes.
- [x] `cargo clippy` passes.
- [x] ESLint passes.
- [x] Tests pass.
- [x] Documentation updated.
- [x] No secrets committed.
```

---

## PR Review Rules

### For Authors

**Before Creating PR**:
1. Self-review your changes (`git diff main`)
2. Run all checks locally
3. Write clear PR description
4. Keep PR focused (split if needed)

**After Creating PR**:
1. Respond to all review comments
2. Make changes in new commits (not amend)
3. Mark conversations as resolved
4. Don't force push after review starts

### For Reviewers

**Review Criteria**:
1. Does PR have clear scope?
2. Is PR minimal (not mixing unrelated changes)?
3. Does code follow AGENTS.md guidelines?
4. Are tests adequate?
5. Is documentation updated?

**Review Turnaround**:
- Aim for review within 24 hours
- If review will take > 60 minutes, request split

---

## Merge Rules

### Squash Merge Policy

**When to Squash**:
- Feature branches → main
- Sub-branches → parent branches

**Why Squash**:
- Clean history on main/parent
- Single commit per feature
- Easier to revert if needed

**Squash Message**:
```
feat: implement agent task domain model (#123)

Implement Task domain model with state machine, repository trait,
and comprehensive unit tests.

- Add Task struct with status field
- Implement TaskRepository trait
- Add 12 unit tests for state transitions

Reviewed-by: Reviewer Name
```

### Merge Criteria

PR can be merged when:
1. ✅ All checks pass
2. ✅ At least 1 approval
3. ✅ No unresolved conversations
4. ✅ Branch is up-to-date with base
5. ✅ Linear history maintained

---

## Anti-Patterns to Avoid

### 1. Giant PR

```text
❌ Bad:
PR: "Implement entire Agent Runtime"
Changes: 5000 lines, 50 files
Review: 3+ hours

✅ Good:
PR #1: "feat: add agent task domain" (400 lines)
PR #2: "feat: add agent runtime core" (600 lines)
PR #3: "feat: add provider integration" (500 lines)
PR #4: "test: add integration tests" (300 lines)
```

### 2. Mixed Concerns

```text
❌ Bad:
PR: "Add feature X and fix bugs"
- Feature X implementation (600 lines)
- Unrelated bug fix (50 lines)

✅ Good:
PR #1: "feat: implement feature X"
PR #2: "fix: resolve bug in module Y"
```

### 3. Skipping Review

```text
❌ Bad:
Sub-branch PR: "WIP: partial implementation"
Review: Self-approved or no review

✅ Good:
Sub-branch PR: "feat: implement sub-component"
Review: Requires same review process as main PRs
```

### 4. Unclear Scope

```text
❌ Bad:
PR: "Update code"
Summary: "Various changes"

✅ Good:
PR: "feat: add task cancellation support"
Summary: "Implement cooperative cancellation for agent tasks using CancellationToken"
```

---

## Checklist for Minimal PR

Before creating PR, verify:

- [ ] **Single purpose**: PR solves ONE problem
- [ ] **Related changes**: All changes belong together
- [ ] **Under 800 lines**: PR is reviewable in reasonable time
- [ ] **Clear scope**: What's included/excluded is documented
- [ ] **Tests included**: New functionality has tests
- [ ] **Docs updated**: Architecture changes documented
- [ ] **Self-reviewed**: You reviewed your own changes first
- [ ] **Checks pass**: All CI checks green

---

## Examples

### Good Minimal PR

**Title**: `feat: add task state transition validation`

**Summary**: Implement validation for task state transitions to prevent invalid state changes.

**Scope**:
- Included: Task::transition() method with validation
- Included: Unit tests for all valid/invalid transitions
- NOT included: UI for state display (separate PR)

**Testing**:
```bash
cargo test task::tests::transitions
# Result: 15 tests passed
```

**Checklist**: All items checked ✅

**Review**: Approved after 20 minutes

---

### PR That Should Be Split

**Title**: `feat: implement agent runtime`

**Changes**:
- Task domain model (300 lines)
- Runtime implementation (400 lines)
- Provider trait (200 lines)
- SQLite repository (250 lines)
- Tests (300 lines)
- Total: 1450 lines, 25 files

**Problem**: Too large, multiple concerns

**Should Be**:
- PR #1: Task domain model (300 lines)
- PR #2: Runtime implementation (400 lines)
- PR #3: Provider trait (200 lines)
- PR #4: SQLite repository (250 lines)
- PR #5: Integration tests (300 lines)

---

**Document Version**: 1.0
**Last Updated**: 2026-07-31