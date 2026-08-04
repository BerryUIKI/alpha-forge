# ADR-0004: Goose Agent Integration Topology

## Status

Proposed

## Context

Milestone M10 requires integrating the Goose agent runtime to improve evidence-grounded research while AlphaForge retains task control, permissions, credentials, persistence, and mandatory human review.

Upstream facts verified 2026-08-03:
- **Source**: github.com/aaif-goose/goose
- **License**: Apache 2.0, Agentic AI Foundation (Linux Foundation)
- **Interfaces**: Desktop app, CLI, and API
- **MCP**: Model Context Protocol with 70+ extensions
- **Recipes**: Portable YAML configs with structured response schema support
- **Permissions**: Tool levels (Always Allow / Ask Before / Never Allow)
- **Headless**: CLI execution via `goose run --recipe`

Three integration approaches considered:

### Option A: Pinned Bundled Sidecar (Supervised by Rust)

Goose binary bundled with AlphaForge, started as a supervised subprocess.

**Pros**:
- Clear isolation boundary
- Version pinning with integrity verification
- Process lifecycle control (timeout, cancellation, crash recovery)
- Independent update cycle from AlphaForge core
- Failure isolation (crash doesn't affect AlphaForge)

**Cons**:
- Larger package size (~50-100MB additional)
- Update requires new AlphaForge release or explicit migration
- Process IPC overhead

### Option B: Direct Rust Library Integration

Link Goose as a Rust crate dependency.

**Pros**:
- No separate process management
- Lower latency
- Smaller package

**Cons**:
- Tight coupling to Goose internal APIs (unstable)
- Error propagation complexity
- No isolation boundary
- Goose update forces AlphaForge rebuild
- Unsafe defaults harder to prevent

### Option C: Local Loopback API Process with Authenticated Transport

Goose runs as a persistent local API service.

**Pros**:
- Service reuse across sessions
- HTTP/gRPC interface familiarity

**Cons**:
- Complex lifecycle (service must be started before AlphaForge)
- Persistent attack surface
- Secret management for transport auth
- Resource usage when idle
- Recovery complexity on service crash

## Decision

**Selected: Option A — Pinned Bundled Sidecar (Supervised by Rust)**

Implementation details:

1. **Binary Management**
   - Exact version pinned in `Cargo.toml` or build config
   - SHA-256 checksum verified at startup
   - Binary bundled in platform-specific releases
   - No auto-download or user-supplied executable path
   - Fail closed if integrity check fails

2. **Process Lifecycle**
   - Rust `GooseAdapter` trait owns process spawning
   - No shell involvement; direct executable invocation
   - Fixed argument list from validated recipe
   - Timeout enforced by Rust supervisor
   - Cancellation via process termination + cleanup
   - Orphan prevention on crash/restart

3. **Credential Model**
   - AlphaForge-owned provider gateway (recommended)
   - Rust retains credentials in OS keyring or encrypted storage
   - Never passed via CLI args, env vars, or recipe files
   - Goose session receives time-limited token if needed
   - No file-based secret fallback in production

4. **Recipe Constraints**
   - Explicit structured response JSON schema required
   - Only allowlisted AlphaForge MCP extension enabled
   - No Developer, Computer Controller, Extension Manager, Summon
   - Max turns, output bytes, token/cost budgets enforced
   - User-provided recipe content prohibited

5. **Update Strategy**
   - Version bump requires explicit review and ADR update
   - Compatibility matrix documented per platform
   - Rollback possible via previous AlphaForge version
   - SBOM includes Goose version and dependencies

## Consequences

### Positive
- Security boundary between Goose and AlphaForge
- Controlled, auditable execution surface
- Clear failure isolation
- Cancellation and timeout enforcement at process level
- Credentials never exposed to Goose environment

### Negative
- Package size increase (~50-100MB)
- Platform-specific binary management complexity
- Update coordination required

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Binary tampering | SHA-256 verification at startup; fail closed |
| Process escape | No shell access; bounded args; no arbitrary MCP |
| Recipe injection | Fixed recipes; no user-provided recipe content |
| Credential exposure | AlphaForge-owned gateway; no CLI/env secrets |
| Version drift | Pinned version; explicit update process |
| Orphan processes | Rust supervisor tracks PID; cleanup on exit |

## Compliance

- Apache 2.0 license attribution required in release artifacts
- No copyleft obligations
- Agentic AI Foundation governance acceptable for post-MVP

## References

- [M10 Milestone Definition](../MILESTONE_ROADMAP.md#m10--goose-agent-integration-)
- [Goose Integration Roadmap](../goose/INTEGRATION_ROADMAP.md)
- [Goose Recipe Reference](https://goose-docs.ai/docs/guides/recipes/recipe-reference/)
- [Goose Headless Mode](https://goose-docs.ai/docs/tutorials/headless-goose/)
