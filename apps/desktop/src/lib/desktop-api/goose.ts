// Goose Agent desktop API.
// Provides Zod-validated types and commands for Goose shadow-mode analysis and human-approved proposals (M10).

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

// ============================================================================
// Schemas
// ============================================================================

export const RiskSeveritySchema = z.enum(["low", "medium", "high", "critical"]);
export type RiskSeverity = z.infer<typeof RiskSeveritySchema>;

export const ClaimSchema = z.object({
  id: z.string(),
  claim: z.string(),
  confidence: z.number().int().min(0).max(100),
  source_ids: z.array(z.string()).default([]),
  contradicting_source_ids: z.array(z.string()).default([]),
});
export type Claim = z.infer<typeof ClaimSchema>;

export const EvidenceRelationSchema = z.enum(["supports", "contradicts", "neutral"]);
export type EvidenceRelation = z.infer<typeof EvidenceRelationSchema>;

export const EvidenceSchema = z.object({
  claim_id: z.string(),
  source_id: z.string(),
  excerpt: z.string(),
  relation: EvidenceRelationSchema.default("supports"),
  confidence: z.number().int().min(0).max(100).optional().nullable(),
});
export type Evidence = z.infer<typeof EvidenceSchema>;

export const ContradictionSchema = z.object({
  description: z.string(),
  claim_ids: z.array(z.string()).default([]),
  source_ids: z.array(z.string()).default([]),
});
export type Contradiction = z.infer<typeof ContradictionSchema>;

export const RiskSchema = z.object({
  id: z.string(),
  risk: z.string(),
  severity: RiskSeveritySchema.default("medium"),
  related_claim_ids: z.array(z.string()).default([]),
  mitigation: z.string().optional().nullable(),
});
export type Risk = z.infer<typeof RiskSchema>;

export const StructuredResponseSchema = z.object({
  summary: z.string(),
  claims: z.array(ClaimSchema).default([]),
  evidence: z.array(EvidenceSchema).default([]),
  contradictions: z.array(ContradictionSchema).default([]),
  risks: z.array(RiskSchema).default([]),
  unknowns: z.array(z.string()).default([]),
  source_ids: z.array(z.string()).default([]),
  confidence: z.number().int().min(0).max(100),
  provider: z.string().optional().nullable(),
  model: z.string().optional().nullable(),
  recipe_version: z.string().optional().nullable(),
});
export type StructuredResponse = z.infer<typeof StructuredResponseSchema>;

export const StartShadowAnalysisInputSchema = z.object({
  workspace_id: z.string(),
  thesis_id: z.string().optional(),
  research_project_id: z.string().optional(),
  instructions: z.string().optional(),
});
export type StartShadowAnalysisInput = z.infer<typeof StartShadowAnalysisInputSchema>;

export const ShadowAnalysisResultSchema = z.object({
  run_id: z.string(),
  workspace_id: z.string(),
  response: StructuredResponseSchema,
  duration_ms: z.number().nonnegative(),
  provider: z.string().optional().nullable(),
  model: z.string().optional().nullable(),
});
export type ShadowAnalysisResult = z.infer<typeof ShadowAnalysisResultSchema>;

export const GooseHealthStatusSchema = z.object({
  binary_available: z.boolean(),
  shadow_mode_enabled: z.boolean(),
  max_concurrent: z.number().int().nonnegative(),
});
export type GooseHealthStatus = z.infer<typeof GooseHealthStatusSchema>;

// Proposal Schemas (M10-G4)

export const ProposalTypeSchema = z.enum(["evidence_candidate", "research_note", "report_outline"]);
export type ProposalType = z.infer<typeof ProposalTypeSchema>;

export const ProposalStatusSchema = z.enum(["pending", "accepted", "rejected"]);
export type ProposalStatus = z.infer<typeof ProposalStatusSchema>;

export const ProposalSchema = z.object({
  id: z.string(),
  workspace_id: z.string(),
  run_id: z.string(),
  proposal_type: ProposalTypeSchema,
  title: z.string(),
  summary: z.string(),
  payload: z.unknown(),
  status: ProposalStatusSchema,
  created_at: z.string(),
  reviewed_at: z.string().optional().nullable(),
  resulting_entity_id: z.string().optional().nullable(),
});
export type Proposal = z.infer<typeof ProposalSchema>;

export const CreateProposalInputSchema = z.object({
  workspace_id: z.string(),
  run_id: z.string(),
  proposal_type: ProposalTypeSchema,
  title: z.string(),
  summary: z.string(),
  payload: z.unknown(),
});
export type CreateProposalInput = z.infer<typeof CreateProposalInputSchema>;

export const EvidenceCandidatePayloadSchema = z.object({
  thesis_id: z.string(),
  source_id: z.string(),
  excerpt: z.string(),
  relation: z.string().default("supports"),
  confidence: z.number().int().min(0).max(100).optional().nullable(),
});
export type EvidenceCandidatePayload = z.infer<typeof EvidenceCandidatePayloadSchema>;

export const ResearchNotePayloadSchema = z.object({
  document_id: z.string(),
  content: z.string(),
});
export type ResearchNotePayload = z.infer<typeof ResearchNotePayloadSchema>;

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

// ============================================================================
// IPC Helpers
// ============================================================================

async function invokeGoose<T>(
  command: string,
  args: Record<string, unknown> | undefined,
  schema: z.ZodType<T, z.ZodTypeDef, unknown>,
): Promise<T> {
  const response: unknown = await invoke(command, args);
  return schema.parse(response);
}

async function invokeGooseVoid(command: string, args: Record<string, unknown>): Promise<void> {
  const response: unknown = await invoke(command, args);
  VoidResponseSchema.parse(response);
}

// ============================================================================
// Commands
// ============================================================================

/**
 * Start a Goose shadow analysis.
 */
export async function startShadowAnalysis(
  input: StartShadowAnalysisInput,
): Promise<ShadowAnalysisResult> {
  const validatedInput = StartShadowAnalysisInputSchema.parse(input);
  return invokeGoose("start_goose_shadow_analysis", { input: validatedInput }, ShadowAnalysisResultSchema);
}

/**
 * Cancel a running Goose analysis.
 */
export async function cancelAnalysis(runId: string): Promise<void> {
  return invokeGooseVoid("cancel_goose_analysis", { runId });
}

/**
 * Check Goose service health.
 */
export async function checkGooseHealth(): Promise<GooseHealthStatus> {
  return invokeGoose("check_goose_health", undefined, GooseHealthStatusSchema);
}

/**
 * Create a new proposal from an agent run (M10-G4).
 */
export async function createProposal(input: CreateProposalInput): Promise<Proposal> {
  const validated = CreateProposalInputSchema.parse(input);
  return invokeGoose("create_goose_proposal", { input: validated }, ProposalSchema);
}

/**
 * List proposals for a workspace (M10-G4).
 */
export async function listProposals(
  workspaceId: string,
  status?: ProposalStatus,
): Promise<Proposal[]> {
  return invokeGoose(
    "list_goose_proposals",
    { workspace_id: workspaceId, status: status ?? null },
    z.array(ProposalSchema),
  );
}

/**
 * Get a proposal by ID (M10-G4).
 */
export async function getProposal(id: string): Promise<Proposal | null> {
  return invokeGoose("get_goose_proposal", { id }, z.union([ProposalSchema, z.null()]));
}

/**
 * Accept a proposal and commit domain writes (M10-G4).
 */
export async function acceptProposal(id: string): Promise<Proposal> {
  return invokeGoose("accept_goose_proposal", { id }, ProposalSchema);
}

/**
 * Reject a proposal (M10-G4).
 */
export async function rejectProposal(id: string): Promise<Proposal> {
  return invokeGoose("reject_goose_proposal", { id }, ProposalSchema);
}