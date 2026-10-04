import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate } from "@/lib/i18n/locale";
import { OptionContractTable } from "./OptionContractTable";

const deleteMock = vi.fn();
const listMock = vi.fn();

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    options: {
      listOptionContracts: () => listMock(),
      deleteOptionContract: (id: string) => deleteMock(id),
    },
  },
}));

function renderWithProviders(ui: React.ReactElement) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <LocaleContext.Provider
      value={{ locale: "en", setLocale: async () => undefined, t: (k) => translate("en", k) }}
    >
      <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("OptionContractTable", () => {
  const contract = {
    id: "contract-1",
    workspaceId: "ws-1",
    chainId: "chain-1",
    symbol: "AAPL",
    optionType: "call" as const,
    strike: 150,
    expiration: new Date().toISOString(),
    contractMultiplier: 100,
    bid: 4,
    ask: 5,
    last: 4.5,
    volume: 10,
    openInterest: 20,
    impliedVolatility: 0.25,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
    listMock.mockResolvedValue([contract]);
    deleteMock.mockResolvedValue(undefined);
  });

  it("renders contracts and confirms deletion via in-app ConfirmDialog", async () => {
    renderWithProviders(
      <OptionContractTable chainId="chain-1" showDelete={true} />,
    );

    expect(await screen.findByText("150.00")).toBeInTheDocument();

    const trashBtn = screen.getByRole("button", { name: "Delete this contract?" });
    fireEvent.click(trashBtn);

    // Dialog opens
    expect(await screen.findByRole("alertdialog")).toBeInTheDocument();
    expect(screen.getAllByText("Delete this contract?").length).toBeGreaterThan(0);

    // Confirm deletion
    const confirmBtn = screen.getByRole("button", { name: "Delete" });
    fireEvent.click(confirmBtn);

    await waitFor(() => {
      expect(deleteMock).toHaveBeenCalledWith("contract-1");
    });
  });
});
