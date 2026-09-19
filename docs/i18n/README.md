# Internationalization (i18n)

This directory is the canonical documentation set for localizing the AlphaForge desktop application. It covers application UI and runtime messages; translated repository README files remain separate project-marketing artifacts.

## Current baseline

The M8 i18n foundation and S6 re-acceptance are complete on `dev`. AlphaForge
has one application locale provider, English and Simplified-Chinese namespaced
catalogs, persisted locale selection, shared formatters, localized stable-error
presentation, and catalog-parity tests. Option and Goose surfaces were added
through M9 and M10. Reuse and extend this framework; do not create a second
locale system.

## Accepted MVP boundary

The accepted MVP foundation supports Simplified Chinese (`zh-CN`) and English
(`en`). English is the source and missing-key fallback locale so development
remains deterministic. Additional locales require a separately approved scope.

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
| [Terminology Guide](TERMINOLOGY_GUIDE.md)               | Canonical English source terms for investment research, portfolio, and agent domains   |
| [String Inventory](STRING_INVENTORY.md)                 | Complete inventory of all user-visible strings by namespace and owner                  |
| [Milestone Roadmap](../MILESTONE_ROADMAP.md)            | Program sequencing and M8 acceptance gates                                             |
| [Delivery Playbook](../milestones/DELIVERY_PLAYBOOK.md) | Rules an implementation agent must follow for every milestone work package             |

## Recorded ownership decisions

| Decision                                            | Owner                        | Status                                |
| --------------------------------------------------- | ---------------------------- | ------------------------------------- |
| Launch default and supported locale identifiers     | Product owner                | Recorded for the accepted rollout     |
| Product name and finance terminology glossary       | Product + bilingual reviewer | Published and maintained              |
| Currency display policy for mixed-market portfolios | Product owner                | Implemented through shared formatters |
| Translation reviewer and review SLA                 | @BerryUIKI (product owner)   | Required for catalog changes          |

Locale selection changes presentation only. Stored timestamps remain UTC/ISO 8601, persisted enum values and error codes remain stable, and monetary values retain their original currency code.
