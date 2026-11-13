# AlphaForge CLI

**Status**: Available on `dev`
**Binary**: `alphaforge`
**Crate**: `apps/cli` (`alphaforge-cli`)

## Overview

The `alphaforge` CLI is a headless terminal front end for the same local research
workspace the desktop application uses. It opens the desktop SQLite database
through the existing `database::connection` and `database::migrations` modules
and calls the same workspace, agent task, artifact, and credential code paths.

It does not start a Tauri runtime, does not open a second persistence path, and
does not implement its own version of any domain rule.

The CLI is a research tool. It never places trades and never makes investment
decisions on behalf of a user. Starting a task only produces research output;
acting on that output remains a human decision.

## Building and running

```bash
# Build the binary
cargo build -p alphaforge-cli

# Run from the workspace
cargo run -p alphaforge-cli -- --help

# Run the built binary
./target/debug/alphaforge --help
```

The crate is a workspace member, so `cargo test --workspace` covers it.

Because the CLI links the desktop Rust library, a Linux build needs the same
system packages as the Tauri desktop build (for example `libwebkit2gtk-4.1-dev`);
these are build-time link dependencies only. The CLI never creates a Tauri
application or a WebView.

## Database location

By default the CLI uses the desktop application's data directory, mirroring
Tauri's `app_data_dir()`:

| Platform | Default database |
|----------|------------------|
| Windows | `%APPDATA%\com.berry.alphaforge\alpha_forge.db` |
| macOS | `~/Library/Application Support/com.berry.alphaforge/alpha_forge.db` |
| Linux | `$XDG_DATA_HOME/com.berry.alphaforge/alpha_forge.db` (or `~/.local/share/...`) |

Pass `--db <path>` to use any other SQLite file. The file and its parent
directory are created on first use, then the standard migrations run. The
desktop app and the CLI can use the same database; normal SQLite WAL locking
applies.

## Global options

| Option | Meaning |
|--------|---------|
| `--db <path>` | Use a specific SQLite database file. |
| `--json` | Print machine-readable JSON instead of human-readable text. |
| `--help` | Show command help. |
| `--version` | Show the CLI version. |

## Commands

### Workspaces

```bash
alphaforge workspace list
alphaforge workspace create "Alpha Research"
```

### Agent tasks

```bash
# List tasks in a workspace
alphaforge task list --workspace <workspace-id>

# Create a task
alphaforge task create \
  --workspace <workspace-id> \
  --title "Assess demand for TSMC" \
  --description "Compare demand drivers and contradicting evidence."

# Start a task and wait for the result
alphaforge task start <task-id>

# Cancel a task that has not reached a terminal state
alphaforge task cancel <task-id>
```

`task start` follows the same lifecycle as the desktop application:

```text
Created → Queued → Running → Completed
                             → Failed
                             → Cancelled
```

The command queues the task when needed, starts it through the shared
`TaskExecutor`, streams progress in text mode, and waits for the terminal state.
Progress, completion output, and failures are persisted in the same
`agent_task_events` timeline the desktop UI reads.

- Completion output is the structured research result (`summary`, `claims`,
  `evidence`, `risks`, `confidence`).
- If no OpenAI key is stored, the task fails with the shared provider
  credentials message. Store a key with `provider set-key` first.
- `--timeout <seconds>` cancels the task when the wait exceeds the limit. The
  executor's own task timeout (5 minutes by default) still applies.
- `Ctrl+C` cancels the running task through the same cancellation path.

Only one process owns a started task. If the CLI process exits or is killed while
a provider request is in flight, the task can remain `running`; the desktop app
has the same in-process execution model. `task cancel` can move such a task to
`cancelled`.

### Provider configuration

```bash
alphaforge provider status
alphaforge provider set-key     # prompts without echoing the key
alphaforge provider clear-key
```

`set-key` reads the key with terminal echo disabled and stores it in the OS
keychain under the canonical `openai.api_key` credential. `status` reports only
whether a key exists; the CLI never prints, logs, or returns the secret.

### Artifacts

```bash
alphaforge artifact list
alphaforge artifact list --workspace <workspace-id>
alphaforge artifact list --task <task-id>
```

Without filters the command lists artifacts across all workspaces. Artifacts are
read-only in the CLI; the desktop application owns generation and viewing.

## Output formats

Text output is a table or a short record. JSON output is a single pretty-printed
document, so it can be piped into `jq` and other tools.

```bash
alphaforge --json workspace list
```

```json
[
  {
    "id": "aae2c1c4-4a6f-4a2e-9b47-1c1f0d5a9b8e",
    "name": "Alpha Research",
    "created_at": "2026-11-13T10:00:00Z",
    "updated_at": "2026-11-13T10:00:00Z"
  }
]
```

`task start --json` returns the final task and its persisted event timeline:

```json
{
  "task": { "id": "…", "status": "completed", "…": "…" },
  "events": [
    { "event_type": "task_started", "…": "…" },
    { "event_type": "task_progress", "payload": "Preparing structured research request...", "…": "…" },
    { "event_type": "task_completed", "payload": "{\"summary\":\"…\"}", "…": "…" }
  ]
}
```

Progress is not written to stdout in `--json` mode, so the document stays valid.

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Command succeeded. A started task completed. |
| `1` | Command failed (validation, database, keychain, or terminal error). |
| `2` | A started task ended in the `failed` state. |
| `3` | A started task ended in the `cancelled` state. |

Errors are written to stderr as `error: <message>`.

## Security and product boundaries

- Credentials come from the OS keychain through the shared
  `security::credentials` adapter. The CLI never prints or returns a secret.
- The CLI does not accept shell commands, arbitrary file paths for execution, or
  remote URLs as task input.
- Task prompts and provider requests use the same bounded, structured request
  builder as the desktop app.
- The CLI cannot execute trades, place orders, or make autonomous investment
  decisions. It exposes research workspace operations only.
- Human approval remains mandatory for any decision that acts on research
  output.

## Developer notes

### Crate layout

```text
apps/cli/
├── src/
│   ├── main.rs          Binary entry point
│   ├── lib.rs           Command dispatch
│   ├── cli.rs           Clap definitions
│   ├── context.rs       Database and service wiring
│   ├── commands/        workspace, task, provider, artifact handlers
│   ├── events.rs        Channel-backed TaskEventEmitter
│   ├── output.rs        Text and JSON rendering
│   └── error.rs         CliError and exit-code contract
└── tests/
    └── cli_integration.rs
```

### Why the CLI depends on the desktop library

`alphaforge-cli` depends on the `alpha-forge` library crate
(`apps/desktop/src-tauri`). The alternative — extracting the repository, service,
provider, and credential layers into a new shared core crate — would be a
repository-wide refactor. Depending on the library keeps a single implementation
of every domain rule and still satisfies the headless requirement because the
CLI never calls `alpha_forge_lib::run()`.

The dependency is intentional and narrow: the CLI uses the library's public
modules and does not reach into Tauri state, commands, or windows.

### Task executor event boundary

`TaskExecutor` takes an `Arc<dyn TaskEventEmitter>` instead of a Tauri
`AppHandle`. The desktop registers `AppHandle` as the emitter and forwards events
to React windows; the CLI registers a channel-backed emitter and prints progress
from the command that started the task. This keeps one execution path for both
front ends. See [`docs/AGENT_PROTOCOL.md`](AGENT_PROTOCOL.md).

### Adding a command

1. Add the clap variant in `src/cli.rs`.
2. Add a handler in `src/commands/` that calls an existing service method.
3. Reuse `output.rs` renderers; add tests for new rendering.
4. Extend `tests/cli_integration.rs` with a flow against a temporary database.
5. Update this document.

Do not add a direct SQL query or a Tauri command call in the CLI. If a needed
operation has no service method, add it to the existing repository/service layer
with tests, as was done for `ArtifactService::list_all_artifacts`.

## Testing

```bash
cargo test -p alphaforge-cli
cargo clippy -p alphaforge-cli --all-targets --all-features -- -D warnings
cargo fmt --check
```

The integration suite runs the real migrations against a temporary SQLite file
and substitutes an in-memory credential store and a deterministic research
provider, so it needs no network access and no OS keychain.
