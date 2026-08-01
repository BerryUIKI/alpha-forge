import { invoke } from "@tauri-apps/api/core";

export interface ResearchProject { id: string; workspace_id: string; title: string; description: string | null; status: "active" | "archived" | "completed"; created_at: string; updated_at: string; }
export interface ResearchDocument { id: string; project_id: string; document_type: "pdf" | "web_page" | "note" | "report"; title: string; content: string | null; source_url: string | null; file_path: string | null; created_at: string; updated_at: string; }
export interface ResearchSource { id: string; document_id: string; url: string | null; title: string | null; retrieved_at: string | null; created_at: string; }
export interface ResearchNote { id: string; document_id: string; content: string; created_at: string; updated_at: string; }
export interface ResearchReport { id: string; project_id: string; title: string; content: string; report_type: "analysis" | "summary" | "thesis" | "recommendation"; created_at: string; updated_at: string; }

export function createResearchProject(workspaceId: string, title: string, description?: string): Promise<ResearchProject> { return invoke("create_research_project", { workspaceId, title, description: description || null }); }
export function listResearchProjects(workspaceId: string): Promise<ResearchProject[]> { return invoke("list_research_projects", { workspaceId }); }
export function createResearchDocument(projectId: string, title: string, content?: string): Promise<ResearchDocument> { return invoke("create_research_document", { projectId, documentType: "note", title, content: content || null, sourceUrl: null, filePath: null }); }
export function listResearchDocuments(projectId: string): Promise<ResearchDocument[]> { return invoke("list_research_documents", { projectId }); }
export function createResearchReport(projectId: string, title: string, content: string, reportType: ResearchReport["report_type"] = "analysis"): Promise<ResearchReport> { return invoke("create_research_report", { projectId, title, content, reportType }); }
export function listResearchReports(projectId: string): Promise<ResearchReport[]> { return invoke("list_research_reports", { projectId }); }
export function createResearchSource(documentId: string, url?: string, title?: string): Promise<ResearchSource> { return invoke("create_research_source", { documentId, url: url || null, title: title || null }); }
export function listResearchSources(documentId: string): Promise<ResearchSource[]> { return invoke("list_research_sources", { documentId }); }
export function createResearchNote(documentId: string, content: string): Promise<ResearchNote> { return invoke("create_research_note", { documentId, content }); }
export function listResearchNotes(documentId: string): Promise<ResearchNote[]> { return invoke("list_research_notes", { documentId }); }
export function deleteResearchNote(id: string): Promise<void> { return invoke("delete_research_note", { id }); }
