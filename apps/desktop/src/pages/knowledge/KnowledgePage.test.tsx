import "@testing-library/jest-dom";
import { describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { KnowledgePage } from "./KnowledgePage";

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({ t: (key: string) => key }),
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspaceId: () => "ws-1",
}));

const entities = vi.hoisted(() => [
  {
    id: "e1",
    workspaceId: "ws-1",
    entityType: "company",
    name: "NVIDIA",
    description: null,
    createdAt: "2026-08-01T00:00:00Z",
    updatedAt: "2026-08-01T00:00:00Z",
  },
  {
    id: "e2",
    workspaceId: "ws-1",
    entityType: "technology",
    name: "HBM",
    description: "High bandwidth memory",
    createdAt: "2026-08-02T00:00:00Z",
    updatedAt: "2026-08-02T00:00:00Z",
  },
]);

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    knowledgeGraph: {
      listKnowledgeEntities: vi.fn().mockResolvedValue(entities),
      createKnowledgeEntity: vi.fn(),
    },
    settings: {
      getSetting: vi.fn().mockResolvedValue(null),
      setSetting: vi.fn().mockResolvedValue(undefined),
    },
  },
}));

function renderPage(entry: string) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={client}>
      <MemoryRouter initialEntries={[entry]}>
        <KnowledgePage />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

describe("KnowledgePage deep links", () => {
  it("highlights the entity named by the URL", async () => {
    renderPage("/knowledge?entity=e2");

    await waitFor(() => {
      expect(document.getElementById("knowledge-entity-e2")).toHaveAttribute(
        "aria-current",
        "true",
      );
    });
    expect(document.getElementById("knowledge-entity-e1")).not.toHaveAttribute("aria-current");
  });

  it("explains a stale entity link and offers to clear it", async () => {
    renderPage("/knowledge?entity=missing");

    expect(await screen.findByText("knowledgeEntityNotFound")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "clearFilter" })).toBeInTheDocument();
  });
});
