# Developer Documentation Map — AlphaForge User Documentation

This document is for **maintainers and developers**. It maps every user
documentation file to the project page/module it describes, explains how to link
pages to documents, and defines the maintenance rules for this documentation
set.

## 1. File-to-Module Mapping

### User documentation (English, canonical)

| Documentation file | Project page / module | Primary source code |
|--------------------|----------------------|---------------------|
| `docs/user/README.md` | Entry point / language switcher (no page) | — |
| `docs/user/en/README.md` | Documentation index (no page) | — |
| `docs/user/en/installation.md` | Developer setup / release build | `README.md` (Getting Started), `scripts/*.sh`, `rust-toolchain.toml`, `.node-version` |
| `docs/user/en/configuration.md` | Settings page (all sections) | `apps/desktop/src/pages/settings/SettingsPage.tsx`, `apps/desktop/src/features/plugins/components/InternalPluginsPanel.tsx`, `apps/desktop/src/lib/i18n/locale.ts`, `.env.example` |
| `docs/user/en/configuration.md` (Agent) | Agent configuration | `apps/desktop/src/features/agent/components/AgentConfigGuide.tsx`, `apps/desktop/src/lib/desktop-api/credentials.ts` |
| `docs/user/en/daily-operations.md` (Workspaces) | Workspace switcher / active-workspace context | `apps/desktop/src/features/workspace/`, `apps/desktop/src/components/layout/TopBar/WorkspaceSwitcher.tsx` |
| `docs/user/en/daily-operations.md` (Today) | Today dashboard | `apps/desktop/src/pages/today/TodayPage.tsx` + `tabs/` (Overview, Performance, Activity) |
| `docs/user/en/daily-operations.md` (Research) | Research page | `apps/desktop/src/pages/research/ResearchPage.tsx` + `components/` |
| `docs/user/en/daily-operations.md` (Theses/Journal) | Theses / Journal pages | `apps/desktop/src/pages/theses/ThesesPage.tsx`, `apps/desktop/src/pages/journal/JournalPage.tsx`, `apps/desktop/src/features/thesis/` |
| `docs/user/en/daily-operations.md` (Portfolio) | Portfolio page | `apps/desktop/src/pages/portfolio/PortfolioPage.tsx`, `apps/desktop/src/features/portfolio/` |
| `docs/user/en/daily-operations.md` (Knowledge) | Knowledge page | `apps/desktop/src/pages/knowledge/KnowledgePage.tsx`, `apps/desktop/src/lib/desktop-api/knowledge-graph.ts` |
| `docs/user/en/daily-operations.md` (Options) | Options page | `apps/desktop/src/pages/options/OptionsPage.tsx`, `apps/desktop/src/features/options/`, `crates/option-core/` |
| `docs/user/en/daily-operations.md` (Artifacts) | Artifacts page + artifact window | `apps/desktop/src/pages/artifacts/ArtifactsPage.tsx`, `apps/desktop/src/pages/artifacts/ArtifactWindowPage.tsx`, `apps/desktop/src/features/artifacts/`, `apps/desktop/src/features/plugins/components/CompanyComparisonArtifactForm.tsx` |
| `docs/user/en/daily-operations.md` (Global Search) | Global search dialog | `apps/desktop/src/features/search/` |
| `docs/user/en/daily-operations.md` (Agent Tasks) | Agent task list / creation | `apps/desktop/src/features/agent/`, `apps/desktop/src/hooks/useAgentStatus.ts`, `crates/agent-core/` |
| `docs/user/en/troubleshooting.md` | All modules (error scenarios) | Cross-cutting: `apps/desktop/src/lib/errors/`, `apps/desktop/src/components/common/` (EmptyState, ErrorState, LoadingSpinner) |
| `docs/user/en/faq.md` | All modules | Cross-cutting |

### Translations

| Documentation file | Status |
|-------------------|--------|
| `docs/user/zh/README.md` | Placeholder — structure reserved, translation pending |

## 2. Linking Suggestions

### In-app links (help / about surfaces)

- **Settings → About & privacy** already opens repository documents
  (`docs/PRIVACY.md`, `docs/INVESTMENT_RESEARCH_DISCLAIMER.md`). The same pattern
  can open user documentation:
  `https://github.com/BerryUIKI/alpha-forge/blob/dev/docs/user/en/<file>.md`.
- **Agent Configuration Guide dialog** — consider linking
  `configuration.md#ai-provider-agent-configuration` when the "go to settings"
  action is not enough.
- **Empty states** — each module's empty state could link to the matching
  section of `daily-operations.md` (see mapping table) to guide first-time users.

### Repository links

- `README.md` **Documentation** section: add a row "User documentation" pointing
  to `docs/user/README.md`.
- Use relative links inside `docs/user/` so links survive branch/repo moves.

### GitHub Wiki

The wiki mirrors the English set (Home, Installation, Configuration,
Daily Operations, Troubleshooting, FAQ). Keep the wiki in sync with the
canonical English files; the wiki is a presentation surface, not a source of
truth.

## 3. Maintenance Instructions

### When to update

Update the affected documentation file in the **same change** as the code:

| Code change | Docs to update |
|-------------|----------------|
| New/changed Settings option | `configuration.md` |
| New/changed module or page | `daily-operations.md` + mapping table in this file |
| New error scenario | `troubleshooting.md` |
| New FAQ topic | `faq.md` |
| New supported locale | New directory under `docs/user/` + language table in `docs/user/README.md` |
| New prerequisite / build step | `installation.md` |

### Review checklist

1. File names and section anchors referenced by links still exist
   (`#anchor` links break silently).
2. The mapping table in this document lists every new doc file.
3. The language table in `docs/user/README.md` matches the language directories.
4. No secrets or internal paths in user-facing files.
5. Wiki pages still mirror the English set (or are updated).

### Conventions

- **Language**: user documentation is written in English; translations are
  added per language directory (`en/`, `zh/`, ...) without renaming files.
- **Naming**: lowercase, hyphenated file names (`installation.md`,
  `daily-operations.md`).
- **One doc per concern**: installation, configuration, daily operations,
  troubleshooting, FAQ — do not merge topics into one mega-file.
- **Cross-links**: prefer relative links within `docs/user/`; use the mapping
  table above when linking from code.
