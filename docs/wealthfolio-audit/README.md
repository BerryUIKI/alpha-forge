# Wealthfolio Audit Documents

These documents were produced by an independent audit of the
[Wealthfolio](https://github.com/wealthfolio/wealthfolio) codebase (v3.7.0)
and are copied here for reference while integrating Wealthfolio's portfolio
functionality into this repository (Investment OS / AlphaForge).

**Authoritative plan:** [`../PORTFOLIO_INTEGRATION_PLAN.md`](../PORTFOLIO_INTEGRATION_PLAN.md)

The source project lives at `F:\dev\wealthfolio` (a local clone). The audit
documents are read-only references — do not edit them in this workspace;
if corrections are needed, update the source clone and re-copy.

---

## Index

| File | Contents |
|------|----------|
| [`01-project-overview.md`](./01-project-overview.md) | Tech stack, directory structure, env vars, run commands. |
| [`02-architecture-and-lifecycle.md`](./02-architecture-and-lifecycle.md) | Architecture layers, data flow, module dependency graph, lifecycle. |
| [`03-full-feature-inventory.md`](./03-full-feature-inventory.md) | Complete feature inventory. |
| [`04-data-structure-spec.md`](./04-data-structure-spec.md) | Full database schema (Diesel migrations, 48 migrations). |
| [`05-business-workflow-breakdown.md`](./05-business-workflow-breakdown.md) | Business workflows. |
| [`06-external-dependency-inventory.md`](./06-external-dependency-inventory.md) | External dependencies per crate. |
| [`07-code-debt-risk-report.md`](./07-code-debt-risk-report.md) | Coupling, hard-coded secrets/URLs, dead code, 3,264 unwrap/expect. |
| [`08-reusable-module-assessment.md`](./08-reusable-module-assessment.md) | Per-crate reusability grades (Fully-Reusable / Minor-Modification / Non-Detachable). |
| [`09-tech-stack-migration-guide.md`](./09-tech-stack-migration-guide.md) | Migration difficulty ratings, strangler-fig strategy. |
| [`10-api-and-internal-function-list.md`](./10-api-and-internal-function-list.md) | API and internal function list. |
| [`11-configuration-reference.md`](./11-configuration-reference.md) | Configuration reference. |
| [`12-optimization-todo-list.md`](./12-optimization-todo-list.md) | Optimization TODOs. |
| [`13-agent-development-manual.md`](./13-agent-development-manual.md) | Agent development manual. |
| [`14-appendix-glossary-index.md`](./14-appendix-glossary-index.md) | Glossary and index. |

---

## Most relevant documents for this integration

1. **`08-reusable-module-assessment.md`** — tells you which Wealthfolio crates
   can be ported as-is (`market-data` is Fully-Reusable) and which cannot
   (`storage-sqlite` is Non-Detachable — must be rewritten on SQLx).
2. **`04-data-structure-spec.md`** — the source of truth for the financial
   domain schema you will re-implement with SQLx migrations.
3. **`07-code-debt-risk-report.md`** — quantifies the 3,264 unwrap/expect
   panic points and the Diesel coupling that must not be copied over.
4. **`09-tech-stack-migration-guide.md`** — the Diesel → SQLx migration playbook.
