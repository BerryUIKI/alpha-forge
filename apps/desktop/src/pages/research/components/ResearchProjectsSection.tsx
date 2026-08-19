/**
 * Research Projects Section
 *
 * Create / select / archive / complete / delete research projects within
 * the active workspace.
 *
 * @module pages/research/components/ResearchProjectsSection
 */

import { Trash2, Archive, CheckCircle } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import type { ResearchProject } from "@/lib/desktop-api/research";

interface ResearchProjectsSectionProps {
  projects: ResearchProject[] | undefined;
  projectTitle: string;
  onProjectTitleChange: (value: string) => void;
  createPending: boolean;
  onCreateProject: () => void;
  onSelectProject: (id: string) => void;
  archiveProject: (id: string) => void;
  completeProject: (id: string) => void;
  deleteProject: (id: string) => void;
}

/**
 * Projects management section: a create form plus the project list with
 * archive / complete / delete actions for active projects.
 */
export function ResearchProjectsSection({
  projects,
  projectTitle,
  onProjectTitleChange,
  createPending,
  onCreateProject,
  onSelectProject,
  archiveProject,
  completeProject,
  deleteProject,
}: ResearchProjectsSectionProps) {
  const { t } = useLocale();

  return (
    <section className="space-y-3 rounded-lg border p-4">
      <h2 className="font-semibold">{t("projects")}</h2>
      <form
        className="flex gap-2"
        onSubmit={(event) => {
          event.preventDefault();
          onCreateProject();
        }}
      >
        <input
          aria-label={t("projectTitle")}
          className="flex-1 rounded border bg-background p-2"
          placeholder={t("projectTitle")}
          value={projectTitle}
          onChange={(event) => onProjectTitleChange(event.target.value)}
        />
        <button
          className="rounded border px-3"
          disabled={!projectTitle.trim() || createPending}
        >
          {t("create")}
        </button>
      </form>
      <div className="flex flex-wrap gap-2">
        {projects?.map((item) => (
          <div key={item.id} className="flex items-center gap-2">
            <button
              className="rounded border px-3 py-1 text-sm"
              onClick={() => onSelectProject(item.id)}
            >
              {item.title}
            </button>
            <span className="text-xs text-muted-foreground">
              ({item.status})
            </span>
            <div className="flex gap-1">
              {item.status === "active" && (
                <>
                  <button
                    onClick={() => archiveProject(item.id)}
                    className="rounded p-1 hover:bg-accent"
                    title={t("archiveProject")}
                  >
                    <Archive className="h-4 w-4 text-muted-foreground" />
                  </button>
                  <button
                    onClick={() => completeProject(item.id)}
                    className="rounded p-1 hover:bg-accent"
                    title={t("completeProject")}
                  >
                    <CheckCircle className="h-4 w-4 text-muted-foreground" />
                  </button>
                </>
              )}
              <button
                onClick={() => deleteProject(item.id)}
                className="rounded p-1 hover:bg-accent"
                title={t("deleteProject")}
              >
                <Trash2 className="h-4 w-4 text-destructive" />
              </button>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
