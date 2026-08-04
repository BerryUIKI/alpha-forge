// Goose Agent desktop API.
// Provides types and commands for Goose shadow-mode analysis (M10).

import { invoke } from "@tauri-apps/api/core";

// ============================================================================
// Types
// ============================================================================

/**
 * Input for starting a Goose shadow analysis.
 */
export interface StartShadowAnalysisInput {
  /** Workspace to analyze */
  workspace_id: string;
  /** Optional thesis to focus on */
  thesis_id?: string;
  /** Optional research project to focus on */
  research_project_id?: string;
  /** Custom instructions for Goose */
  instructions?: string;
}

/**
 * A claim with confidence and source references.
 */
export interface Claim {
  id: string;
  claim: string;
  confidence: number;
  source_ids: string[];
  contradicting_source_ids: string[];
}

/**
 * Evidence linking claims to sources.
 */
export interface Evidence {
  claim_id: string;
  source_id: string;
  excerpt: string;
  relation: "supports" | "contradicts" | "neutral";
  confidence?: number;
}

/**
 * Risk severity levels.
 */
export type RiskSeverity = "low" | "medium" | "high" | "critical";

/**
 * A risk identified in the analysis.
 */
export interface Risk {
  id: string;
  risk: string;
  severity: RiskSeverity;
  related_claim_ids: string[];
  mitigation?: string;
}

/**
 * Structured response from Goose analysis.
 */
export interface StructuredResponse {
  summary: string;
  claims: Claim[];
  evidence: Evidence[];
  contradictions: Array<{
    description: string;
    claim_ids: string[];
    source_ids: string[];
  }>;
  risks: Risk[];
  unknowns: string[];
  source_ids: string[];
  confidence: number;
  provider?: string;
  model?: string;
  recipe_version?: string;
}

/**
 * Result of a shadow analysis run.
 */
export interface ShadowAnalysisResult {
  /** Run ID */
  run_id: string;
  /** Workspace analyzed */
  workspace_id: string;
  /** Structured response from Goose */
  response: StructuredResponse;
  /** Execution duration in milliseconds */
  duration_ms: number;
  /** Provider used */
  provider?: string;
  /** Model used */
  model?: string;
}

/**
 * Health status of the Goose service.
 */
export interface GooseHealthStatus {
  binary_available: boolean;
  shadow_mode_enabled: boolean;
  max_concurrent: number;
}

// ============================================================================
// Commands
// ============================================================================

/**
 * Start a Goose shadow analysis.
 */
export async function startShadowAnalysis(
  input: StartShadowAnalysisInput
): Promise<ShadowAnalysisResult> {
  return invoke<ShadowAnalysisResult>("start_goose_shadow_analysis", { input });
}

/**
 * Cancel a running Goose analysis.
 */
export async function cancelAnalysis(runId: string): Promise<void> {
  return invoke("cancel_goose_analysis", { runId });
}

/**
 * Check Goose service health.
 */
export async function checkGooseHealth(): Promise<GooseHealthStatus> {
  return invoke<GooseHealthStatus>("check_goose_health");
}