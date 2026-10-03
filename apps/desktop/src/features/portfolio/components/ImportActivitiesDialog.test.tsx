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

describe("ImportActivitiesDialog", () => {
  const defaultProps = {
    isOpen: true,
    onClose: vi.fn(),
    accountId: "acc-123",
    onSuccess: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
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
    expect(screen.getByRole("button", { name: "Import Statement" })).toBeInTheDocument();
  });

  it("disables import button when csv text is empty and enables when entered", () => {
    renderWithProviders(<ImportActivitiesDialog {...defaultProps} />);
    const importBtn = screen.getByRole("button", { name: "Import Statement" });
    expect(importBtn).toBeDisabled();

    const textarea = screen.getByPlaceholderText(/date,type,symbol/);
    fireEvent.change(textarea, { target: { value: "date,type\n2026-08-01,BUY" } });
    expect(importBtn).toBeEnabled();
  });

  it("successfully imports CSV, displays audit summary metrics, and does not auto-close", async () => {
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
        total_rows: 5,
        created_count: 4,
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
      target: { value: "date,type,symbol,quantity,price,amount,currency\n2026-08-01,BUY,NVDA,10,120,1200,USD" },
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Statement" }));

    await waitFor(() => {
      expect(screen.getByText("Statement imported successfully!")).toBeInTheDocument();
    });

    expect(financialMock.importActivitiesCsv).toHaveBeenCalledWith(
      "acc-123",
      "GENERIC",
      expect.stringContaining("NVDA"),
    );
    expect(defaultProps.onSuccess).toHaveBeenCalled();

    // Verify summary metrics cards
    expect(screen.getByText("Total Rows")).toBeInTheDocument();
    expect(screen.getByText("5")).toBeInTheDocument();
    expect(screen.getByText("Created")).toBeInTheDocument();
    expect(screen.getByText("4")).toBeInTheDocument();
    expect(screen.getByText("Skipped")).toBeInTheDocument();
    expect(screen.getByText("1")).toBeInTheDocument();

    // Dialog stays open for review
    expect(defaultProps.onClose).not.toHaveBeenCalled();

    // User explicitly clicks Done to dismiss
    fireEvent.click(screen.getByRole("button", { name: "Done" }));
    expect(defaultProps.onClose).toHaveBeenCalled();
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

    const textarea = screen.getByPlaceholderText(/date,type,symbol/);
    fireEvent.change(textarea, {
      target: { value: "date,type,symbol,quantity,price,amount,currency\n2026-08-01,SELL,AAPL,10,150,1500,USD" },
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Statement" }));

    await waitFor(() => {
      expect(screen.getByText("Import completed with errors.")).toBeInTheDocument();
    });

    expect(screen.getByText("Row 1: Insufficient lots to sell 10 shares")).toBeInTheDocument();

    // Click Import Another
    fireEvent.click(screen.getByRole("button", { name: "Import Another" }));
    expect(screen.getByRole("button", { name: "Import Statement" })).toBeInTheDocument();
    expect(screen.queryByText("Import completed with errors.")).not.toBeInTheDocument();
  });
});
