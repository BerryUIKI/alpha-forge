import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { CreateAccountDialog } from "./CreateAccountDialog";
import { AddAssetDialog } from "./AddAssetDialog";
import { AddActivityDialog } from "./AddActivityDialog";
import { LocaleContext } from "@/lib/i18n/locale-context";
import type { Locale } from "@/lib/i18n/locale";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock desktopApi financial
const financialMock = vi.hoisted(() => ({
  createFinancialAccount: vi.fn(),
  createAsset: vi.fn(),
  createActivity: vi.fn(),
  createLot: vi.fn(),
  recordSell: vi.fn(),
  listActiveAssets: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: financialMock,
  },
}));

// Mock hooks
vi.mock("@/lib/hooks", () => ({
  useFocusTrap: () => ({ current: null }),
  useEscapeKey: vi.fn(),
}));

function t(key: string) {
  const messages: Record<string, string> = {
    createAccountTitle: "Create Account",
    accountNameLabel: "Account name",
    accountNamePlaceholder: "Brokerage",
    accountNameRequired: "Account name is required.",
    accountTypeLabel: "Account type",
    accountTypeSecurities: "Securities",
    accountTypeCash: "Cash",
    accountTypeCreditCard: "Credit Card",
    accountTypeCryptocurrency: "Cryptocurrency",
    accountGroupNameLabel: "Group name",
    accountGroupNamePlaceholder: "Investments",
    accountCurrencyLabel: "Currency",
    accountNumberLabel: "Account number",
    accountNumberPlaceholder: "12345678",
    trackingModeLabel: "Tracking mode",
    trackingModeTransactions: "Transactions",
    trackingModeHoldings: "Holdings",
    addAssetTitle: "Add Asset",
    assetKindLabel: "Asset kind",
    assetNameLabel: "Asset name",
    assetNamePlaceholder: "Apple Inc.",
    assetNameRequired: "Asset name is required.",
    displayCodeLabel: "Display code",
    displayCodePlaceholder: "AAPL",
    instrumentTypeLabel: "Instrument type",
    instrumentTypeEquity: "Equity",
    instrumentSymbolLabel: "Symbol",
    instrumentSymbolPlaceholder: "AAPL",
    exchangeMicLabel: "Exchange MIC",
    exchangeMicPlaceholder: "XNAS",
    quoteModeLabel: "Quote mode",
    quoteModeMarket: "Market",
    quoteModeManual: "Manual",
    quoteCurrencyLabel: "Quote currency",
    addActivityTitle: "Add Activity",
    activityTypeLabel: "Activity type",
    activityTypeBuy: "Buy",
    activityTypeSell: "Sell",
    activityTypeDividend: "Dividend",
    activityDateLabel: "Date",
    unitPriceLabel: "Unit price",
    unitPricePlaceholder: "150.00",
    amountLabel: "Amount",
    amountPlaceholder: "3000.00",
    feeLabel: "Fee",
    taxLabel: "Tax",
    quantityLabel: "Quantity",
    settlementDateLabel: "Settlement date",
    statusLabel: "Status",
    statusPosted: "Posted",
    statusPending: "Pending",
    statusCanceled: "Canceled",
    notesLabel: "Notes",
    notesPlaceholder: "Optional notes…",
    cancel: "Cancel",
    create: "Create",
    creating: "Creating…",
    unableToCreateAccount: "Unable to create account.",
    unableToCreateAsset: "Unable to create asset.",
    unableToCreateActivity: "Unable to create activity.",
    assetRequired: "Asset is required for buy/sell activities.",
    quantityRequired: "Quantity is required for buy/sell activities.",
    priceRequired: "Unit price is required for buy/sell activities.",
  };
  return messages[key] || key;
}

function renderWithProviders(ui: React.ReactElement, locale: Locale = "en") {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const setLocale = vi.fn();
  return render(
    <LocaleContext.Provider value={{ locale, setLocale, t }}>
      <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("CreateAccountDialog", () => {
  const defaultProps = {
    isOpen: true,
    onClose: vi.fn(),
    defaultCurrency: "USD",
    onSuccess: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders when open", () => {
    renderWithProviders(<CreateAccountDialog {...defaultProps} />);
    expect(screen.getByText("Create Account")).toBeInTheDocument();
    expect(screen.getByLabelText("Account name")).toBeInTheDocument();
    expect(screen.getByLabelText("Account type")).toBeInTheDocument();
    expect(screen.getByLabelText("Currency")).toBeInTheDocument();
    expect(screen.getByLabelText("Tracking mode")).toBeInTheDocument();
  });

  it("does not render when closed", () => {
    renderWithProviders(<CreateAccountDialog {...defaultProps} isOpen={false} />);
    expect(screen.queryByText("Create Account")).not.toBeInTheDocument();
  });

  it("shows validation error when name is empty", async () => {
    renderWithProviders(<CreateAccountDialog {...defaultProps} />);
    fireEvent.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => {
      expect(screen.getByText("Account name is required.")).toBeInTheDocument();
    });
  });

  it("submits form and calls onSuccess", async () => {
    financialMock.createFinancialAccount.mockResolvedValue({ id: "account-1" });
    renderWithProviders(<CreateAccountDialog {...defaultProps} />);

    fireEvent.change(screen.getByLabelText("Account name"), {
      target: { value: "My Brokerage" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));

    await waitFor(() => {
      expect(financialMock.createFinancialAccount).toHaveBeenCalledWith({
        workspace_id: null,
        name: "My Brokerage",
        account_type: "securities",
        group_name: null,
        currency: "USD",
        is_default: false,
        platform_id: null,
        account_number: null,
        tracking_mode: "transactions",
      });
    });
    expect(defaultProps.onSuccess).toHaveBeenCalledWith("account-1");
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("shows error on mutation failure", async () => {
    financialMock.createFinancialAccount.mockRejectedValue(new Error("DB error"));
    renderWithProviders(<CreateAccountDialog {...defaultProps} />);

    fireEvent.change(screen.getByLabelText("Account name"), {
      target: { value: "My Brokerage" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));

    await waitFor(() => {
      expect(screen.getByText("DB error")).toBeInTheDocument();
    });
  });

  it("calls onClose on backdrop click", () => {
    renderWithProviders(<CreateAccountDialog {...defaultProps} />);
    const dialog = screen.getByRole("dialog");
    fireEvent.click(dialog);
    expect(defaultProps.onClose).toHaveBeenCalled();
  });
});

describe("AddAssetDialog", () => {
  const defaultProps = {
    isOpen: true,
    onClose: vi.fn(),
    onSuccess: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders when open", () => {
    renderWithProviders(<AddAssetDialog {...defaultProps} />);
    expect(screen.getByText("Add Asset")).toBeInTheDocument();
    expect(screen.getByLabelText("Asset name")).toBeInTheDocument();
    expect(screen.getByLabelText("Asset kind")).toBeInTheDocument();
    expect(screen.getByLabelText("Instrument type")).toBeInTheDocument();
  });

  it("does not render when closed", () => {
    renderWithProviders(<AddAssetDialog {...defaultProps} isOpen={false} />);
    expect(screen.queryByText("Add Asset")).not.toBeInTheDocument();
  });

  it("shows validation error when name is empty", async () => {
    renderWithProviders(<AddAssetDialog {...defaultProps} />);
    fireEvent.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => {
      expect(screen.getByText("Asset name is required.")).toBeInTheDocument();
    });
  });

  it("submits form and calls onSuccess", async () => {
    financialMock.createAsset.mockResolvedValue({ id: "asset-1" });
    renderWithProviders(<AddAssetDialog {...defaultProps} />);

    fireEvent.change(screen.getByLabelText("Asset name"), {
      target: { value: "Apple Inc." },
    });
    fireEvent.change(screen.getByLabelText("Display code"), {
      target: { value: "AAPL" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));

    await waitFor(() => {
      expect(financialMock.createAsset).toHaveBeenCalled();
    });
    expect(defaultProps.onSuccess).toHaveBeenCalledWith("asset-1");
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("shows error on mutation failure", async () => {
    financialMock.createAsset.mockRejectedValue(new Error("Duplicate key"));
    renderWithProviders(<AddAssetDialog {...defaultProps} />);

    fireEvent.change(screen.getByLabelText("Asset name"), {
      target: { value: "Apple Inc." },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));

    await waitFor(() => {
      expect(screen.getByText("Duplicate key")).toBeInTheDocument();
    });
  });
});

describe("AddActivityDialog", () => {
  const defaultProps = {
    isOpen: true,
    onClose: vi.fn(),
    accountId: "account-1",
    accountCurrency: "USD",
    onSuccess: vi.fn(),
  };

  /** Wait until the listActiveAssets query populates the asset <select>. */
  async function assetOptionsLoaded() {
    await screen.findByLabelText("Asset name");
    await waitFor(() => {
      expect(screen.getByLabelText("Asset name").querySelectorAll("option")).toHaveLength(3);
    });
  }

  beforeEach(() => {
    vi.clearAllMocks();
    financialMock.listActiveAssets.mockResolvedValue([
      { id: "asset-1", display_code: "AAPL", name: "Apple Inc." },
      { id: "asset-2", display_code: "MSFT", name: "Microsoft Corp." },
    ]);
  });

  it("renders when open", () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    expect(screen.getByText("Add Activity")).toBeInTheDocument();
    expect(screen.getByLabelText("Activity type")).toBeInTheDocument();
    expect(screen.getByLabelText("Date")).toBeInTheDocument();
  });

  it("does not render when closed", () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} isOpen={false} />);
    expect(screen.queryByText("Add Activity")).not.toBeInTheDocument();
  });

  it("shows asset selector for buy activity", async () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    const select = screen.getByLabelText("Asset name");
    const options = Array.from(select.querySelectorAll("option"));
    expect(options[1]?.textContent).toContain("AAPL");
    expect(options[2]?.textContent).toContain("MSFT");
  });

  it("shows quantity and price fields for buy activity", async () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    expect(screen.getByLabelText("Quantity")).toBeInTheDocument();
    expect(screen.getByLabelText("Unit price")).toBeInTheDocument();
  });

  it("hides asset fields for non-buy/sell types", async () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    fireEvent.change(screen.getByLabelText("Activity type"), {
      target: { value: "dividend" },
    });
    await waitFor(() => {
      expect(screen.queryByLabelText("Asset name")).not.toBeInTheDocument();
    });
  });

  it("shows validation error when asset is not selected for buy", async () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    fireEvent.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => {
      expect(
        screen.getByText("Asset is required for buy/sell activities."),
      ).toBeInTheDocument();
    });
  });

  it("shows validation error when quantity is empty for buy", async () => {
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    fireEvent.change(screen.getByLabelText("Asset name"), {
      target: { value: "asset-1" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => {
      expect(
        screen.getByText("Quantity is required for buy/sell activities."),
      ).toBeInTheDocument();
    });
  });

  it("submits form and calls onSuccess for buy activity", async () => {
    financialMock.createActivity.mockResolvedValue({ id: "activity-1" });
    financialMock.createLot.mockResolvedValue({ id: "lot-1" });
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    fireEvent.change(screen.getByLabelText("Asset name"), {
      target: { value: "asset-1" },
    });
    fireEvent.change(screen.getByLabelText("Quantity"), {
      target: { value: "10" },
    });
    fireEvent.change(screen.getByLabelText("Unit price"), {
      target: { value: "150.00" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => {
      expect(financialMock.createActivity).toHaveBeenCalled();
    });
    expect(defaultProps.onSuccess).toHaveBeenCalledWith("activity-1");
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("shows error on mutation failure", async () => {
    financialMock.createActivity.mockRejectedValue(new Error("Invalid data"));
    renderWithProviders(<AddActivityDialog {...defaultProps} />);
    await assetOptionsLoaded();
    fireEvent.change(screen.getByLabelText("Asset name"), {
      target: { value: "asset-1" },
    });
    fireEvent.change(screen.getByLabelText("Quantity"), {
      target: { value: "10" },
    });
    fireEvent.change(screen.getByLabelText("Unit price"), {
      target: { value: "150.00" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create" }));
    await waitFor(() => {
      expect(screen.getByText("Invalid data")).toBeInTheDocument();
    });
  });
});