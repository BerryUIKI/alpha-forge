# Configuration

This guide covers everything you can configure in AlphaForge: language, the AI
provider used by agent tasks, and the rest of the Settings page.

## The Settings Page

Open **Settings** from the left navigation. Every configuration option lives on
this single page.

| Section | What it controls |
|---------|------------------|
| Language | UI language for the whole application |
| Agent configuration | OpenAI API key used by agent tasks |
| Internal plugins | Built-in analysis plugins and their inputs |
| Database health | Integrity check of the local database |
| Local backup | Export all data to a single backup file |
| Updates | Check for a newer application version |
| About & privacy | Links to the privacy notice and research disclaimer |

---

## Language

1. Open **Settings**.
2. In the **Language** section, choose **English** or **简体中文 (Simplified
   Chinese)**.
3. The interface switches immediately — no restart required.

Your choice is saved automatically and restored on the next launch.

---

## AI Provider (Agent Configuration)

Agent features (creating and running research/analysis tasks) need access to an
OpenAI-compatible API. There are two ways to configure it.

### Option A — In the app (recommended)

1. Open **Settings → Agent configuration**.
2. Enter your API key in the **API Key** field (starts with `sk-...`).
3. Click **Save**.

The key is stored in the **operating system keychain** (e.g. Windows Credential
Manager, macOS Keychain), not in the database or in plaintext files. Saving a new
key overwrites the previous one; leaving the field empty keeps the existing key.

> If you try to create an agent task before configuring a key, the app shows the
> **Agent Configuration Guide** dialog with the steps above and a button that
> jumps straight to `Settings → Agent`.

### Option B — Environment file (advanced)

The desktop app also reads the following variables from `.env` at build/run time:

| Variable | Default | Purpose |
|----------|---------|---------|
| `VITE_OPENAI_API_KEY` | *(empty)* | API key |
| `VITE_OPENAI_BASE_URL` | `https://api.openai.com/v1` | API base URL (allows proxies or compatible endpoints) |
| `VITE_OPENAI_MODEL` | `gpt-4o` | Model identifier |

Example:

```dotenv
VITE_OPENAI_API_KEY=sk-your-key
VITE_OPENAI_BASE_URL=https://api.openai.com/v1
VITE_OPENAI_MODEL=gpt-4o
```

> Future data-provider keys (`MARKET_DATA_API_KEY`, `NEWS_API_KEY`) are reserved
> in `.env.example` and are not used yet.

### Agent connection errors

If agent features fail to connect, the most common causes are:

1. **Invalid API key** — the key was rejected by the provider.
2. **Proxy / base URL misconfiguration** — a custom `VITE_OPENAI_BASE_URL` that
   is unreachable or requires different credentials.
3. **Service unavailable** — the provider is down or the network is blocked.

See [Troubleshooting](troubleshooting.md#agent-connection-fails) for the fix.

---

## Internal Plugins

AlphaForge ships with built-in analysis plugins that produce visual artifacts:

| Plugin | Purpose |
|--------|---------|
| Company Comparison | Compare companies side-by-side |
| Earnings Analyzer | Analyze earnings results |
| Industry Map | Visualize an industry landscape |
| Macro Dashboard | Macroeconomic indicator dashboard |
| Portfolio Risk | Portfolio risk analysis |
| Timeline | Event/timeline visualization |
| Valuation Model | Valuation scenario modeling |

In **Settings → Internal plugins** you can see the plugin list and, for each
plugin, configure its inputs. See [Artifacts](daily-operations.md#artifacts) in
Daily Operations for how artifacts are presented.

---

## Database Health

**Settings → Database health → Check database health** runs an integrity check on
the local SQLite database. A green **healthy** status means the database passed.
If the check fails, follow the recovery steps in
[Troubleshooting](troubleshooting.md#database-health-check-fails).

---

## Data and Backup

**Settings → Local backup → Export local backup** writes all application data to
a single backup file. Use it to:

- Archive a snapshot of your research before major changes.
- Migrate data to another machine.

There is no automatic cloud backup — backups are manual and local by design.

---

## Updates

**Settings → Updates → Check for updates** compares your version against the
latest release. If a newer version is available, the app opens the release page
so you can download it. Installation is manual; there is no auto-update in the
MVP.

---

## About & Privacy

- **Open privacy notice** — opens `docs/PRIVACY.md`.
- **Open research disclaimer** — opens the Investment Research Disclaimer.

Both documents are part of the repository and are always available.

---

## Next Steps

- [Daily Operations](daily-operations.md) — how to use every module.
- [Troubleshooting](troubleshooting.md) — if something does not behave as expected.
