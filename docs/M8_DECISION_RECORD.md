# M8 Production & Commercialization Decision Record

**Status**: Pending product-owner decisions

This record must be completed before M8 implementation begins. It prevents the desktop application from embedding an unapproved identity provider, payment system, cloud service, or signing identity.

## 1. Authentication

| Decision | Required answer |
|---|---|
| Is authentication required for the desktop MVP? | |
| Identity provider or local-only approach | |
| Supported sign-in methods | |
| Session storage and expiry policy | |
| Account deletion and data-export requirements | |
| Owner | |

**Acceptance criteria**: Credentials and refresh tokens never reach React, sign-out clears local session state, and authentication failure has a recoverable user-facing state.

## 2. Licensing and Subscription

| Decision | Required answer |
|---|---|
| Licensing model | Perpetual / subscription / trial / other |
| Billing provider | |
| Entitlements | |
| Offline grace period | |
| Refund and cancellation behavior | |
| Owner | |

**Acceptance criteria**: License checks are deterministic, cached entitlement data has an expiry policy, and the application remains usable according to the approved offline policy.

## 3. Cloud Backup (Optional)

| Decision | Required answer |
|---|---|
| Is cloud backup in scope? | Yes / No |
| Storage provider and region | |
| Encryption model | |
| Retention and deletion policy | |
| Restore conflict behavior | |
| Owner | |

**Acceptance criteria**: Backup is opt-in, preserves research provenance, and exposes clear progress, failure, and restore-conflict states.

## 4. Release and Signing

| Decision | Required answer |
|---|---|
| Supported platforms | |
| Signing certificate owner | |
| Notarization requirements | |
| Installer format per platform | |
| Update channel and rollback policy | |
| Release owner | |

**Acceptance criteria**: Release secrets are stored only in the approved CI secret manager, release artifacts are signed for every supported platform, and an update rollback procedure is documented and tested.

## 5. Security and Privacy Approval

| Decision | Required answer |
|---|---|
| Privacy notice owner and publication location | |
| Data classification and telemetry policy | |
| Incident-response contact | |
| Security review approver | |

**Approval**

| Role | Name | Date |
|---|---|---|
| Product owner | | |
| Security reviewer | | |
| Release owner | | |
