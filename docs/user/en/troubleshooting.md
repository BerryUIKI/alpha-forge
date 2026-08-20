# Troubleshooting

Common problems and their fixes. If your issue is not listed here, open an issue
on the [GitHub issue tracker](https://github.com/BerryUIKI/alpha-forge/issues).

## Contents

- [Agent connection fails](#agent-connection-fails)
- [Agent tasks hang or never finish](#agent-tasks-hang-or-never-finish)
- [Database health check fails](#database-health-check-fails)
- [Application does not start / build fails](#application-does-not-start--build-fails)
- [Language does not change](#language-does-not-change)
- [No data appears in a module](#no-data-appears-in-a-module)
- [Options chain fetch fails](#options-chain-fetch-fails)
- [I want to back up or migrate my data](#i-want-to-back-up-or-migrate-my-data)
- [Gatekeeper warning on macOS](#gatekeeper-warning-on-macos)

---

## Agent connection fails

**Symptoms**: creating an agent task shows "Agent connection failed" / "Agent
needs configuration", or the connection status shows an error.

**Likely causes and fixes**, in order:

1. **No API key configured**
   - Open **Settings → Agent configuration**, enter your API key (`sk-...`),
     and click **Save**.
   - Alternatively set `VITE_OPENAI_API_KEY` in `.env` (advanced; see
     [Configuration](configuration.md#option-b--environment-file-advanced)).

2. **Invalid API key**
   - The key was rejected by the provider. Double-check it was copied completely
     (no trailing spaces) and that it is active in your provider account.
   - Save the corrected key in Settings — this overwrites the old one.

3. **Proxy / base URL misconfiguration**
   - If you use a custom `VITE_OPENAI_BASE_URL` (proxy or compatible endpoint),
     confirm the URL is reachable and that network settings are correct.
   - Verify `.env` changes were picked up (rebuild/restart the app).

4. **Service unavailable**
   - The provider is down or your network blocks it. Retry later or check your
     connection/offline indicator.

After fixing the cause, try creating the task again.

---

## Agent tasks hang or never finish

**Likely causes and fixes:**

- **Provider latency** — long research tasks are normal; progress is streamed
  into the UI, so check the task status (`queued`/`running`).
- **Cancelled or failed state** — if a task shows `failed`, review the error
  message shown next to it. If it shows `cancelled`, it was intentionally
  stopped.
- **Stale configuration** — restart the app to pick up configuration changes.

You can always **cancel** a running task; nothing is executed on your behalf
without your action.

---

## Database health check fails

**Symptoms**: Settings → Database health reports an error status.

**Fixes:**

1. Make sure no other instance of the app is running (close and reopen).
2. Run the health check again — transient lock failures often clear.
3. If it still fails, export a backup first if the app allows it, then reinstall
   or restore from your last [local backup](configuration.md#data-and-backup).

> If the database cannot be opened at all, contact maintainers via the issue
> tracker; do not delete the database file manually unless you have a backup.

---

## Application does not start / build fails

**Symptom**: `pnpm tauri dev` or `pnpm tauri build` fails.

**Fixes:**

1. **Rust toolchain** — confirm `rustc --version` works and, on Windows, the MSVC
   toolchain is installed (`rustup default stable-msvc`).
2. **Node/pnpm versions** — Node.js 22+ and pnpm 9+ are required
   ([Installation](installation.md#prerequisites)).
3. **Dependencies** — run `pnpm install` again after pulling new changes.
4. **Platform system dependencies** — Linux: install the
   [Tauri prerequisites](https://tauri.app/start/prerequisites/); macOS: run
   `xcode-select --install`.
5. **First build is slow** — the first Rust compile takes several minutes; that
   is expected, not a hang.

---

## Language does not change

**Symptom**: selecting a language in Settings has no visible effect.

**Fixes:**

- The change applies immediately; if the window is on a page that caches
  strings, restart the app — the choice is saved and restored on launch.
- Only **English** and **简体中文** are currently available; other options are not
  listed.

---

## No data appears in a module

**Likely causes and fixes:**

- **Empty state vs. error state** — an empty list with a clear "no data" message
  means there is genuinely nothing there yet (create your first item). An error
  message means a load failed — check the error text and your workspace.
- **Wrong workspace** — research projects, options analysis, and artifacts are
  workspace-scoped. Switch to the workspace that contains your data.
- **Fresh install** — a new database starts empty by design.

---

## Options chain fetch fails

**Likely causes and fixes:**

- **Symbol format** — symbols allow letters, digits, `.` and `-`, up to 10
  characters (for example `AAPL`, `BRK.B`). The app uppercases and trims input.
- **Network** — chain data is fetched from a market-data provider; check your
  connection.
- **Provider coverage** — not every symbol/expiry is available from every
  provider; try another chain or symbol.

---

## I want to back up or migrate my data

Use **Settings → Local backup → Export local backup** to write a single backup
file, then copy it to the other machine. There is no cloud backup — this is
manual and local by design.

---

## Gatekeeper warning on macOS

**Symptom**: opening a downloaded build shows "cannot be opened because the
developer cannot be verified".

**Cause**: the MVP does not include macOS notarization.

**Workaround**: right-click the app and choose **Open**, or temporarily allow it
in **System Settings → Privacy & Security**. This is a known release risk, not a
corrupted file.

---

## Still stuck?

Open an issue at
<https://github.com/BerryUIKI/alpha-forge/issues> with:

1. Your OS and app version.
2. The exact steps that caused the problem.
3. The error message text (screenshots help).
