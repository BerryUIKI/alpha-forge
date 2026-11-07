# Agent Protocol

## Role of the Agent

The agent is a research assistant, not a decision-maker. It collects information, structures knowledge, identifies patterns, and surfaces contradictions. It never makes autonomous investment decisions or executes trades.

## Task Lifecycle

```text
Created → Queued → Running → Waiting For Input → Completed
                        → Failed
                        → Cancelled
```

### States

| State | Meaning |
|-------|---------|
| **Created** | Task record exists, not yet queued for execution. |
| **Queued** | Task is waiting for an available execution slot. |
| **Running** | Task is scheduled or actively executing — waiting for a slot, making API calls, or processing data. |
| **Waiting For Input** | Agent needs clarification or additional information from the user. |
| **Completed** | Task finished successfully. Output is available. |
| **Failed** | Task encountered an unrecoverable error. Error details are recorded. |
| **Cancelled** | User cancelled the task before completion. |

### Transitions

- `Created → Queued`: Task is submitted for execution.
- `Queued → Running`: The task is accepted for execution. It may wait for a free slot before the runtime is invoked.
- `Running → Waiting For Input`: Agent needs the user to provide more context.
- `Waiting For Input → Running`: User provides the requested input.
- `Running → Completed`: Task produces valid structured output.
- `Running → Failed`: Unrecoverable error occurs.
- `Any non-terminal → Cancelled`: User cancels the task.

## Execution Model

### Async, Non-Blocking

Tasks run asynchronously in the Rust Tokio runtime. The Tauri command returns a `task_id` immediately — it does not block waiting for completion. Progress is communicated through Tauri events.

```text
User submits task
  → Tauri command returns task_id
    → Task runs in background
      → Progress events stream to React
        → React updates UI
          → Task completes → final artifact
```

### Concurrency

- Maximum concurrent tasks: configurable, default 2 (`AgentService::with_max_concurrency`).
- Additional tasks are scheduled immediately but wait for a slot, emitting a "Waiting for an execution slot..." progress event.
- Each task has a configurable timeout (default 5 minutes).
- Tasks track token and cost budgets.

### Recovery and Retry

- On startup, tasks left in `running` by a previous process are marked `failed` with an "interrupted by application restart" error and a matching `task_failed` event.
- Failed and cancelled tasks can be requeued with `retry_agent_task`; retrying clears the previous output/error and records a `task_queued` event.
- Transient provider failures (rate limits, 5xx, network errors) are retried up to 2 times with exponential backoff inside the provider client. Authentication and validation errors fail immediately.

### Cancellation

Cancellation is cooperative — the agent checks a cancellation token between steps. Long-running operations (API calls) are wrapped with timeouts.

## Tool Usage

Agents use tools to interact with the system and external services. Tools are registered capabilities that the agent can invoke during task execution.

### Principles

- **Least privilege.** Agents receive only the tools needed for the current task.
- **Explicit approval.** Destructive or external actions require user confirmation.
- **Auditable.** Every tool invocation is logged as a task event.
- **Deterministic.** Tool inputs and outputs are validated against schemas.

### Tool Calling

- Tool definitions use the OpenAI function-calling shape (`tools[].function.name` / `description` / `parameters`) with JSON Schema parameters, matching the MCP `inputSchema` model.
- The runtime allows at most 4 tool rounds per task; exceeding the limit fails the task.
- Tool results are appended as `role: "tool"` messages carrying the matching `tool_call_id`.
- Tool execution errors are returned to the model as `{"error": ...}` tool results instead of aborting, so it can correct course. MCP models the same behavior with `isError` inside the tool result.
- Tools are read-only in this phase; none of them mutates research data.

### Available Tools

- `list_tasks` — List recent agent tasks in a workspace, optionally filtered by status.
- `search_tasks` — Search task titles and descriptions by substring.
- `get_task_output` — Read the stored output or error of a previous task.
- `search_research` — Search research documents in a workspace; returns ids, titles, and excerpts.
- `get_research_document` — Read a document's title, summary, and content by id.
- `calculate` — Evaluate arithmetic with `+`, `-`, `*`, `/`, and parentheses.

### Planned Tools

- `search_research` — Search local research documents (after M4).
- `fetch_web` — Retrieve web content (allowlist-constrained).
- `fetch_market_data` — Query market data providers.
- `create_note`, `create_thesis`, `add_evidence`, `generate_artifact` — write tools, behind explicit approval.

## Structured Output

Agent output must use explicit schemas. Free-form Markdown is not acceptable as the primary output format.

Example research output schema:

```json
{
  "summary": "string",
  "claims": [
    {
      "claim": "string",
      "confidence": "number (0-100)",
      "evidence": ["string"],
      "sources": ["string (source ID)"]
    }
  ],
  "risks": ["string"],
  "companies": ["string (ticker)"],
  "themes": ["string"],
  "portfolioImpact": ["string"],
  "confidence": "number (0-100)",
  "citations": [
    { "kind": "document", "id": "doc-1", "title": "Tesla Q4" },
    { "kind": "task", "id": "task-1", "title": "Earlier analysis" }
  ]
}
```

Validation: TypeScript validates with Zod; Rust validates with Serde.

## Context Management

Each task starts with:

- The task title and description.
- The workspace ID (so tools can query it) and name.
- Up to five active research documents: the document `summary` when present, otherwise the first 400 characters of content.
- Up to five recently completed tasks in the workspace: the structured `summary` when present, otherwise truncated output.
- Tool definitions: names, descriptions, and JSON Schemas.

Injected summaries are capped at roughly 4,000 characters (five documents and
five tasks at up to 400 characters each; structural labels add a small
overhead). See [AGENT_MEMORY.md](AGENT_MEMORY.md) for the full memory standard,
retrieval tools, citation rules, and write policy. Portfolio context and
semantic retrieval are future work.

## Error Handling

- **Transient errors** (API rate limit, network timeout): Retry with exponential backoff, up to configurable max retries.
- **Permanent errors** (invalid input, auth failure): Fail immediately with a clear, user-facing message.
- **Partial results:** If a task produces some output before failing, consider saving partial results.
- **Tool errors:** Returned to the model as `{"error": ...}` tool results so it can retry or adapt. Malformed tool arguments are rejected by the registry and surfaced the same way.
- **Error messages:** Must be actionable. "API error" is not sufficient; "OpenAI API rate limit exceeded — retrying in 30 seconds" is.

## Human Approval Boundaries

The agent may autonomously:

- Search and retrieve information.
- Structure and summarize data.
- Draft theses and notes.
- Generate artifacts.

The agent must NOT, without explicit user approval:

- Make external API calls that incur cost beyond a configurable budget.
- Modify or delete existing research documents, theses, or portfolio data.
- Share data externally.
- Execute any action marked as destructive.

## Reference Designs

- Model Context Protocol — tools (`tools/list`, `tools/call`, JSON Schema `inputSchema`, `isError` tool results): https://modelcontextprotocol.io/specification/2025-06-18/server/tools
- OpenAI Chat Completions — function calling (`tools`, assistant `tool_calls`, `role: "tool"` results): https://platform.openai.com/docs/guides/function-calling
- Rig — Rust agent framework with tool calling and an MCP client: https://github.com/0xPlaygrounds/rig

## Structured Event Protocol

All agent activity is communicated through typed events:

```text
task.started       { task_id }
task.thinking      { task_id, message }
task.tool_call     { task_id, tool_name, input }
task.tool_result   { task_id, tool_name, output }
task.streaming     { task_id, chunk }
task.progress      { task_id, percent, message }
task.completed     { task_id, output }
task.failed        { task_id, error_code, error_message }
task.cancelled     { task_id }
```
