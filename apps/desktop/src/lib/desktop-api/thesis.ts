// Thesis desktop API.

import { invoke } from "@tauri-apps/api/core";

export type ThesisStatus = "draft" | "active" | "validating" | "validated" | "closed";
export type EvidenceDirection = "supporting" | "contradicting";

export interface InvestmentThesis {
  id: string;
  workspace_id: string;
  title: string;
  thesis: string;
  confidence: number;
  status: ThesisStatus;
  validation_date: string | null;
  outcome: string | null;
  created_at: string;
  updated_at: string;
}

export interface ThesisEvidence {
  id: string;
  thesis_id: string;
  direction: EvidenceDirection;
  evidence: string;
  source_id: string | null;
  created_at: string;
}

export interface ThesisConfidenceSnapshot {
  id: string;
  thesis_id: string;
  confidence: number;
  recorded_at: string;
}

export interface CreateThesisParams {
  workspaceId: string;
  title: string;
  thesis: string;
  confidence?: number;
}

export async function createThesis(params: CreateThesisParams): Promise<InvestmentThesis> {
  return invoke("create_thesis", { ...params });
}

export async function getThesis(id: string): Promise<InvestmentThesis | null> {
  return invoke("get_thesis", { id });
}

export async function listTheses(workspaceId: string): Promise<InvestmentThesis[]> {
  return invoke("list_theses", { workspaceId });
}

export async function activateThesis(id: string): Promise<InvestmentThesis> {
  return invoke("activate_thesis", { id });
}

export async function startThesisValidation(id: string): Promise<InvestmentThesis> {
  return invoke("start_thesis_validation", { id });
}

export async function completeThesisValidation(
  id: string,
  outcome: string,
  validated: boolean
): Promise<InvestmentThesis> {
  return invoke("complete_thesis_validation", { id, outcome, validated });
}

export async function updateThesisConfidence(
  thesisId: string,
  confidence: number
): Promise<InvestmentThesis> {
  return invoke("update_thesis_confidence", { thesisId, confidence });
}

export async function closeThesis(id: string): Promise<InvestmentThesis> {
  return invoke("close_thesis", { id });
}

export async function deleteThesis(id: string): Promise<void> {
  return invoke("delete_thesis", { id });
}

export async function addThesisEvidence(
  thesisId: string,
  direction: EvidenceDirection,
  evidence: string,
  sourceId?: string
): Promise<ThesisEvidence> {
  return invoke("add_thesis_evidence", {
    thesisId,
    direction,
    evidence,
    sourceId: sourceId || null,
  });
}

export async function listThesisEvidence(thesisId: string): Promise<ThesisEvidence[]> {
  return invoke("list_thesis_evidence", { thesisId });
}

export async function deleteThesisEvidence(id: string): Promise<void> {
  return invoke("delete_thesis_evidence", { id });
}

export async function listThesisConfidenceHistory(
  thesisId: string
): Promise<ThesisConfidenceSnapshot[]> {
  return invoke("list_thesis_confidence_history", { thesisId });
}
