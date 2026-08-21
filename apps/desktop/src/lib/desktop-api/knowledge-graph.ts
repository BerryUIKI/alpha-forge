import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

export const KnowledgeEntityTypeSchema = z.enum([
  "company",
  "industry",
  "technology",
  "macro_theme",
]);
export type KnowledgeEntityType = z.infer<typeof KnowledgeEntityTypeSchema>;

export const KnowledgeEntitySchema = z
  .object({
    id: z.string().min(1),
    workspaceId: z.string().min(1),
    entityType: KnowledgeEntityTypeSchema,
    name: z.string().min(1),
    description: z.string().nullable(),
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();
export type KnowledgeEntity = z.infer<typeof KnowledgeEntitySchema>;

export const KnowledgeRelationshipSchema = z
  .object({
    id: z.string().min(1),
    sourceEntityId: z.string().min(1),
    targetEntityId: z.string().min(1),
    relationshipType: z.string().min(1),
    createdAt: z.string().min(1),
  })
  .strict();
export type KnowledgeRelationship = z.infer<typeof KnowledgeRelationshipSchema>;

export const ThesisEntityLinkSchema = z
  .object({
    thesisId: z.string().min(1),
    entityId: z.string().min(1),
    createdAt: z.string().min(1),
  })
  .strict();
export type ThesisEntityLink = z.infer<typeof ThesisEntityLinkSchema>;

export async function createKnowledgeEntity(
  workspaceId: string,
  entityType: KnowledgeEntityType,
  name: string,
  description?: string
): Promise<KnowledgeEntity> {
  const response: unknown = await invoke("create_knowledge_entity", {
    workspaceId,
    entityType,
    name,
    description: description || null,
  });
  return KnowledgeEntitySchema.parse(response);
}

export async function listKnowledgeEntities(
  workspaceId: string
): Promise<KnowledgeEntity[]> {
  const response: unknown = await invoke("list_knowledge_entities", { workspaceId });
  return z.array(KnowledgeEntitySchema).parse(response);
}

export async function createKnowledgeRelationship(
  sourceEntityId: string,
  targetEntityId: string,
  relationshipType: string
): Promise<KnowledgeRelationship> {
  const response: unknown = await invoke("create_knowledge_relationship", {
    sourceEntityId,
    targetEntityId,
    relationshipType,
  });
  return KnowledgeRelationshipSchema.parse(response);
}

export async function listKnowledgeRelationships(
  workspaceId: string
): Promise<KnowledgeRelationship[]> {
  const response: unknown = await invoke("list_knowledge_relationships", {
    workspaceId,
  });
  return z.array(KnowledgeRelationshipSchema).parse(response);
}

export async function linkThesisKnowledgeEntity(
  thesisId: string,
  entityId: string
): Promise<ThesisEntityLink> {
  const response: unknown = await invoke("link_thesis_knowledge_entity", {
    thesisId,
    entityId,
  });
  return ThesisEntityLinkSchema.parse(response);
}

export async function listThesisKnowledgeLinks(
  thesisId: string
): Promise<ThesisEntityLink[]> {
  const response: unknown = await invoke("list_thesis_knowledge_links", {
    thesisId,
  });
  return z.array(ThesisEntityLinkSchema).parse(response);
}
