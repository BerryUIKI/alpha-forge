// Portfolio desktop API.

import { invoke } from "@tauri-apps/api/core";

export interface PortfolioAccount { id: string; workspace_id: string; name: string; account_type: string; currency: string; created_at: string; updated_at: string; }
export interface Position { id: string; account_id: string; symbol: string; quantity: number; cost_basis: number | null; created_at: string; updated_at: string; }
export interface CreatePortfolioAccountParams { workspaceId: string; name: string; accountType: string; currency: string; }
export interface CreatePortfolioPositionParams { accountId: string; symbol: string; quantity: number; costBasis?: number; }
export interface PortfolioTransaction { id: string; account_id: string; symbol: string; transaction_type: "buy" | "sell"; quantity: number; price: number; executed_at: string; created_at: string; }
export interface PortfolioAllocation { symbol: string; allocated_cost: number; weight_percent: number; account_count: number; }
export interface ConcentrationRisk { symbol: string; weight_percent: number; severity: "moderate" | "high"; message: string; }
export interface ThemeExposure { entity_id: string; theme_name: string; allocated_cost: number; weight_percent: number; }
export interface ThesisAlignment { symbol: string; thesis_id: string; thesis_title: string; confidence: number; status: string; }
export interface PortfolioReview { generated_at: string; concentration_risks: ConcentrationRisk[]; unaligned_symbols: string[]; }

export function createPortfolioAccount(params: CreatePortfolioAccountParams): Promise<PortfolioAccount> { return invoke("create_portfolio_account", { ...params }); }
export function listPortfolioAccounts(workspaceId: string): Promise<PortfolioAccount[]> { return invoke("list_portfolio_accounts", { workspaceId }); }
export function createPortfolioPosition(params: CreatePortfolioPositionParams): Promise<Position> { return invoke("create_portfolio_position", { ...params, costBasis: params.costBasis ?? null }); }
export function listPortfolioPositions(accountId: string): Promise<Position[]> { return invoke("list_portfolio_positions", { accountId }); }
export function importPortfolioTransactionsCsv(accountId: string, csvText: string): Promise<PortfolioTransaction[]> { return invoke("import_portfolio_transactions_csv", { accountId, csvText }); }
export function listPortfolioTransactions(accountId: string): Promise<PortfolioTransaction[]> { return invoke("list_portfolio_transactions", { accountId }); }
export function getPortfolioAllocation(workspaceId: string): Promise<PortfolioAllocation[]> { return invoke("get_portfolio_allocation", { workspaceId }); }
export function getPortfolioConcentrationRisks(workspaceId: string): Promise<ConcentrationRisk[]> { return invoke("get_portfolio_concentration_risks", { workspaceId }); }
export function linkPortfolioTheme(workspaceId: string, symbol: string, entityId: string): Promise<void> { return invoke("link_portfolio_theme", { workspaceId, symbol, entityId }); }
export function getPortfolioThemeExposure(workspaceId: string): Promise<ThemeExposure[]> { return invoke("get_portfolio_theme_exposure", { workspaceId }); }
export function getPortfolioThesisAlignment(workspaceId: string): Promise<ThesisAlignment[]> { return invoke("get_portfolio_thesis_alignment", { workspaceId }); }
export function generatePortfolioReview(workspaceId: string): Promise<PortfolioReview> { return invoke("generate_portfolio_review", { workspaceId }); }
