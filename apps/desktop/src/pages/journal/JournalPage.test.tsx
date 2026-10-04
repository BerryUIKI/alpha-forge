import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate } from "@/lib/i18n/locale";
import { JournalPage } from "./JournalPage";

const listThesesMock = vi.fn();
const listProjectsMock = vi.fn();
const listTasksMock = vi.fn();

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    thesis: {
      listTheses: () => listThesesMock(),
    },
    research: {
      listResearchProjects: () => listProjectsMock(),
    },
    agent: {
      listAgentTasks: () => listTasksMock(),
    },
  },
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspaceId: () => "ws-journal-1",
}));

function renderWithProviders(ui: React.ReactElement) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <LocaleContext.Provider
      value={{ locale: "en", setLocale: async () => undefined, t: (k) => translate("en", k) }}
    >
      <QueryClientProvider client={queryClient}>
        <MemoryRouter>{ui}</MemoryRouter>
      </QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("JournalPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders empty state when there are no items", async () => {
    listThesesMock.mockResolvedValue([]);
    listProjectsMock.mockResolvedValue([]);
    listTasksMock.mockResolvedValue([]);

    renderWithProviders(<JournalPage />);

    expect(await screen.findByText("Investment & Decision Journal")).toBeInTheDocument();
    expect(await screen.findByText("No journal entries yet")).toBeInTheDocument();
  });

  it("renders chronological timeline and filters by category", async () => {
    listThesesMock.mockResolvedValue([
      {
        id: "th-1",
        title: "Semiconductor Fab Expansion Thesis",
        hypothesis: "Capex expansion outpaces consensus",
        status: "active",
        confidence: 85,
        validationOutcome: "Confirmed by supply chain data",
        createdAt: "2026-10-01T10:00:00Z",
      },
    ]);
    listProjectsMock.mockResolvedValue([
      {
        id: "proj-1",
        title: "AI Server Infrastructure Analysis",
        description: "Comparative study across top server providers",
        createdAt: "2026-10-02T12:00:00Z",
      },
    ]);
    listTasksMock.mockResolvedValue([
      {
        id: "task-1",
        prompt: "Extract GPU delivery lead times",
        status: "completed",
        createdAt: "2026-10-03T08:00:00Z",
        completedAt: "2026-10-03T08:05:00Z",
      },
    ]);

    renderWithProviders(<JournalPage />);

    // All entries visible initially
    expect(await screen.findByText("Semiconductor Fab Expansion Thesis")).toBeInTheDocument();
    expect(await screen.findByText("AI Server Infrastructure Analysis")).toBeInTheDocument();
    expect(await screen.findByText("Extract GPU delivery lead times")).toBeInTheDocument();
    expect(screen.getByText("Confirmed by supply chain data")).toBeInTheDocument();

    // Filter by Theses only
    const thesesFilter = screen.getByRole("button", { name: /Theses & Decisions/ });
    fireEvent.click(thesesFilter);

    expect(screen.getByText("Semiconductor Fab Expansion Thesis")).toBeInTheDocument();
    expect(screen.queryByText("AI Server Infrastructure Analysis")).not.toBeInTheDocument();
    expect(screen.queryByText("Extract GPU delivery lead times")).not.toBeInTheDocument();

    // Filter by Research only
    const researchFilter = screen.getByRole("button", { name: /Research Projects/ });
    fireEvent.click(researchFilter);

    expect(screen.getByText("AI Server Infrastructure Analysis")).toBeInTheDocument();
    expect(screen.queryByText("Semiconductor Fab Expansion Thesis")).not.toBeInTheDocument();

    // Filter by Tasks only
    const tasksFilter = screen.getByRole("button", { name: /Agent Tasks/ });
    fireEvent.click(tasksFilter);

    expect(screen.getByText("Extract GPU delivery lead times")).toBeInTheDocument();
    expect(screen.queryByText("AI Server Infrastructure Analysis")).not.toBeInTheDocument();
  });
});
