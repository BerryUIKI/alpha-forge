# Goose Integration Threat Model

## Scope

This threat model covers the **read-only first use case** for Goose integration (M10-G1 through M10-G3):

> User selects "Goose shadow analysis" → Goose reads allowlisted AlphaForge context → Structured claims/evidence/risks returned → User reviews Artifact → No domain record changes automatically

This model does not cover:
- Proposal acceptance flow (M10-G4)
- Credential/provider policy variations (M10-G5)
- Packaged release distribution (M10-G6)

## Trust Boundaries

```
┌─────────────────────────────────────────────────────────────────────┐
│                     UNTRUSTED ZONE                                  │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐ │
│  │  Goose Process  │    │  MCP Extension  │    │ External APIs   │ │
│  │  (Sidecar)      │    │  (if loaded)    │    │ (OpenAI, etc.)  │ │
│  └────────┬────────┘    └────────┬────────┘    └────────┬────────┘ │
│           │                      │                      │          │
└───────────┼──────────────────────┼──────────────────────┼──────────┘
            │                      │                      │
            │  stdin/stdout/IPC    │  MCP Protocol        │  HTTP
            │                      │                      │
┌───────────┼──────────────────────┼──────────────────────┼──────────┐
│           ▼                      ▼                      ▼          │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │                    TRUSTED ZONE                            │   │
│  │              AlphaForge Rust Runtime                       │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐│   │
│  │  │  GooseAdapter   │  │  MCP Bridge     │  │  Services   ││   │
│  │  │  (Supervisor)   │  │  (Read-only)    │  │  (Domain)   ││   │
│  │  └────────┬────────┘  └────────┬────────┘  └──────┬──────┘│   │
│  │           │                    │                   │       │   │
│  │           └────────────────────┼───────────────────┘       │   │
│  │                                ▼                           │   │
│  │  ┌────────────────────────────────────────────────────┐   │   │
│  │  │              SQLite + Filesystem                   │   │   │
│  │  │              (Protected by Rust)                   │   │   │
│  │  └────────────────────────────────────────────────────┘   │   │
│  └────────────────────────────────────────────────────────────┘   │
│                        TRUSTED ZONE                               │
└─────────────────────────────────────────────────────────────────────┘
```

## Assets

| Asset | Location | Sensitivity | Protection |
|-------|----------|-------------|------------|
| SQLite database | Rust process | High | No Goose access |
| API keys / credentials | OS keyring / encrypted | Critical | Never passed to Goose |
| User workspace data | SQLite | High | Read-only via MCP bridge |
| Research thesis / decisions | SQLite | High | No direct access |
| Goose recipe | Rust-owned | Medium | Fixed, validated |
| Goose output | stdin/stdout | Medium | Validated before use |

## Threat Categories

### T1: Prompt Injection

**Description**: Malicious content in research sources causes Goose to execute unintended actions.

**Attack Vector**:
- User imports compromised research document
- Document contains hidden prompt injection
- Goose interprets injection as instructions
- Goose attempts to exfiltrate data or perform writes

**Mitigations**:
1. **Read-only MCP bridge**: No write tools available
2. **Allowlisted tools only**: Goose cannot invoke arbitrary commands
3. **Workspace scoping**: Rust injects authorized workspace ID; IDs from Goose ignored
4. **Output validation**: Structured schema validation; unexpected fields rejected
5. **No shell access**: Goose runs as direct process, no shell interpolation

**Residual Risk**: Low — injection cannot cause writes; output validated before persistence

### T2: Malicious MCP Server

**Description**: Goose loads an MCP server that bypasses AlphaForge constraints.

**Attack Vector**:
- Recipe specifies unauthorized MCP extension
- Extension provides filesystem/shell access
- Goose escapes read-only boundary

**Mitigations**:
1. **Fixed recipes**: No user-provided recipe content
2. **Allowlisted extensions**: Only AlphaForge MCP bridge permitted
3. **Extension deny-by-default**: All extensions not explicitly allowed are blocked
4. **Recipe validation**: Rust validates recipe before execution
5. **No network MCP**: External MCP servers not permitted

**Residual Risk**: Very low — recipe controlled by AlphaForge; no extension loading

### T3: Recipe Tampering

**Description**: Recipe file modified to enable unsafe capabilities.

**Attack Vector**:
- Attacker modifies bundled recipe file on disk
- Recipe enables Developer or filesystem extension
- Goose gains write access

**Mitigations**:
1. **Recipe in binary or signed storage**: Recipe content embedded or cryptographically verified
2. **No user-provided recipes**: Recipe source is AlphaForge-owned
3. **Checksum verification**: Recipe hash validated at runtime
4. **Fail closed**: Any tampering detected → abort Goose execution

**Residual Risk**: Very low — recipe integrity verified

### T4: Process Escape

**Description**: Goose process escapes supervision and performs unauthorized actions.

**Attack Vector**:
- Goose spawns child processes
- Goose accesses filesystem directly
- Goose makes network calls to exfiltrate data

**Mitigations**:
1. **No shell**: Direct process spawn, no shell wrapper
2. **Bounded arguments**: Fixed argument list; user input never in command line
3. **Process supervision**: Rust tracks PID; timeout enforced; termination on cancel
4. **Network allowlist**: Only approved provider endpoints permitted (if network access granted)
5. **Sandbox options**: Consider OS-level sandboxing (seccomp, AppArmor, Windows Integrity Level)

**Residual Risk**: Low — process boundaries enforced; escape requires OS vulnerability

### T5: Path Traversal

**Description**: Goose attempts to access files outside workspace scope.

**Attack Vector**:
- Goose MCP bridge receives path input
- Path contains `../` or absolute path to sensitive files
- Bridge returns unauthorized file content

**Mitigations**:
1. **No path inputs**: MCP bridge uses IDs, not paths
2. **ID validation**: Rust validates IDs against authorized workspace
3. **Path normalization**: Any path generated by Rust is normalized
4. **Scope enforcement**: Rust attaches workspace scope; Goose-provided IDs rejected

**Residual Risk**: Very low — no path-based API; ID-based access controlled by Rust

### T6: Secret Exposure

**Description**: Credentials or secrets leaked to Goose process.

**Attack Vector**:
- Credential passed via environment variable
- Credential in CLI argument
- Credential in recipe file
- Credential in log output

**Mitigations**:
1. **AlphaForge-owned gateway**: Rust manages credentials; Goose never sees them
2. **No env/CLI secrets**: Credentials never in Goose process environment
3. **Redacted logs**: Goose stderr redacted before logging
4. **No file-based secrets**: Production config disables Goose file-based credential fallback

**Residual Risk**: Very low — credential flow isolated to Rust

### T7: Unbounded Output / Resource Exhaustion

**Description**: Goose produces excessive output or consumes unbounded resources.

**Attack Vector**:
- Goose generates massive response (memory exhaustion)
- Goose loops indefinitely (CPU/time exhaustion)
- Goose accumulates context without limit (token exhaustion)

**Mitigations**:
1. **Output byte limit**: Rust enforces max output bytes; truncates if exceeded
2. **Timeout**: Hard timeout enforced by Rust supervisor
3. **Max turns**: Recipe includes `max_turns` setting
4. **Token budget**: Cost tracking enforced per task
5. **Cancellation**: User can cancel; process terminated immediately

**Residual Risk**: Low — resource limits enforced at supervisor level

### T8: Unauthorized Writes

**Description**: Goose attempts to modify AlphaForge state without user approval.

**Attack Vector**:
- Goose output contains malicious payload
- Rust blindly persists output
- State corrupted or malicious data injected

**Mitigations**:
1. **Read-only first use case**: No write capability in M10-G1 through M10-G3
2. **Schema validation**: Output validated against strict schema
3. **No auto-persist**: Output shown to user; persistence requires explicit approval (M10-G4)
4. **Service layer**: All writes go through existing Rust services with validation

**Residual Risk**: None for read-only use case; M10-G4 requires separate threat model review

## Attack Tree: Read-Only Shadow Analysis

```
Goal: Compromise AlphaForge via Goose read-only integration

OR
├── Access SQLite directly
│   └── Blocked: No SQLite handle passed to Goose
│
├── Access filesystem arbitrarily
│   ├── Via MCP extension
│   │   └── Blocked: Only AlphaForge bridge allowed; no filesystem tools
│   └── Via path traversal
│       └── Blocked: MCP uses IDs, not paths; IDs validated against workspace
│
├── Execute arbitrary code
│   ├── Via shell command
│   │   └── Blocked: No shell; direct process spawn
│   └── Via malicious MCP
│       └── Blocked: Extension allowlist; no user-provided MCP
│
├── Exfiltrate data
│   ├── Via network
│   │   └── Mitigated: Network restricted to approved providers
│   └── Via output
│       └── Mitigated: Output validated; size-limited; user reviews before persist
│
├── Inject malicious output
│   └── Mitigated: Schema validation; no auto-persist; user reviews
│
└── Exhaust resources
    └── Mitigated: Timeout, max turns, output limit, token budget
```

## Security Requirements

### Mandatory for M10-G1 (Spike)

| Requirement | Verification |
|-------------|--------------|
| Binary integrity check at startup | Unit test: modified binary rejected |
| No shell in process spawn | Code review: `Command::new()` without shell |
| Recipe validation before execution | Unit test: invalid recipe rejected |
| Output byte limit enforced | Integration test: large output truncated |
| Timeout enforced | Integration test: hanging process killed |
| Cancellation terminates process | Integration test: cancel kills process |
| No orphan processes on crash | Integration test: crash cleanup verified |

### Mandatory for M10-G2 (MCP Bridge)

| Requirement | Verification |
|-------------|--------------|
| Only allowlisted tools available | Integration test: unknown tool rejected |
| Workspace ID injected by Rust | Unit test: Goose-provided ID ignored |
| Path traversal blocked | Integration test: `../` and absolute paths rejected |
| Cross-workspace access blocked | Integration test: unauthorized workspace ID rejected |
| Output size limits | Integration test: oversized query rejected |

### Mandatory for M10-G3 (Shadow Mode)

| Requirement | Verification |
|-------------|--------------|
| No auto-persist | E2E test: output not persisted without approval |
| Schema validation | Unit test: invalid schema rejected |
| User reviews output | E2E test: output shown in Artifact |
| Provenance recorded | Integration test: source IDs captured |

## Residual Risk Summary

| Threat | Initial Risk | Mitigations | Residual Risk |
|--------|--------------|-------------|---------------|
| Prompt Injection | High | Read-only MCP, output validation | Low |
| Malicious MCP | High | Allowlist, fixed recipes | Very Low |
| Recipe Tampering | Medium | Integrity check, embedded recipe | Very Low |
| Process Escape | High | No shell, supervision, sandbox | Low |
| Path Traversal | Medium | ID-based API, scope enforcement | Very Low |
| Secret Exposure | Critical | AlphaForge gateway, no env secrets | Very Low |
| Resource Exhaustion | Medium | Limits, timeout, cancellation | Low |
| Unauthorized Writes | High | Read-only mode, no auto-persist | None |

## Approval

This threat model approves the **read-only first use case** for M10-G1 through M10-G3, subject to:

1. Implementation of all mandatory security requirements
2. Verification tests passing before each gate
3. Separate review for M10-G4 (proposal acceptance) before enabling write proposals

**Approved for**: M10-G1 (Spike), M10-G2 (MCP Bridge), M10-G3 (Shadow Mode)
**Not approved for**: M10-G4 (requires additional threat model for write proposals)
