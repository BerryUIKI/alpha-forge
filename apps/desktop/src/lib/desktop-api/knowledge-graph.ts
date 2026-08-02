import { invoke } from "@tauri-apps/api/core";

export type KnowledgeEntityType = "company" | "industry" | "technology" | "macro_theme";
export interface KnowledgeEntity { id: string; workspace_id: string; entity_type: KnowledgeEntityType; name: string; description: string | null; created_at: string; updated_at: string; }
export interface KnowledgeRelationship { id: string; source_entity_id: string; target_entity_id: string; relationship_type: string; created_at: string; }
export interface ThesisEntityLink { thesis_id: string; entity_id: string; created_at: string; }

export function createKnowledgeEntity(workspaceId: string, entityType: KnowledgeEntityType, name: string, description?: string) { return invoke<KnowledgeEntity>("create_knowledge_entity", { workspaceId, entityType, name, description: description || null }); }
export function listKnowledgeEntities(workspaceId: string) { return invoke<KnowledgeEntity[]>("list_knowledge_entities", { workspaceId }); }
export function createKnowledgeRelationship(sourceEntityId: string, targetEntityId: string, relationshipType: string) { return invoke<KnowledgeRelationship>("create_knowledge_relationship", { sourceEntityId, targetEntityId, relationshipType }); }
export function listKnowledgeRelationships(workspaceId: string) { return invoke<KnowledgeRelationship[]>("list_knowledge_relationships", { workspaceId }); }
export function linkThesisKnowledgeEntity(thesisId: string, entityId: string) { return invoke<ThesisEntityLink>("link_thesis_knowledge_entity", { thesisId, entityId }); }
export function listThesisKnowledgeLinks(thesisId: string) { return invoke<ThesisEntityLink[]>("list_thesis_knowledge_links", { thesisId }); }
