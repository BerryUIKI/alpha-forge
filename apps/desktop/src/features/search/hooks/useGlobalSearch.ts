/**
 * Global Search Hook
 *
 * Aggregates workspace entities (research projects/documents/reports, theses,
 * knowledge entities, artifacts) and filters them client-side against a query.
 * Reuses the same TanStack Query keys as the feature pages so the command
 * palette piggybacks on already-cached lists.
 *
 * Design: docs/GLOBAL_SEARCH_AGENT_CHAT.md
 *
 * @module features/search/hooks/useGlobalSearch
 */

import { useMemo } from "react";
import { useQuery, useQueries } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace.context";

export type SearchSectionId =
  "projects" | "documents" | "reports" | "theses" | "knowledge" | "artifacts";

/** A single searchable entity with its navigation target. */
export interface SearchEntry {
  section: SearchSectionId;
  id: string;
  title: string;
  subtitle: string | null;
  to: string;
}

/** A section of grouped search results. */
export interface SearchSection {
  id: SearchSectionId;
  entries: SearchEntry[];
}

function textOf(value: unknown): string {
  if (value == null) return "";
  return typeof value === "string" ? value : JSON.stringify(value);
}

/**
 * Maximum number of results rendered per section. Retrieval stays bounded on
 * the display side; a future Rust-side search command can add pagination.
 */
export const MAX_RESULTS_PER_SECTION = 20;

/**
 * Aggregates and filters every searchable entity for the active workspace.
 *
 * `enabled` gates all search-only queries: a closed palette performs no
 * workspace-wide loading.
 *
 * The active workspace comes from the global active-workspace context
 * (ADR-0008); search-result links keep the `?workspace=` parameter as a
 * deep-link entry point. Passing `query` is cheap: filtering happens on the
 * in-memory query cache, not the backend.
 */
export function useGlobalSearch(query: string, enabled = true) {
  const workspaceId = useActiveWorkspaceId();
  const active = enabled && Boolean(workspaceId);

  const projects = useQuery({
    queryKey: ["research", "projects", workspaceId],
    queryFn: () => desktopApi.research.listResearchProjects(workspaceId),
    enabled: active,
  });

  const projectIds = (projects.data ?? []).map((project) => project.id);

  const documents = useQueries({
    queries: projectIds.map((projectId) => ({
      queryKey: ["research", "documents", projectId],
      queryFn: () => desktopApi.research.listResearchDocuments(projectId),
      enabled: active,
    })),
  });

  const reports = useQueries({
    queries: projectIds.map((projectId) => ({
      queryKey: ["research", "reports", projectId],
      queryFn: () => desktopApi.research.listResearchReports(projectId),
      enabled: active,
    })),
  });

  const theses = useQuery({
    queryKey: ["theses", "list", workspaceId],
    queryFn: () => desktopApi.thesis.listTheses(workspaceId),
    enabled: active,
  });

  const knowledge = useQuery({
    queryKey: ["knowledge-graph", "entities", workspaceId],
    queryFn: () => desktopApi.knowledgeGraph.listKnowledgeEntities(workspaceId),
    enabled: active,
  });

  const artifacts = useQuery({
    queryKey: ["artifacts", "workspace", workspaceId],
    queryFn: () => desktopApi.artifacts.listArtifacts(workspaceId),
    enabled: active,
  });

  const searchResult = useMemo<{
    sections: SearchSection[];
    totalMatches: number;
  }>(() => {
    const needle = query.trim().toLowerCase();
    if (!needle || !workspaceId) return { sections: [], totalMatches: 0 };

    const match = (...fields: Array<string | null | undefined>) =>
      fields.some((field) => field != null && field.toLowerCase().includes(needle));

    const projectEntries: SearchEntry[] = (projects.data ?? [])
      .filter((project) => match(project.title, project.description))
      .map((project) => ({
        section: "projects",
        id: project.id,
        title: project.title,
        subtitle: project.description,
        to: `/research?workspace=${workspaceId}&project=${project.id}`,
      }));

    const documentEntries: SearchEntry[] = documents
      .flatMap((result) => result.data ?? [])
      .filter((document) => match(document.title, document.content))
      .map((document) => ({
        section: "documents",
        id: document.id,
        title: document.title,
        subtitle: document.projectId,
        to: `/research?workspace=${workspaceId}&project=${document.projectId}&document=${document.id}`,
      }));

    const reportEntries: SearchEntry[] = reports
      .flatMap((result) => result.data ?? [])
      .filter((report) => match(report.title, report.content))
      .map((report) => ({
        section: "reports",
        id: report.id,
        title: report.title,
        subtitle: report.projectId,
        to: `/research?workspace=${workspaceId}&project=${report.projectId}&report=${report.id}`,
      }));

    const thesisEntries: SearchEntry[] = (theses.data ?? [])
      .filter((thesis) => match(thesis.title, thesis.thesis))
      .map((thesis) => ({
        section: "theses",
        id: thesis.id,
        title: thesis.title,
        subtitle: thesis.status,
        to: `/theses?thesis=${thesis.id}`,
      }));

    const knowledgeEntries: SearchEntry[] = (knowledge.data ?? [])
      .filter((entity) => match(entity.name, entity.description))
      .map((entity) => ({
        section: "knowledge",
        id: entity.id,
        title: entity.name,
        subtitle: entity.entityType,
        to: `/knowledge?entity=${entity.id}`,
      }));

    const artifactEntries: SearchEntry[] = (artifacts.data ?? [])
      .filter((artifact) =>
        match(
          artifact.artifactType,
          artifact.error,
          textOf(artifact.input),
          textOf(artifact.output),
        ),
      )
      .map((artifact) => ({
        section: "artifacts",
        id: artifact.id,
        title: artifact.artifactType,
        subtitle: artifact.status,
        to: `/artifact/${artifact.id}/${encodeURIComponent(artifact.artifactType)}`,
      }));

    const allSections: SearchSection[] = [
      { id: "projects", entries: projectEntries },
      { id: "documents", entries: documentEntries },
      { id: "reports", entries: reportEntries },
      { id: "theses", entries: thesisEntries },
      { id: "knowledge", entries: knowledgeEntries },
      { id: "artifacts", entries: artifactEntries },
    ];

    const totalMatches = allSections.reduce((sum, section) => sum + section.entries.length, 0);
    const sections = allSections
      .map((section) => ({
        ...section,
        entries: section.entries.slice(0, MAX_RESULTS_PER_SECTION),
      }))
      .filter((section) => section.entries.length > 0);

    return { sections, totalMatches };
  }, [
    query,
    workspaceId,
    projects.data,
    documents,
    reports,
    theses.data,
    knowledge.data,
    artifacts.data,
  ]);

  const { sections, totalMatches } = searchResult;
  const total = sections.reduce((sum, section) => sum + section.entries.length, 0);

  const isLoading =
    projects.isLoading ||
    theses.isLoading ||
    knowledge.isLoading ||
    artifacts.isLoading ||
    documents.some((result) => result.isLoading) ||
    reports.some((result) => result.isLoading);

  const failedSources: SearchSectionId[] = [];
  if (projects.isError) failedSources.push("projects");
  if (documents.some((result) => result.isError)) failedSources.push("documents");
  if (reports.some((result) => result.isError)) failedSources.push("reports");
  if (theses.isError) failedSources.push("theses");
  if (knowledge.isError) failedSources.push("knowledge");
  if (artifacts.isError) failedSources.push("artifacts");

  const refetch = async () => {
    await Promise.all([
      projects.refetch(),
      theses.refetch(),
      knowledge.refetch(),
      artifacts.refetch(),
      ...documents.map((result) => result.refetch()),
      ...reports.map((result) => result.refetch()),
    ]);
  };

  return {
    workspaceId,
    isLoading,
    sections,
    total,
    totalMatches,
    truncated: totalMatches > total,
    hasError: failedSources.length > 0,
    failedSources,
    refetch,
  };
}
