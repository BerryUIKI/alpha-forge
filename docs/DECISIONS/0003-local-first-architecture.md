# ADR 0003: Local-First Architecture

**Date:** 2026-07-31

**Status:** Accepted

## Context

AlphaForge stores sensitive user data: research documents, investment theses, portfolio positions, and API credentials. We needed to decide whether to build a cloud-first architecture (backend API + database) or a local-first architecture (everything on the user's machine).

## Decision

**Local-first architecture.** All data lives on the user's device. No cloud backend. No multi-tenant database. No user authentication system.

## Rationale

### Data Ownership

Investment research is proprietary. Users should not have to trust a third party with their theses, portfolio data, or research notes. Local-first means the user owns their data completely.

### Privacy

No telemetry, no analytics, no cloud sync unless explicitly enabled. The application works entirely offline (AI features excepted — those require API calls to external providers, which the user configures).

### Simplicity

A cloud backend would require:

- User authentication and session management.
- Multi-tenant database design.
- API server deployment and maintenance.
- Data synchronization logic.
- GDPR/privacy compliance for stored user data.

All of this is unnecessary for a single-user desktop application. Eliminating the cloud backend dramatically reduces development complexity and ongoing operational cost.

### Offline Capability

Research doesn't stop when the internet goes down. Users can browse past research, review theses, and edit notes without connectivity. AI-powered features are the only online-dependent component.

## What "Local-First" Means

| Aspect | Implementation |
|--------|---------------|
| Database | SQLite file in the user's app data directory. |
| File storage | Local filesystem, user's documents directory. |
| Credentials | OS keychain (Keychain, Credential Manager, libsecret). |
| Backups | User-managed. Optional export functionality in future. |
| Sync | None. Optional team sync in future phases (opt-in only). |
| AI API calls | Direct from the user's machine to OpenAI (user's API key). |

## Trade-offs Accepted

- **No multi-device access.** Research is tied to one machine. Mitigated by the desktop-first focus — deep research is typically done on one primary device.
- **No team collaboration in MVP.** Teams cannot share research data. Future phases may add optional, encrypted team sync.
- **User responsible for backups.** No automatic cloud backup. Mitigated by documentation and optional export features.
- **Data loss risk.** If the user's machine fails, data may be lost. Mitigated by encouraging regular backups and future optional sync.

## Consequences

- **Positive:** Complete data ownership, privacy, offline capability, reduced development complexity.
- **Negative:** No multi-device or team access in MVP. User must manage their own backups.
- **Neutral:** AI features require internet (API calls to OpenAI). This is inherent — local LLM inference is out of scope for MVP.
