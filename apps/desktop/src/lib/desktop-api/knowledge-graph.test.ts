import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  createKnowledgeEntity,
  createKnowledgeRelationship,
  linkThesisKnowledgeEntity,
  listKnowledgeEntities,
  listKnowledgeRelationships,
  listThesisKnowledgeLinks,
} from "./knowledge-graph";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);

describe("knowledge graph API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("creates a typed workspace entity with camelCase schema", async () => {
    const validEntity = {
      id: "entity-1",
      workspaceId: "workspace-1",
      entityType: "company",
      name: "NVIDIA",
      description: "Accelerated computing company",
      createdAt: "2026-08-21T10:00:00Z",
      updatedAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validEntity);
    const result = await createKnowledgeEntity(
      "workspace-1",
      "company",
      "NVIDIA",
      "Accelerated computing company"
    );
    expect(result).toEqual(validEntity);
    expect(mockInvoke).toHaveBeenCalledWith("create_knowledge_entity", {
      workspaceId: "workspace-1",
      entityType: "company",
      name: "NVIDIA",
      description: "Accelerated computing company",
    });
  });

  it("rejects malformed entity in list", async () => {
    mockInvoke.mockResolvedValueOnce([
      {
        id: "entity-1",
        workspace_id: "workspace-1", // snake_case is invalid
      },
    ]);
    await expect(listKnowledgeEntities("workspace-1")).rejects.toThrow();
  });

  it("creates directed relationships and thesis links", async () => {
    const validRel = {
      id: "relationship-1",
      sourceEntityId: "entity-1",
      targetEntityId: "entity-2",
      relationshipType: "enables",
      createdAt: "2026-08-21T10:00:00Z",
    };
    const validLink = {
      thesisId: "thesis-1",
      entityId: "entity-1",
      createdAt: "2026-08-21T10:00:00Z",
    };
    mockInvoke.mockResolvedValueOnce(validRel).mockResolvedValueOnce(validLink);
    const relResult = await createKnowledgeRelationship("entity-1", "entity-2", "enables");
    const linkResult = await linkThesisKnowledgeEntity("thesis-1", "entity-1");
    expect(relResult).toEqual(validRel);
    expect(linkResult).toEqual(validLink);
    expect(mockInvoke).toHaveBeenNthCalledWith(1, "create_knowledge_relationship", {
      sourceEntityId: "entity-1",
      targetEntityId: "entity-2",
      relationshipType: "enables",
    });
    expect(mockInvoke).toHaveBeenNthCalledWith(2, "link_thesis_knowledge_entity", {
      thesisId: "thesis-1",
      entityId: "entity-1",
    });
  });

  it("lists relationships and thesis links", async () => {
    mockInvoke.mockResolvedValueOnce([]).mockResolvedValueOnce([]);
    await expect(listKnowledgeRelationships("workspace-1")).resolves.toEqual([]);
    await expect(listThesisKnowledgeLinks("thesis-1")).resolves.toEqual([]);
  });
});

