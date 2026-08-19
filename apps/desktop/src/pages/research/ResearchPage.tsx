/**
 * Research Page
 *
 * Main page for research project management.
 * URL params are the source of truth for workspace/project/document context.
 *
 * @module pages/research/ResearchPage
 */

import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import { processAppError } from "@/lib/errors";
import { ResearchProjectsSection } from "./components/ResearchProjectsSection";
import { ResearchDocumentsSection } from "./components/ResearchDocumentsSection";
import { ResearchNotesSection } from "./components/ResearchNotesSection";

export function ResearchPage() {
  const { t } = useLocale();
  const client = useQueryClient();
  const [searchParams, setSearchParams] = useSearchParams();
  const workspaceId = searchParams.get("workspace") || "";
  const projectId = searchParams.get("project") || "";

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
  const [searchMode, setSearchMode] = useState<"lexical" | "semantic">(
    "lexical"
  );
  const [error, setError] = useState("");

  // Queries
  const workspaces = useQuery({
    queryKey: ["workspaces"],
    queryFn: desktopApi.workspace.listWorkspaces,
  });
  const workspaceIsValid =
    workspaces.isSuccess &&
    Boolean(workspaceId) &&
    (workspaces.data ?? []).some((item) => item.id === workspaceId);

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

  // The URL is the source of truth for the selected research context. Clean
  // stale IDs only after the corresponding query has succeeded, so loading
  // and error states do not erase a deep link that may still become valid.
  useEffect(() => {
    if (!workspaces.isSuccess || workspaces.isFetching) return;

    if (workspaceIsValid) return;

    const next = new URLSearchParams(searchParams);
    if (!next.has("workspace") && !next.has("project")) return;
    next.delete("workspace");
    next.delete("project");
    if (next.toString() !== searchParams.toString()) {
      setSearchParams(next, { replace: true });
    }
  }, [
    searchParams,
    setSearchParams,
    workspaceIsValid,
    workspaces.isFetching,
    workspaces.isSuccess,
  ]);

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

  const updateContext = (workspace: string, project?: string, options?: { replace?: boolean }) => {
    const next = new URLSearchParams(searchParams);
    if (workspace) next.set("workspace", workspace);
    else next.delete("workspace");
    if (project) next.set("project", project);
    else next.delete("project");
    if (next.toString() !== searchParams.toString()) {
      setSearchParams(next, options);
    }
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
    onSuccess: (_result, deletedProjectId) => {
      if (deletedProjectId === projectId) {
        updateContext(workspaceId, undefined, { replace: true });
      }
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
        <p className="text-sm text-muted-foreground">
          {t("researchDescription")}
        </p>
      </div>

      {error && <p className="text-sm text-destructive">{error}</p>}

      {researchOperationError && (
        <p className="text-sm text-destructive" role="alert">
          {t("unexpectedError")}
        </p>
      )}

      {/* Workspace selector */}
      <label className="block max-w-md text-sm font-medium">
        {t("workspace")}
        <select
          className="mt-1 w-full rounded border bg-background p-2"
          value={workspaceId}
          onChange={(event) => {
            updateContext(event.target.value);
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
      {workspaceIsValid && (
        <ResearchProjectsSection
          projects={projects.data}
          projectTitle={projectTitle}
          onProjectTitleChange={setProjectTitle}
          createPending={createProject.isPending}
          onCreateProject={() => run(() => createProject.mutateAsync())}
          onSelectProject={(id) => updateContext(workspaceId, id)}
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
          onSelectDocument={setDocumentId}
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
            searchMode === "semantic"
              ? semanticDocumentSearch.data
              : documentSearch.data
          }
          searchPending={
            documentSearch.isPending || semanticDocumentSearch.isPending
          }
          onSearch={() =>
            run(() =>
              searchMode === "semantic"
                ? semanticDocumentSearch.mutateAsync()
                : documentSearch.mutateAsync()
            )
          }
        />
      )}
    </div>
  );
}
