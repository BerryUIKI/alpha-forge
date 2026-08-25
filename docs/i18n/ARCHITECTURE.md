# i18n Architecture

## Goals

Professional financial terminology is a separate preference layer. Ordinary interface copy is
always localized, while domain labels can remain in their source form unless the user enables
`app.i18n.professionalTermsEnabled`. User overrides are validated and stored under
`app.i18n.professionalTermOverrides`; they must never alter stable protocol or database enum values.

The i18n layer must make the React UI translatable without moving presentation logic into Rust, weakening type safety, or changing persisted domain values. It should be small enough for the two-locale MVP and structured enough to add reviewed locales later.

## Runtime flow

```text
Application startup
  -> LocaleProvider loads saved locale through desktopApi.settings
  -> validate against supported locale identifiers
  -> fall back to the approved launch default
  -> load the source catalog and selected catalog
  -> React renders translated messages
  -> Intl formats dates, numbers, percentages, and currencies

Settings change
  -> validate locale
  -> persist through desktopApi.settings
  -> update provider state
  -> rerender without restarting
```

React owns the locale context, translation lookup, interpolation, and formatting. Rust owns setting persistence and stable error codes. Tauri remains the typed IPC boundary.

## Implemented code layout (M8-03)

```text
apps/desktop/src/lib/i18n/
  index.ts                  # Module barrel file
  locale.ts                 # Supported locales, validation, fallback, inline messages
  locale-context.ts         # React context definition
  LocaleProvider.tsx        # Startup loading and runtime switching
  LocaleProvider.test.tsx   # Provider tests
  useLocale.ts              # Context hook
  locale.test.ts            # Core locale tests
  formatters.ts             # Intl wrappers for date, number, percent, currency
  formatters.test.ts        # Formatter unit tests
  catalog-parity.test.ts    # Catalog key parity test
  catalogs/
    en/
      index.ts
      common.ts             # Common UI strings
      navigation.ts         # Navigation labels
      settings.ts           # Settings page strings
    zh-CN/
      index.ts
      common.ts
      navigation.ts
      settings.ts
```

Feature catalogs are namespaced but assembled by a single i18n module. Components must not import catalogs directly; they use `useLocale()` so the implementation can evolve without touching every caller.

## Locale and key model

- Use BCP 47 identifiers: `en`, `zh-CN`, with architecture supporting future additions.
- Keep the supported-locale allowlist explicit; never trust a persisted or URL-provided locale.
- Use semantic dotted keys such as `research.empty.title`, not source sentences as keys.
- Preserve key parity between the source and translated catalogs with a test.
- Keep interpolation variables named and typed at the translation boundary.
- Do not concatenate translated fragments. Use complete messages so translators can reorder grammar.
- Do not place Markdown or untrusted HTML in catalogs. Render rich UI through React components with controlled placeholders.

### Adding new locales

The architecture supports easy addition of new locales:

1. Add locale identifier to `LOCALES` array in `locale.ts`
2. Create new catalog directory: `catalogs/{locale-id}/`
3. Translate all catalog files maintaining key parity
4. Add locale to parity tests
5. Update language selector in Settings
6. Update documentation with translator credits

No code changes required in components or business logic.

## Persistence and startup behavior

The locale setting uses the existing Settings repository and `desktopApi.settings`; React must not read SQLite or operating-system files. Startup must render a deterministic fallback while the stored setting loads so the app does not flash between two languages.

**Locale precedence (updated for multi-language support):**

1. Valid saved application setting (user's explicit choice).
2. System locale detection (if supported and valid).
3. Product-specific default locale.
4. `en` as the final catalog fallback for missing keys.

### System Language Detection

The application detects the user's system language at first startup:

- **Detection method**: `navigator.language` or `Intl.DateTimeFormat().resolvedOptions().locale`
- **Supported locales**: Only BCP 47 identifiers in the allowlist (`en`, `zh-CN` in MVP)
- **Fallback chain**: If system locale is unsupported (e.g., `fr`, `de`), use product default
- **User override**: Users can always change language in Settings, which takes precedence

**Implementation principle:**
- First-time users get their system language if supported
- Returning users get their saved preference
- All behavior is deterministic and testable
- Easy to add new locales without code changes

**Example flow:**
```
User in China with zh-CN system locale:
  -> First launch: auto-select zh-CN
  -> User changes to en in Settings
  -> Next launch: use saved en preference

User in France with fr system locale:
  -> First launch: fr not in supported list
  -> Fall back to product default (zh-CN)
  -> User can manually select en or stay with zh-CN
```

## Formatting rules

Use `Intl.DateTimeFormat`, `Intl.NumberFormat`, and `Intl.RelativeTimeFormat` behind shared helpers. Callers pass semantic inputs, including currency codes; they do not build locale strings manually.

| Value                | Storage or protocol                 | Presentation                                      |
| -------------------- | ----------------------------------- | ------------------------------------------------- |
| Timestamp            | UTC ISO 8601                        | User locale and explicit time zone policy         |
| Decimal              | Numeric value                       | Locale grouping and decimal separator             |
| Percent              | Ratio or documented percent value   | One shared percent helper to avoid 100x errors    |
| Currency             | Amount plus ISO 4217 code           | Locale format without converting currency         |
| Option strike/Greeks | Numeric value with domain precision | Locale separators; domain precision remains fixed |

Formatting must never alter database values, API payloads, calculation inputs, CSV import semantics, or evidence quotations.

## Error localization

Rust production paths return the application error envelope:

```text
code
message
context
recoverable
```

The frontend maps stable `code` values to translation keys and uses safe, allowlisted context for interpolation. Unknown codes fall back to a generic localized message while structured logs retain the original diagnostic. Raw SQL errors, paths, provider responses, and secrets must not be interpolated into user-facing translations.

## Artifacts and plugins

Predefined React artifact renderers may receive the current locale and use the same formatter helpers. Plugin manifests, schemas, persisted artifact types, and agent-generated JSON remain locale-neutral. Agent-produced text is user content and is not automatically translated by the i18n layer.

Artifact windows must not receive catalog-loading filesystem access or broader Tauri permissions. If a renderer needs messages, bundle the approved namespace with that renderer or pass a narrow locale/message payload.

## Accessibility and layout

- Language selectors expose localized names plus unambiguous language codes.
- Every locale change remains keyboard accessible and announces the new language.
- Visible focus states, labels, tooltips, and `aria-label` values use the same message source.
- Components must tolerate at least 30 percent text expansion without clipping.
- Tests cover empty, loading, error, partial, and offline messages—not only successful data states.

## Quality gates

- Catalog parity test passes with no missing or orphaned keys.
- No critical-flow component contains newly introduced user-facing string literals outside approved exceptions.
- Both locales pass typecheck, unit/component tests, and a packaged-build smoke test.
- A bilingual reviewer approves finance terminology, disclaimer text, and destructive-action labels.
- Date, number, percent, currency, and Option-domain precision snapshots are reviewed in both locales.
