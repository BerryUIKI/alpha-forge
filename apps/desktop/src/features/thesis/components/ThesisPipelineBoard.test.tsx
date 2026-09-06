import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThesisPipelineBoard } from "./ThesisPipelineBoard";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";
import * as thesisHooks from "../hooks/useTheses";

// Mock useTheses & mutation hooks
vi.mock("../hooks/useTheses", () => ({
  useTheses: vi.fn(),
  useThesisEvidence: vi.fn(() => ({ data: [] })),
  useActivateThesis: vi.fn(() => ({ mutateAsync: vi.fn() })),
  useStartThesisValidation: vi.fn(() => ({ mutateAsync: vi.fn() })),
  useCompleteThesisValidation: vi.fn(() => ({ mutateAsync: vi.fn() })),
}));

const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

function renderWithProviders(ui: React.ReactElement) {
  return render(
    <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>
  );
}

const mockTheses: InvestmentThesis[] = [
  {
    id: "thesis-1",
    workspaceId: "ws-1",
    title: "Blackwell Rack Ramp",
    thesis: "Volume shipments to exceed consensus by 20%",
    confidence: 88,
    status: "active",
    validationDate: null,
    outcome: null,
    createdAt: "2026-09-01T00:00:00Z",
    updatedAt: "2026-09-01T00:00:00Z",
  },
  {
    id: "thesis-2",
    workspaceId: "ws-1",
    title: "High-NA EUV Cycle",
    thesis: "Order inflection driven by 2nm pilot runs",
    confidence: 65,
    status: "validating",
    validationDate: null,
    outcome: null,
    createdAt: "2026-09-02T00:00:00Z",
    updatedAt: "2026-09-02T00:00:00Z",
  },
];

describe("ThesisPipelineBoard", () => {
  it("renders pipeline board columns and thesis cards", () => {
    vi.mocked(thesisHooks.useTheses).mockReturnValue({
      data: mockTheses,
      isLoading: false,
      error: null,
      refetch: vi.fn(),
    } as any);

    const handleSelect = vi.fn();
    renderWithProviders(
      <ThesisPipelineBoard
        workspaceId="ws-1"
        selectedId={undefined}
        onSelect={handleSelect}
      />
    );

    expect(screen.getByText("Blackwell Rack Ramp")).toBeInTheDocument();
    expect(screen.getByText("High-NA EUV Cycle")).toBeInTheDocument();
    expect(screen.getByText("88%")).toBeInTheDocument();
    expect(screen.getByText("65%")).toBeInTheDocument();
  });

  it("triggers onSelect when clicking a card", () => {
    vi.mocked(thesisHooks.useTheses).mockReturnValue({
      data: mockTheses,
      isLoading: false,
      error: null,
      refetch: vi.fn(),
    } as any);

    const handleSelect = vi.fn();
    renderWithProviders(
      <ThesisPipelineBoard
        workspaceId="ws-1"
        selectedId={undefined}
        onSelect={handleSelect}
      />
    );

    const card = screen.getByText("Blackwell Rack Ramp");
    fireEvent.click(card);
    expect(handleSelect).toHaveBeenCalledWith(mockTheses[0]);
  });
});
