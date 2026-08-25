# Goose Agent Integration

This directory defines the post-MVP integration boundary for the Goose Agent. In this plan, “Goose” means the open-source project originally published by Block and now maintained under the Agentic AI Foundation at the Linux Foundation.

## Schedule gate

Goose integration is milestone M10 and must not begin until the M8 MVP milestone is complete. The planned sequence completes M9 Option integration first unless the product owner explicitly authorizes M10 as an independent post-MVP workstream. Documentation, dependency review, and throwaway technical research may occur earlier, but no Goose binary, runtime, provider credential, extension, or background execution path is shipped in the MVP.

## Why integrate Goose

Goose can provide a reusable agent execution engine, provider support, MCP-based extensions, recipes, and structured output. AlphaForge should use those capabilities only where they improve its investment research loop:

```text
Information collection
  -> evidence-grounded synthesis
  -> structured research proposal
  -> human review
  -> AlphaForge persistence
  -> later validation and review
```

The integration is not intended to turn AlphaForge into a generic chat wrapper, expose a developer shell, or delegate investment decisions.

## Non-negotiable boundary

- Goose never executes securities trades or brokerage actions.
- Goose never autonomously creates a final investment decision, recommendation, or portfolio change.
- The first shipped capability is read-only “shadow mode.”
- Goose receives only the minimum workspace context required for the selected task.
- AlphaForge SQLite, credentials, unrestricted files, shell, and privileged Tauri commands are never exposed directly.
- Any proposed note, thesis evidence item, report, or Artifact is validated and shown to the user before a Rust service persists it.
- Every claim includes source references, retrieval/publication timestamps where available, model/provider identity, and uncertainty.
- Existing AlphaForge task states, cancellation, timeout, concurrency, token, and cost limits remain authoritative.

## Recommended integration shape

```text
React task UI
  -> typed desktopApi request
  -> Rust GooseAdapter starts an AlphaForge task
  -> supervised, version-pinned Goose sidecar
  -> explicit recipe with structured response schema
  -> allowlisted read-only AlphaForge MCP bridge
  -> validated result returned to Rust
  -> provenance persisted
  -> predefined React Artifact renderer
  -> user accepts or rejects any proposed write
```

This sidecar adapter is the preferred first spike because it isolates an evolving upstream runtime behind one Rust interface. A direct library or service API may replace it only after the M10 technical spike proves it safer and easier to package.

Do not enable Tauri shell access for React. Rust starts the approved executable directly with a fixed path and bounded arguments; user input never becomes a command line or shell fragment.

ADR-0010 generalizes this supervision model for all long-running Agent backends.
Goose should adopt the common worker lifecycle, diagnostics, cancellation, and
process-tree controls where practical, while retaining its stricter recipe, MCP,
credential, provider, and packaging policies. See the
[Agent Runtime documentation](../agent/README.md).

## Document map

| Document                                                | Purpose                                                                            |
| ------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| [Integration Roadmap](INTEGRATION_ROADMAP.md)           | Post-MVP phases, work packages, technical path, security gates, tests, and rollout |
| [Milestone Roadmap](../MILESTONE_ROADMAP.md)            | Program sequence and M10 entry criteria                                            |
| [Delivery Playbook](../milestones/DELIVERY_PLAYBOOK.md) | Execution and evidence rules for implementation agents                             |
| [Agent Protocol](../AGENT_PROTOCOL.md)                  | AlphaForge task/event contract that remains authoritative                          |
| [Managed Agent Workers](../agent/README.md)             | Shared subprocess architecture, implementation roadmap, and checklist              |
| [Security](../SECURITY.md)                              | Application-wide trust and permission model                                        |

## Upstream facts to reverify at M10 kickoff

Goose evolves quickly, so the implementation agent must recheck these sources before choosing a version or API:

- [AAIF Goose repository](https://github.com/aaif-goose/goose) — ownership, license, releases, source, and supported surfaces.
- [Goose documentation](https://goose-docs.ai/) — current CLI, Desktop, API, providers, and architecture.
- [MCP extensions](https://goose-docs.ai/docs/mcp/filesystem-mcp/) — extension model and built-in capability behavior.
- [Recipe reference](https://goose-docs.ai/docs/guides/recipes/recipe-reference/) — explicit extensions, parameters, limits, and structured response schema.
- [Permission modes](https://goose-docs.ai/docs/guides/goose-permissions/) and [tool permissions](https://goose-docs.ai/docs/guides/managing-tools/tool-permissions/) — current approval semantics.
- [Configuration files](https://goose-docs.ai/docs/guides/config-files/) — credential storage and configuration precedence.
- [Headless mode](https://goose-docs.ai/docs/tutorials/headless-goose/) — non-interactive behavior, limitations, and exit handling.

As of this documentation pass, upstream describes Goose as a local Rust-based agent with CLI, Desktop, API, MCP extensions, recipes, and multiple permission modes. Treat that summary as planning context, not a pinned compatibility guarantee.
