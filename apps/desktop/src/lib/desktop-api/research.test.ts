import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { createResearchNote, createResearchReport, createResearchSource, importResearchPdf, listResearchNotes, listResearchSources, searchResearchDocument } from "./research";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked(invoke);

describe("research desktop API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("creates a source with the document identifier", async () => {
    mockInvoke.mockResolvedValueOnce({ id: "source-1" });
    await createResearchSource("document-1", "https://example.com", "Example");
    expect(mockInvoke).toHaveBeenCalledWith("create_research_source", { documentId: "document-1", url: "https://example.com", title: "Example" });
  });

  it("lists sources and notes for a document", async () => {
    mockInvoke.mockResolvedValueOnce([]).mockResolvedValueOnce([]);
    await expect(listResearchSources("document-1")).resolves.toEqual([]);
    await expect(listResearchNotes("document-1")).resolves.toEqual([]);
    expect(mockInvoke).toHaveBeenNthCalledWith(1, "list_research_sources", { documentId: "document-1" });
    expect(mockInvoke).toHaveBeenNthCalledWith(2, "list_research_notes", { documentId: "document-1" });
  });

  it("creates a note with its content", async () => {
    mockInvoke.mockResolvedValueOnce({ id: "note-1" });
    await createResearchNote("document-1", "Evidence to review");
    expect(mockInvoke).toHaveBeenCalledWith("create_research_note", { documentId: "document-1", content: "Evidence to review" });
  });

  it("creates a report with its explicit type", async () => {
    mockInvoke.mockResolvedValueOnce({ id: "report-1" });
    await createResearchReport("project-1", "Quarterly review", "Findings", "summary");
    expect(mockInvoke).toHaveBeenCalledWith("create_research_report", {
      projectId: "project-1", title: "Quarterly review", content: "Findings", reportType: "summary",
    });
  });

  it("searches a document with the supplied query", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await expect(searchResearchDocument("document-1", "revenue growth")).resolves.toEqual([]);
    expect(mockInvoke).toHaveBeenCalledWith("search_research_document", { id: "document-1", query: "revenue growth" });
  });

  it("starts a native PDF import for a project", async () => {
    mockInvoke.mockResolvedValueOnce(null);
    await expect(importResearchPdf("project-1")).resolves.toBeNull();
    expect(mockInvoke).toHaveBeenCalledWith("import_research_pdf", { projectId: "project-1" });
  });
});
