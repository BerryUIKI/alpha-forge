import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  activateThesis,
  addThesisEvidence,
  completeThesisValidation,
  createThesis,
  listTheses,
  listThesisConfidenceHistory,
  updateThesisConfidence,
} from "./thesis";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked(invoke);

describe("thesis API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("creates a thesis with the complete input", async () => {
    const thesis = { id: "thesis-1", title: "Demand remains durable" };
    mockInvoke.mockResolvedValueOnce(thesis);

    await expect(createThesis({ workspaceId: "workspace-1", title: thesis.title, thesis: "Demand exceeds supply.", confidence: 70 })).resolves.toEqual(thesis);
    expect(mockInvoke).toHaveBeenCalledWith("create_thesis", {
      workspaceId: "workspace-1", title: thesis.title, thesis: "Demand exceeds supply.", confidence: 70,
    });
  });

  it("lists theses for the requested workspace", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await expect(listTheses("workspace-1")).resolves.toEqual([]);
    expect(mockInvoke).toHaveBeenCalledWith("list_theses", { workspaceId: "workspace-1" });
  });

  it("updates confidence and starts lifecycle actions", async () => {
    mockInvoke.mockResolvedValue({ id: "thesis-1" });
    await updateThesisConfidence("thesis-1", 80);
    await activateThesis("thesis-1");
    await completeThesisValidation("thesis-1", "Revenue growth confirmed", true);
    expect(mockInvoke).toHaveBeenNthCalledWith(1, "update_thesis_confidence", { thesisId: "thesis-1", confidence: 80 });
    expect(mockInvoke).toHaveBeenNthCalledWith(2, "activate_thesis", { id: "thesis-1" });
    expect(mockInvoke).toHaveBeenNthCalledWith(3, "complete_thesis_validation", { id: "thesis-1", outcome: "Revenue growth confirmed", validated: true });
  });

  it("adds evidence with an optional source", async () => {
    mockInvoke.mockResolvedValueOnce({ id: "evidence-1" });
    await addThesisEvidence("thesis-1", "contradicting", "Margins are contracting", "source-1");
    expect(mockInvoke).toHaveBeenCalledWith("add_thesis_evidence", {
      thesisId: "thesis-1", direction: "contradicting", evidence: "Margins are contracting", sourceId: "source-1",
    });
  });

  it("lists confidence history for a thesis", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await expect(listThesisConfidenceHistory("thesis-1")).resolves.toEqual([]);
    expect(mockInvoke).toHaveBeenCalledWith("list_thesis_confidence_history", {
      thesisId: "thesis-1",
    });
  });
});
