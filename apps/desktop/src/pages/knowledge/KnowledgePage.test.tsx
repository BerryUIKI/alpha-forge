import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { MemoryRouter } from "react-router-dom";
import { KnowledgePage } from "./KnowledgePage";
import { desktopApi } from "@/lib/desktop-api";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    knowledgeGraph: {
      listKnowledgeEntities: vi.fn(),
      createKnowledgeEntity: vi.fn(),
    },
  },
}));

vi.mock("@/features/workspace/hooks/useWorkspaces", () => ({
  useWorkspaces: () => ({
    data: [{ id: "ws-1", name: "Test Workspace" }],
    isLoading: false,
  }),
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace", () => ({
  useActiveWorkspaceId: () => "ws-1",
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        knowledgeGraph: "Knowledge Graph",
        knowledgeGraphDescription: "Connect companies, industries, technologies, and macro themes.",
      };
      return map[key] || key;
    },
  }),
}));

function renderPage() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <KnowledgePage />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("KnowledgePage", () => {
  it("shows loading state while fetching entities", () => {
    vi.mocked(desktopApi.knowledgeGraph.listKnowledgeEntities).mockReturnValue(
      new Promise(() => {}),
    );
    const { container } = renderPage();
    expect(container.querySelector(".animate-spin")).toBeTruthy();
  });

  it("shows empty state when no entities exist", async () => {
    vi.mocked(desktopApi.knowledgeGraph.listKnowledgeEntities).mockResolvedValue([]);
    renderPage();
    expect(
      await screen.findByText(/Start building your knowledge network/),
    ).toBeInTheDocument();
  });

  it("shows error state when API fails", async () => {
    vi.mocked(desktopApi.knowledgeGraph.listKnowledgeEntities).mockRejectedValue(
      new Error("API error"),
    );
    renderPage();
    expect(
      await screen.findByText("Failed to load knowledge entities"),
    ).toBeInTheDocument();
  });

  it("renders entity cards when data exists", async () => {
    vi.mocked(desktopApi.knowledgeGraph.listKnowledgeEntities).mockResolvedValue([
      {
        id: "e1",
        workspace_id: "ws-1",
        entity_type: "company",
        name: "NVIDIA",
        description: "AI chip company",
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
      },
      {
        id: "e2",
        workspace_id: "ws-1",
        entity_type: "technology",
        name: "CUDA",
        description: "Parallel computing platform",
        created_at: "2026-01-02T00:00:00Z",
        updated_at: "2026-01-02T00:00:00Z",
      },
    ]);
    renderPage();
    expect(await screen.findByText("NVIDIA")).toBeInTheDocument();
    expect(screen.getByText("CUDA")).toBeInTheDocument();
    expect(screen.getByText("Company")).toBeInTheDocument();
    expect(screen.getByText("Technology")).toBeInTheDocument();
  });

  it("toggles create form when Add Entity is clicked", async () => {
    vi.mocked(desktopApi.knowledgeGraph.listKnowledgeEntities).mockResolvedValue([]);
    renderPage();
    await screen.findByText(/Start building/);
    fireEvent.click(screen.getByText("Add Entity"));
    expect(screen.getByText("New Knowledge Entity")).toBeInTheDocument();
    expect(screen.getByPlaceholderText("e.g. NVIDIA")).toBeInTheDocument();
  });
});