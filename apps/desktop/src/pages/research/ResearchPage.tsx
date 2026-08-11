/**
 * Research Page
 *
 * Main page for research project management.
 */

import { useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Trash2, Archive, CheckCircle } from "lucide-react";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import { processAppError } from "@/lib/errors";

export function ResearchPage() {
  const { t } = useLocale();
  const client = useQueryClient();

  // State
  const [workspaceId, setWorkspaceId] = useState("");
  const [projectId, setProjectId] = useState("");
  const [documentId, setDocumentId] = useState("");
  const [projectTitle, setProjectTitle] = useState("");
  const [documentTitle, setDocumentTitle] = useState("");
  const [noteContent, setNoteContent] = useState("");
  const [sourceUrl, setSourceUrl] = useState("");
  const [sourceTitle, setSourceTitle] = useState("");
  const [webPageUrl, setWebPageUrl] = useState("");
  const [reportTitle, setReportTitle] = useState("");
  const [reportContent, setReportContent] = useState("");
  const [reportType, setReportType] = useState<
    "analysis" | "summary" | "thesis" | "recommendation"
  >("analysis");
  const [searchQuery, setSearchQuery] = useState("");
  const [searchMode, setSearchMode] = useState<"lexical" | "semantic">(
    "lexical"
  );
  const [error, setError] = useState("");

  // Queries
  const workspaces = useQuery({
    queryKey: ["workspaces"],
    queryFn: desktopApi.workspace.listWorkspaces,
  });

  const projects = useQuery({
    queryKey: ["research", "projects", workspaceId],
    queryFn: () => desktopApi.research.listResearchProjects(workspaceId),
    enabled: Boolean(workspaceId),
  });

  const documents = useQuery({
    queryKey: ["research", "documents", projectId],
    queryFn: () => desktopApi.research.listResearchDocuments(projectId),
    enabled: Boolean(projectId),
  });

  const reports = useQuery({
    queryKey: ["research", "reports", projectId],
    queryFn: () => desktopApi.research.listResearchReports(projectId),
    enabled: Boolean(projectId),
  });

  const notes = useQuery({
    queryKey: ["research", "notes", documentId],
    queryFn: () => desktopApi.research.listResearchNotes(documentId),
    enabled: Boolean(documentId),
  });

  const sources = useQuery({
    queryKey: ["research", "sources", documentId],
    queryFn: () => desktopApi.research.listResearchSources(documentId),
    enabled: Boolean(documentId),
  });

  const refresh = (kind: string, id: string) =>
    client.invalidateQueries({ queryKey: ["research", kind, id] });

  // Mutations
  const createProject = useMutation({
    mutationFn: () =>
      desktopApi.research.createResearchProject(workspaceId, projectTitle),
    onSuccess: () => {
      setProjectTitle("");
      refresh("projects", workspaceId);
    },
  });

  const createDocument = useMutation({
    mutationFn: () =>
      desktopApi.research.createResearchDocument(projectId, documentTitle),
    onSuccess: () => {
      setDocumentTitle("");
      refresh("documents", projectId);
    },
  });

  const importPdf = useMutation({
    mutationFn: () => desktopApi.research.importResearchPdf(projectId),
    onSuccess: () => refresh("documents", projectId),
  });

  const importWebPage = useMutation({
    mutationFn: () =>
      desktopApi.research.importResearchWebPage(projectId, webPageUrl),
    onSuccess: () => {
      setWebPageUrl("");
      refresh("documents", projectId);
    },
  });

  const createReport = useMutation({
    mutationFn: () =>
      desktopApi.research.createResearchReport(
        projectId,
        reportTitle,
        reportContent,
        reportType
      ),
    onSuccess: () => {
      setReportTitle("");
      setReportContent("");
      refresh("reports", projectId);
    },
  });

  const createNote = useMutation({
    mutationFn: () =>
      desktopApi.research.createResearchNote(documentId, noteContent),
    onSuccess: () => {
      setNoteContent("");
      refresh("notes", documentId);
    },
  });

  const createSource = useMutation({
    mutationFn: () =>
      desktopApi.research.createResearchSource(
        documentId,
        sourceUrl,
        sourceTitle
      ),
    onSuccess: () => {
      setSourceUrl("");
      setSourceTitle("");
      refresh("sources", documentId);
    },
  });

  const documentSearch = useMutation({
    mutationFn: () =>
      desktopApi.research.searchResearchDocument(documentId, searchQuery),
  });

  const semanticDocumentSearch = useMutation({
    mutationFn: () =>
      desktopApi.research.semanticSearchResearchDocument(
        documentId,
        searchQuery
      ),
  });

  // Delete/archive/complete mutations
  const deleteProject = useMutation({
    mutationFn: (id: string) =>
      desktopApi.research.deleteResearchProject(id),
    onSuccess: () => {
      setProjectId("");
      refresh("projects", workspaceId);
    },
  });

  const archiveProject = useMutation({
    mutationFn: (id: string) =>
      desktopApi.research.archiveResearchProject(id),
    onSuccess: () => refresh("projects", workspaceId),
  });

  const completeProject = useMutation({
    mutationFn: (id: string) =>
      desktopApi.research.completeResearchProject(id),
    onSuccess: () => refresh("projects", workspaceId),
  });

  const deleteDocument = useMutation({
    mutationFn: (id: string) =>
      desktopApi.research.deleteResearchDocument(id),
    onSuccess: () => {
      setDocumentId("");
      refresh("documents", projectId);
    },
  });

  const deleteReport = useMutation({
    mutationFn: (id: string) =>
      desktopApi.research.deleteResearchReport(id),
    onSuccess: () => refresh("reports", projectId),
  });

  // Error handler with proper type safety
  async function run(action: () => Promise<unknown>) {
    try {
      setError("");
      await action();
    } catch (err) {
      const errorMessages = processAppError("en", err);
      setError(errorMessages.title);
    }
  }

  return (
    <div className="space-y-6 p-6">
      <div>
        <h1 className="text-2xl font-bold">{t("researchTitle")}</h1>
        <p className="text-sm text-muted-foreground">
          {t("researchDescription")}
        </p>
      </div>

      {error && <p className="text-sm text-destructive">{error}</p>}

      {/* Workspace selector */}
      <label className="block max-w-md text-sm font-medium">
        {t("workspace")}
        <select
          className="mt-1 w-full rounded border bg-background p-2"
          value={workspaceId}
          onChange={(event) => {
            setWorkspaceId(event.target.value);
            setProjectId("");
            setDocumentId("");
          }}
        >
          <option value="">{t("selectWorkspace")}</option>
          {workspaces.data?.map((item) => (
            <option key={item.id} value={item.id}>
              {item.name}
            </option>
          ))}
        </select>
      </label>

      {/* Projects section */}
      {workspaceId && (
        <section className="space-y-3 rounded-lg border p-4">
          <h2 className="font-semibold">{t("projects")}</h2>
          <form
            className="flex gap-2"
            onSubmit={(event) => {
              event.preventDefault();
              run(() => createProject.mutateAsync());
            }}
          >
            <input
              aria-label={t("projectTitle")}
              className="flex-1 rounded border bg-background p-2"
              placeholder={t("projectTitle")}
              value={projectTitle}
              onChange={(event) => setProjectTitle(event.target.value)}
            />
            <button
              className="rounded border px-3"
              disabled={!projectTitle.trim() || createProject.isPending}
            >
              {t("create")}
            </button>
          </form>
          <div className="flex flex-wrap gap-2">
            {projects.data?.map((item) => (
              <div key={item.id} className="flex items-center gap-2">
                <button
                  className="rounded border px-3 py-1 text-sm"
                  onClick={() => {
                    setProjectId(item.id);
                    setDocumentId("");
                  }}
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
                        onClick={() => archiveProject.mutate(item.id)}
                        className="rounded p-1 hover:bg-accent"
                        title={t("archiveProject")}
                      >
                        <Archive className="h-4 w-4 text-muted-foreground" />
                      </button>
                      <button
                        onClick={() => completeProject.mutate(item.id)}
                        className="rounded p-1 hover:bg-accent"
                        title={t("completeProject")}
                      >
                        <CheckCircle className="h-4 w-4 text-muted-foreground" />
                      </button>
                    </>
                  )}
                  <button
                    onClick={() => deleteProject.mutate(item.id)}
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
      )}

      {/* Documents and Reports section */}
      {projectId && (
        <section className="grid gap-4 rounded-lg border p-4 lg:grid-cols-2">
          <div className="space-y-3">
            <h2 className="font-semibold">{t("documents")}</h2>
            <form
              className="flex gap-2"
              onSubmit={(event) => {
                event.preventDefault();
                run(() => createDocument.mutateAsync());
              }}
            >
              <input
                aria-label={t("documentTitle")}
                className="flex-1 rounded border bg-background p-2"
                placeholder={t("documentTitle")}
                value={documentTitle}
                onChange={(event) => setDocumentTitle(event.target.value)}
              />
              <button
                className="rounded border px-3"
                disabled={!documentTitle.trim() || createDocument.isPending}
              >
                {t("add")}
              </button>
            </form>
            <button
              className="rounded border px-3 py-1 text-sm"
              onClick={() => run(() => importPdf.mutateAsync())}
              disabled={importPdf.isPending}
            >
              {importPdf.isPending ? t("importingPdf") : t("importPdf")}
            </button>
            <form
              className="flex gap-2"
              onSubmit={(event) => {
                event.preventDefault();
                run(() => importWebPage.mutateAsync());
              }}
            >
              <input
                aria-label={t("webPageUrl")}
                className="flex-1 rounded border bg-background p-2"
                placeholder={t("webPageUrlPlaceholder")}
                value={webPageUrl}
                onChange={(event) => setWebPageUrl(event.target.value)}
              />
              <button
                className="rounded border px-3"
                disabled={!webPageUrl.trim() || importWebPage.isPending}
              >
                {importWebPage.isPending ? t("importing") : t("importWebPage")}
              </button>
            </form>
            <p className="text-xs text-muted-foreground">{t("importHint")}</p>
            <div className="flex flex-wrap gap-2">
              {documents.data?.map((item) => (
                <div key={item.id} className="flex items-center gap-1">
                  <button
                    className="rounded border px-3 py-1 text-sm"
                    onClick={() => setDocumentId(item.id)}
                  >
                    {item.title}
                  </button>
                  <button
                    onClick={() => deleteDocument.mutate(item.id)}
                    className="rounded p-1 hover:bg-accent"
                    title={t("deleteDocument")}
                  >
                    <Trash2 className="h-3 w-3 text-destructive" />
                  </button>
                </div>
              ))}
            </div>
          </div>

          <div className="space-y-3">
            <h2 className="font-semibold">{t("reports")}</h2>
            <form
              className="space-y-2"
              onSubmit={(event) => {
                event.preventDefault();
                run(() => createReport.mutateAsync());
              }}
            >
              <input
                aria-label={t("reportTitle")}
                className="w-full rounded border bg-background p-2"
                placeholder={t("reportTitle")}
                value={reportTitle}
                onChange={(event) => setReportTitle(event.target.value)}
              />
              <textarea
                aria-label={t("reportContent")}
                className="w-full rounded border bg-background p-2"
                placeholder={t("reportContent")}
                value={reportContent}
                onChange={(event) => setReportContent(event.target.value)}
              />
              <select
                aria-label={t("reportType")}
                className="rounded border bg-background p-2"
                value={reportType}
                onChange={(event) =>
                  setReportType(event.target.value as typeof reportType)
                }
              >
                <option value="analysis">{t("reportTypeAnalysis")}</option>
                <option value="summary">{t("reportTypeSummary")}</option>
                <option value="thesis">{t("reportTypeThesis")}</option>
                <option value="recommendation">
                  {t("reportTypeRecommendation")}
                </option>
              </select>
              <button
                className="ml-2 rounded border px-3 py-1"
                disabled={
                  !reportTitle.trim() ||
                  !reportContent.trim() ||
                  createReport.isPending
                }
              >
                {t("saveReport")}
              </button>
            </form>
            <ul className="space-y-2">
              {reports.data?.map((item) => (
                <li key={item.id} className="rounded bg-muted p-2 text-sm">
                  <div className="flex items-start justify-between">
                    <div>
                      <p className="font-medium">
                        {item.title}{" "}
                        <span className="font-normal text-muted-foreground">
                          ({item.report_type})
                        </span>
                      </p>
                      <p>{item.content}</p>
                    </div>
                    <button
                      onClick={() => deleteReport.mutate(item.id)}
                      className="rounded p-1 hover:bg-accent"
                      title={t("deleteReport")}
                    >
                      <Trash2 className="h-4 w-4 text-destructive" />
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </section>
      )}

      {/* Notes, Sources, and Search section */}
      {documentId && (
        <section className="grid gap-4 rounded-lg border p-4 md:grid-cols-2">
          <div className="space-y-3">
            <h2 className="font-semibold">{t("notes")}</h2>
            <form
              className="space-y-2"
              onSubmit={(event) => {
                event.preventDefault();
                run(() => createNote.mutateAsync());
              }}
            >
              <textarea
                aria-label={t("noteContent")}
                className="w-full rounded border bg-background p-2"
                value={noteContent}
                onChange={(event) => setNoteContent(event.target.value)}
              />
              <button
                className="rounded border px-3 py-1"
                disabled={!noteContent.trim() || createNote.isPending}
              >
                {t("addNote")}
              </button>
            </form>
            <ul className="space-y-2">
              {notes.data?.map((item) => (
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
                run(() => createSource.mutateAsync());
              }}
            >
              <input
                aria-label={t("sourceUrl")}
                className="w-full rounded border bg-background p-2"
                placeholder={t("sourceUrlPlaceholder")}
                value={sourceUrl}
                onChange={(event) => setSourceUrl(event.target.value)}
              />
              <input
                aria-label={t("sourceTitle")}
                className="w-full rounded border bg-background p-2"
                placeholder={t("sourceTitlePlaceholder")}
                value={sourceTitle}
                onChange={(event) => setSourceTitle(event.target.value)}
              />
              <button
                className="rounded border px-3 py-1"
                disabled={!sourceUrl.trim() || createSource.isPending}
              >
                {t("addSource")}
              </button>
            </form>
            <p className="text-xs text-muted-foreground">
              {t("sourcesHint")}
            </p>
            <ul className="space-y-2">
              {sources.data?.map((item) => (
                <li key={item.id} className="rounded bg-muted p-2 text-sm">
                  {item.url ? (
                    <a
                      className="underline"
                      href={item.url}
                      target="_blank"
                      rel="noreferrer"
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
                run(() =>
                  searchMode === "semantic"
                    ? semanticDocumentSearch.mutateAsync()
                    : documentSearch.mutateAsync()
                );
              }}
            >
              <input
                aria-label={t("searchDocument")}
                className="flex-1 rounded border bg-background p-2"
                placeholder={t("searchPlaceholder")}
                value={searchQuery}
                onChange={(event) => setSearchQuery(event.target.value)}
              />
              <select
                aria-label={t("searchMode")}
                className="rounded border bg-background p-2"
                value={searchMode}
                onChange={(event) =>
                  setSearchMode(event.target.value as typeof searchMode)
                }
              >
                <option value="lexical">{t("searchModeLexical")}</option>
                <option value="semantic">{t("searchModeSemantic")}</option>
              </select>
              <button
                className="rounded border px-3 py-1"
                disabled={
                  !searchQuery.trim() ||
                  documentSearch.isPending ||
                  semanticDocumentSearch.isPending
                }
              >
                {t("search")}
              </button>
            </form>
            <p className="text-xs text-muted-foreground">
              {t("searchHint")}
            </p>
            {(searchMode === "semantic"
              ? semanticDocumentSearch.data
              : documentSearch.data) && (
              <ul className="space-y-2">
                {(
                  searchMode === "semantic"
                    ? semanticDocumentSearch.data
                    : documentSearch.data
                )?.map((match) => (
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
      )}
    </div>
  );
}