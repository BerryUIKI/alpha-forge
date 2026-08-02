# M8 Production & Commercialization Decision Workbook

**Status**: Awaiting product-owner decisions
**How to use this file**: Replace every `TODO` with an answer, select one option where choices are listed, and add a short rationale for any non-default choice. You may also reply in chat using the template at the end of this document.

M8 introduces external services and release responsibilities. No authentication provider, billing service, cloud storage, analytics platform, or release identity will be added until these decisions are approved.

## Recommended MVP boundary

The recommended first commercial release keeps the local-first product model:

- No required account or cloud backend.
- Local data remains the source of truth.
- Optional manual export is preferred over automatic cloud backup.
- A simple license file or offline entitlement is preferred over subscription billing.
- macOS is the first supported release platform, with Windows considered after the release workflow is proven.

Choose a different direction only if it is a deliberate product requirement.

## 1. Product scope

| Decision | Your answer | Recommended default | Why it is needed |
|---|---|---|---|
| Target release date or quarter | TODO | No fixed date until the gate is approved | Determines scope and release planning |
| Primary customer | TODO | Individual investment researcher | Guides pricing, privacy, and support |
| Commercial model | TODO | Paid local desktop application | Defines entitlement work |
| Is a cloud account required in M8? | TODO: Yes / No | No | Changes architecture and privacy obligations |
| Countries or regions to support at launch | TODO | Your home market only | Affects payment, tax, legal, and privacy work |
| Product owner | TODO | Named individual | Owns scope decisions |

## 2. Authentication and identity

Complete this section only if an account is required. If not, write `Not in M8` for every provider-specific answer.

| Decision | Your answer | Recommended default |
|---|---|---|
| Is sign-in required for the desktop MVP? | TODO: Yes / No | No |
| Identity approach | TODO: local-only / Clerk / Auth0 / Supabase Auth / other | Local-only |
| Supported sign-in methods | TODO | None for local-only |
| Account deletion and data-export requirement | TODO | Local export only; no account if local-only |
| Session expiry and sign-out policy | TODO | N/A for local-only |
| Identity-data region and retention | TODO | N/A for local-only |
| Owner | TODO | Product owner |

**Required boundary if sign-in is approved**: credentials and refresh tokens must never reach React; tokens are stored only through the approved native credential mechanism; sign-out clears local session state; authentication errors are recoverable and user-facing.

## 3. Licensing, payment, and entitlement

| Decision | Your answer | Recommended default |
|---|---|---|
| License model | TODO: perpetual / subscription / free trial / other | Perpetual with optional paid upgrades |
| Price and currency | TODO | TBD |
| Billing provider | TODO: none / Stripe / Paddle / Lemon Squeezy / other | None for the first offline license workflow |
| License activation method | TODO: license file / activation code / account sign-in / other | Signed license file |
| Offline grace period | TODO | Perpetual license: no periodic online check |
| Refund and cancellation policy owner | TODO | Product owner / legal reviewer |
| Entitlements | TODO | App access and version/update eligibility |

**Required boundary**: the app must have deterministic entitlement behavior, explain an expired or invalid license clearly, and never collect payment data directly.

## 4. Backup, export, and privacy

| Decision | Your answer | Recommended default |
|---|---|---|
| Is automatic cloud backup in scope? | TODO: Yes / No | No |
| Is manual local export in scope? | TODO: Yes / No | Yes |
| Backup or storage provider | TODO | None for automatic backup |
| Storage region | TODO | N/A when no cloud storage exists |
| Encryption model | TODO | Local export is user-controlled; cloud backup requires client-side encryption |
| Retention and deletion policy | TODO | User-controlled local files; no server retention |
| Restore conflict behavior | TODO | N/A without sync/backup |
| Privacy notice publication location and owner | TODO | Product website / product owner |
| Telemetry policy | TODO: none / opt-in anonymous / other | No telemetry by default |

**Required boundary for cloud backup**: it is opt-in, presents progress and failure states, preserves research provenance, documents deletion behavior, and cannot silently overwrite local work.

## 5. Platform, signing, updates, and distribution

| Decision | Your answer | Recommended default |
|---|---|---|
| Supported platforms at launch | TODO | macOS only |
| macOS CPU architectures | TODO: Apple Silicon / Intel / universal | Apple Silicon first |
| Windows support date | TODO | After macOS release workflow is proven |
| Linux support date | TODO | Not in first commercial release |
| Installer format per platform | TODO | macOS DMG |
| Code-signing certificate owner | TODO | Named release owner / organization account |
| macOS notarization required? | TODO: Yes / No | Yes |
| Update delivery | TODO: manual download / Tauri updater / other | Manual download first |
| Update channel | TODO: stable / beta / both | Stable only |
| Rollback procedure owner | TODO | Release owner |
| Release owner | TODO | Named individual |

**Required boundary**: signing keys and release credentials live only in the approved CI secret manager; no signing secret is committed or placed in the app; every release has a documented rollback procedure.

## 6. Legal, support, and incident response

| Decision | Your answer | Recommended default |
|---|---|---|
| Terms of use owner and publication location | TODO | Product owner / website |
| Investment-research disclaimer owner | TODO | Product owner / legal reviewer |
| Privacy reviewer | TODO | Named reviewer |
| Security incident contact | TODO | Dedicated security email address |
| Support channel | TODO | Support email address |
| Supported response time | TODO | Best effort during beta |

## 7. Approval checklist

- [ ] Product scope approved
- [ ] Identity approach approved or explicitly deferred
- [ ] Licensing and billing model approved
- [ ] Backup and privacy model approved
- [ ] Supported platforms and release process approved
- [ ] Legal and support owners assigned
- [ ] Security reviewer confirms the selected external services and data flows

| Role | Name | Date | Approval / notes |
|---|---|---|---|
| Product owner | TODO | TODO | TODO |
| Security reviewer | TODO | TODO | TODO |
| Release owner | TODO | TODO | TODO |

## Chat reply template

Copy, complete, and send this block back to me. Short answers are fine.

```text
M8 product scope:
- Target customer:
- Commercial model:
- Launch regions:

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
