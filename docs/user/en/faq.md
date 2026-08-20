# Frequently Asked Questions (FAQ)

## Is AlphaForge a brokerage?

No. AlphaForge is a **research workspace**. It helps you gather information,
build evidence-backed theses, track holdings, and validate outcomes — but it does
**not** execute trades, connect to brokers, or make autonomous investment
decisions. See the
[Investment Research Disclaimer](../../INVESTMENT_RESEARCH_DISCLAIMER.md).

## Where is my data stored?

Everything is stored **locally** on your machine in a SQLite database inside the
app's user data directory. There is no cloud storage and no account required.
Use **Settings → Local backup** to export a backup file.

## Is my API key safe?

Yes. The API key entered in **Settings → Agent configuration** is stored in the
**operating system keychain** (Windows Credential Manager, macOS Keychain, or
the Linux equivalent), not in the database or in plaintext files.

## Do I need an OpenAI account to use AlphaForge?

You need a provider API key to use **agent tasks** (AI-assisted research). All
other modules — Research, Portfolio, Knowledge, Options, Artifacts viewing — work
without one. A key is entered under **Settings → Agent configuration**.

## Can I use a different AI provider or a proxy?

The app supports an OpenAI-compatible endpoint. Set `VITE_OPENAI_BASE_URL` (and
optionally `VITE_OPENAI_MODEL`) in `.env` to point at a compatible API or proxy.
See [Configuration](configuration.md#option-b--environment-file-advanced).

## How do updates work?

**Settings → Updates → Check for updates** compares versions and opens the
release page when a newer one exists. Installation is manual — there is no
auto-update in the MVP.

## Is there a cloud backup or sync?

No. Backups are manual and local (**Settings → Local backup**). Cloud backup and
sync are explicitly out of scope for the local MVP.

## Which languages does the app support?

English and Simplified Chinese (简体中文). Switch anytime in **Settings →
Language** — no restart needed.

## Can I use AlphaForge without an internet connection?

Most features work offline because data is local. Agent tasks and options chain
fetches require network access to their providers. The UI shows an offline state
when the network is unavailable.

## How do I get help?

- [Troubleshooting](troubleshooting.md) — common issues and fixes.
- [GitHub Issues](https://github.com/BerryUIKI/alpha-forge/issues) — report bugs
  and request features.
- [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions) —
  ask the community.
