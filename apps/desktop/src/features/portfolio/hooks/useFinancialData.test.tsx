import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import {
  financialKeys,
  useHoldings,
  useAllHoldings,
  useValuationSeries,
  useAllocation,
  useNetWorth,
  useSnapshots,
  useCreateSnapshot,
  useListFinancialAccounts,
  useListAllFinancialAccounts,
  useCreateFinancialAccount,
  useListActiveAssets,
  useCreateAsset,
  useListActivitiesByAccount,
  useCreateActivity,
  useCreateLot,
  useRecordSell,
} from "./useFinancialData";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: {
      getHoldings: vi.fn(),
      getAllHoldings: vi.fn(),
      getValuationSeries: vi.fn(),
      getAllocation: vi.fn(),
      computeNetWorth: vi.fn(),
      listSnapshots: vi.fn(),
      createSnapshot: vi.fn(),
      listFinancialAccounts: vi.fn(),
      listAllFinancialAccounts: vi.fn(),
      createFinancialAccount: vi.fn(),
      listActiveAssets: vi.fn(),
      createAsset: vi.fn(),
      listActivitiesByAccount: vi.fn(),
      createActivity: vi.fn(),
      createLot: vi.fn(),
      recordSell: vi.fn(),
    },
  },
}));

function wrapper({ children }: { children: ReactNode }) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("useHoldings", () => {
  it("fetches holdings for an account", async () => {
    vi.mocked(desktopApi.financial.getHoldings).mockResolvedValue(
      [{ asset_id: "a1", quantity: "10" }] as any,
    );
    const { result } = renderHook(() => useHoldings("acct-1", "2026-08-18"), {
      wrapper,
    });
    expect(desktopApi.financial.getHoldings).toHaveBeenCalledWith("acct-1", "2026-08-18");
    await waitFor(() => expect(result.current.data).toEqual([{ asset_id: "a1", quantity: "10" }]));
  });

  it("is disabled without accountId", () => {
    const { result } = renderHook(() => useHoldings(undefined, "2026-08-18"), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
    expect(desktopApi.financial.getHoldings).not.toHaveBeenCalled();
  });
});

describe("useAllHoldings", () => {
  it("fetches all holdings with the allHoldings key", async () => {
    vi.mocked(desktopApi.financial.getAllHoldings).mockResolvedValue([] as any);
    const { result } = renderHook(() => useAllHoldings("2026-08-18"), { wrapper });
    expect(desktopApi.financial.getAllHoldings).toHaveBeenCalledWith("2026-08-18");
    await waitFor(() => expect(result.current.data).toEqual([]));
  });
});

describe("useValuationSeries", () => {
  it("fetches valuation series for an account", async () => {
    vi.mocked(desktopApi.financial.getValuationSeries).mockResolvedValue(
      [{ date: "2026-08-01", total_value: "10000" }] as any,
    );
    const { result } = renderHook(() => useValuationSeries("acct-1"), { wrapper });
    expect(desktopApi.financial.getValuationSeries).toHaveBeenCalledWith("acct-1");
    await waitFor(() =>
      expect(result.current.data).toEqual([{ date: "2026-08-01", total_value: "10000" }]),
    );
  });

  it("is disabled without accountId", () => {
    const { result } = renderHook(() => useValuationSeries(undefined), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });
});

describe("useAllocation", () => {
  it("fetches allocation for a scope", async () => {
    vi.mocked(desktopApi.financial.getAllocation).mockResolvedValue(
      [{ asset_id: "a1", weight_bps: 2500 }] as any,
    );
    const { result } = renderHook(
      () => useAllocation("account", "acct-1", "2026-08-18"),
      { wrapper },
    );
    expect(desktopApi.financial.getAllocation).toHaveBeenCalledWith("account", "acct-1", "2026-08-18");
    await waitFor(() => expect(result.current.data).toEqual([{ asset_id: "a1", weight_bps: 2500 }]));
  });

  it("is disabled without scopeType", () => {
    const { result } = renderHook(() => useAllocation(undefined, null, "2026-08-18"), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });
});

describe("useNetWorth", () => {
  it("fetches net worth with base currency", async () => {
    vi.mocked(desktopApi.financial.computeNetWorth).mockResolvedValue({ total_assets: "10000" } as any);
    const { result } = renderHook(() => useNetWorth("2026-08-18", "USD"), { wrapper });
    expect(desktopApi.financial.computeNetWorth).toHaveBeenCalledWith("2026-08-18", "USD");
    await waitFor(() => expect(result.current.data).toBeDefined());
  });
});

describe("useSnapshots", () => {
  it("is disabled without accountId", () => {
    const { result } = renderHook(() => useSnapshots(undefined), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });

  it("fetches snapshots for an account", async () => {
    vi.mocked(desktopApi.financial.listSnapshots).mockResolvedValue(
      [{ id: "s1", snapshot_date: "2026-08-01" }] as any,
    );
    const { result } = renderHook(() => useSnapshots("acct-1"), { wrapper });
    expect(desktopApi.financial.listSnapshots).toHaveBeenCalledWith("acct-1");
    await waitFor(() => expect(result.current.data).toEqual([{ id: "s1", snapshot_date: "2026-08-01" }]));
  });
});

describe("useCreateSnapshot mutation", () => {
  it("calls createSnapshot and invalidates financial queries", async () => {
    vi.mocked(desktopApi.financial.createSnapshot).mockResolvedValue({ id: "snap-1" } as any);
    const { result } = renderHook(() => useCreateSnapshot(), { wrapper });
    result.current.mutate({ accountId: "acct-1", snapshotDate: "2026-08-18", label: "Quarterly" });
    await waitFor(() =>
      expect(desktopApi.financial.createSnapshot).toHaveBeenCalledWith("acct-1", "2026-08-18", "Quarterly"),
    );
  });
});

describe("Financial account hooks", () => {
  it("useListFinancialAccounts fetches accounts for workspace", async () => {
    vi.mocked(desktopApi.financial.listFinancialAccounts).mockResolvedValue(
      [{ id: "acct-1", name: "Brokerage" }] as any,
    );
    const { result } = renderHook(() => useListFinancialAccounts("ws-1"), { wrapper });
    expect(desktopApi.financial.listFinancialAccounts).toHaveBeenCalledWith("ws-1");
    await waitFor(() => expect(result.current.data).toEqual([{ id: "acct-1", name: "Brokerage" }]));
  });

  it("useListFinancialAccounts is disabled without workspaceId", () => {
    const { result } = renderHook(() => useListFinancialAccounts(undefined), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });

  it("useListAllFinancialAccounts fetches every account (global portfolio)", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue(
      [
        { id: "acct-1", name: "Brokerage" },
        { id: "acct-2", name: "Cash" },
      ] as any,
    );
    const { result } = renderHook(() => useListAllFinancialAccounts(), { wrapper });
    expect(desktopApi.financial.listAllFinancialAccounts).toHaveBeenCalled();
    await waitFor(() =>
      expect(result.current.data).toEqual([
        { id: "acct-1", name: "Brokerage" },
        { id: "acct-2", name: "Cash" },
      ]),
    );
  });

  it("useCreateFinancialAccount calls createFinancialAccount", async () => {
    vi.mocked(desktopApi.financial.createFinancialAccount).mockResolvedValue({ id: "acct-new" } as any);
    const input = {
      workspace_id: "ws-1",
      name: "New Account",
      account_type: "securities" as const,
      currency: "USD",
      tracking_mode: "tracking" as const,
      is_archived: false,
      group_name: null,
    } as any;
    const { result } = renderHook(() => useCreateFinancialAccount(), { wrapper });
    result.current.mutate(input);
    await waitFor(() =>
      expect(desktopApi.financial.createFinancialAccount).toHaveBeenCalledWith(input),
    );
  });
});

describe("Asset hooks", () => {
  it("useListActiveAssets fetches active assets", async () => {
    vi.mocked(desktopApi.financial.listActiveAssets).mockResolvedValue(
      [{ id: "asset-1", name: "Apple" }] as any,
    );
    const { result } = renderHook(() => useListActiveAssets(), { wrapper });
    expect(desktopApi.financial.listActiveAssets).toHaveBeenCalled();
    await waitFor(() => expect(result.current.data).toEqual([{ id: "asset-1", name: "Apple" }]));
  });

  it("useCreateAsset calls createAsset", async () => {
    vi.mocked(desktopApi.financial.createAsset).mockResolvedValue({ id: "asset-new" } as any);
    const input = {
      kind: "equity" as const,
      name: "Apple",
      display_code: "AAPL",
      notes: null,
      is_active: true,
      quote_mode: "live" as const,
      quote_ccy: "USD",
      instrument_type: "EQUITY" as const,
      instrument_symbol: "AAPL",
      instrument_exchange_mic: "XNAS",
      provider_config: null,
    } as any;
    const { result } = renderHook(() => useCreateAsset(), { wrapper });
    result.current.mutate(input);
    await waitFor(() => expect(desktopApi.financial.createAsset).toHaveBeenCalledWith(input));
  });
});

describe("Activity hooks", () => {
  it("useListActivitiesByAccount fetches activities", async () => {
    vi.mocked(desktopApi.financial.listActivitiesByAccount).mockResolvedValue(
      [{ id: "act-1", activity_type: "buy" }] as any,
    );
    const { result } = renderHook(() => useListActivitiesByAccount("acct-1"), { wrapper });
    expect(desktopApi.financial.listActivitiesByAccount).toHaveBeenCalledWith("acct-1");
    await waitFor(() => expect(result.current.data).toEqual([{ id: "act-1", activity_type: "buy" }]));
  });

  it("useCreateActivity calls createActivity", async () => {
    vi.mocked(desktopApi.financial.createActivity).mockResolvedValue({ id: "act-new" } as any);
    const input = {
      account_id: "acct-1",
      asset_id: "asset-1",
      activity_type: "buy" as const,
      quantity: "10",
      unit_price: "100",
      amount: "1000",
      fee: "0",
      tax: "0",
      activity_date: "2026-08-18",
      currency: "USD",
      notes: null,
    } as any;
    const { result } = renderHook(() => useCreateActivity(), { wrapper });
    result.current.mutate(input);
    await waitFor(() => expect(desktopApi.financial.createActivity).toHaveBeenCalledWith(input));
  });
});

describe("Lot hooks", () => {
  it("useCreateLot calls createLot", async () => {
    vi.mocked(desktopApi.financial.createLot).mockResolvedValue({ id: "lot-1" } as any);
    const input = {
      account_id: "acct-1",
      asset_id: "asset-1",
      open_date: "2026-08-01",
      open_activity_id: null,
      original_quantity: "10",
      cost_per_unit: "100",
      original_cost_basis: "1000",
      fee_allocated: "0",
      currency: "USD",
      base_currency: "USD",
      fx_rate_to_base: "1",
      fx_rate_to_account: null,
      account_currency: null,
      cost_basis_method: "fifo" as const,
    };
    const { result } = renderHook(() => useCreateLot(), { wrapper });
    result.current.mutate(input);
    await waitFor(() => expect(desktopApi.financial.createLot).toHaveBeenCalledWith(input));
  });

  it("useRecordSell calls recordSell", async () => {
    vi.mocked(desktopApi.financial.recordSell).mockResolvedValue({ account_id: "acct-1" } as any);
    const { result } = renderHook(() => useRecordSell(), { wrapper });
    result.current.mutate({ accountId: "acct-1", assetId: "asset-1", activityId: "act-1" });
    await waitFor(() =>
      expect(desktopApi.financial.recordSell).toHaveBeenCalledWith("acct-1", "asset-1", "act-1"),
    );
  });
});

describe("financialKeys", () => {
  it("builds hierarchical query keys", () => {
    expect(financialKeys.all).toEqual(["financial"]);
    expect(financialKeys.holdings("a", "2026-08-18")).toEqual(["financial", "holdings", "a", "2026-08-18"]);
    expect(financialKeys.accounts("ws")).toEqual(["financial", "accounts", "ws"]);
    expect(financialKeys.allAccounts()).toEqual(["financial", "accounts", "all"]);
    expect(financialKeys.assets()).toEqual(["financial", "assets"]);
    expect(financialKeys.activities("acct")).toEqual(["financial", "activities", "acct"]);
    expect(financialKeys.performance("a", "2026-08-01", "2026-08-18")).toEqual([
      "financial",
      "performance",
      "a",
      "2026-08-01",
      "2026-08-18",
    ]);
  });
});