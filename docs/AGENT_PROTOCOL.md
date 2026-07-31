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
| **Running** | Task is actively executing — making API calls, processing data. |
| **Waiting For Input** | Agent needs clarification or additional information from the user. |
| **Completed** | Task finished successfully. Output is available. |
| **Failed** | Task encountered an unrecoverable error. Error details are recorded. |
| **Cancelled** | User cancelled the task before completion. |

### Transitions

- `Created → Queued`: Task is submitted for execution.
- `Queued → Running`: An execution slot becomes available.
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

- Maximum concurrent tasks: configurable, default 2.
- Additional tasks are queued.
- Each task has a configurable timeout (default 5 minutes).
- Tasks track token and cost budgets.

### Cancellation

Cancellation is cooperative — the agent checks a cancellation token between steps. Long-running operations (API calls) are wrapped with timeouts.

## Tool Usage

Agents use tools to interact with the system and external services. Tools are registered capabilities that the agent can invoke during task execution.

### Principles

- **Least privilege.** Agents receive only the tools needed for the current task.
- **Explicit approval.** Destructive or external actions require user confirmation.
- **Auditable.** Every tool invocation is logged as a task event.
- **Deterministic.** Tool inputs and outputs are validated against schemas.

### Planned Tools (Future Phases)

- `search_research` — Search local research documents.
- `fetch_web` — Retrieve web content (allowlist-constrained).
- `fetch_market_data` — Query market data providers.
- `create_note` — Create a research note.
- `create_thesis` — Create an investment thesis draft.
- `add_evidence` — Attach evidence to a thesis.
- `generate_artifact` — Produce a structured artifact.

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
  "confidence": "number (0-100)"
}
```

Validation: TypeScript validates with Zod; Rust validates with Serde.

## Context Management

The agent maintains context within a task but not across tasks (in MVP). Each task starts with:

- The user's input.
- Relevant research documents (manually selected or auto-retrieved).
- Current portfolio summary (if relevant).
- Tool definitions.

Future phases will add long-term context: prior task summaries, thesis history, persistent user preferences.

## Error Handling

- **Transient errors** (API rate limit, network timeout): Retry with exponential backoff, up to configurable max retries.
- **Permanent errors** (invalid input, auth failure): Fail immediately with a clear, user-facing message.
- **Partial results:** If a task produces some output before failing, consider saving partial results.
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
