# Simple Global Search and Agent Chat

## Status

Proposed — implementation to follow this document.

## Purpose

This document specifies two small, user-visible improvements identified in the
product review, and the minimal design chosen to implement them. It is written
first so the implementation matches an agreed contract.

1. **Simple global search.** The top bar shows a static `Search... ⌘K`
   affordance that does nothing. Make it a real command palette that finds
   research projects, documents, reports, investment theses, knowledge entities,
   and artifacts inside the current workspace.
2. **Agent chat.** The Agent panel chat input clears on send without doing
   anything (`// TODO: wire up to actual agent chat`). Wire it to create and
   start a research task so typing a question produces real agent activity.

Both features are deliberately scoped to "simple": frontend-only, no new Rust
commands, no schema changes, and no changes to the existing research page URL
contract.

## Background

Current state (verified 2026-08-19):

- The top bar renders a non-interactive `<div>` labelled `Search...` with a
  `⌘K` hint (`apps/desktop/src/components/layout/TopBar/TopBar.tsx`). There is
  no `input`, no `onClick`, and no keyboard binding.
- The Agent panel input (`AgentInput` in
  `apps/desktop/src/components/layout/RightSidebar/AgentPanel.tsx`) clears its
  text on send and does nothing else. The panel shows three hard-coded sample
  messages "until the chat is wired to real agent output".
- The backend already provides everything the two features need. The agent task
  pipeline (`create_agent_task` → `queue_agent_task` → `start_agent_task`) is
  fully implemented in `src-tauri/src/commands/agent.rs`, and the executor runs
  real provider completions and emits `task_progress`/`task_completed` events.
  List commands exist for every searchable entity type. There is no
  cross-entity search command, so search is implemented client-side over the
  cached list queries.

## Feature 1 — Simple Global Search

### Searchable scope

Search is workspace-scoped. Given the active workspace and a query string,
results are gathered from:

| Section | Source | Matched fields | Navigates to |
|---|---|---|---|
| Projects | `list_research_projects` | title, description | `/research?workspace={w}&project={p}` |
| Documents | `list_research_documents` per project | title, content | `/research?workspace={w}&project={p}` |
| Reports | `list_research_reports` per project | title, content | `/research?workspace={w}&project={p}` |
| Theses | `list_theses` | title, thesis | `/journal` |
| Knowledge entities | `list_knowledge_entities` | name, description | `/knowledge` |
| Artifacts | `list_artifacts` | artifact_type, input/output text | `/artifact/{id}/{type}` |

Notes:

- Research notes attached to documents are searched through the parent
  document's content. Standalone `research_note` rows are out of scope for v1
  (the Research page does not expose a global list command for them).
- Per-project document/report fetches are cheap local SQLite queries and are
  cached by TanStack Query under the same keys the Research page uses, so a
  second open of the palette is free.
- The artifact window route `/artifact/{id}/{type}` already exists and is
  isolated from the main layout, so artifact results deep-link directly.
- Deep-linking to an individual document, report, or thesis is deferred: the
  target pages select those items with local component state, not URL
  parameters. v1 navigates to the containing page. Follow-up work can add URL
  parameters (see "Future work").

### Matching

Case-insensitive substring match on the concatenation of the matched fields.
A result must contain the trimmed query. No tokenization, stemming, or ranking
beyond section ordering — this is deliberately simple.

### Workspace resolution

The active workspace is resolved as:

1. The `workspace` query parameter of the current route, if present; otherwise
2. The first workspace returned by `useWorkspaces`, if any.

If no workspace exists, the dialog shows an empty state directing the user to
create one.

### UI

A `GlobalSearchDialog` command palette:

- Mounted once in `MainLayout`, so it is available on every routed page.
- Opened by clicking the top bar search control and by the `Cmd/Ctrl+K`
  shortcut (global keydown listener in `MainLayout`), matching the hint already
  shown in the top bar.
- Contains a text input, grouped results with section headers and icons, and a
  "no results" state. `ArrowUp`/`ArrowDown` move the selection, `Enter`
  navigates, `Esc` closes.
- Navigation uses `useNavigate` and closes the dialog on selection.

### New files

- `apps/desktop/src/features/search/hooks/useGlobalSearch.ts` — aggregates the
  entity lists for the active workspace and filters them by the query.
- `apps/desktop/src/features/search/components/GlobalSearchDialog.tsx` — the
  palette UI.
- `apps/desktop/src/features/search/index.ts` — feature exports.

### Modified files

- `apps/desktop/src/components/layout/MainLayout.tsx` — owns the dialog open
  state, binds `Cmd/Ctrl+K`, renders the dialog, passes `onOpenSearch` down.
- `apps/desktop/src/components/layout/MainContent/MainContent.tsx` — forwards
  `onOpenSearch` to the top bar.
- `apps/desktop/src/components/layout/TopBar/TopBar.tsx` — turns the static
  search box into a real button wired to `onOpenSearch`.
- `apps/desktop/src/components/layout/types.ts` — adds `onOpenSearch` to
  `TopBarProps`.
- `apps/desktop/src/lib/i18n/locale.ts` — new keys (see below), added to both
  `zh-CN` and `en` catalogs.

### i18n keys

| Key | zh-CN | en |
|---|---|---|
| `globalSearchTitle` | 全局搜索 | Global Search |
| `globalSearchPlaceholder` | 搜索项目、文档、论点、知识实体… | Search projects, documents, theses, knowledge… |
| `searchSectionProjects` | 研究项目 | Projects |
| `searchSectionDocuments` | 研究文档 | Documents |
| `searchSectionReports` | 研究报告 | Reports |
| `searchSectionTheses` | 投资论点 | Theses |
| `searchSectionKnowledge` | 知识实体 | Knowledge |
| `searchSectionArtifacts` | 研究产物 | Artifacts |
| `searchNoResults` | 没有匹配结果 | No matching results |
| `searchNoWorkspace` | 请先创建工作区后再搜索 | Create a workspace first to search |
| `searchResultsFor` | “{query}”的结果 | Results for "{query}" |

## Feature 2 — Agent Chat

### Behavior

The chat input becomes a quick task launcher:

1. On send (Enter, or the send button) with a non-empty, trimmed query:
   - If the agent is `unconfigured` or `error` (per `useAgentStatus`), open the
     existing `AgentConfigGuide` instead of creating a task.
   - Otherwise create a task with `useCreateAgentTask`
     (`title = query`, no description) and immediately auto-start it with
     `useRunAgentTask({ taskId, status: "created" })`, which queues and then
     starts execution.
2. The query is appended to the message list as a user bubble, and the created
   task becomes the selected task so the existing task-detail card shows its
   status with start/cancel/error handling.
3. The hard-coded sample messages are removed; a single `info`-style welcome
   message explains that sending a question queues a research task. Sending
   errors (task create failure, queue failure) surface through the existing
   task card error banner and inline error text.

### Messages area

`AgentPanel` keeps a small local conversation state:

```ts
type ConversationMessage =
  | { id: string; role: "user"; text: string }
  | { id: string; role: "info"; text: string };
```

Rendered above the create-task button and task list. The user bubble uses an
existing message style (right-aligned user styling) and the info bubble reuses
the current `info` styling.

### Modified files

- `apps/desktop/src/components/layout/RightSidebar/AgentPanel.tsx` — wires
  `AgentInput` to `onSend`, replaces sample messages with the conversation,
  adds `useCreateAgentTask`, and removes `getSampleMessages`.
- `apps/desktop/src/components/layout/RightSidebar/AgentPanel.test.tsx` —
  updates mocks (`useCreateAgentTask`) and adds coverage for sending a message.
- `apps/desktop/src/features/agent/hooks/useAgentTasks.ts` — adds a 5s
  `refetchInterval` to `useAgentTask` so the selected task's status advances
  without manual refresh (matches the existing 5s poll in `useAgentStatus`).
- `apps/desktop/src/lib/i18n/locale.ts` — removes the now-unused
  `agentSample*` keys and adds the new keys below, in both catalogs.

### i18n keys

| Key | zh-CN | en |
|---|---|---|
| `agentChatWelcome` | 在下方输入研究问题，将以研究任务的形式排队执行。 | Ask a research question below; it will be queued as a research task. |
| `agentChatSendFailed` | 创建研究任务失败 | Failed to create the research task |
| `agentChatNeedsConfig` | 请先完成 Agent 配置后再提问 | Configure the Agent before asking |

## Verification

For each feature branch:

- `npx tsc --noEmit -p apps/desktop/tsconfig.json` — clean.
- `npx eslint <changed files>` — clean.
- `npx vitest run apps/desktop` — full desktop suite passes, including the
  catalog-parity tests.

## Out of scope / Future work

- Backend `search_all` command with proper ranking and pagination.
- URL deep-linking for documents, reports, and theses.
- Rendering completed task output (structured JSON) inside the Agent panel
  instead of the status card.
- Streaming task progress events into the message list.

## References

- Product review findings (search placeholder that does nothing; dead agent
  chat input).
- `docs/FRONTEND_BACKEND_INTEGRATION.md` — command/type/i18n conventions.
- `src-tauri/src/commands/agent.rs` — agent task pipeline.
- `src-tauri/src/agent/executor.rs` — provider completion and event emission.
