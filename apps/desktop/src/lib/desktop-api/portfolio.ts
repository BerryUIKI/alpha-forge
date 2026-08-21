// Portfolio desktop API with strict Zod schema validation.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

// ── Enums & Schemas ─────────────────────────────────────────────────────────

export const TransactionTypeSchema = z.enum(["buy", "sell"]);
export type TransactionType = z.infer<typeof TransactionTypeSchema>;

export const ConcentrationSeveritySchema = z.enum(["moderate", "high"]);
export type ConcentrationSeverity = z.infer<typeof ConcentrationSeveritySchema>;

export const PortfolioAccountSchema = z.object({
  id: z.string(),
  workspaceId: z.string(),
  name: z.string(),
  accountType: z.string(),
  currency: z.string(),
  createdAt: z.string(),
  updatedAt: z.string(),
});
export type PortfolioAccount = z.infer<typeof PortfolioAccountSchema>;

export const PositionSchema = z.object({
  id: z.string(),
  accountId: z.string(),
  symbol: z.string(),
  quantity: z.number(),
  costBasis: z.number().nullable(),
  createdAt: z.string(),
  updatedAt: z.string(),
});
export type Position = z.infer<typeof PositionSchema>;

export const PortfolioTransactionSchema = z.object({
  id: z.string(),
  accountId: z.string(),
  symbol: z.string(),
  transactionType: TransactionTypeSchema,
  quantity: z.number(),
  price: z.number(),
  executedAt: z.string(),
  createdAt: z.string(),
});
export type PortfolioTransaction = z.infer<typeof PortfolioTransactionSchema>;

export const PortfolioAllocationSchema = z.object({
  symbol: z.string(),
  allocatedCost: z.number(),
  weightPercent: z.number(),
  accountCount: z.number(),
});
export type PortfolioAllocation = z.infer<typeof PortfolioAllocationSchema>;

export const ConcentrationRiskSchema = z.object({
  symbol: z.string(),
  weightPercent: z.number(),
  severity: ConcentrationSeveritySchema,
  message: z.string(),
});
export type ConcentrationRisk = z.infer<typeof ConcentrationRiskSchema>;

export const ThemeExposureSchema = z.object({
  entityId: z.string(),
  themeName: z.string(),
  allocatedCost: z.number(),
  weightPercent: z.number(),
});
export type ThemeExposure = z.infer<typeof ThemeExposureSchema>;

export const ThesisAlignmentSchema = z.object({
  symbol: z.string(),
  thesisId: z.string(),
  thesisTitle: z.string(),
  confidence: z.number(),
  status: z.string(),
});
export type ThesisAlignment = z.infer<typeof ThesisAlignmentSchema>;

export const PortfolioReviewSchema = z.object({
  generatedAt: z.string(),
  concentrationRisks: z.array(ConcentrationRiskSchema),
  unalignedSymbols: z.array(z.string()),
});
export type PortfolioReview = z.infer<typeof PortfolioReviewSchema>;

// ── Params ──────────────────────────────────────────────────────────────────

export interface CreatePortfolioAccountParams {
  workspaceId: string;
  name: string;
  accountType: string;
  currency: string;
}

export interface CreatePortfolioPositionParams {
  accountId: string;
  symbol: string;
  quantity: number;
  costBasis?: number;
}

// ── Desktop API Wrappers ────────────────────────────────────────────────────

export async function createPortfolioAccount(
  params: CreatePortfolioAccountParams,
): Promise<PortfolioAccount> {
  const res = await invoke("create_portfolio_account", { ...params });
  return PortfolioAccountSchema.parse(res);
}

export async function listPortfolioAccounts(
  workspaceId: string,
): Promise<PortfolioAccount[]> {
  const res = await invoke("list_portfolio_accounts", { workspaceId });
  return z.array(PortfolioAccountSchema).parse(res);
}

export async function createPortfolioPosition(
  params: CreatePortfolioPositionParams,
): Promise<Position> {
  const res = await invoke("create_portfolio_position", {
    ...params,
    costBasis: params.costBasis ?? null,
  });
  return PositionSchema.parse(res);
}

export async function listPortfolioPositions(
  accountId: string,
): Promise<Position[]> {
  const res = await invoke("list_portfolio_positions", { accountId });
  return z.array(PositionSchema).parse(res);
}

export async function importPortfolioTransactionsCsv(
  accountId: string,
  csvText: string,
): Promise<PortfolioTransaction[]> {
  const res = await invoke("import_portfolio_transactions_csv", {
    accountId,
    csvText,
  });
  return z.array(PortfolioTransactionSchema).parse(res);
}

export async function listPortfolioTransactions(
  accountId: string,
): Promise<PortfolioTransaction[]> {
  const res = await invoke("list_portfolio_transactions", { accountId });
  return z.array(PortfolioTransactionSchema).parse(res);
}

export async function getPortfolioAllocation(
  workspaceId: string,
): Promise<PortfolioAllocation[]> {
  const res = await invoke("get_portfolio_allocation", { workspaceId });
  return z.array(PortfolioAllocationSchema).parse(res);
}

export async function getPortfolioConcentrationRisks(
  workspaceId: string,
): Promise<ConcentrationRisk[]> {
  const res = await invoke("get_portfolio_concentration_risks", {
    workspaceId,
  });
  return z.array(ConcentrationRiskSchema).parse(res);
}

export async function linkPortfolioTheme(
  workspaceId: string,
  symbol: string,
  entityId: string,
): Promise<void> {
  await invoke("link_portfolio_theme", { workspaceId, symbol, entityId });
}

export async function getPortfolioThemeExposure(
  workspaceId: string,
): Promise<ThemeExposure[]> {
  const res = await invoke("get_portfolio_theme_exposure", { workspaceId });
  return z.array(ThemeExposureSchema).parse(res);
}

export async function getPortfolioThesisAlignment(
  workspaceId: string,
): Promise<ThesisAlignment[]> {
  const res = await invoke("get_portfolio_thesis_alignment", { workspaceId });
  return z.array(ThesisAlignmentSchema).parse(res);
}

export async function generatePortfolioReview(
  workspaceId: string,
): Promise<PortfolioReview> {
  const res = await invoke("generate_portfolio_review", { workspaceId });
  return PortfolioReviewSchema.parse(res);
}
