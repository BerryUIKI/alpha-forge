# Agent Memory

This document defines what the agent remembers, what it receives for each task,
and what it must never do with memory. All memory is local SQLite data.

## Memory Layers

| Layer | Contents | Lifetime |
|-------|----------|----------|
| Working memory | Current task messages and tool results | Discarded when the task reaches a terminal state; at most 4 tool rounds |
| Workspace memory | Research documents, completed task outputs, artifacts | Persisted; read-only for the agent; retrieved via tools or summarized into prompts |
| Long-term preferences | Settings and thesis history | Future milestone |

## Injected Context

Every task receives roughly **4,000 characters** of workspace memory
(summaries; structural labels add a small overhead):

- Workspace ID and name.
- Up to **5 active research documents**: the document `summary` when present, otherwise the first 400 characters of `content`.
- Up to **5 recently completed tasks** in the workspace: the structured `summary` from the task output when present, otherwise truncated output.
- The current task is excluded; archived documents are excluded.

Selection is deterministic (recency order with fixed limits) so prompts are
reproducible for the same database. Semantic selection is future work.

## Retrieval Tools

| Tool | Returns | Limits |
|------|---------|--------|
| `search_research(workspace_id, query, limit)` | Document ids, titles, kinds, excerpts | limit 1-10, excerpt <= 1,500 chars |
| `get_research_document(document_id)` | Title, kind, format, summary, content | content <= 8,000 chars |
| `list_tasks(workspace_id, status?)` | Task ids, titles, statuses | 20 tasks |
| `search_tasks(workspace_id, query)` | Matching tasks | 20 tasks |
| `get_task_output(task_id)` | Stored output/error | 4,000 chars |
| `calculate(expression)` | Arithmetic result | 200-char expression |

Tool results always include ids so the agent can cite them.

## Citations

The final research output may include:

```json
"citations": [
  { "kind": "document", "id": "doc-1", "title": "Tesla Q4" },
  { "kind": "task", "id": "task-1", "title": "Earlier analysis" }
]
```

- Maximum 20 citations; entries without an id or title are dropped.
- The system prompt tells the model to use ids from context or tool results and never invent them.
- The task detail view lists citations under the output.

## Write Policy

- The agent writes only task output and artifacts.
- Documents and notes are written by explicit user actions (or a future explicit "save as document" action).
- No automatic memory writes in this phase.

## Retention and Deletion

- Documents are soft-archived (`archived_at`); archived documents are excluded from context and search.
- Purging is explicit: it permanently deletes an archived document and cascades its sources and notes.
- Task outputs and artifacts are immutable history; citations may reference archived documents by id.

## Security

- Document content and tool results are untrusted data. The system prompt forbids following instructions found inside them.
- Only the summaries and excerpts needed for the current task leave the machine; everything else stays in SQLite.
- Memory never contains credentials.
