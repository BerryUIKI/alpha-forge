import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor, act } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import {
  usePortfolioAccounts,
  usePortfolioPositions,
  usePortfolioTransactions,
  usePortfolioAllocation,
  usePortfolioConcentrationRisks,
  usePortfolioThemeExposure,
  usePortfolioThesisAlignment,
  useCreatePortfolioAccount,
  useCreatePortfolioPosition,
  useImportPortfolioTransactions,
  useLinkPortfolioTheme,
  usePortfolioReview,
} from "./usePortfolio";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    portfolio: {
      listPortfolioAccounts: vi.fn(),
      listPortfolioPositions: vi.fn(),
      listPortfolioTransactions: vi.fn(),
      getPortfolioAllocation: vi.fn(),
      getPortfolioConcentrationRisks: vi.fn(),
      getPortfolioThemeExposure: vi.fn(),
      getPortfolioThesisAlignment: vi.fn(),
      createPortfolioAccount: vi.fn(),
      createPortfolioPosition: vi.fn(),
      importPortfolioTransactionsCsv: vi.fn(),
      linkPortfolioTheme: vi.fn(),
      generatePortfolioReview: vi.fn(),
    },
  },
}));

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
  return { queryClient, wrapper };
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("usePortfolio hooks", () => {
  describe("usePortfolioAccounts", () => {
    it("fetches accounts for a workspace", async () => {
      const mockAccounts = [
        {
          id: "acc-1",
          workspaceId: "ws-1",
          name: "Main Brokerage",
          accountType: "securities",
          currency: "USD",
          createdAt: "2026-08-24T00:00:00Z",
          updatedAt: "2026-08-24T00:00:00Z",
        },
      ];
      vi.mocked(desktopApi.portfolio.listPortfolioAccounts).mockResolvedValue(
        mockAccounts,
      );

      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioAccounts("ws-1"), {
        wrapper,
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockAccounts);
      expect(
        desktopApi.portfolio.listPortfolioAccounts,
      ).toHaveBeenCalledWith("ws-1");
    });

    it("does not fetch when workspaceId is empty", () => {
      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioAccounts(""), {
        wrapper,
      });

      expect(result.current.fetchStatus).toBe("idle");
      expect(
        desktopApi.portfolio.listPortfolioAccounts,
      ).not.toHaveBeenCalled();
    });
  });

  describe("usePortfolioPositions", () => {
    it("fetches positions for an account", async () => {
      const mockPositions = [
        {
          id: "pos-1",
          accountId: "acc-1",
          symbol: "AAPL",
          quantity: 10,
          costBasis: 1500,
          createdAt: "2026-08-24T00:00:00Z",
          updatedAt: "2026-08-24T00:00:00Z",
        },
      ];
      vi.mocked(desktopApi.portfolio.listPortfolioPositions).mockResolvedValue(
        mockPositions,
      );

      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioPositions("acc-1"), {
        wrapper,
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockPositions);
      expect(
        desktopApi.portfolio.listPortfolioPositions,
      ).toHaveBeenCalledWith("acc-1");
    });
  });

  describe("usePortfolioTransactions", () => {
    it("fetches transactions for an account", async () => {
      const mockTxs = [
        {
          id: "tx-1",
          accountId: "acc-1",
          symbol: "AAPL",
          transactionType: "buy" as const,
          quantity: 10,
          price: 150,
          executedAt: "2026-08-24T00:00:00Z",
          createdAt: "2026-08-24T00:00:00Z",
        },
      ];
      vi.mocked(
        desktopApi.portfolio.listPortfolioTransactions,
      ).mockResolvedValue(mockTxs);

      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioTransactions("acc-1"), {
        wrapper,
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockTxs);
      expect(
        desktopApi.portfolio.listPortfolioTransactions,
      ).toHaveBeenCalledWith("acc-1");
    });
  });

  describe("usePortfolioAllocation", () => {
    it("fetches allocation breakdown for a workspace", async () => {
      const mockAlloc = [
        {
          symbol: "AAPL",
          allocatedCost: 1500,
          weightPercent: 60,
          accountCount: 1,
        },
      ];
      vi.mocked(desktopApi.portfolio.getPortfolioAllocation).mockResolvedValue(
        mockAlloc,
      );

      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioAllocation("ws-1"), {
        wrapper,
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockAlloc);
      expect(
        desktopApi.portfolio.getPortfolioAllocation,
      ).toHaveBeenCalledWith("ws-1");
    });
  });

  describe("usePortfolioConcentrationRisks", () => {
    it("fetches concentration risks for a workspace", async () => {
      const mockRisks = [
        {
          symbol: "AAPL",
          weightPercent: 60,
          severity: "high" as const,
          message: "High allocation concentration",
        },
      ];
      vi.mocked(
        desktopApi.portfolio.getPortfolioConcentrationRisks,
      ).mockResolvedValue(mockRisks);

      const { wrapper } = createWrapper();
      const { result } = renderHook(
        () => usePortfolioConcentrationRisks("ws-1"),
        { wrapper },
      );

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockRisks);
      expect(
        desktopApi.portfolio.getPortfolioConcentrationRisks,
      ).toHaveBeenCalledWith("ws-1");
    });
  });

  describe("usePortfolioThemeExposure", () => {
    it("fetches theme exposure for a workspace", async () => {
      const mockThemes = [
        {
          entityId: "theme-1",
          themeName: "AI & Tech",
          allocatedCost: 1500,
          weightPercent: 60,
        },
      ];
      vi.mocked(
        desktopApi.portfolio.getPortfolioThemeExposure,
      ).mockResolvedValue(mockThemes);

      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioThemeExposure("ws-1"), {
        wrapper,
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockThemes);
      expect(
        desktopApi.portfolio.getPortfolioThemeExposure,
      ).toHaveBeenCalledWith("ws-1");
    });
  });

  describe("usePortfolioThesisAlignment", () => {
    it("fetches thesis alignment for a workspace", async () => {
      const mockAlignment = [
        {
          symbol: "AAPL",
          thesisId: "th-1",
          thesisTitle: "Ecosystem Growth",
          confidence: 80,
          status: "active",
        },
      ];
      vi.mocked(
        desktopApi.portfolio.getPortfolioThesisAlignment,
      ).mockResolvedValue(mockAlignment);

      const { wrapper } = createWrapper();
      const { result } = renderHook(() => usePortfolioThesisAlignment("ws-1"), {
        wrapper,
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockAlignment);
      expect(
        desktopApi.portfolio.getPortfolioThesisAlignment,
      ).toHaveBeenCalledWith("ws-1");
    });
  });

  describe("useCreatePortfolioAccount", () => {
    it("creates account and invalidates accounts query", async () => {
      const createdAccount = {
        id: "acc-2",
        workspaceId: "ws-1",
        name: "New Account",
        accountType: "securities",
        currency: "USD",
        createdAt: "2026-08-24T00:00:00Z",
        updatedAt: "2026-08-24T00:00:00Z",
      };
      vi.mocked(desktopApi.portfolio.createPortfolioAccount).mockResolvedValue(
        createdAccount,
      );

      const { queryClient, wrapper } = createWrapper();
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      const { result } = renderHook(() => useCreatePortfolioAccount(), {
        wrapper,
      });

      await act(async () => {
        await result.current.mutateAsync({
          workspaceId: "ws-1",
          name: "New Account",
          accountType: "securities",
          currency: "USD",
        });
      });

      expect(desktopApi.portfolio.createPortfolioAccount).toHaveBeenCalledWith({
        workspaceId: "ws-1",
        name: "New Account",
        accountType: "securities",
        currency: "USD",
      });
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "accounts", "ws-1"],
      });
    });
  });

  describe("useCreatePortfolioPosition", () => {
    it("creates position and invalidates queries", async () => {
      const createdPos = {
        id: "pos-2",
        accountId: "acc-1",
        symbol: "GOOGL",
        quantity: 5,
        costBasis: 1000,
        createdAt: "2026-08-24T00:00:00Z",
        updatedAt: "2026-08-24T00:00:00Z",
      };
      vi.mocked(desktopApi.portfolio.createPortfolioPosition).mockResolvedValue(
        createdPos,
      );

      const { queryClient, wrapper } = createWrapper();
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      const { result } = renderHook(() => useCreatePortfolioPosition(), {
        wrapper,
      });

      await act(async () => {
        await result.current.mutateAsync({
          accountId: "acc-1",
          symbol: "GOOGL",
          quantity: 5,
          costBasis: 1000,
        });
      });

      expect(desktopApi.portfolio.createPortfolioPosition).toHaveBeenCalledWith(
        {
          accountId: "acc-1",
          symbol: "GOOGL",
          quantity: 5,
          costBasis: 1000,
        },
      );
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "positions", "acc-1"],
      });
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio"],
      });
    });
  });

  describe("useImportPortfolioTransactions", () => {
    it("imports CSV transactions and invalidates queries", async () => {
      const importedTxs = [
        {
          id: "tx-2",
          accountId: "acc-1",
          symbol: "NVDA",
          transactionType: "buy" as const,
          quantity: 10,
          price: 120,
          executedAt: "2026-08-24T00:00:00Z",
          createdAt: "2026-08-24T00:00:00Z",
        },
      ];
      vi.mocked(
        desktopApi.portfolio.importPortfolioTransactionsCsv,
      ).mockResolvedValue(importedTxs);

      const { queryClient, wrapper } = createWrapper();
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      const { result } = renderHook(() => useImportPortfolioTransactions(), {
        wrapper,
      });

      await act(async () => {
        await result.current.mutateAsync({
          accountId: "acc-1",
          csvText: "symbol,transaction_type,quantity,price,executed_at\nNVDA,buy,10,120,2026-08-24T00:00:00Z",
        });
      });

      expect(
        desktopApi.portfolio.importPortfolioTransactionsCsv,
      ).toHaveBeenCalledWith(
        "acc-1",
        "symbol,transaction_type,quantity,price,executed_at\nNVDA,buy,10,120,2026-08-24T00:00:00Z",
      );
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "transactions", "acc-1"],
      });
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "positions", "acc-1"],
      });
    });
  });

  describe("useLinkPortfolioTheme", () => {
    it("links theme and invalidates theme query", async () => {
      vi.mocked(desktopApi.portfolio.linkPortfolioTheme).mockResolvedValue(
        undefined,
      );

      const { queryClient, wrapper } = createWrapper();
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      const { result } = renderHook(() => useLinkPortfolioTheme(), { wrapper });

      await act(async () => {
        await result.current.mutateAsync({
          workspaceId: "ws-1",
          symbol: "AAPL",
          entityId: "theme-1",
        });
      });

      expect(desktopApi.portfolio.linkPortfolioTheme).toHaveBeenCalledWith(
        "ws-1",
        "AAPL",
        "theme-1",
      );
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "themes", "ws-1"],
      });
    });
  });

  describe("usePortfolioReview", () => {
    it("generates portfolio review and invalidates alignment and risk queries", async () => {
      const reviewResult = {
        generatedAt: "2026-08-24T00:00:00Z",
        concentrationRisks: [],
        unalignedSymbols: ["TSLA"],
      };
      vi.mocked(desktopApi.portfolio.generatePortfolioReview).mockResolvedValue(
        reviewResult,
      );

      const { queryClient, wrapper } = createWrapper();
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      const { result } = renderHook(() => usePortfolioReview(), { wrapper });

      await act(async () => {
        await result.current.mutateAsync("ws-1");
      });

      expect(
        desktopApi.portfolio.generatePortfolioReview,
      ).toHaveBeenCalledWith("ws-1");
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "alignment", "ws-1"],
      });
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["portfolio", "risks", "ws-1"],
      });
    });
  });
});
