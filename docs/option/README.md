# Option Module Documentation

This directory is the canonical, consolidated documentation set for AlphaForge's Option analysis module. Start here before changing Option code.

## Status at consolidation

Documentation and code status are intentionally reported separately.

| Area                                                           | `dev` baseline                                | Integration candidate                    | Interpretation                                                                       |
| -------------------------------------------------------------- | --------------------------------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------ |
| Product, use cases, architecture, data, and API specifications | Present                                       | Present                                  | Documentation baseline is complete but needs implementation-time validation          |
| Rust domain models                                             | Present in `crates/domain/src/option.rs`      | Extended                                 | Implemented on `dev`                                                                 |
| SQL schema and runtime registration                            | Historical `0004_options_support.sql` plus canonical `0014_options_support.sql` | Present | Canonical 0014 is registered by the custom runner; historical 0004 remains unchanged |
| Repositories                                                   | Present under `database/repositories`         | Present                                  | Implemented against the canonical runtime schema; repository CRUD/isolation tests remain incomplete |
| TypeScript protocol types                                      | Present in `apps/desktop/src/types/option.ts` | Extended                                 | Implemented on `dev`; Option response schemas now parse the command-boundary camelCase contract, including nullable Rust `Option<T>` outputs |
| Pricing engine, services, IPC, and UI                          | Present in `dev`                              | Present on `integration/option`          | Implemented modules exist; Option IPC naming and registration parity are repaired, while UI reachability gaps keep M9 unaccepted; Option plugins are not present |
| End-to-end Option workflow                                     | Present as partial implementation             | Claimed complete by the candidate branch | Not verified end to end; chain-to-contract and persisted-strategy acceptance remains pending |

The phrase “candidate branch” refers to `origin/integration/option` at the time of this consolidation. A branch name or historical commit message is not acceptance evidence.

PR #95 merged the chain-to-contract view. The current corrective slice, `codex/feat-option-strategy-persistence`, validates contract references, derives leg fields server-side, and persists strategies and legs atomically; its controlled UI remains the next separate PR, so M9 is not accepted.

## Canonical document set

| Document                                            | Role                                                | Status                                                                       |
| --------------------------------------------------- | --------------------------------------------------- | ---------------------------------------------------------------------------- |
| [Product Specification](PRODUCT.md)                 | Product boundary, users, workflows, success metrics | Consolidated                                                                 |
| [Use Cases](USE_CASES.md)                           | User journeys and acceptance behavior               | Consolidated                                                                 |
| [Architecture](ARCHITECTURE.md)                     | Target layers, providers, plugins, security, tests  | Consolidated                                                                 |
| [Data Model](DATA_MODEL.md)                         | Entities, persistence, validation, repositories     | Consolidated                                                                 |
| [API Specification](API_SPEC.md)                    | Typed IPC contracts and events                      | Consolidated                                                                 |
| [Roadmap](ROADMAP.md)                               | Seven-phase feature decomposition                   | Consolidated; execution status is governed here and by the milestone roadmap |
| [Implementation Details](IMPLEMENTATION_DETAILS.md) | Current-code-aware file and interface path          | New canonical implementation guide                                           |
| [Integration Plan](INTEGRATION_PLAN.md)             | Audit, sequencing, quality gates, rollout, rollback | Reconciled from the completed Option planning branch                         |
| [Git Workflow](GIT_WORKFLOW.md)                     | Option-specific branch and PR policy                | Reconciled from the completed Option planning branch                         |
| [Milestone Roadmap](../MILESTONE_ROADMAP.md)        | Program priority and scheduling                     | Program source of truth                                                      |

These ten entries replace references to an unspecified “ten planned documents.” New Option documents should be added only when they have a distinct owner and do not duplicate this set.

## Product guardrails

- The module supports research, education, scenario analysis, and decision records.
- It does not place orders, connect to a brokerage for execution, or make autonomous investment decisions.
- Prices, Greeks, implied volatility, and strategy outcomes are analytical estimates with explicit model and data provenance.
- Live data providers are not enabled until their licensing, credentials, timeout, validation, and failure behavior are approved.
- Option results become structured, traceable research artifacts; they do not bypass the thesis and evidence workflow.

## How an implementation agent should start

1. Read the [Milestone Roadmap](../MILESTONE_ROADMAP.md) and confirm M9 is active.
2. Follow the repository-wide [Delivery Playbook](../milestones/DELIVERY_PLAYBOOK.md).
3. Run the baseline audit in [Integration Plan](INTEGRATION_PLAN.md).
4. Use [Implementation Details](IMPLEMENTATION_DETAILS.md) to select one vertical slice.
5. Follow the Option [Git Workflow](GIT_WORKFLOW.md) and open a narrowly scoped PR.
6. Update documentation and attach real test evidence; never mark a phase complete from file presence alone.
