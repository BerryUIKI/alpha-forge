# Goose Agent Integration Roadmap

## Milestone position

This roadmap implements M10 after the M8 MVP completion gate and, by default, after M9. The product owner may authorize it as an independent post-MVP workstream, but it can never move into the MVP. M10 is not active merely because planning documents exist.

## Impact assessment

| Area               | Expected impact                                                                                    |
| ------------------ | -------------------------------------------------------------------------------------------------- |
| Frontend           | Agent mode selector, consent/approval UI, progress, result review, error states                    |
| Rust               | Goose adapter, supervised process/API client, MCP bridge, policy enforcement, budgets, persistence |
| Database           | Append-only run/provenance fields only if existing task/event storage is insufficient              |
| Tauri              | No React shell permission; fixed sidecar/resource configuration may be required                    |
| Artifacts          | Validated Goose result rendered by predefined components                                           |
| Plugins/extensions | Allowlisted AlphaForge MCP surface; arbitrary Goose extensions disabled                            |
| Tests              | Adapter contract, policy, process lifecycle, recipe schema, redaction, E2E, packaging              |
| Documentation      | ADRs, version matrix, threat model, operator troubleshooting, release notes                        |

## M10-G0: Revalidation and architecture decision ✅

**Status:** Complete (ADR-0004 Accepted)

**Goal:** select an upstream version and integration surface using current evidence.

1. Verify the AAIF repository, license, supported platforms, release signatures/checksums, CLI/API stability, and security policy.
2. Compare three approaches:
   - pinned bundled sidecar supervised by Rust;
   - direct Rust library integration;
   - local loopback API process with authenticated transport.
3. Produce an ADR recording packaging size, update coupling, cancellation, streaming, credential ownership, observability, and license obligations.
4. Produce a threat model covering prompt injection, malicious MCP servers, recipe tampering, process escape, path traversal, secret exposure, unbounded output, and unauthorized writes.
5. Define the first read-only research use case and its structured response schema.

**Recommended starting decision:** a pinned sidecar behind a `GooseAdapter` trait, no auto-download, no user-supplied executable path, and an explicit recipe with only the AlphaForge read-only MCP extension.

**Exit gate:** ADR and threat model approved; exact version, checksum/source, platforms, and removal plan recorded.

## M10-G1: Isolated technical spike ✅

**Status:** Complete

**Goal:** prove lifecycle and structured-output behavior without accessing real user data.

Create a disposable spike branch and test fixture that:

- Starts the pinned Goose runtime from Rust without a shell.
- Supplies a fixed recipe with an explicit structured response JSON Schema.
- Enables no Developer, Computer Controller, Extension Manager, Summon/subagent, filesystem, or arbitrary external MCP extension.
- Uses synthetic research sources from a temporary, bounded fixture.
- Streams progress into AlphaForge task events.
- Enforces timeout, maximum turns, output bytes, token/cost budget, cancellation, and concurrent-run limit.
- Captures exit status and bounded redacted stderr without leaking prompts or credentials.
- Terminates and cleans its task-owned temporary directory after success, failure, cancellation, and application restart.

The spike must be discarded or isolated if it cannot meet the policy. Do not normalize unsafe defaults merely to make it run.

**Exit gate:** deterministic fixture passes on supported development platforms; cancellation and failure leave no orphan process; output validates before use.

## M10-G2: Read-only AlphaForge MCP bridge ✅

**Status:** Complete

**Goal:** give Goose narrowly scoped research context without exposing storage or privileged capabilities.

Recommended first tools:

| Tool                      | Input                              | Output                                     | Permission |
| ------------------------- | ---------------------------------- | ------------------------------------------ | ---------- |
| `get_workspace_summary`   | workspace ID from Rust-owned scope | names and counts, no credentials           | Read only  |
| `search_research_sources` | validated query and bounded limit  | source IDs, titles, excerpts, provenance   | Read only  |
| `get_research_source`     | allowlisted source ID              | bounded content and metadata               | Read only  |
| `get_thesis_context`      | allowlisted thesis ID              | thesis, evidence links, confidence history | Read only  |
| `list_related_artifacts`  | workspace and entity scope         | validated artifact metadata                | Read only  |

The bridge calls AlphaForge services; it never gives Goose a database handle, SQL tool, arbitrary path, unrestricted URL fetcher, or Tauri command proxy. Rust attaches the authorized workspace and task scope rather than trusting IDs supplied by the model.

MCP responses have schema validation, size limits, pagination, provenance, and redaction. Tool names and descriptions must not imply write authority.

**Exit gate:** policy tests prove cross-workspace IDs, traversal strings, oversized queries, unknown tools, and write attempts are rejected.

## M10-G3: Shadow-mode research slice

**Status:** Active

```text
User selects “Goose shadow analysis”
  -> reviews sources and budget
  -> starts task
  -> Rust launches bounded recipe
  -> Goose reads allowlisted context
  -> structured claims/evidence/risks returned
  -> Rust validates and persists run provenance
  -> user reviews Artifact
  -> no domain record changes automatically
```

Required response fields:

```json
{
  "summary": "",
  "claims": [],
  "evidence": [],
  "contradictions": [],
  "risks": [],
  "unknowns": [],
  "sourceIds": [],
  "confidence": 0,
  "provider": "",
  "model": "",
  "recipeVersion": ""
}
```

The frontend implements initial, loading, progress, waiting, completed, empty, partial, offline, failed, and cancelled states. It shows budgets and data scope before execution and provenance after execution.

**Exit gate:** a user can run, cancel, inspect, and later reopen an evidence-grounded result; no Goose output becomes thesis evidence or a decision without confirmation.

## M10-G4: Human-approved proposals

**Goal:** let Goose propose structured changes while AlphaForge remains the only writer.

1. Add proposal schemas for research notes, evidence candidates, report outlines, or Artifact payloads.
2. Render a field-level preview with source links and uncertainty.
3. Require an explicit user action to accept each proposal.
4. Revalidate the accepted payload in Rust and call the normal domain service.
5. Store proposer metadata, recipe/version, accepted/rejected state, user timestamp, and resulting entity ID.

Do not expose generic create/update/delete MCP tools. Do not allow proposals for trades, target positions, or autonomous portfolio changes.

**Exit gate:** all writes use existing services, are attributable to a user confirmation, and are reversible where the domain allows.

## M10-G5: Credentials and provider policy

Choose and document exactly one credential model:

- **Goose-owned keyring for the pilot:** Goose reads an OS-keyring secret configured outside React. AlphaForge does not pass a key in arguments, logs, recipes, or plaintext files.
- **AlphaForge-owned provider gateway:** Rust retains credentials and exposes a constrained authenticated local provider interface to the sidecar.

The second model offers centralized policy but is more complex. In either case:

- React never reads provider secrets.
- No secret appears in environment dumps, CLI arguments, recipes, logs, crash reports, or SQLite.
- File-based Goose secret fallback is disabled for a production integration unless a separate security review approves it.
- Provider/model allowlists, cost ceilings, and data-retention disclosures are visible to the user.

**Exit gate:** credential flow and provider data handling pass security review on every supported platform.

## M10-G6: Packaging, update, and support

- Bundle or acquire Goose only through the approved release process with an exact version and integrity record.
- Do not download or execute a newer runtime automatically.
- Maintain a platform/architecture compatibility matrix and software bill of materials.
- Include upstream license and attribution in release artifacts.
- Verify macOS and Windows packaging, quarantine/signing behavior, first-run errors, missing binary, corrupted binary, and version mismatch.
- Add diagnostics that report versions, enabled policy profile, last stable error code, and process state without secrets or sensitive content.
- Document how to disable Goose and reopen existing results if the runtime is unavailable.

**Exit gate:** packaged builds reproduce the approved binary, pass smoke tests, and fail closed when integrity or compatibility checks fail.

## Verification matrix

| Gate                       | Evidence                                                                                           |
| -------------------------- | -------------------------------------------------------------------------------------------------- |
| Functional                 | Adapter, recipe, MCP, service, task event, persistence, UI, and E2E tests                          |
| Safety                     | Threat-model cases, tool allowlist, workspace isolation, proposal confirmation, no trade paths     |
| Reliability                | Timeout, retry policy, cancellation, crash, restart recovery, output limit, concurrency tests      |
| Financial research quality | Claim-to-source traceability, contradictions, stale-source behavior, confidence calibration review |
| Security                   | Binary integrity, credential redaction, path/URL validation, malicious recipe/extension rejection  |
| Release                    | macOS and Windows packaged smoke tests, SBOM, attribution, rollback/disable procedure              |

Run the standard repository checks plus Goose-specific integration and packaged tests:

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pnpm test:e2e
pnpm tauri build
```

## Rollout stages

1. Developer fixtures only.
2. Internal read-only shadow mode with synthetic and copied non-sensitive workspaces.
3. Opt-in beta for real local workspaces with explicit scope review.
4. Human-approved proposals.
5. General availability only after the security, quality, packaging, and support gates pass.

Every stage has a kill switch that prevents new Goose runs while retaining already persisted, validated results.

## Definition of done

- M8 is complete and M10 was explicitly activated.
- An ADR pins the upstream source, version, integration surface, license obligations, and credential model.
- Goose has no direct SQLite, arbitrary filesystem, shell, credential, or trade capability.
- All tools are allowlisted, read-only, typed, scoped, bounded, and tested.
- Tasks support progress, cancellation, failure, restart handling, time/token/cost budgets, and concurrency limits.
- Outputs are structured, validated, evidence-grounded, and rendered through controlled Artifacts.
- Domain writes require user confirmation and pass through existing Rust services.
- Supported packaged builds pass integrity and smoke checks.
- Documentation and operator rollback procedures are current.
