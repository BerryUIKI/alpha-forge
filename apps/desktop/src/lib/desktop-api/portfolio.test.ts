import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  createPortfolioAccount,
  createPortfolioPosition,
  getPortfolioAllocation,
  getPortfolioConcentrationRisks,
  getPortfolioThemeExposure,
  getPortfolioThesisAlignment,
  generatePortfolioReview,
  importPortfolioTransactionsCsv,
  listPortfolioAccounts,
  listPortfolioPositions,
  listPortfolioTransactions,
  linkPortfolioTheme,
} from "./portfolio";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);

describe("portfolio API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("creates and lists workspace-scoped accounts", async () => {
    const mockAccount = {
      id: "account-1",
      workspaceId: "workspace-1",
      name: "Brokerage",
      accountType: "securities",
      currency: "USD",
      createdAt: "2026-08-21T00:00:00Z",
      updatedAt: "2026-08-21T00:00:00Z",
    };
    mockInvoke.mockResolvedValue(mockAccount);
    const created = await createPortfolioAccount({
      workspaceId: "workspace-1",
      name: "Brokerage",
      accountType: "securities",
      currency: "USD",
    });
    expect(created.workspaceId).toBe("workspace-1");

    mockInvoke.mockResolvedValue([mockAccount]);
    const list = await listPortfolioAccounts("workspace-1");
    expect(list).toHaveLength(1);
    expect(list[0]?.name).toBe("Brokerage");
  });

  it("creates and lists account positions", async () => {
    const mockPosition = {
      id: "pos-1",
      accountId: "account-1",
      symbol: "MSFT",
      quantity: 12,
      costBasis: 4200,
      createdAt: "2026-08-21T00:00:00Z",
      updatedAt: "2026-08-21T00:00:00Z",
    };
    mockInvoke.mockResolvedValue(mockPosition);
    const created = await createPortfolioPosition({
      accountId: "account-1",
      symbol: "MSFT",
      quantity: 12,
      costBasis: 4200,
    });
    expect(created.symbol).toBe("MSFT");

    mockInvoke.mockResolvedValue([mockPosition]);
    const list = await listPortfolioPositions("account-1");
    expect(list).toHaveLength(1);
    expect(list[0]?.accountId).toBe("account-1");
  });

  it("imports and lists immutable account transactions", async () => {
    const mockTx = {
      id: "tx-1",
      accountId: "account-1",
      symbol: "MSFT",
      transactionType: "buy",
      quantity: 2,
      price: 420,
      executedAt: "2026-08-01T00:00:00Z",
      createdAt: "2026-08-01T00:00:00Z",
    };
    mockInvoke.mockResolvedValue([mockTx]);
    const imported = await importPortfolioTransactionsCsv(
      "account-1",
      "symbol,transaction_type,quantity,price,executed_at\nMSFT,buy,2,420,2026-08-01T00:00:00Z",
    );
    expect(imported[0]?.transactionType).toBe("buy");

    const list = await listPortfolioTransactions("account-1");
    expect(list).toHaveLength(1);
    expect(list[0]?.executedAt).toBe("2026-08-01T00:00:00Z");
  });

  it("gets allocation for a workspace", async () => {
    const mockAlloc = {
      symbol: "MSFT",
      allocatedCost: 840,
      weightPercent: 100,
      accountCount: 1,
    };
    mockInvoke.mockResolvedValue([mockAlloc]);
    const res = await getPortfolioAllocation("workspace-1");
    expect(res[0]?.weightPercent).toBe(100);
  });

  it("gets transparent concentration risks for a workspace", async () => {
    const mockRisk = {
      symbol: "MSFT",
      weightPercent: 45,
      severity: "high",
      message: "High concentration",
    };
    mockInvoke.mockResolvedValue([mockRisk]);
    const res = await getPortfolioConcentrationRisks("workspace-1");
    expect(res[0]?.severity).toBe("high");
  });

  it("links theme, gets theme exposure, thesis alignment, and review", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await linkPortfolioTheme("workspace-1", "MSFT", "theme-1");
    expect(mockInvoke).toHaveBeenCalledWith("link_portfolio_theme", {
      workspaceId: "workspace-1",
      symbol: "MSFT",
      entityId: "theme-1",
    });

    mockInvoke.mockResolvedValue([
      {
        entityId: "theme-1",
        themeName: "AI Infrastructure",
        allocatedCost: 840,
        weightPercent: 100,
      },
    ]);
    const exposure = await getPortfolioThemeExposure("workspace-1");
    expect(exposure[0]?.themeName).toBe("AI Infrastructure");

    mockInvoke.mockResolvedValue([
      {
        symbol: "MSFT",
        thesisId: "t-1",
        thesisTitle: "Cloud growth",
        confidence: 85,
        status: "active",
      },
    ]);
    const alignment = await getPortfolioThesisAlignment("workspace-1");
    expect(alignment[0]?.confidence).toBe(85);

    mockInvoke.mockResolvedValue({
      generatedAt: "2026-08-21T00:00:00Z",
      concentrationRisks: [],
      unalignedSymbols: ["NVDA"],
    });
    const review = await generatePortfolioReview("workspace-1");
    expect(review.unalignedSymbols).toContain("NVDA");
  });

  it("rejects malformed responses with Zod schema validation", async () => {
    mockInvoke.mockResolvedValue({
      id: "acc-1",
      // missing required fields
    });
    await expect(
      createPortfolioAccount({
        workspaceId: "workspace-1",
        name: "Brokerage",
        accountType: "securities",
        currency: "USD",
      }),
    ).rejects.toThrow();
  });
});
