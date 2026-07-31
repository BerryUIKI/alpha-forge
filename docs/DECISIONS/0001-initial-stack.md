# ADR 0001: Initial Technology Stack

**Date:** 2026-07-31

**Status:** Accepted

## Context

We needed to choose a technology stack for a desktop-first AI investment research workspace. The stack must support: a rich UI, local data persistence, background task execution, AI API integration, and secure credential management.

## Decision

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Desktop Shell | **Tauri 2** | Smaller binary, Rust backend, native performance. |
| Backend Language | **Rust** | Memory safety, async runtime (Tokio), strong type system. |
| Frontend Framework | **React 19** | Mature ecosystem, component model, broad AI agent familiarity. |
| Frontend Language | **TypeScript** | Type safety across the frontend, Zod for runtime validation. |
| Build Tool | **Vite 6** | Fast HMR, native ESM, good Tauri integration. |
| CSS | **Tailwind CSS 4** | Utility-first, fast iteration, consistent design system. |
| UI Components | **shadcn/ui + Radix** | Accessible, customizable, no vendor lock-in. |
| Database | **SQLite via SQLx** | Zero-config, local-first, async Rust driver. |
| Package Manager | **pnpm** | Fast, disk-efficient, strict dependency resolution. |

### Alternatives Considered

#### Electron instead of Tauri

- **Pro:** Larger ecosystem, more tutorials, Chrome DevTools.
- **Con:** ~150MB binary, separate Chromium instance, JavaScript backend.
- **Decision:** Tauri chosen for smaller footprint, Rust backend, and native performance. The trade-off is a smaller ecosystem, acceptable for a focused desktop application.

#### Next.js instead of Vite + React Router

- **Pro:** Server-side rendering, file-based routing.
- **Con:** Unnecessary for a desktop app (no SSR needed). Adds complexity.
- **Decision:** Vite + React Router for simplicity. Desktop apps don't benefit from SSR.

#### PostgreSQL instead of SQLite

- **Pro:** More powerful, concurrent access.
- **Con:** Requires separate server process, configuration, and maintenance.
- **Decision:** SQLite for zero-config local-first architecture. Sufficient for a single-user desktop application.

## Consequences

- **Positive:** Small binary, fast startup, local-first data ownership.
- **Negative:** Smaller Tauri ecosystem means fewer off-the-shelf plugins. Rust learning curve for contributors.
- **Neutral:** shadcn/ui requires manual component installation (not a library import), but gives full control over component code.
