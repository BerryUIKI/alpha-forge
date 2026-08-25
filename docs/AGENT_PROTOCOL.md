# Agent Protocol

**Status**: M2 Complete
**Last Updated**: 2026-08-01

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

### Current Model (M2)

### Background Task Execution

Tasks execute in background Tokio tasks with:

- **Concurrent execution** (default: 5 tasks max)
- **CancellationToken** for graceful cancellation
- **Timeout enforcement** (default: 5 minutes)
- **Progress event streaming** via Tauri events

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

### Target Managed-Worker Model

ADR-0010 accepts a managed subprocess boundary for long-running, tool-using, and
third-party Agent workloads. The product task states and Tauri event names remain
stable; process lifecycle details are internal run events.

```text
User submits task
  -> Tauri command returns task_id
  -> Rust AgentOrchestrator persists and scopes the run
  -> WorkerSupervisor starts an approved ephemeral worker
  -> worker requests provider/tool operations through Rust brokers
  -> Rust streams normalized task events to React
  -> Rust validates and persists the terminal result
```

Rust remains the only owner of credentials, provider network access, SQLite,
files, domain writes, budgets, and audit state. React has no worker or shell
permission. See [`docs/agent/SUBPROCESS_ARCHITECTURE.md`](agent/SUBPROCESS_ARCHITECTURE.md).

### Event Streaming

Real-time task updates are emitted to the frontend via Tauri events:

| Event Name | When Fired |
|------------|------------|
| `task:progress` | Progress updates during execution |
| `task:completed` | Task finished successfully |
| `task:failed` | Task encountered an error |
| `task:cancelled` | Task was cancelled by user |

### Cancellation Flow

```text
User calls cancel_agent_task
    ↓
Executor cancels CancellationToken
    ↓
Background task receives cancellation signal
    ↓
Task cleans up and stops
    ↓
Event emitted: task:cancelled
    ↓
Status updated in database
```

For a subprocess-backed run, cancellation additionally sends a graceful protocol
message, waits for a bounded grace period, terminates the complete process tree,
reaps the child, and cleans task-owned temporary data before recording the final
state. A worker process exit is not itself a successful task result.

---

## Architecture (M2)

### TaskExecutor

**File**: `apps/desktop/src-tauri/src/agent/executor.rs`

Responsibilities:
- Manages concurrent task execution
- Handles cancellation via CancellationToken
- Emits progress events to frontend
- Enforces timeout limits

### Event System

**File**: `apps/desktop/src-tauri/src/agent/events.rs`

Functions:
- `emit_task_event`: Generic event emission
- `emit_progress`: Progress updates
- `emit_completion`: Task completion
- `emit_failure`: Error reporting
- `emit_cancellation`: User cancellation

---

## API

### Tauri Commands

```rust
// Start task execution in background
start_agent_task(task_id: String) -> Result<AgentTask, AppError>

// Cancel a running task
cancel_agent_task(task_id: String) -> Result<AgentTask, AppError>
```

### Frontend Integration

Listen for task events:

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen for progress updates
const unlisten = await listen('task:progress', (event) => {
  console.log('Task progress:', event.payload);
});

// Listen for completion
await listen('task:completed', (event) => {
  console.log('Task completed:', event.payload);
});
```

---

## Concurrency Controls

### Maximum Concurrent Tasks

Default: 5 tasks

Configurable via `ExecutorConfig::max_concurrent`.

### Task Timeout

Default: 300 seconds (5 minutes)

Configurable via `ExecutorConfig::default_timeout_secs`.

### Execution Slot

Before starting a task, the executor checks:
1. Task is not already running
2. Concurrency limit not reached

If limits are hit, returns validation error.
