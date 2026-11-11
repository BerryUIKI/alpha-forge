/**
 * Research Page
 *
 * Main page for research project management.
 * The active workspace comes from the global active-workspace context (ADR-0008);
 * the URL `project` parameter remains a deep-link for the selected project.
 *
 * @module pages/research/ResearchPage
 */

import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import { processAppError } from "@/lib/errors";
import { useActiveWorkspace } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { CreateWorkspaceDialog } from "@/features/workspace/components/CreateWorkspaceDialog";
import { ResearchProjectsSection } from "./components/ResearchProjectsSection";
import { ResearchDocumentsSection } from "./components/ResearchDocumentsSection";
import { ResearchNotesSection } from "./components/ResearchNotesSection";

export function ResearchPage() {
  const { t } = useLocale();
  const client = useQueryClient();
  const [searchParams, setSearchParams] = useSearchParams();
  const { workspaceId, workspaces, isLoading, setActiveWorkspace } = useActiveWorkspace();
  const projectId = searchParams.get("project") || "";
  const requestedDocumentId = searchParams.get("document") || "";
  const requestedReportId = searchParams.get("report") || "";
  const createRequested = searchParams.get("create") === "1";
  const [isCreateWorkspaceOpen, setIsCreateWorkspaceOpen] = useState(false);

  // State
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
  const [searchMode, setSearchMode] = useState<"lexical" | "semantic">("lexical");
  const [error, setError] = useState("");

  // The active workspace is always valid once resolved; it can only be "" while
  // loading or when no workspace exists yet.
  const workspaceIsValid =
    !isLoading && Boolean(workspaceId) && workspaces.some((item) => item.id === workspaceId);

  // The header's "New Research" action and Cmd/Ctrl+N navigate here with
  // `create=1`. Focus the project title field, then consume the intent so a
  // refresh or unrelated navigation does not steal focus again.
  useEffect(() => {
    if (!createRequested || !workspaceIsValid) return;

    const frame = requestAnimationFrame(() => {
      const input = document.getElementById("new-research-project-title");
      if (!input) return;
      input.focus();
      const next = new URLSearchParams(searchParams);
      next.delete("create");
      setSearchParams(next, { replace: true });
    });

    return () => cancelAnimationFrame(frame);
  }, [createRequested, searchParams, setSearchParams, workspaceIsValid]);

  const projects = useQuery({
    queryKey: ["research", "projects", workspaceId],
    queryFn: () => desktopApi.research.listResearchProjects(workspaceId),
    enabled: workspaceIsValid,
  });
  const projectIsValid =
    workspaceIsValid &&
    projects.isSuccess &&
    Boolean(projectId) &&
    (projects.data ?? []).some((item) => item.id === projectId);

  // The URL `project` parameter is the source of truth for the selected
  // project. Clean a stale project id only after the corresponding query has
  // succeeded, so loading and error states do not erase a deep link that may
  // still become valid.
  useEffect(() => {
    if (!workspaceIsValid || !projects.isSuccess || projects.isFetching) return;
    if (!projectId && !searchParams.has("project")) return;
    if (projectIsValid) return;

    const next = new URLSearchParams(searchParams);
    next.delete("project");
    if (next.toString() !== searchParams.toString()) {
      setSearchParams(next, { replace: true });
    }
  }, [
    projectId,
    projects.isFetching,
    projects.isSuccess,
    projectIsValid,
    searchParams,
    setSearchParams,
    workspaceIsValid,
  ]);

  useEffect(() => {
    setDocumentId("");
  }, [projectId, workspaceId]);

  const updateContext = (project?: string, options?: { replace?: boolean }) => {
    const next = new URLSearchParams(searchParams);
    if (project) next.set("project", project);
    else next.delete("project");
    // Selecting a project invalidates any entity deep link from the previous
    // project; the receiving effects will re-add one when appropriate.
    next.delete("document");
    next.delete("report");
    if (next.toString() !== searchParams.toString()) {
      setSearchParams(next, options);
    }
  };

  const selectDocument = (id: string) => {
    setDocumentId(id);
    const next = new URLSearchParams(searchParams);
    next.set("document", id);
    setSearchParams(next, { replace: true });
  };

  const documents = useQuery({
    queryKey: ["research", "documents", projectId],
    queryFn: () => desktopApi.research.listResearchDocuments(projectId),
    enabled: projectIsValid,
  });

  const reports = useQuery({
    queryKey: ["research", "reports", projectId],
    queryFn: () => desktopApi.research.listResearchReports(projectId),
    enabled: projectIsValid,
  });

  // Deep links from global search: open the exact document or reveal the exact
  // report. Invalid ids are removed after the owning query succeeds so loading
  // or errors never erase a link that may still become valid.
  useEffect(() => {
    if (!projectIsValid || !documents.isSuccess || !requestedDocumentId) return;
    const exists = (documents.data ?? []).some((document) => document.id === requestedDocumentId);
    if (exists) {
      setDocumentId(requestedDocumentId);
      return;
    }

    const next = new URLSearchParams(searchParams);
    next.delete("document");
    setSearchParams(next, { replace: true });
  }, [
    projectIsValid,
    documents.isSuccess,
    documents.data,
    requestedDocumentId,
    searchParams,
    setSearchParams,
  ]);

  useEffect(() => {
    if (!projectIsValid || !reports.isSuccess || !requestedReportId) return;
    const exists = (reports.data ?? []).some((report) => report.id === requestedReportId);
    if (exists) {
      document
        .getElementById(`research-report-${requestedReportId}`)
        ?.scrollIntoView?.({ block: "nearest" });
      return;
    }

    const next = new URLSearchParams(searchParams);
    next.delete("report");
    setSearchParams(next, { replace: true });
  }, [
    projectIsValid,
    reports.isSuccess,
    reports.data,
    requestedReportId,
    searchParams,
    setSearchParams,
  ]);

  const notes = useQuery({
    queryKey: ["research", "notes", documentId],
    queryFn: () => desktopApi.research.listResearchNotes(documentId),
    enabled: projectIsValid && Boolean(documentId),
  });

  const sources = useQuery({
    queryKey: ["research", "sources", documentId],
    queryFn: () => desktopApi.research.listResearchSources(documentId),
    enabled: projectIsValid && Boolean(documentId),
  });

  const refresh = (kind: string, id: string) =>
    client.invalidateQueries({ queryKey: ["research", kind, id] });

  // Mutations
  const createProject = useMutation({
    mutationFn: () => desktopApi.research.createResearchProject(workspaceId, projectTitle),
    onSuccess: () => {
      setProjectTitle("");
      refresh("projects", workspaceId);
    },
  });

  const createDocument = useMutation({
    mutationFn: () => desktopApi.research.createResearchDocument(projectId, documentTitle),
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
    mutationFn: () => desktopApi.research.importResearchWebPage(projectId, webPageUrl),
    onSuccess: () => {
      setWebPageUrl("");
      refresh("documents", projectId);
    },
  });

  const createReport = useMutation({
    mutationFn: () =>
      desktopApi.research.createResearchReport(projectId, reportTitle, reportContent, reportType),
    onSuccess: () => {
      setReportTitle("");
      setReportContent("");
      refresh("reports", projectId);
    },
  });

  const createNote = useMutation({
    mutationFn: () => desktopApi.research.createResearchNote(documentId, noteContent),
    onSuccess: () => {
      setNoteContent("");
      refresh("notes", documentId);
    },
  });

  const createSource = useMutation({
    mutationFn: () => desktopApi.research.createResearchSource(documentId, sourceUrl, sourceTitle),
    onSuccess: () => {
      setSourceUrl("");
      setSourceTitle("");
      refresh("sources", documentId);
    },
  });

  const documentSearch = useMutation({
    mutationFn: () => desktopApi.research.searchResearchDocument(documentId, searchQuery),
  });

  const semanticDocumentSearch = useMutation({
    mutationFn: () => desktopApi.research.semanticSearchResearchDocument(documentId, searchQuery),
  });

  // Delete/archive/complete mutations
  const deleteProject = useMutation({
    mutationFn: (id: string) => desktopApi.research.deleteResearchProject(id),
    onSuccess: (_result, deletedProjectId) => {
      if (deletedProjectId === projectId) {
        updateContext(undefined, { replace: true });
      }
      refresh("projects", workspaceId);
    },
  });

  const archiveProject = useMutation({
    mutationFn: (id: string) => desktopApi.research.archiveResearchProject(id),
    onSuccess: () => refresh("projects", workspaceId),
  });

  const completeProject = useMutation({
    mutationFn: (id: string) => desktopApi.research.completeResearchProject(id),
    onSuccess: () => refresh("projects", workspaceId),
  });

  const deleteDocument = useMutation({
    mutationFn: (id: string) => desktopApi.research.deleteResearchDocument(id),
    onSuccess: () => {
      setDocumentId("");
      refresh("documents", projectId);
    },
  });

  const deleteReport = useMutation({
    mutationFn: (id: string) => desktopApi.research.deleteResearchReport(id),
    onSuccess: () => refresh("reports", projectId),
  });

  // .mutate(id) swallows errors into isError (unlike run(), which throws and
  // is caught above). Surface those failures with a banner.
  const researchOperationError =
    archiveProject.isError ||
    completeProject.isError ||
    deleteProject.isError ||
    deleteDocument.isError ||
    deleteReport.isError;

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
        <p className="text-sm text-muted-foreground">{t("researchDescription")}</p>
      </div>

      {error && <p className="text-sm text-destructive">{error}</p>}

      {researchOperationError && (
        <p className="text-sm text-destructive" role="alert">
          {t("unexpectedError")}
        </p>
      )}

      {/* No workspace empty state */}
      {!isLoading && workspaces.length === 0 && (
        <div className="rounded-lg border border-dashed border-border p-8 text-center">
          <p className="text-sm font-semibold text-foreground">{t("createWorkspaceFirst")}</p>
          <p className="mt-1 text-sm text-muted-foreground">
            {t("createWorkspaceFirstDescription")}
          </p>
          <button
            type="button"
            onClick={() => setIsCreateWorkspaceOpen(true)}
            className="mt-4 inline-flex items-center rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            {t("createWorkspace")}
          </button>
        </div>
      )}

      {/* Projects section */}
      {workspaceIsValid && (
        <ResearchProjectsSection
          projects={projects.data}
          projectTitle={projectTitle}
          onProjectTitleChange={setProjectTitle}
          createPending={createProject.isPending}
          onCreateProject={() => run(() => createProject.mutateAsync())}
          onSelectProject={(id) => updateContext(id)}
          archiveProject={(id) => archiveProject.mutate(id)}
          completeProject={(id) => completeProject.mutate(id)}
          deleteProject={(id) => deleteProject.mutate(id)}
        />
      )}

      {/* Documents and Reports section */}
      {projectIsValid && (
        <ResearchDocumentsSection
          documents={documents.data}
          documentTitle={documentTitle}
          onDocumentTitleChange={setDocumentTitle}
          createDocumentPending={createDocument.isPending}
          onCreateDocument={() => run(() => createDocument.mutateAsync())}
          onSelectDocument={selectDocument}
          importPdfPending={importPdf.isPending}
          onImportPdf={() => run(() => importPdf.mutateAsync())}
          webPageUrl={webPageUrl}
          onWebPageUrlChange={setWebPageUrl}
          importWebPagePending={importWebPage.isPending}
          onImportWebPage={() => run(() => importWebPage.mutateAsync())}
          reports={reports.data}
          reportTitle={reportTitle}
          onReportTitleChange={setReportTitle}
          reportContent={reportContent}
          onReportContentChange={setReportContent}
          reportType={reportType}
          onReportTypeChange={setReportType}
          createReportPending={createReport.isPending}
          onCreateReport={() => run(() => createReport.mutateAsync())}
          highlightedReportId={requestedReportId || null}
          deleteDocument={(id) => deleteDocument.mutate(id)}
          deleteReport={(id) => deleteReport.mutate(id)}
        />
      )}

      {/* Notes, Sources, and Search section */}
      {documentId && (
        <ResearchNotesSection
          notes={notes.data}
          sources={sources.data}
          noteContent={noteContent}
          onNoteContentChange={setNoteContent}
          createNotePending={createNote.isPending}
          onCreateNote={() => run(() => createNote.mutateAsync())}
          sourceUrl={sourceUrl}
          onSourceUrlChange={setSourceUrl}
          sourceTitle={sourceTitle}
          onSourceTitleChange={setSourceTitle}
          createSourcePending={createSource.isPending}
          onCreateSource={() => run(() => createSource.mutateAsync())}
          searchQuery={searchQuery}
          onSearchQueryChange={setSearchQuery}
          searchMode={searchMode}
          onSearchModeChange={setSearchMode}
          searchResults={
            searchMode === "semantic" ? semanticDocumentSearch.data : documentSearch.data
          }
          searchPending={documentSearch.isPending || semanticDocumentSearch.isPending}
          onSearch={() =>
            run(() =>
              searchMode === "semantic"
                ? semanticDocumentSearch.mutateAsync()
                : documentSearch.mutateAsync(),
            )
          }
        />
      )}

      <CreateWorkspaceDialog
        isOpen={isCreateWorkspaceOpen}
        onClose={() => setIsCreateWorkspaceOpen(false)}
        onSuccess={(workspace) => setActiveWorkspace(workspace.id)}
      />
    </div>
  );
}
