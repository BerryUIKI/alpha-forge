import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import * as api from "./options";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);
const iso = "2024-01-02T03:04:05.000Z";
const workspaceId = "00000000-0000-4000-8000-000000000001";
const chainId = "00000000-0000-4000-8000-000000000002";
const contractId = "00000000-0000-4000-8000-000000000003";
const strategyId = "00000000-0000-4000-8000-000000000004";
const scoped = { workspaceId, createdAt: iso };
const chain = {
  id: chainId,
  ...scoped,
  symbol: "AAPL",
  underlyingPrice: 150,
  asOf: iso,
  dataSource: "demo",
};
const contract = {
  id: contractId,
  ...scoped,
  chainId,
  symbol: "AAPL",
  optionType: "call",
  strike: 150,
  expiration: iso,
  contractMultiplier: 100,
  bid: 4,
  ask: 5,
  last: null,
  volume: 10,
  openInterest: 20,
  impliedVolatility: 0.25,
  updatedAt: iso,
};
const strategy = {
  id: strategyId,
  ...scoped,
  name: "Demo spread",
  strategyType: "bull_call_spread",
  underlying: "AAPL",
  totalCost: 100,
  maxProfit: null,
  maxLoss: -100,
  breakEvenPoints: [151],
  updatedAt: iso,
};
const pricing = {
  optionType: "call" as const,
  underlyingPrice: 150,
  strike: 150,
  expirationYears: 1,
  riskFreeRate: 0.05,
  volatility: 0.25,
};
const contractParams = {
  workspaceId,
  chainId,
  symbol: "AAPL",
  optionType: "call" as const,
  strike: 150,
  expiration: iso,
  last: null,
};
describe("options desktop API", () => {
  beforeEach(() => mockInvoke.mockReset());
  it("uses registered camelCase commands", async () => {
    mockInvoke.mockResolvedValueOnce(chain).mockResolvedValueOnce([chain]);
    await expect(api.fetchOptionChain({ symbol: "AAPL", workspaceId })).resolves.toEqual(chain);
    await expect(api.listOptionChains(workspaceId)).resolves.toEqual([chain]);
    expect(mockInvoke.mock.calls).toEqual([
      ["fetch_option_chain", { params: { symbol: "AAPL", workspaceId } }],
      ["list_option_chains", { workspaceId }],
    ]);
  });
  it("nests creation params", async () => {
    mockInvoke.mockResolvedValueOnce(contract).mockResolvedValueOnce(strategy);
    await expect(api.createOptionContract(contractParams)).resolves.toEqual(contract);
    await expect(
      api.createOptionStrategy({
        workspaceId,
        name: "Demo spread",
        strategyType: "bull_call_spread",
        underlying: "AAPL",
      }),
    ).resolves.toEqual(strategy);
    expect(mockInvoke.mock.calls[0]).toEqual([
      "create_option_contract",
      { params: expect.objectContaining({ chainId }) },
    ]);
    expect(mockInvoke.mock.calls[1]).toEqual([
      "create_option_strategy",
      { params: expect.objectContaining({ strategyType: "bull_call_spread" }) },
    ]);
  });
  it("rejects malformed and snake_case responses", async () => {
    mockInvoke.mockResolvedValueOnce([{ ...contract, bid: "4" }]);
    await expect(api.listOptionContracts(chainId)).rejects.toThrow();
    mockInvoke.mockResolvedValueOnce({
      ...Object.fromEntries(Object.entries(chain).filter(([key]) => key !== "workspaceId")),
      workspace_id: workspaceId,
    });
    await expect(api.getOptionChain(chainId)).rejects.toThrow();
  });
  it("validates numbers and void deletes", async () => {
    mockInvoke.mockResolvedValueOnce({
      delta: 0.5,
      gamma: 0.01,
      theta: -0.1,
      vega: 0.2,
      rho: 0.03,
    });
    await expect(api.calculateGreeks(pricing)).resolves.toMatchObject({ delta: 0.5 });
    mockInvoke.mockResolvedValueOnce("not-a-number");
    await expect(api.calculateOptionPrice(pricing)).rejects.toThrow();
    mockInvoke.mockResolvedValueOnce(null);
    await expect(api.deleteOptionChain(chainId)).resolves.toBeUndefined();
  });
  it("requires nullable strategy fields", async () => {
    mockInvoke.mockResolvedValueOnce([strategy]);
    await expect(api.listOptionStrategies(workspaceId)).resolves.toEqual([strategy]);
    mockInvoke.mockResolvedValueOnce({ ...strategy, maxProfit: undefined });
    await expect(api.listOptionStrategies(workspaceId)).rejects.toThrow();
  });
});
