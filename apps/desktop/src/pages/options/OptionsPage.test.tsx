import "@testing-library/jest-dom";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { OptionChainList } from "@/features/options";
import { OptionContractTable } from "@/features/options";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate } from "@/lib/i18n/locale";
import { OptionsPage } from "./OptionsPage";

const workspaceId = "00000000-0000-4000-8000-000000000001";
const chainId = "00000000-0000-4000-8000-000000000002";
const contractId = "00000000-0000-4000-8000-000000000003";
const strategyId = "00000000-0000-4000-8000-000000000004";
const legId = "00000000-0000-4000-8000-000000000005";
const iso = "2026-08-14T00:00:00.000Z";

const workspaceMock = vi.hoisted(() => ({ listWorkspaces: vi.fn() }));
const optionsMock = vi.hoisted(() => ({
  fetchOptionChain: vi.fn(),
  listOptionChains: vi.fn(),
  listOptionContracts: vi.fn(),
  listOptionStrategies: vi.fn(),
  createOptionStrategy: vi.fn(),
  deleteOptionStrategy: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: { workspace: workspaceMock, options: optionsMock },
}));

const chain = {
  id: chainId,
  workspaceId,
  symbol: "AAPL",
  underlyingPrice: 150,
  asOf: iso,
  dataSource: "demo" as const,
  createdAt: iso,
};
const contract = {
  id: contractId,
  workspaceId,
  chainId,
  symbol: "AAPL",
  optionType: "call" as const,
  strike: 150,
  expiration: iso,
  contractMultiplier: 100,
  bid: 4,
  ask: 5,
  last: 4.5,
  volume: 10,
  openInterest: 20,
  impliedVolatility: 0.25,
  createdAt: iso,
  updatedAt: iso,
};
const strategy = {
  id: strategyId,
  workspaceId,
  name: "Call spread",
  strategyType: "bull_call_spread" as const,
  underlying: "AAPL",
  totalCost: 500,
  maxProfit: null,
  maxLoss: null,
  breakEvenPoints: [],
  legs: [
    {
      id: legId,
      strategyId,
      optionContractId: contractId,
      quantity: 1,
      positionType: "long" as const,
      premium: 5,
      strike: 150,
      expiration: iso,
      optionType: "call" as const,
    },
  ],
  createdAt: iso,
  updatedAt: iso,
};

function renderWithQuery(ui: React.ReactNode) {
  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  return render(
    <LocaleContext.Provider
      value={{ locale: "en", setLocale: async () => undefined, t: (key) => translate("en", key) }}
    >
      <QueryClientProvider client={queryClient}>
        <MemoryRouter initialEntries={[`/options?workspace=${workspaceId}`]}>{ui}</MemoryRouter>
      </QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("Option chain contract view", () => {
  afterEach(() => vi.restoreAllMocks());

  beforeEach(() => {
    vi.clearAllMocks();
    vi.spyOn(console, "error").mockImplementation(() => undefined);
    workspaceMock.listWorkspaces.mockResolvedValue([
      { id: workspaceId, name: "Options workspace" },
    ]);
    optionsMock.listOptionChains.mockResolvedValue([]);
    optionsMock.listOptionContracts.mockResolvedValue([]);
    optionsMock.listOptionStrategies.mockResolvedValue([]);
    optionsMock.deleteOptionStrategy.mockResolvedValue(undefined);
  });

  it("fetches a normalized demo chain, refreshes the list, and selects its contracts", async () => {
    let persistedChains: (typeof chain)[] = [];
    optionsMock.listOptionChains.mockImplementation(async () => persistedChains);
    optionsMock.fetchOptionChain.mockImplementation(async (params: { symbol: string }) => {
      persistedChains = [chain];
      expect(params.symbol).toBe("AAPL");
      return chain;
    });
    optionsMock.listOptionContracts.mockResolvedValue([contract]);

    renderWithQuery(<OptionsPage />);
    await screen.findByText("No option chains");

    fireEvent.change(screen.getByLabelText("Symbol"), { target: { value: " aapl " } });
    fireEvent.click(screen.getByRole("button", { name: "Fetch demo chain" }));

    await waitFor(() =>
      expect(optionsMock.fetchOptionChain).toHaveBeenCalledWith({
        workspaceId,
        symbol: "AAPL",
        provider: "demo",
      }),
    );
    expect(await screen.findByText("Option contracts")).toBeInTheDocument();
    expect(await screen.findByText("150.00")).toBeInTheDocument();
    expect(optionsMock.listOptionContracts).toHaveBeenCalledWith(chainId);
    const selectedChain = await screen.findByRole("button", { name: "Select option chain AAPL" });
    await waitFor(() => expect(optionsMock.listOptionChains).toHaveBeenCalledTimes(2));
    expect(selectedChain).toHaveAttribute("aria-pressed", "true");
  });

  it("allows retrying a failed chain list request", async () => {
    optionsMock.listOptionChains
      .mockRejectedValueOnce(new Error("offline"))
      .mockResolvedValueOnce([chain]);

    renderWithQuery(<OptionChainList workspaceId={workspaceId} />);
    await screen.findByRole("alert");
    fireEvent.click(screen.getByRole("button", { name: "Try Again" }));

    expect(await screen.findByText("AAPL")).toBeInTheDocument();
    expect(optionsMock.listOptionChains).toHaveBeenCalledTimes(2);
  });

  it("allows retrying a failed demo fetch", async () => {
    let attempts = 0;
    optionsMock.fetchOptionChain.mockImplementation(async () => {
      attempts += 1;
      if (attempts === 1) throw new Error("offline");
      return chain;
    });

    renderWithQuery(<OptionsPage />);
    await screen.findByText("No option chains");
    fireEvent.change(screen.getByLabelText("Symbol"), { target: { value: "AAPL" } });
    fireEvent.click(screen.getByRole("button", { name: "Fetch demo chain" }));
    await screen.findByText("Chain fetch failed");
    fireEvent.click(screen.getByRole("button", { name: "Try Again" }));

    expect(await screen.findByText("Option contracts")).toBeInTheDocument();
    expect(attempts).toBe(2);
  });

  it("allows retrying a failed contract request", async () => {
    optionsMock.listOptionContracts
      .mockRejectedValueOnce(new Error("offline"))
      .mockResolvedValueOnce([contract]);

    renderWithQuery(<OptionContractTable chainId={chainId} />);
    await screen.findByRole("alert");
    fireEvent.click(screen.getByRole("button", { name: "Try Again" }));

    expect(await screen.findByText("150.00")).toBeInTheDocument();
    expect(optionsMock.listOptionContracts).toHaveBeenCalledTimes(2);
  });

  it("creates and reloads a strategy from controlled contract selections", async () => {
    let strategies: (typeof strategy)[] = [];
    optionsMock.listOptionChains.mockResolvedValue([chain]);
    optionsMock.listOptionContracts.mockResolvedValue([contract]);
    optionsMock.listOptionStrategies.mockImplementation(async () => strategies);
    optionsMock.createOptionStrategy.mockImplementation(async () => {
      strategies = [strategy];
      return strategy;
    });

    renderWithQuery(<OptionsPage />);
    fireEvent.click(await screen.findByRole("button", { name: "Select option chain AAPL" }));
    const contractSelection = await screen.findByRole("checkbox", {
      name: "Select contract AAPL 150.00 call",
    });
    fireEvent.click(contractSelection);
    fireEvent.click(screen.getByRole("button", { name: "Create option strategy" }));
    fireEvent.change(screen.getByLabelText("Strategy name"), {
      target: { value: " Call spread " },
    });
    fireEvent.change(screen.getByLabelText("Quantity 150.00"), { target: { value: "0" } });
    expect(screen.getByRole("button", { name: "Save strategy" })).toBeDisabled();
    fireEvent.change(screen.getByLabelText("Quantity 150.00"), { target: { value: "2" } });
    fireEvent.change(screen.getByLabelText("Direction 150.00"), {
      target: { value: "short" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save strategy" }));

    await waitFor(() =>
      expect(optionsMock.createOptionStrategy.mock.calls[0]?.[0]).toEqual({
        workspaceId,
        name: "Call spread",
        strategyType: "custom",
        legs: [{ contractId, quantity: 2, positionType: "short" }],
      }),
    );
    expect(await screen.findByText("Call spread")).toBeInTheDocument();
    await waitFor(() => expect(optionsMock.listOptionStrategies).toHaveBeenCalledTimes(2));
    expect(contractSelection).not.toBeChecked();
  });

  it("keeps strategy input available after a create error so the user can retry", async () => {
    optionsMock.listOptionChains.mockResolvedValue([chain]);
    optionsMock.listOptionContracts.mockResolvedValue([contract]);
    optionsMock.createOptionStrategy
      .mockRejectedValueOnce(new Error("offline"))
      .mockResolvedValueOnce(strategy);

    renderWithQuery(<OptionsPage />);
    fireEvent.click(await screen.findByRole("button", { name: "Select option chain AAPL" }));
    fireEvent.click(
      await screen.findByRole("checkbox", { name: "Select contract AAPL 150.00 call" }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Create option strategy" }));
    fireEvent.change(screen.getByLabelText("Strategy name"), { target: { value: "Retry" } });
    fireEvent.click(screen.getByRole("button", { name: "Save strategy" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Unable to create");
    fireEvent.click(screen.getByRole("button", { name: "Save strategy" }));
    await waitFor(() => expect(optionsMock.createOptionStrategy).toHaveBeenCalledTimes(2));
  });

  it("deletes a persisted strategy and refreshes the empty state", async () => {
    let strategies = [strategy];
    optionsMock.listOptionStrategies.mockImplementation(async () => strategies);
    optionsMock.deleteOptionStrategy.mockImplementation(async () => {
      strategies = [];
    });
    vi.spyOn(window, "confirm").mockReturnValue(true);

    renderWithQuery(<OptionsPage />);
    fireEvent.click(
      await screen.findByRole("button", { name: "Delete option strategy: Call spread" }),
    );

    await waitFor(() =>
      expect(optionsMock.deleteOptionStrategy.mock.calls[0]?.[0]).toBe(strategyId),
    );
    expect(await screen.findByText("No option strategies")).toBeInTheDocument();
  });
});
