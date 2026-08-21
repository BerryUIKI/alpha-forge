import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  createResearchDocument,
  createResearchNote,
  createResearchProject,
  createResearchReport,
  createResearchSource,
  getResearchProject,
  importResearchPdf,
  importResearchWebPage,
  listResearchDocuments,
  listResearchNotes,
  listResearchProjects,
  listResearchReports,
  listResearchSources,
  searchResearchDocument,
  semanticSearchResearchDocument,
} from "./research";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked(invoke);

describe("research desktop API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("creates a research project with camelCase properties", async () => {
    const validProject = {
      id: "project-1",
      workspaceId: "workspace-1",
      title: "AI Hardware",
      description: "Research into chip makers",
      status: "active",
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validProject);
    const result = await createResearchProject("workspace-1", "AI Hardware", "Research into chip makers");
    expect(result).toEqual(validProject);
    expect(mockInvoke).toHaveBeenCalledWith("create_research_project", {
      workspaceId: "workspace-1",
      title: "AI Hardware",
      description: "Research into chip makers",
    });
  });

  it("rejects malformed project response", async () => {
    mockInvoke.mockResolvedValueOnce({
      id: "project-1",
      workspace_id: "workspace-1", // snake_case is invalid at the Zod layer
    });
    await expect(getResearchProject("project-1")).rejects.toThrow();
  });

  it("lists research projects", async () => {
    const validProject = {
      id: "project-1",
      workspaceId: "workspace-1",
      title: "AI Hardware",
      description: null,
      status: "active",
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce([validProject]);
    const result = await listResearchProjects("workspace-1");
    expect(result).toEqual([validProject]);
  });

  it("creates a document with note defaults", async () => {
    const validDoc = {
      id: "doc-1",
      projectId: "project-1",
      documentType: "note",
      title: "Notes on GPU clusters",
      content: "Cluster architecture overview",
      sourceUrl: null,
      filePath: null,
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validDoc);
    const result = await createResearchDocument("project-1", "Notes on GPU clusters", "Cluster architecture overview");
    expect(result).toEqual(validDoc);
    expect(mockInvoke).toHaveBeenCalledWith("create_research_document", {
      projectId: "project-1",
      documentType: "note",
      title: "Notes on GPU clusters",
      content: "Cluster architecture overview",
      sourceUrl: null,
      filePath: null,
    });
  });

  it("lists documents for a project", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await expect(listResearchDocuments("project-1")).resolves.toEqual([]);
  });

  it("creates a source with the document identifier", async () => {
    const validSource = {
      id: "source-1",
      documentId: "document-1",
      url: "https://example.com",
      title: "Example",
      retrievedAt: null,
      createdAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validSource);
    const result = await createResearchSource("document-1", "https://example.com", "Example");
    expect(result).toEqual(validSource);
    expect(mockInvoke).toHaveBeenCalledWith("create_research_source", {
      documentId: "document-1",
      url: "https://example.com",
      title: "Example",
    });
  });

  it("lists sources and notes for a document", async () => {
    mockInvoke.mockResolvedValueOnce([]).mockResolvedValueOnce([]);
    await expect(listResearchSources("document-1")).resolves.toEqual([]);
    await expect(listResearchNotes("document-1")).resolves.toEqual([]);
    expect(mockInvoke).toHaveBeenNthCalledWith(1, "list_research_sources", { documentId: "document-1" });
    expect(mockInvoke).toHaveBeenNthCalledWith(2, "list_research_notes", { documentId: "document-1" });
  });

  it("creates a note with its content", async () => {
    const validNote = {
      id: "note-1",
      documentId: "document-1",
      content: "Evidence to review",
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validNote);
    const result = await createResearchNote("document-1", "Evidence to review");
    expect(result).toEqual(validNote);
    expect(mockInvoke).toHaveBeenCalledWith("create_research_note", {
      documentId: "document-1",
      content: "Evidence to review",
    });
  });

  it("creates a report with its explicit type", async () => {
    const validReport = {
      id: "report-1",
      projectId: "project-1",
      title: "Quarterly review",
      content: "Findings",
      reportType: "summary",
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validReport);
    const result = await createResearchReport("project-1", "Quarterly review", "Findings", "summary");
    expect(result).toEqual(validReport);
    expect(mockInvoke).toHaveBeenCalledWith("create_research_report", {
      projectId: "project-1",
      title: "Quarterly review",
      content: "Findings",
      reportType: "summary",
    });
  });

  it("lists reports for a project", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await expect(listResearchReports("project-1")).resolves.toEqual([]);
  });

  it("searches a document with the supplied query", async () => {
    const match = { ordinal: 1, content: "revenue growth of 25%", score: 10 };
    mockInvoke.mockResolvedValueOnce([match]);
    const result = await searchResearchDocument("document-1", "revenue growth");
    expect(result).toEqual([match]);
    expect(mockInvoke).toHaveBeenCalledWith("search_research_document", { id: "document-1", query: "revenue growth" });
  });

  it("starts a native PDF import for a project", async () => {
    mockInvoke.mockResolvedValueOnce(null);
    await expect(importResearchPdf("project-1")).resolves.toBeNull();
    expect(mockInvoke).toHaveBeenCalledWith("import_research_pdf", { projectId: "project-1" });
  });

  it("imports a web page through Rust", async () => {
    const validDoc = {
      id: "document-1",
      projectId: "project-1",
      documentType: "web_page",
      title: "Research Article",
      content: "Web content",
      sourceUrl: "https://example.com/research",
      filePath: null,
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validDoc);
    const result = await importResearchWebPage("project-1", "https://example.com/research");
    expect(result).toEqual(validDoc);
    expect(mockInvoke).toHaveBeenCalledWith("import_research_web_page", { projectId: "project-1", url: "https://example.com/research" });
  });

  it("uses the semantic search command when requested", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await semanticSearchResearchDocument("document-1", "revenue growth");
    expect(mockInvoke).toHaveBeenCalledWith("semantic_search_research_document", { id: "document-1", query: "revenue growth" });
  });
});

