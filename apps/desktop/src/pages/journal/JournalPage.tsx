/**
 * Journal Page
 *
 * Chronological ledger of investment thesis evolution, validation milestones,
 * and key research activities across the active workspace.
 *
 * Core product loop: Information → Knowledge → Thesis → Decision → Validation → Review → Improvement
 *
 * @version GUI-M12
 */

import { useState, useMemo } from "react";
import { Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import {
  BookMarked,
  FileText,
  Bot,
  FolderGit2,
  CheckCircle2,
  Clock,
  Sparkles,
  ArrowRight,
} from "lucide-react";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { useTheses } from "@/features/thesis/hooks/useTheses";
import { useAgentTasks } from "@/features/agent/hooks/useAgentTasks";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import { EmptyState, LoadingSpinner, ErrorState } from "@/components/common";

type JournalFilter = "all" | "theses" | "research" | "tasks";

interface JournalTimelineItem {
  id: string;
  type: "thesis" | "research" | "task";
  title: string;
  description?: string | null;
  status?: string;
  date: string;
  confidence?: number;
  outcome?: string | null;
  link?: string;
}

export function JournalPage() {
  const { t } = useLocale();
  const workspaceId = useActiveWorkspaceId();
  const [filter, setFilter] = useState<JournalFilter>("all");

  const {
    data: theses = [],
    isLoading: thesesLoading,
    error: thesesError,
    refetch: refetchTheses,
  } = useTheses(workspaceId);

  const {
    data: tasks = [],
    isLoading: tasksLoading,
    error: tasksError,
    refetch: refetchTasks,
  } = useAgentTasks(workspaceId);

  const {
    data: projects = [],
    isLoading: projectsLoading,
    error: projectsError,
    refetch: refetchProjects,
  } = useQuery({
    queryKey: ["research", "projects", workspaceId],
    queryFn: () => desktopApi.research.listResearchProjects(workspaceId),
    enabled: Boolean(workspaceId),
  });

  const isLoading = thesesLoading || tasksLoading || projectsLoading;
  const isError = Boolean(thesesError || tasksError || projectsError);

  const timelineItems: JournalTimelineItem[] = useMemo(() => {
    const items: JournalTimelineItem[] = [];

    // Theses entries
    for (const th of theses) {
      items.push({
        id: `thesis-${th.id}`,
        type: "thesis",
        title: th.title,
        description: th.thesis,
        status: th.status,
        date: th.createdAt,
        confidence: th.confidence,
        outcome: th.outcome,
        link: "/theses",
      });
    }

    // Research projects
    for (const proj of projects) {
      items.push({
        id: `project-${proj.id}`,
        type: "research",
        title: proj.title,
        description: proj.description,
        status: "active",
        date: proj.createdAt,
        link: `/research?workspace=${workspaceId}&project=${proj.id}`,
      });
    }

    // Agent research tasks
    for (const task of tasks) {
      items.push({
        id: `task-${task.id}`,
        type: "task",
        title: task.title,
        description: task.description,
        status: task.status,
        date: task.createdAt,
        outcome: task.status === "completed" ? "Completed" : null,
      });
    }

    // Sort descending by date
    items.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    return items;
  }, [theses, projects, tasks, workspaceId]);

  const filteredItems = useMemo(() => {
    if (filter === "all") return timelineItems;
    if (filter === "theses") return timelineItems.filter((i) => i.type === "thesis");
    if (filter === "research") return timelineItems.filter((i) => i.type === "research");
    if (filter === "tasks") return timelineItems.filter((i) => i.type === "task");
    return timelineItems;
  }, [timelineItems, filter]);

  if (isLoading) {
    return (
      <div className="flex h-96 items-center justify-center">
        <LoadingSpinner />
      </div>
    );
  }

  if (isError) {
    return (
      <div className="p-6">
        <ErrorState
          message={t("unknownError")}
          onRetry={() => {
            void refetchTheses();
            void refetchTasks();
            void refetchProjects();
          }}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 p-6">
      {/* Header */}
      <div className="flex flex-col gap-2">
        <div className="flex items-center gap-2.5 text-primary">
          <BookMarked className="h-6 w-6" />
          <h1 className="text-2xl font-bold tracking-tight text-foreground">
            {t("journalTitle")}
          </h1>
        </div>
        <p className="max-w-2xl text-sm text-muted-foreground">
          {t("journalDescription")}
        </p>
      </div>

      {/* Filter Tabs */}
      <div className="flex items-center gap-2 border-b border-border pb-3">
        <button
          type="button"
          onClick={() => setFilter("all")}
          className={`rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${
            filter === "all"
              ? "bg-primary text-primary-foreground shadow-sm"
              : "text-muted-foreground hover:bg-muted hover:text-foreground"
          }`}
        >
          {t("journalFilterAll")} ({timelineItems.length})
        </button>
        <button
          type="button"
          onClick={() => setFilter("theses")}
          className={`rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${
            filter === "theses"
              ? "bg-cyan-600 text-white shadow-sm"
              : "text-muted-foreground hover:bg-muted hover:text-foreground"
          }`}
        >
          {t("journalFilterTheses")} ({theses.length})
        </button>
        <button
          type="button"
          onClick={() => setFilter("research")}
          className={`rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${
            filter === "research"
              ? "bg-purple-600 text-white shadow-sm"
              : "text-muted-foreground hover:bg-muted hover:text-foreground"
          }`}
        >
          {t("journalFilterResearch")} ({projects.length})
        </button>
        <button
          type="button"
          onClick={() => setFilter("tasks")}
          className={`rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${
            filter === "tasks"
              ? "bg-emerald-600 text-white shadow-sm"
              : "text-muted-foreground hover:bg-muted hover:text-foreground"
          }`}
        >
          {t("journalFilterTasks")} ({tasks.length})
        </button>
      </div>

      {/* Timeline Stream */}
      {filteredItems.length === 0 ? (
        <EmptyState
          icon={<BookMarked className="h-8 w-8 text-muted-foreground" />}
          title={t("journalNoEntries")}
          description={t("journalNoEntriesDescription")}
        />
      ) : (
        <div className="relative border-l border-border/80 ml-4 pl-6 space-y-6">
          {filteredItems.map((item) => (
            <div key={item.id} className="relative group">
              {/* Timeline Dot */}
              <div
                className={`absolute -left-[31px] top-1.5 flex h-4 w-4 items-center justify-center rounded-full border-2 border-background shadow-sm ${
                  item.type === "thesis"
                    ? "bg-cyan-500"
                    : item.type === "research"
                      ? "bg-purple-500"
                      : "bg-emerald-500"
                }`}
              />

              {/* Card */}
              <div className="rounded-xl border border-border bg-card p-4 transition-all hover:border-primary/40 hover:shadow-sm">
                <div className="flex flex-wrap items-center justify-between gap-2 mb-2">
                  <div className="flex items-center gap-2">
                    <span
                      className={`inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wider ${
                        item.type === "thesis"
                          ? "bg-cyan-500/15 text-cyan-400 border border-cyan-500/30"
                          : item.type === "research"
                            ? "bg-purple-500/15 text-purple-400 border border-purple-500/30"
                            : "bg-emerald-500/15 text-emerald-400 border border-emerald-500/30"
                      }`}
                    >
                      {item.type === "thesis" ? (
                        <FileText className="h-3 w-3" />
                      ) : item.type === "research" ? (
                        <FolderGit2 className="h-3 w-3" />
                      ) : (
                        <Bot className="h-3 w-3" />
                      )}
                      {item.type}
                    </span>

                    {item.status && (
                      <span className="rounded bg-muted px-2 py-0.5 text-[10px] font-mono text-muted-foreground uppercase">
                        {item.status}
                      </span>
                    )}

                    {item.confidence !== undefined && (
                      <span className="inline-flex items-center gap-1 rounded bg-blue-500/10 px-2 py-0.5 text-[10px] font-mono text-blue-400 border border-blue-500/20">
                        <Sparkles className="h-2.5 w-2.5" />
                        {t("journalConfidence")}: {item.confidence}%
                      </span>
                    )}
                  </div>

                  <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
                    <Clock className="h-3.5 w-3.5" />
                    <time dateTime={item.date}>
                      {new Date(item.date).toLocaleDateString(undefined, {
                        year: "numeric",
                        month: "short",
                        day: "numeric",
                        hour: "2-digit",
                        minute: "2-digit",
                      })}
                    </time>
                  </div>
                </div>

                <h3 className="text-base font-semibold text-foreground group-hover:text-primary transition-colors">
                  {item.title}
                </h3>

                {item.description && (
                  <p className="mt-1 text-sm text-muted-foreground leading-relaxed">
                    {item.description}
                  </p>
                )}

                {item.outcome && (
                  <div className="mt-2.5 flex items-center gap-1.5 rounded-lg bg-muted/40 p-2 text-xs text-muted-foreground border border-border/50">
                    <CheckCircle2 className="h-3.5 w-3.5 text-emerald-500 shrink-0" />
                    <span>
                      <strong className="text-foreground">{t("journalOutcome")}:</strong> {item.outcome}
                    </span>
                  </div>
                )}

                {item.link && (
                  <div className="mt-3 flex justify-end">
                    <Link
                      to={item.link}
                      className="inline-flex items-center gap-1 text-xs font-medium text-primary hover:underline"
                    >
                      {t("viewAll")} <ArrowRight className="h-3 w-3" />
                    </Link>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
