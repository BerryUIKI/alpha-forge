# M8 Production & Commercialization Decision Record

**Status**: Product decisions recorded; implementation is limited to the public local-desktop MVP.
**Decision owner**: @BerryUIKI
**Recorded**: 2026-08-03

This record distinguishes the free, open-source MVP from later commercial work. It does not authorize authentication, payment collection, cloud storage, telemetry, or automatic investment decisions.

## 1. Product scope

| Decision                       | Recorded decision                                                         |
| ------------------------------ | ------------------------------------------------------------------------- |
| Target release date or quarter | TBD by @BerryUIKI                                                         |
| Primary customer               | Individual investment researchers                                         |
| MVP commercial model           | Free and open-source local desktop application                            |
| Later commercial direction     | Paid commercial offering, to be reviewed before implementation            |
| Cloud account required         | No                                                                        |
| Launch markets                 | Mainland China primary; English-language overseas users secondary         |
| Launch language policy         | Simplified Chinese default client locale; user-selectable English locale  |
| Product coverage               | A-share, Hong Kong, United States equities, options, and futures research |
| Product owner                  | @BerryUIKI                                                                |

## 2. Authentication and identity

Authentication is **not in the MVP**. The app has no account, provider, session, or server-side user-data retention. Any future identity implementation must keep credentials out of React and use an approved native credential store.

## 3. Licensing, payment, and entitlement

Licensing, payment, activation, and entitlement enforcement are **not in the MVP**. They will be reconsidered for the commercial release. The currently preferred later activation mechanism is a signed license, subject to a separate security and legal review. The app must never collect payment-card data directly.

## 4. Backup, export, and privacy

| Decision                    | Recorded decision                                                                                          |
| --------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Automatic cloud backup      | No                                                                                                         |
| Manual local export         | Yes                                                                                                        |
| Storage provider and region | None; no cloud storage is operated                                                                         |
| Backup behavior             | Native save dialog; consistent SQLite export; existing files are not overwritten                           |
| Restore conflict behavior   | Not applicable; restore/import is not part of this MVP change                                              |
| Telemetry                   | Disabled by default; no analytics service is configured                                                    |
| Privacy notice              | Published by @BerryUIKI on the official website; mirrored in the app's About and privacy settings          |
| Retention and deletion      | The user controls local application data and exported files; the application operates no user-data service |

Manual exports are user-controlled copies and may contain sensitive research. The user is responsible for choosing a safe destination and retaining backups. The product must not silently upload, synchronize, or overwrite local data.

## 5. Platforms, updates, and distribution

| Decision                    | Recorded decision                                                                                                 |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Supported platforms         | macOS on Apple Silicon and Windows                                                                                |
| Installer formats           | macOS DMG and Windows EXE (NSIS)                                                                                  |
| Linux                       | Not in the MVP release scope                                                                                      |
| Code-signing owner          | @BerryUIKI                                                                                                        |
| macOS notarization          | Not planned for the MVP                                                                                           |
| Update discovery            | The client queries GitHub Releases only when the user requests a check; downloads and installation are manual     |
| Release owner               | @BerryUIKI                                                                                                        |
| Release visibility in China | GitHub connectivity and potential mirrors are a known launch risk; mirrors are deferred until separately approved |

Signing credentials and release credentials must live only in the approved release environment, never in the repository or application. Because the MVP is not notarized, macOS users may see a Gatekeeper warning; the release documentation must state this clearly.

## 6. Legal, support, and incident response

| Decision                                       | Recorded decision                                         |
| ---------------------------------------------- | --------------------------------------------------------- |
| Terms and investment-research disclaimer owner | @BerryUIKI                                                |
| Privacy notice owner                           | @BerryUIKI                                                |
| Investment-research disclaimer                 | Published in-app and in repository documentation          |
| Security incident contact                      | TBD before public release                                 |
| Support channels                               | GitHub Issues and a contact email published by @BerryUIKI |
| Legal/privacy review                           | Required before a commercial or public production release |

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

## Known risks

1. Supporting both macOS and Windows increases release and QA scope.
2. The absence of notarization can create a macOS Gatekeeper warning.
3. GitHub Release availability can be inconsistent for users in Mainland China; a mirror is a later decision.
4. Commercial licensing, payment, and activation decisions must be re-reviewed before implementation.
5. The local-first model deliberately leaves backup custody with the user.
