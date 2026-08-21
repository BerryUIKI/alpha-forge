// Thesis desktop API.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

export const ThesisStatusSchema = z.enum([
  "draft",
  "active",
  "validating",
  "validated",
  "closed",
]);
export type ThesisStatus = z.infer<typeof ThesisStatusSchema>;

export const EvidenceDirectionSchema = z.enum(["supporting", "contradicting"]);
export type EvidenceDirection = z.infer<typeof EvidenceDirectionSchema>;

export const InvestmentThesisSchema = z
  .object({
    id: z.string().min(1),
    workspaceId: z.string().min(1),
    title: z.string().min(1),
    thesis: z.string(),
    confidence: z.number().int().min(0).max(100),
    status: ThesisStatusSchema,
    validationDate: z.string().nullable(),
    outcome: z.string().nullable(),
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();
export type InvestmentThesis = z.infer<typeof InvestmentThesisSchema>;

export const ThesisEvidenceSchema = z
  .object({
    id: z.string().min(1),
    thesisId: z.string().min(1),
    direction: EvidenceDirectionSchema,
    evidence: z.string(),
    sourceId: z.string().nullable(),
    createdAt: z.string().min(1),
  })
  .strict();
export type ThesisEvidence = z.infer<typeof ThesisEvidenceSchema>;

export const ThesisConfidenceSnapshotSchema = z
  .object({
    id: z.string().min(1),
    thesisId: z.string().min(1),
    confidence: z.number().int().min(0).max(100),
    recordedAt: z.string().min(1),
  })
  .strict();
export type ThesisConfidenceSnapshot = z.infer<typeof ThesisConfidenceSnapshotSchema>;

export interface CreateThesisParams {
  workspaceId: string;
  title: string;
  thesis: string;
  confidence?: number;
}

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

export async function createThesis(params: CreateThesisParams): Promise<InvestmentThesis> {
  const response: unknown = await invoke("create_thesis", {
    workspaceId: params.workspaceId,
    title: params.title,
    thesis: params.thesis,
    confidence: params.confidence ?? null,
  });
  return InvestmentThesisSchema.parse(response);
}

export async function getThesis(id: string): Promise<InvestmentThesis | null> {
  const response: unknown = await invoke("get_thesis", { id });
  return z.nullable(InvestmentThesisSchema).parse(response);
}

export async function listTheses(workspaceId: string): Promise<InvestmentThesis[]> {
  const response: unknown = await invoke("list_theses", { workspaceId });
  return z.array(InvestmentThesisSchema).parse(response);
}

export async function activateThesis(id: string): Promise<InvestmentThesis> {
  const response: unknown = await invoke("activate_thesis", { id });
  return InvestmentThesisSchema.parse(response);
}

export async function startThesisValidation(id: string): Promise<InvestmentThesis> {
  const response: unknown = await invoke("start_thesis_validation", { id });
  return InvestmentThesisSchema.parse(response);
}

export async function completeThesisValidation(
  id: string,
  outcome: string,
  validated: boolean
): Promise<InvestmentThesis> {
  const response: unknown = await invoke("complete_thesis_validation", {
    id,
    outcome,
    validated,
  });
  return InvestmentThesisSchema.parse(response);
}

export async function updateThesisConfidence(
  thesisId: string,
  confidence: number
): Promise<InvestmentThesis> {
  const response: unknown = await invoke("update_thesis_confidence", { thesisId, confidence });
  return InvestmentThesisSchema.parse(response);
}

export async function closeThesis(id: string): Promise<InvestmentThesis> {
  const response: unknown = await invoke("close_thesis", { id });
  return InvestmentThesisSchema.parse(response);
}

export async function deleteThesis(id: string): Promise<void> {
  const response: unknown = await invoke("delete_thesis", { id });
  VoidResponseSchema.parse(response);
}

export async function addThesisEvidence(
  thesisId: string,
  direction: EvidenceDirection,
  evidence: string,
  sourceId?: string
): Promise<ThesisEvidence> {
  const response: unknown = await invoke("add_thesis_evidence", {
    thesisId,
    direction,
    evidence,
    sourceId: sourceId || null,
  });
  return ThesisEvidenceSchema.parse(response);
}

export async function listThesisEvidence(thesisId: string): Promise<ThesisEvidence[]> {
  const response: unknown = await invoke("list_thesis_evidence", { thesisId });
  return z.array(ThesisEvidenceSchema).parse(response);
}

export async function deleteThesisEvidence(id: string): Promise<void> {
  const response: unknown = await invoke("delete_thesis_evidence", { id });
  VoidResponseSchema.parse(response);
}

export async function listThesisConfidenceHistory(
  thesisId: string
): Promise<ThesisConfidenceSnapshot[]> {
  const response: unknown = await invoke("list_thesis_confidence_history", { thesisId });
  return z.array(ThesisConfidenceSnapshotSchema).parse(response);
}
