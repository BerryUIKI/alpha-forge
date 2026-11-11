import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ImportActivitiesDialog } from "./ImportActivitiesDialog";
import { LocaleContext } from "@/lib/i18n/locale-context";
import type { Locale } from "@/lib/i18n/locale";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock desktopApi financial
const financialMock = vi.hoisted(() => ({
  previewActivitiesCsv: vi.fn(),
  importActivitiesCsv: vi.fn(),
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
    importActivitiesTitle: "Import Activities",
    importActivitiesDescription: "Import broker statement CSVs or generic transactions",
  };
  return messages[key] || key;
}

function renderWithProviders(ui: React.ReactElement, locale: Locale = "en") {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  const setLocale = vi.fn();
  return render(
    <LocaleContext.Provider value={{ locale, setLocale, t }}>
      <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

const previewResult = {
  format: "GENERIC",
  total_rows: 2,
  new_count: 1,
  duplicate_count: 1,
  invalid_count: 0,
  rows: [
    {
      row_number: 1,
      activity_date: "2026-08-01",
      activity_type: "BUY",
      symbol: "NVDA",
      quantity: "10",
      amount: "1200",
      outcome: "new",
      message: null,
    },
    {
      row_number: 2,
      activity_date: "2026-08-01",
      activity_type: "SELL",
      symbol: "NVDA",
      quantity: "4",
      amount: "480",
      outcome: "duplicate",
      message: "already imported",
    },
  ],
};

describe("ImportActivitiesDialog", () => {
  const defaultProps = {
    isOpen: true,
    onClose: vi.fn(),
    accountId: "acc-123",
    onSuccess: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
    financialMock.previewActivitiesCsv.mockResolvedValue(previewResult);
  });

  it("does not render when closed", () => {
    renderWithProviders(<ImportActivitiesDialog {...defaultProps} isOpen={false} />);
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("renders statement options and inputs when open", () => {
    renderWithProviders(<ImportActivitiesDialog {...defaultProps} />);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
    expect(screen.getByText("Import Activities")).toBeInTheDocument();
    expect(screen.getByText("Statement Format")).toBeInTheDocument();
    expect(screen.getByRole("combobox")).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/date,type,symbol/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Preview Statement" })).toBeInTheDocument();
  });

  it("disables preview when csv text is empty and enables when entered", () => {
    renderWithProviders(<ImportActivitiesDialog {...defaultProps} />);
    const previewBtn = screen.getByRole("button", { name: "Preview Statement" });
    expect(previewBtn).toBeDisabled();

    const textarea = screen.getByPlaceholderText(/date,type,symbol/);
    fireEvent.change(textarea, { target: { value: "date,type\n2026-08-01,BUY" } });
    expect(previewBtn).toBeEnabled();
  });

  it("previews rows without writing anything, then applies the import", async () => {
    const mockRun = {
      id: "run-001",
      account_id: "acc-123",
      source_system: "GENERIC_CSV",
      run_type: "ACTIVITIES",
      mode: "APPEND",
      status: "COMPLETED",
      started_at: "2026-08-01T10:00:00Z",
      finished_at: "2026-08-01T10:00:02Z",
      review_mode: "AUTOMATIC",
      applied_at: "2026-08-01T10:00:02Z",
      checkpoint_in: null,
      checkpoint_out: null,
      summary: JSON.stringify({
        total_rows: 2,
        created_count: 1,
        skipped_count: 1,
        failed_count: 0,
        errors: [],
      }),
      warnings: null,
      error: null,
      created_at: "2026-08-01T10:00:00Z",
      updated_at: "2026-08-01T10:00:02Z",
    };
    financialMock.importActivitiesCsv.mockResolvedValueOnce(mockRun);

    renderWithProviders(<ImportActivitiesDialog {...defaultProps} />);

    const textarea = screen.getByPlaceholderText(/date,type,symbol/);
    fireEvent.change(textarea, {
      target: {
        value:
          "date,type,symbol,quantity,price,amount,currency\n2026-08-01,BUY,NVDA,10,120,1200,USD",
      },
    });

    fireEvent.click(screen.getByRole("button", { name: "Preview Statement" }));

    await waitFor(() => {
      expect(screen.getByText("Apply import (1)")).toBeInTheDocument();
    });
    expect(financialMock.importActivitiesCsv).not.toHaveBeenCalled();
    expect(screen.getByText("duplicate")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Apply import (1)" }));

    await waitFor(() => {
      expect(screen.getByText("Statement imported successfully!")).toBeInTheDocument();
    });

    expect(financialMock.importActivitiesCsv).toHaveBeenCalledWith(
      "acc-123",
      "GENERIC",
      expect.stringContaining("NVDA"),
    );
    expect(defaultProps.onSuccess).toHaveBeenCalled();
    expect(screen.getByText("Total Rows")).toBeInTheDocument();
    expect(screen.getByText("Created")).toBeInTheDocument();
    expect(defaultProps.onClose).not.toHaveBeenCalled();

    fireEvent.click(screen.getByRole("button", { name: "Done" }));
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("returns to the input view from the preview without importing", async () => {
    renderWithProviders(<ImportActivitiesDialog {...defaultProps} />);

    fireEvent.change(screen.getByPlaceholderText(/date,type,symbol/), {
      target: { value: "date,type\n2026-08-01,BUY" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Preview Statement" }));

    await screen.findByText("Apply import (1)");
    fireEvent.click(screen.getByRole("button", { name: "Back" }));

    expect(screen.getByRole("button", { name: "Preview Statement" })).toBeInTheDocument();
    expect(financialMock.importActivitiesCsv).not.toHaveBeenCalled();
  });

  it("displays partial failure errors and allows importing another statement", async () => {
    const mockFailedRun = {
      id: "run-002",
      account_id: "acc-123",
      source_system: "GENERIC_CSV",
      run_type: "ACTIVITIES",
      mode: "APPEND",
      status: "FAILED",
      started_at: "2026-08-01T10:00:00Z",
      finished_at: "2026-08-01T10:00:01Z",
      review_mode: "AUTOMATIC",
      applied_at: null,
      checkpoint_in: null,
      checkpoint_out: null,
      summary: JSON.stringify({
        total_rows: 1,
        created_count: 0,
        skipped_count: 0,
        failed_count: 1,
        errors: ["Row 1: Insufficient lots to sell 10 shares"],
      }),
      warnings: null,
      error: "Row 1 failed",
      created_at: "2026-08-01T10:00:00Z",
      updated_at: "2026-08-01T10:00:01Z",
    };

    financialMock.importActivitiesCsv.mockResolvedValueOnce(mockFailedRun);

    renderWithProviders(<ImportActivitiesDialog {...defaultProps} />);

    fireEvent.change(screen.getByPlaceholderText(/date,type,symbol/), {
      target: {
        value:
          "date,type,symbol,quantity,price,amount,currency\n2026-08-01,SELL,AAPL,10,150,1500,USD",
      },
    });

    fireEvent.click(screen.getByRole("button", { name: "Preview Statement" }));
    fireEvent.click(await screen.findByRole("button", { name: "Apply import (1)" }));

    await waitFor(() => {
      expect(screen.getByText("Import completed with errors.")).toBeInTheDocument();
    });

    expect(screen.getByText("Row 1: Insufficient lots to sell 10 shares")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Import Another" }));
    expect(screen.getByRole("button", { name: "Preview Statement" })).toBeInTheDocument();
    expect(screen.queryByText("Import completed with errors.")).not.toBeInTheDocument();
  });
});
