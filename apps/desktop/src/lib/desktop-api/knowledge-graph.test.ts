import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { createKnowledgeEntity, createKnowledgeRelationship, linkThesisKnowledgeEntity } from "./knowledge-graph";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);

describe("knowledge graph API", () => {
  beforeEach(() => mockInvoke.mockReset());
  it("creates a typed workspace entity", async () => {
    mockInvoke.mockResolvedValueOnce({ id: "entity-1" });
    await createKnowledgeEntity("workspace-1", "company", "NVIDIA", "Accelerated computing company");
    expect(mockInvoke).toHaveBeenCalledWith("create_knowledge_entity", { workspaceId: "workspace-1", entityType: "company", name: "NVIDIA", description: "Accelerated computing company" });
  });
  it("creates directed relationships and thesis links", async () => {
    mockInvoke.mockResolvedValue({ id: "relationship-1" });
    await createKnowledgeRelationship("entity-1", "entity-2", "enables");
    await linkThesisKnowledgeEntity("thesis-1", "entity-1");
    expect(mockInvoke).toHaveBeenNthCalledWith(1, "create_knowledge_relationship", { sourceEntityId: "entity-1", targetEntityId: "entity-2", relationshipType: "enables" });
    expect(mockInvoke).toHaveBeenNthCalledWith(2, "link_thesis_knowledge_entity", { thesisId: "thesis-1", entityId: "entity-1" });
  });
});
