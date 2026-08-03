# Internationalization (i18n)

This directory is the canonical documentation set for localizing the AlphaForge desktop application. It covers application UI and runtime messages; translated repository README files remain separate project-marketing artifacts.

## Current baseline

At the `dev` baseline used to write this plan:

- The React application does not have an application-wide locale provider or message catalog.
- User-facing strings are embedded in components.
- Locale-aware number and date presentation is a UI requirement in [UI Guidelines](../UI_GUIDELINES.md), but it is not yet centralized.
- The M8 launch-locale decision is still governed by the [M8 Decision Record](../M8_DECISION_RECORD.md).
- A separate, unmerged M8 branch contains a small Chinese-first locale prototype. It is an implementation candidate, not evidence that i18n is complete on `dev`.

Do not create a second translation framework without first auditing that candidate and the current `dev` tree.

## MVP boundary

The planned MVP foundation supports Simplified Chinese (`zh-CN`) and English (`en`). The product owner must record the launch default in the M8 decision record before implementation is merged. English is the source and missing-key fallback locale so development remains deterministic.

The first delivery includes:

- A typed locale identifier and one application locale provider.
- Namespaced message catalogs owned by frontend features.
- Locale persistence through the existing Settings service and `desktopApi`.
- Centralized date, number, percent, and currency formatting through the browser `Intl` APIs.
- Localized navigation, settings, common asynchronous states, and the critical MVP workflows.
- Localized presentation for stable Rust error codes; Rust logs and internal diagnostics remain locale-neutral.
- Tests for fallback behavior, persistence, interpolation, and critical UI in both locales.

The first delivery does not include:

- Machine-generated translations merged without human review.
- Server-side locale negotiation or cloud translation management.
- Translation of user content, imported research, agent output, evidence, or source quotations.
- Locale-specific investment advice or automatic changes to numeric meaning.
- Right-to-left layout support. The catalog and layout must not block it, but RTL QA is deferred until an RTL locale is scheduled.

## Document map

| Document                                                | Purpose                                                                                |
| ------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [Architecture](ARCHITECTURE.md)                         | Ownership boundaries, catalog design, formatting, error localization, and runtime flow |
| [Implementation Plan](IMPLEMENTATION_PLAN.md)           | Ordered work packages, file-level path, tests, gates, and definition of done           |
| [Milestone Roadmap](../MILESTONE_ROADMAP.md)            | Program sequencing and M8 acceptance gates                                             |
| [Delivery Playbook](../milestones/DELIVERY_PLAYBOOK.md) | Rules an implementation agent must follow for every milestone work package             |

## Required decisions before implementation

| Decision                                            | Owner                        | Gate                              |
| --------------------------------------------------- | ---------------------------- | --------------------------------- |
| Launch default: `zh-CN` or `en`                     | Product owner                | Before catalog rollout            |
| Product name and finance terminology glossary       | Product + bilingual reviewer | Before critical-flow translation  |
| Currency display policy for mixed-market portfolios | Product owner                | Before portfolio localization     |
| Translation reviewer and review SLA | @BerryUIKI (product owner)   | Before declaring M8 i18n complete |

Locale selection changes presentation only. Stored timestamps remain UTC/ISO 8601, persisted enum values and error codes remain stable, and monetary values retain their original currency code.
