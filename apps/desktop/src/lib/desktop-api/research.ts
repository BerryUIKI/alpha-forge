import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

export const ProjectStatusSchema = z.enum(["active", "archived", "completed"]);
export type ProjectStatus = z.infer<typeof ProjectStatusSchema>;

export const ResearchProjectSchema = z
  .object({
    id: z.string().min(1),
    workspaceId: z.string().min(1),
    title: z.string().min(1),
    description: z.string().nullable(),
    status: ProjectStatusSchema,
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();
export type ResearchProject = z.infer<typeof ResearchProjectSchema>;

export const DocumentTypeSchema = z.enum(["pdf", "web_page", "note", "report"]);
export type DocumentType = z.infer<typeof DocumentTypeSchema>;

export const ResearchDocumentSchema = z
  .object({
    id: z.string().min(1),
    projectId: z.string().min(1),
    documentType: DocumentTypeSchema,
    title: z.string().min(1),
    content: z.string().nullable(),
    sourceUrl: z.string().nullable(),
    filePath: z.string().nullable(),
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();
export type ResearchDocument = z.infer<typeof ResearchDocumentSchema>;

export const ResearchSourceSchema = z
  .object({
    id: z.string().min(1),
    documentId: z.string().min(1),
    url: z.string().nullable(),
    title: z.string().nullable(),
    retrievedAt: z.string().nullable(),
    createdAt: z.string().min(1),
  })
  .strict();
export type ResearchSource = z.infer<typeof ResearchSourceSchema>;

export const ResearchNoteSchema = z
  .object({
    id: z.string().min(1),
    documentId: z.string().min(1),
    content: z.string(),
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();
export type ResearchNote = z.infer<typeof ResearchNoteSchema>;

export const ReportTypeSchema = z.enum(["analysis", "summary", "thesis", "recommendation"]);
export type ReportType = z.infer<typeof ReportTypeSchema>;

export const ResearchReportSchema = z
  .object({
    id: z.string().min(1),
    projectId: z.string().min(1),
    title: z.string().min(1),
    content: z.string(),
    reportType: ReportTypeSchema,
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();
export type ResearchReport = z.infer<typeof ResearchReportSchema>;

export const ResearchSearchMatchSchema = z
  .object({
    ordinal: z.number().int().nonnegative(),
    content: z.string(),
    score: z.number().int().nonnegative(),
  })
  .strict();
export type ResearchSearchMatch = z.infer<typeof ResearchSearchMatchSchema>;

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

// Project commands
export async function createResearchProject(
  workspaceId: string,
  title: string,
  description?: string
): Promise<ResearchProject> {
  const response: unknown = await invoke("create_research_project", {
    workspaceId,
    title,
    description: description || null,
  });
  return ResearchProjectSchema.parse(response);
}

export async function getResearchProject(id: string): Promise<ResearchProject | null> {
  const response: unknown = await invoke("get_research_project", { id });
  return z.nullable(ResearchProjectSchema).parse(response);
}

export async function listResearchProjects(workspaceId: string): Promise<ResearchProject[]> {
  const response: unknown = await invoke("list_research_projects", { workspaceId });
  return z.array(ResearchProjectSchema).parse(response);
}

export async function archiveResearchProject(id: string): Promise<ResearchProject> {
  const response: unknown = await invoke("archive_research_project", { id });
  return ResearchProjectSchema.parse(response);
}

export async function completeResearchProject(id: string): Promise<ResearchProject> {
  const response: unknown = await invoke("complete_research_project", { id });
  return ResearchProjectSchema.parse(response);
}

export async function deleteResearchProject(id: string): Promise<void> {
  const response: unknown = await invoke("delete_research_project", { id });
  VoidResponseSchema.parse(response);
}

// Document commands
export async function createResearchDocument(
  projectId: string,
  title: string,
  content?: string
): Promise<ResearchDocument> {
  const response: unknown = await invoke("create_research_document", {
    projectId,
    documentType: "note",
    title,
    content: content || null,
    sourceUrl: null,
    filePath: null,
  });
  return ResearchDocumentSchema.parse(response);
}

export async function getResearchDocument(id: string): Promise<ResearchDocument | null> {
  const response: unknown = await invoke("get_research_document", { id });
  return z.nullable(ResearchDocumentSchema).parse(response);
}

export async function listResearchDocuments(projectId: string): Promise<ResearchDocument[]> {
  const response: unknown = await invoke("list_research_documents", { projectId });
  return z.array(ResearchDocumentSchema).parse(response);
}

export async function deleteResearchDocument(id: string): Promise<void> {
  const response: unknown = await invoke("delete_research_document", { id });
  VoidResponseSchema.parse(response);
}

export async function importResearchPdf(projectId: string): Promise<ResearchDocument | null> {
  const response: unknown = await invoke("import_research_pdf", { projectId });
  return z.nullable(ResearchDocumentSchema).parse(response);
}

export async function importResearchWebPage(
  projectId: string,
  url: string
): Promise<ResearchDocument> {
  const response: unknown = await invoke("import_research_web_page", { projectId, url });
  return ResearchDocumentSchema.parse(response);
}

export async function searchResearchDocument(
  id: string,
  query: string
): Promise<ResearchSearchMatch[]> {
  const response: unknown = await invoke("search_research_document", { id, query });
  return z.array(ResearchSearchMatchSchema).parse(response);
}

export async function semanticSearchResearchDocument(
  id: string,
  query: string
): Promise<ResearchSearchMatch[]> {
  const response: unknown = await invoke("semantic_search_research_document", { id, query });
  return z.array(ResearchSearchMatchSchema).parse(response);
}

// Source commands
export async function createResearchSource(
  documentId: string,
  url?: string,
  title?: string
): Promise<ResearchSource> {
  const response: unknown = await invoke("create_research_source", {
    documentId,
    url: url || null,
    title: title || null,
  });
  return ResearchSourceSchema.parse(response);
}

export async function listResearchSources(documentId: string): Promise<ResearchSource[]> {
  const response: unknown = await invoke("list_research_sources", { documentId });
  return z.array(ResearchSourceSchema).parse(response);
}

// Note commands
export async function createResearchNote(
  documentId: string,
  content: string
): Promise<ResearchNote> {
  const response: unknown = await invoke("create_research_note", { documentId, content });
  return ResearchNoteSchema.parse(response);
}

export async function listResearchNotes(documentId: string): Promise<ResearchNote[]> {
  const response: unknown = await invoke("list_research_notes", { documentId });
  return z.array(ResearchNoteSchema).parse(response);
}

export async function deleteResearchNote(id: string): Promise<void> {
  const response: unknown = await invoke("delete_research_note", { id });
  VoidResponseSchema.parse(response);
}

// Report commands
export async function createResearchReport(
  projectId: string,
  title: string,
  content: string,
  reportType: ReportType = "analysis"
): Promise<ResearchReport> {
  const response: unknown = await invoke("create_research_report", {
    projectId,
    title,
    content,
    reportType,
  });
  return ResearchReportSchema.parse(response);
}

export async function getResearchReport(id: string): Promise<ResearchReport | null> {
  const response: unknown = await invoke("get_research_report", { id });
  return z.nullable(ResearchReportSchema).parse(response);
}

export async function listResearchReports(projectId: string): Promise<ResearchReport[]> {
  const response: unknown = await invoke("list_research_reports", { projectId });
  return z.array(ResearchReportSchema).parse(response);
}

export async function deleteResearchReport(id: string): Promise<void> {
  const response: unknown = await invoke("delete_research_report", { id });
  VoidResponseSchema.parse(response);
}
