// Tests for GlobalSearchDialog.

import "@testing-library/jest-dom";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter, useLocation } from "react-router-dom";
import { useMemo, useState, type PropsWithChildren } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { GlobalSearchDialog } from "./GlobalSearchDialog";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate, type MessageKey } from "@/lib/i18n/locale";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: { listWorkspaces: vi.fn() },
    research: {
      listResearchProjects: vi.fn(),
      listResearchDocuments: vi.fn(),
      listResearchReports: vi.fn(),
    },
    thesis: { listTheses: vi.fn() },
    knowledgeGraph: { listKnowledgeEntities: vi.fn() },
    artifacts: { listArtifacts: vi.fn() },
  },
}));

const { desktopApi } = await import("@/lib/desktop-api");

const mocks = desktopApi as unknown as {
  workspace: { listWorkspaces: ReturnType<typeof vi.fn> };
  research: {
    listResearchProjects: ReturnType<typeof vi.fn>;
    listResearchDocuments: ReturnType<typeof vi.fn>;
    listResearchReports: ReturnType<typeof vi.fn>;
  };
  thesis: { listTheses: ReturnType<typeof vi.fn> };
  knowledgeGraph: { listKnowledgeEntities: ReturnType<typeof vi.fn> };
  artifacts: { listArtifacts: ReturnType<typeof vi.fn> };
};

/** Pins the locale to English so assertions match a stable catalog. */
function EnLocaleProvider({ children }: PropsWithChildren) {
  const value = useMemo(
    () => ({
      locale: "en" as const,
      setLocale: async () => undefined,
      t: (key: MessageKey) => translate("en", key),
    }),
    [],
  );
  return <LocaleContext.Provider value={value}>{children}</LocaleContext.Provider>;
}

function Harness({ initialOpen }: { initialOpen: boolean }) {
  const [isOpen, setIsOpen] = useState(initialOpen);
  const location = useLocation();
  return (
    <>
      <GlobalSearchDialog isOpen={isOpen} onClose={() => setIsOpen(false)} />
      <span data-testid="path">{location.pathname + location.search}</span>
    </>
  );
}

function renderDialog(initialOpen: boolean) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={client}>
      <MemoryRouter initialEntries={["/research"]}>
        <EnLocaleProvider>
          <Harness initialOpen={initialOpen} />
        </EnLocaleProvider>
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

describe("GlobalSearchDialog", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.workspace.listWorkspaces.mockResolvedValue([
      { id: "workspace-1", name: "Workspace" },
    ]);
    mocks.research.listResearchProjects.mockResolvedValue([
      {
        id: "project-1",
        workspace_id: "workspace-1",
        title: "AI Infrastructure",
        description: "Demand for compute",
        status: "active",
        created_at: "2026-08-01T00:00:00Z",
        updated_at: "2026-08-01T00:00:00Z",
      },
    ]);
    mocks.research.listResearchDocuments.mockResolvedValue([]);
    mocks.research.listResearchReports.mockResolvedValue([]);
    mocks.thesis.listTheses.mockResolvedValue([]);
    mocks.knowledgeGraph.listKnowledgeEntities.mockResolvedValue([]);
    mocks.artifacts.listArtifacts.mockResolvedValue([]);
  });

  it("renders nothing when closed", () => {
    renderDialog(false);
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("renders an empty-query hint when open with no text", async () => {
    renderDialog(true);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
    // Wait for the workspace query to resolve so the placeholder branch renders.
    expect(
      await screen.findByText("Search projects, documents, theses, knowledge…"),
    ).toBeInTheDocument();
  });

  it("navigates to a project on Enter and closes", async () => {
    renderDialog(true);
    const input = screen.getByRole("textbox") as HTMLInputElement;

    fireEvent.change(input, { target: { value: "AI Infra" } });

    await waitFor(() =>
      expect(screen.getByRole("button", { name: /AI Infrastructure/ })).toBeInTheDocument(),
    );

    fireEvent.keyDown(input, { key: "Enter" });

    await waitFor(() =>
      expect(screen.getByTestId("path")).toHaveTextContent(
        "/research?workspace=workspace-1&project=project-1",
      ),
    );
    // Palette is closed, so the dialog unmounts.
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("closes on Escape", async () => {
    renderDialog(true);
    await screen.findByRole("textbox");
    fireEvent.keyDown(screen.getByRole("textbox"), { key: "Escape" });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });
});
