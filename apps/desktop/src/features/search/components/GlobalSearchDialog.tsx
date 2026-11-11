/**
 * Global Search Dialog
 *
 * Command palette opened from the top bar search control or the Cmd/Ctrl+K
 * shortcut. Searches the active workspace and navigates to the selected entity.
 *
 * Design: docs/GLOBAL_SEARCH_AGENT_CHAT.md
 *
 * @module features/search/components/GlobalSearchDialog
 */

import { useEffect, useMemo, useRef, useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  BookOpen,
  ClipboardList,
  FileText,
  FolderOpen,
  Puzzle,
  Search,
  Target,
} from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage, type MessageKey } from "@/lib/i18n/locale";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { useEscapeKey, useFocusTrap } from "@/lib/hooks";
import { useGlobalSearch, type SearchEntry, type SearchSectionId } from "../hooks/useGlobalSearch";

const SECTION_LABELS: Record<SearchSectionId, MessageKey> = {
  projects: "searchSectionProjects",
  documents: "searchSectionDocuments",
  reports: "searchSectionReports",
  theses: "searchSectionTheses",
  knowledge: "searchSectionKnowledge",
  artifacts: "searchSectionArtifacts",
};

const SECTION_ICONS: Record<SearchSectionId, typeof FolderOpen> = {
  projects: FolderOpen,
  documents: FileText,
  reports: ClipboardList,
  theses: Target,
  knowledge: BookOpen,
  artifacts: Puzzle,
};

interface GlobalSearchDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function GlobalSearchDialog({ isOpen, onClose }: GlobalSearchDialogProps) {
  const { t } = useLocale();
  const navigate = useNavigate();
  const [query, setQuery] = useState("");
  const [activeIndex, setActiveIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const triggerRef = useRef<HTMLElement | null>(null);
  const { workspaceId, isLoading, sections, total, totalMatches, truncated, hasError, refetch } =
    useGlobalSearch(query, isOpen);

  // Trap focus inside the palette and restore it to the opening control.
  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: isOpen,
    returnFocus: triggerRef.current,
  });

  // Reset state and focus the input whenever the palette opens.
  useEffect(() => {
    if (isOpen) {
      triggerRef.current = document.activeElement as HTMLElement;
      setQuery("");
      setActiveIndex(0);
      inputRef.current?.focus();
    }
  }, [isOpen]);

  useEscapeKey(onClose, isOpen);

  // Flatten sections so ArrowUp/ArrowDown and Enter move across groups.
  const flat = useMemo(() => sections.flatMap((section) => section.entries), [sections]);
  const flatIndexById = useMemo(
    () => new Map(flat.map((entry, index) => [entry.id, index])),
    [flat],
  );

  // Keep the active row visible inside the scrolling results area.
  useEffect(() => {
    if (!isOpen) return;
    document
      .getElementById(`global-search-result-${activeIndex}`)
      ?.scrollIntoView?.({ block: "nearest" });
  }, [activeIndex, isOpen, flat.length]);

  const goTo = (entry: SearchEntry) => {
    onClose();
    navigate(entry.to);
  };

  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (event.nativeEvent.isComposing) return;
    if (event.key === "ArrowDown") {
      event.preventDefault();
      setActiveIndex((index) => Math.min(index + 1, flat.length - 1));
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      setActiveIndex((index) => Math.max(index - 1, 0));
    } else if (event.key === "Enter") {
      event.preventDefault();
      const entry = flat[activeIndex];
      if (entry) goTo(entry);
    }
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center bg-black/40 pt-[18vh]"
      role="dialog"
      aria-modal="true"
      aria-label={t("globalSearchTitle")}
      onMouseDown={onClose}
    >
      <div
        ref={containerRef}
        className="w-full max-w-xl overflow-hidden rounded-xl border border-border bg-background shadow-2xl"
        onMouseDown={(event) => event.stopPropagation()}
      >
        {/* Search input */}
        <div className="flex items-center gap-3 border-b border-border px-4 py-3">
          <Search className="h-4 w-4 shrink-0 text-muted-foreground" />
          <input
            ref={inputRef}
            value={query}
            onChange={(event) => {
              setQuery(event.target.value);
              setActiveIndex(0);
            }}
            onKeyDown={handleKeyDown}
            placeholder={t("globalSearchPlaceholder")}
            aria-label={t("globalSearchPlaceholder")}
            role="combobox"
            aria-expanded={total > 0}
            aria-controls="global-search-results"
            aria-autocomplete="list"
            aria-activedescendant={
              flat.length > 0 ? `global-search-result-${activeIndex}` : undefined
            }
            className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground/40"
          />
          {isLoading && <LoadingSpinner size="sm" />}
          <kbd className="rounded border border-border bg-muted px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
            Esc
          </kbd>
        </div>

        {/* Results */}
        <div
          id="global-search-results"
          role="listbox"
          aria-label={t("globalSearchTitle")}
          className="max-h-[45vh] overflow-y-auto p-2"
        >
          {hasError && query.trim() !== "" && (
            <div
              role="alert"
              className="mx-1 mb-2 flex items-center justify-between gap-2 rounded-md border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-xs text-amber-700 dark:text-amber-400"
            >
              <span>{t("searchPartialError")}</span>
              <button
                type="button"
                onClick={() => void refetch()}
                className="rounded border border-amber-500/30 px-2 py-0.5 font-medium hover:bg-amber-500/10"
              >
                {t("retry")}
              </button>
            </div>
          )}
          {!workspaceId ? (
            <p className="p-6 text-center text-sm text-muted-foreground">
              {t("searchNoWorkspace")}
            </p>
          ) : query.trim() === "" ? (
            <p className="p-6 text-center text-sm text-muted-foreground">
              {t("globalSearchPlaceholder")}
            </p>
          ) : total === 0 && !isLoading ? (
            <p className="p-6 text-center text-sm text-muted-foreground">{t("searchNoResults")}</p>
          ) : (
            sections.map((section) => {
              const Icon = SECTION_ICONS[section.id];
              return (
                <div
                  key={section.id}
                  role="group"
                  aria-label={t(SECTION_LABELS[section.id])}
                  className="mb-2 last:mb-0"
                >
                  <h3 className="flex items-center gap-1.5 px-2 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/70">
                    <Icon className="h-3 w-3" aria-hidden="true" />
                    {t(SECTION_LABELS[section.id])}
                  </h3>
                  <ul>
                    {section.entries.map((entry) => {
                      const flatIndex = flatIndexById.get(entry.id) ?? 0;
                      const active = flatIndex === activeIndex;
                      return (
                        <li key={entry.id}>
                          <button
                            id={`global-search-result-${flatIndex}`}
                            type="button"
                            role="option"
                            aria-selected={active}
                            tabIndex={-1}
                            onClick={() => goTo(entry)}
                            onMouseEnter={() => setActiveIndex(flatIndex)}
                            className={`flex w-full items-center gap-3 rounded-md px-3 py-2 text-left text-sm transition-colors ${
                              active ? "bg-accent" : "hover:bg-accent/60"
                            }`}
                          >
                            <span className="min-w-0 flex-1">
                              <span className="block truncate font-medium">{entry.title}</span>
                              {entry.subtitle && (
                                <span className="block truncate text-xs text-muted-foreground">
                                  {entry.subtitle}
                                </span>
                              )}
                            </span>
                          </button>
                        </li>
                      );
                    })}
                  </ul>
                </div>
              );
            })
          )}
          {truncated && (
            <p className="px-2 pb-1 pt-2 text-center text-[10px] text-muted-foreground">
              {formatMessage(t("searchShowingFirst"), {
                shown: String(total),
                total: String(totalMatches),
              })}
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
