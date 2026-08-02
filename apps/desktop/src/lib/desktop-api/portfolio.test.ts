import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { createPortfolioAccount, createPortfolioPosition, getPortfolioAllocation, getPortfolioConcentrationRisks, importPortfolioTransactionsCsv, listPortfolioAccounts, listPortfolioPositions, listPortfolioTransactions } from "./portfolio";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);

describe("portfolio API", () => {
  beforeEach(() => mockInvoke.mockReset());
  it("creates and lists workspace-scoped accounts", async () => { mockInvoke.mockResolvedValue({ id: "account-1" }); await createPortfolioAccount({ workspaceId: "workspace-1", name: "Brokerage", accountType: "brokerage", currency: "USD" }); await listPortfolioAccounts("workspace-1"); expect(mockInvoke).toHaveBeenNthCalledWith(1, "create_portfolio_account", { workspaceId: "workspace-1", name: "Brokerage", accountType: "brokerage", currency: "USD" }); expect(mockInvoke).toHaveBeenNthCalledWith(2, "list_portfolio_accounts", { workspaceId: "workspace-1" }); });
  it("creates and lists account positions", async () => { mockInvoke.mockResolvedValue({ id: "position-1" }); await createPortfolioPosition({ accountId: "account-1", symbol: "MSFT", quantity: 12 }); await listPortfolioPositions("account-1"); expect(mockInvoke).toHaveBeenNthCalledWith(1, "create_portfolio_position", { accountId: "account-1", symbol: "MSFT", quantity: 12, costBasis: null }); expect(mockInvoke).toHaveBeenNthCalledWith(2, "list_portfolio_positions", { accountId: "account-1" }); });
  it("imports and lists immutable account transactions", async () => { mockInvoke.mockResolvedValue([]); await importPortfolioTransactionsCsv("account-1", "symbol,transaction_type,quantity,price,executed_at\\nMSFT,buy,2,420,2026-08-01T00:00:00Z"); await listPortfolioTransactions("account-1"); expect(mockInvoke).toHaveBeenNthCalledWith(1, "import_portfolio_transactions_csv", { accountId: "account-1", csvText: "symbol,transaction_type,quantity,price,executed_at\\nMSFT,buy,2,420,2026-08-01T00:00:00Z" }); expect(mockInvoke).toHaveBeenNthCalledWith(2, "list_portfolio_transactions", { accountId: "account-1" }); });
  it("gets allocation for a workspace", async () => { mockInvoke.mockResolvedValue([]); await getPortfolioAllocation("workspace-1"); expect(mockInvoke).toHaveBeenCalledWith("get_portfolio_allocation", { workspaceId: "workspace-1" }); });
  it("gets transparent concentration risks for a workspace", async () => { mockInvoke.mockResolvedValue([]); await getPortfolioConcentrationRisks("workspace-1"); expect(mockInvoke).toHaveBeenCalledWith("get_portfolio_concentration_risks", { workspaceId: "workspace-1" }); });
});
