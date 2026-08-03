# M8 Local MVP & Commercialization Decision Workbook

**Status**: Product decisions recorded; implementation is limited to the public local-desktop MVP.
**Decision owner**: @BerryUIKI
**Recorded**: 2026-08-03

This record distinguishes the free, open-source MVP from later commercial work. It does not authorize authentication, payment collection, cloud storage, telemetry, or automatic investment decisions.

M8 introduces external services and release responsibilities. No authentication provider, billing service, cloud storage, analytics platform, or release identity will be added until these decisions are approved.

## Recommended MVP boundary

The recommended first commercial release keeps the local-first product model:

- No required account or cloud backend.
- Local data remains the source of truth.
- Optional manual export is preferred over automatic cloud backup.
- A simple license file or offline entitlement is preferred over subscription billing.
- macOS is the first supported release platform, with Windows considered after the release workflow is proven.
- Simplified Chinese and English are the planned MVP UI locales; the product owner must approve the launch default.

Choose a different direction only if it is a deliberate product requirement.

## 1. Product scope

| Decision | Recorded decision | Recommended default | Why it is needed |
|---|---|---|---|
| Target release date or quarter | TBD by @BerryUIKI | No fixed date until the gate is approved | Determines scope and release planning |
| Primary customer | Individual investment researchers | Individual investment researcher | Guides pricing, privacy, and support |
| MVP commercial model | Free and open-source local desktop application | Paid local desktop application | Defines entitlement work |
| Later commercial direction | Paid commercial offering, to be reviewed before implementation | - | Future planning |
| Is a cloud account required in M8? | No | No | Changes architecture and privacy obligations |
| Countries or regions to support at launch | Mainland China primary; English-language overseas users secondary | Your home market only | Affects payment, tax, legal, and privacy work |
| Product coverage | A-share, Hong Kong, United States equities, options, and futures research | - | Product scope |
| Supported UI locales | Simplified Chinese (`zh-CN`) and English (`en`) | Simplified Chinese (`zh-CN`) and English (`en`) | Defines translation and release QA scope |
| Launch default locale | `zh-CN`, with user-selectable English | `zh-CN`, with user-selectable English | Defines deterministic first-run behavior |
| Translation reviewer | Named bilingual reviewer | Named bilingual reviewer | Owns finance terminology and disclaimer accuracy |
| Product owner | @BerryUIKI | Named individual | Owns scope decisions |

## 2. Authentication and identity

Authentication is **not in the MVP**. The app has no account, provider, session, or server-side user-data retention. Any future identity implementation must keep credentials out of React and use an approved native credential store.

## 3. Licensing, payment, and entitlement

Licensing, payment, activation, and entitlement enforcement are **not in the MVP**. They will be reconsidered for the commercial release. The currently preferred later activation mechanism is a signed license, subject to a separate security and legal review. The app must never collect payment-card data directly.

## 4. Backup, export, and privacy

| Decision | Recorded decision |
| --------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Automatic cloud backup | No |
| Manual local export | Yes |
| Storage provider and region | None; no cloud storage is operated |
| Backup behavior | Native save dialog; consistent SQLite export; existing files are not overwritten |
| Restore conflict behavior | Not applicable; restore/import is not part of this MVP change |
| Telemetry | Disabled by default; no analytics service is configured |
| Privacy notice | Published by @BerryUIKI on the official website; mirrored in the app's About and privacy settings |
| Retention and deletion | The user controls local application data and exported files; the application operates no user-data service |

Manual exports are user-controlled copies and may contain sensitive research. The user is responsible for choosing a safe destination and retaining backups. The product must not silently upload, synchronize, or overwrite local data.

## 5. Platforms, updates, and distribution

| Decision | Recorded decision |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Supported platforms | macOS on Apple Silicon and Windows |
| Installer formats | macOS DMG and Windows EXE (NSIS) |
| Linux | Not in the MVP release scope |
| Code-signing owner | @BerryUIKI |
| macOS notarization | Not planned for the MVP |
| Update discovery | The client queries GitHub Releases only when the user requests a check; downloads and installation are manual |
| Release owner | @BerryUIKI |
| Release visibility in China | GitHub connectivity and potential mirrors are a known launch risk; mirrors are deferred until separately approved |

Signing credentials and release credentials must live only in the approved release environment, never in the repository or application. Because the MVP is not notarized, macOS users may see a Gatekeeper warning; the release documentation must state this clearly.

## 6. Legal, support, and incident response

| Decision | Recorded decision |
| ---------------------------------------------- | --------------------------------------------------------- |
| Terms and investment-research disclaimer owner | @BerryUIKI |
| Privacy notice owner | @BerryUIKI |
| Investment-research disclaimer | Published in-app and in repository documentation |
| Security incident contact | TBD before public release |
| Support channels | GitHub Issues and a contact email published by @BerryUIKI |
| Legal/privacy review | Required before a commercial or public production release |

The product is a research workspace only. It does not execute trades and must not present its output as personalized investment advice.

## 7. Approval state and remaining gates

- [x] Product scope recorded
- [x] Local-only identity approach recorded
- [x] MVP licensing and billing deferral recorded
- [x] Backup and privacy model recorded
- [x] Supported platforms and release ownership recorded
- [ ] Publish the official privacy notice and contact email
- [ ] Assign a security incident contact
- [ ] Complete legal and release-security review before public production release
- [ ] UI locales, launch default, and translation reviewer approved
- [ ] Security reviewer confirms the selected external services and data flows

## Known risks

1. Supporting both macOS and Windows increases release and QA scope.
2. The absence of notarization can create a macOS Gatekeeper warning.
3. GitHub Release availability can be inconsistent for users in Mainland China; a mirror is a later decision.
4. Commercial licensing, payment, and activation decisions must be re-reviewed before implementation.
5. The local-first model deliberately leaves backup custody with the user.

## Decision template

Copy, complete, and send this block back to me. Short answers are fine.

```text
M8 product scope:
- Target customer:
- Commercial model:
- Launch regions:
- Supported UI locales:
- Launch default locale:
- Translation reviewer:

Authentication:
- Required in M8? (yes/no):
- Provider and sign-in methods (or N/A):

Licensing and payments:
- License model:
- Price/currency:
- Billing provider (or none):
- Activation method:

Backup and privacy:
- Automatic cloud backup in M8? (yes/no):
- Manual export in M8? (yes/no):
- Telemetry policy:
- Privacy notice owner/location:

Release:
- Launch platforms and architectures:
- Installer format:
- Signing certificate owner:
- Notarization required? (yes/no):
- Update method:
- Release owner:

Legal and support:
- Terms/disclaimer owner:
- Security incident contact:
- Support channel:
```
