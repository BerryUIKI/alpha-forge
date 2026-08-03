# i18n Implementation Plan

This plan is the execution path for the i18n portion of M8. It is documentation only and does not mark any implementation item complete.

## Impact assessment

| Area              | Expected impact                                                                                |
| ----------------- | ---------------------------------------------------------------------------------------------- |
| Frontend          | Locale provider, catalogs, formatters, settings UI, critical-flow components, tests            |
| Rust              | Reuse Settings service; only add typed setting validation if the current API cannot enforce it |
| Database          | No migration expected; use the existing settings persistence model                             |
| Tauri             | No new window permissions; route all persistence through existing IPC                          |
| Artifacts/plugins | Locale propagation to predefined renderers only                                                |
| Tests             | Catalog parity, provider, formatters, component states, two-locale smoke coverage              |
| Documentation     | Glossary, translator workflow, release checklist, screenshots                                  |

## Work package I18N-0: Decision and inventory

**Status:** ✅ Complete (M8-01, M8-02)

**Goal:** freeze the launch language policy and identify every user-visible string before choosing code changes.

1. ✅ Resolve the launch-default and reviewer fields in [M8 Decision Record](../M8_DECISION_RECORD.md).
2. ✅ Audit the unmerged M8 locale prototype and current `dev`; record reusable code and conflicts.
3. ✅ Inventory strings under `apps/desktop/src`, grouped by critical workflow and async state.
4. ✅ Classify strings as product UI, Rust error, plugin/Artifact UI, user content, or documentation.
5. ✅ Create an approved bilingual glossary for research, thesis, evidence, portfolio, Option, and risk terms.

**Deliverables:**
- [Terminology Guide](TERMINOLOGY_GUIDE.md) — Canonical English source terms
- [String Inventory](STRING_INVENTORY.md) — Complete inventory by namespace (~350 strings)
- [M8 Decision Record](../M8_DECISION_RECORD.md) — Launch locale `zh-CN`, translation reviewer `@BerryUIKI`

**Exit gate:** ✅ Policy approved, inventory has owner for every critical surface, no competing locale provider.

## Work package I18N-1: Runtime foundation

**Status:** ✅ Complete (M8-03)

**Goal:** deliver one typed runtime path with deterministic fallback and persistence.

1. ✅ Add the module layout specified in [Architecture](ARCHITECTURE.md).
2. ✅ Define `SupportedLocale`, validation, source locale, launch default, and fallback behavior in `locale.ts`.
3. ✅ Implement `LocaleProvider` and `useLocale`; mount it in `apps/desktop/src/app/providers.tsx`.
4. ✅ Persist a valid locale through `apps/desktop/src/lib/desktop-api/settings.ts` and the existing Rust Settings service.
5. ✅ Implement shared `Intl` formatters and unit tests.
6. ✅ Add source and translated `common`, `navigation`, and `settings` catalogs plus a parity test.

**Verification:** ✅ Passed

```bash
pnpm typecheck
pnpm test -- apps/desktop/src/lib/i18n
pnpm lint
```

**Exit gate:** ✅ Complete. A user can change locale in Settings, the choice survives restart, invalid stored values recover safely, and no plaintext credential or arbitrary file access is introduced.

## Work package I18N-2: Application shell and common states

**Status:** ✅ Complete (M8-04)

**Goal:** make the persistent shell and reusable asynchronous states complete in both locales.

1. ✅ Localize sidebar navigation, page titles, command palette, settings, dialogs, buttons, and tooltips.
2. ✅ Localize shared loading, empty, error, partial, and offline components.
3. ✅ Add localized labels and announcements for keyboard navigation and focus management.
4. ✅ Replace ad hoc date/number formatting in shared components with the approved helpers.
5. ✅ Add component tests that switch locale at runtime.

**Verification:** ✅ Passed

```bash
pnpm test -- apps/desktop/src/components/navigation/Sidebar.test.tsx \
  apps/desktop/src/components/common/EmptyState.test.tsx \
  apps/desktop/src/components/common/ErrorState.test.tsx \
  apps/desktop/src/components/common/LoadingSpinner.test.tsx \
  apps/desktop/src/components/common/OfflineState.test.tsx --run
# 24 tests passed
```

**Exit gate:** ✅ Complete. Every route can be reached and recovered from an error in either locale without clipping or inaccessible labels.

## Work package I18N-3: Critical MVP workflows

Deliver vertical slices in this order:

1. Workspace create/select and Today dashboard.
2. Research projects, documents, sources, notes, search, and reports.
3. Journal, thesis evidence, confidence history, and validation.
4. Portfolio accounts, positions, imports, allocation, and risk review.
5. Agent task lifecycle and structured Artifact renderers.
6. Option UI after the Option integration milestone begins; Option catalogs must not block M8 if Option remains outside the MVP release.

For each slice:

```text
Inventory keys
  -> add source messages
  -> add reviewed translation
  -> replace component literals
  -> localize error codes and all async states
  -> add component tests
  -> bilingual review
```

**Exit gate:** the core product loop can be completed in both locales without falling back to untranslated critical controls.

## Work package I18N-4: Artifact and error integration

1. Create a stable error-code-to-message-key map in the frontend.
2. Audit Rust commands for stable error codes and safe interpolation context; do not localize logs.
3. Provide locale and formatter access to predefined Artifact renderers without granting new permissions.
4. Confirm plugin IDs, JSON schemas, database enums, and structured agent output remain language-neutral.
5. Add regression tests for unknown errors and partially translated optional plugins.

**Exit gate:** user-visible failures are localized, diagnostics remain actionable and redacted, and Artifact isolation is unchanged.

## Work package I18N-5: Release readiness

1. Run catalog parity and string-literal audits.
2. Test 100%, 125%, and 150% display scaling and 30% text expansion.
3. Smoke-test macOS and Windows packaged builds in both locales.
4. Review locale-specific date, currency, percent, disclaimer, and Option precision samples.
5. Publish the translator workflow and glossary; capture localized release screenshots.
6. Record deferred locales and known untranslated non-critical surfaces.

**Verification:**

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pnpm tauri build
```

Platform smoke testing may require separate macOS and Windows jobs. A command is not marked passed until its output is retained in the implementation PR.

## Branch and PR sequence

Use small branches from the then-current `dev` branch, for example:

```text
feature/i18n-runtime
feature/i18n-shell
feature/i18n-research
feature/i18n-thesis-portfolio
test/i18n-release-qa
```

Each PR completes one visible vertical slice, updates this plan's evidence table in its PR description, and targets `dev`. Do not mix i18n rollout with Option or Goose implementation changes.

## Definition of done

- The launch default and reviewers are recorded.
- `en` and `zh-CN` catalogs have parity and approved terminology.
- Locale persistence, fallback, interpolation, and formatters are tested.
- All critical MVP workflows and asynchronous states are usable in both locales.
- Rust remains responsible for settings and stable errors; React remains responsible for presentation.
- No user content, evidence quotation, or numeric value is silently translated or transformed.
- Packaged smoke tests and accessibility review are documented.

## Risks and mitigations

| Risk                                                     | Mitigation                                                                   |
| -------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Two locale implementations emerge from parallel branches | Audit and select one runtime in I18N-0                                       |
| String replacement creates semantic or financial errors  | Use a reviewed glossary and domain-specific formatting tests                 |
| Missing keys appear only in failure states               | Catalog parity plus explicit async-state component tests                     |
| Locale switch causes startup flicker                     | Deterministic bootstrap state and provider test                              |
| Option integration adds untranslated surfaces later      | Reserve the `options` namespace and make M9 localization an integration gate |
