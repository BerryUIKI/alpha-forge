// Goose Agent desktop API.
// Provides Zod-validated types and commands for Goose shadow-mode analysis (M10).

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