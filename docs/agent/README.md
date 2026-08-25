# AlphaForge Agent Runtime Documentation

This directory is the entry point for AlphaForge's Agent execution architecture.

## Decision Summary

AlphaForge uses a **Rust-supervised subprocess model** for long-running,
tool-using, or third-party Agent workloads. The subprocess is an untrusted
execution worker, not a second application backend.

The trusted Rust host remains the only owner of:

- SQLite and repositories
- operating-system credentials
- provider network requests
- arbitrary filesystem access
- domain writes
- task state, budgets, cancellation, and audit records
- Tauri events and privileged commands

React never starts a process and never receives shell permissions.

## Document Map

| Document | Purpose |
|---|---|
| [ADR-0010](../DECISIONS/0010-managed-agent-worker-subprocess.md) | Accepted decision and alternatives |
| [Subprocess Architecture](SUBPROCESS_ARCHITECTURE.md) | Components, protocol, lifecycle, trust boundaries, and operations |
| [Implementation Roadmap](SUBPROCESS_ROADMAP.md) | Phased delivery plan and acceptance gates |
| [Implementation Checklist](SUBPROCESS_IMPLEMENTATION_CHECKLIST.md) | File-level work packages and verification checklist |
| [Agent Protocol](../AGENT_PROTOCOL.md) | Product task states and frontend event contract |
| [Goose Integration](../goose/README.md) | Specialized Goose worker integration |

## Current State

The current `TaskExecutor` runs provider requests in Tokio tasks inside the
Tauri process. Goose already has a supervised-sidecar adapter and proves the
direction, but there is not yet a shared worker protocol or generic process
supervisor for all Agent backends.

The architecture decision is accepted. Implementation remains planned and must
follow the roadmap gates; these documents do not claim that process isolation is
already complete.
