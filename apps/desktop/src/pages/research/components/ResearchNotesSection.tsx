/**
 * Research Notes & Sources Section
 *
 * Notes, sources, and document search for the selected document.
 *
 * @module pages/research/components/ResearchNotesSection
 */

import { useLocale } from "@/lib/i18n/useLocale";
import type {
  ResearchNote,
  ResearchSource,
  ResearchSearchMatch,
} from "@/lib/desktop-api/research";

type SearchMode = "lexical" | "semantic";

interface ResearchNotesSectionProps {
  notes: ResearchNote[] | undefined;
  sources: ResearchSource[] | undefined;
  noteContent: string;
  onNoteContentChange: (value: string) => void;
  createNotePending: boolean;
  onCreateNote: () => void;
  sourceUrl: string;
  onSourceUrlChange: (value: string) => void;
  sourceTitle: string;
  onSourceTitleChange: (value: string) => void;
  createSourcePending: boolean;
  onCreateSource: () => void;
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  searchMode: SearchMode;
  onSearchModeChange: (value: SearchMode) => void;
  searchResults: ResearchSearchMatch[] | undefined;
  searchPending: boolean;
  onSearch: () => void;
}

/**
 * Two-column section for notes and sources, with a full-width document
 * search at the bottom.
 */
export function ResearchNotesSection({
  notes,
  sources,
  noteContent,
  onNoteContentChange,
  createNotePending,
  onCreateNote,
  sourceUrl,
  onSourceUrlChange,
  sourceTitle,
  onSourceTitleChange,
  createSourcePending,
  onCreateSource,
  searchQuery,
  onSearchQueryChange,
  searchMode,
  onSearchModeChange,
  searchResults,
  searchPending,
  onSearch,
}: ResearchNotesSectionProps) {
  const { t } = useLocale();

  return (
    <section className="grid gap-4 rounded-lg border p-4 md:grid-cols-2">
      <div className="space-y-3">
        <h2 className="font-semibold">{t("notes")}</h2>
        <form
          className="space-y-2"
          onSubmit={(event) => {
            event.preventDefault();
            onCreateNote();
          }}
        >
          <textarea
            aria-label={t("noteContent")}
            className="w-full rounded border bg-background p-2"
            value={noteContent}
            onChange={(event) => onNoteContentChange(event.target.value)}
          />
          <button
            className="rounded border px-3 py-1"
            disabled={!noteContent.trim() || createNotePending}
          >
            {t("addNote")}
          </button>
        </form>
        <ul className="space-y-2">
          {notes?.map((item) => (
            <li key={item.id} className="rounded bg-muted p-2 text-sm">
              {item.content}
            </li>
          ))}
        </ul>
      </div>

      <div className="space-y-3">
        <h2 className="font-semibold">{t("sources")}</h2>
        <form
          className="space-y-2"
          onSubmit={(event) => {
            event.preventDefault();
            onCreateSource();
          }}
        >
          <input
            aria-label={t("sourceUrl")}
            className="w-full rounded border bg-background p-2"
            placeholder={t("sourceUrlPlaceholder")}
            value={sourceUrl}
            onChange={(event) => onSourceUrlChange(event.target.value)}
          />
          <input
            aria-label={t("sourceTitle")}
            className="w-full rounded border bg-background p-2"
            placeholder={t("sourceTitlePlaceholder")}
            value={sourceTitle}
            onChange={(event) => onSourceTitleChange(event.target.value)}
          />
          <button
            className="rounded border px-3 py-1"
            disabled={!sourceUrl.trim() || createSourcePending}
          >
            {t("addSource")}
          </button>
        </form>
        <p className="text-xs text-muted-foreground">
          {t("sourcesHint")}
        </p>
        <ul className="space-y-2">
          {sources?.map((item) => (
            <li key={item.id} className="rounded bg-muted p-2 text-sm">
              {item.url ? (
                <a
                  className="underline"
                  href={item.url}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  {item.title || item.url}
                </a>
              ) : (
                item.title || t("untitledSource")
              )}
            </li>
          ))}
        </ul>
      </div>

      <div className="space-y-3 md:col-span-2">
        <h2 className="font-semibold">{t("searchDocument")}</h2>
        <form
          className="flex gap-2"
          onSubmit={(event) => {
            event.preventDefault();
            onSearch();
          }}
        >
          <input
            aria-label={t("searchDocument")}
            className="flex-1 rounded border bg-background p-2"
            placeholder={t("searchPlaceholder")}
            value={searchQuery}
            onChange={(event) => onSearchQueryChange(event.target.value)}
          />
          <select
            aria-label={t("searchMode")}
            className="rounded border bg-background p-2"
            value={searchMode}
            onChange={(event) =>
              onSearchModeChange(event.target.value as SearchMode)
            }
          >
            <option value="lexical">{t("searchModeLexical")}</option>
            <option value="semantic">{t("searchModeSemantic")}</option>
          </select>
          <button
            className="rounded border px-3 py-1"
            disabled={!searchQuery.trim() || searchPending}
          >
            {t("search")}
          </button>
        </form>
        <p className="text-xs text-muted-foreground">
          {t("searchHint")}
        </p>
        {searchResults && (
          <ul className="space-y-2">
            {searchResults.map((match) => (
              <li
                key={match.ordinal}
                className="rounded bg-muted p-2 text-sm"
              >
                <span className="mr-2 text-muted-foreground">
                  {t("searchScore")} {match.score}
                </span>
                {match.content}
              </li>
            ))}
          </ul>
        )}
      </div>
    </section>
  );
}
