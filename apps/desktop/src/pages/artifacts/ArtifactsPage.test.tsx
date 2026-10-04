import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate } from "@/lib/i18n/locale";
import { ArtifactsPage } from "./ArtifactsPage";

const mocks = vi.hoisted(() => ({
  listWorkspaces: vi.fn(),
  listArtifacts: vi.fn(),
  deleteArtifact: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: {
      listWorkspaces: mocks.listWorkspaces,
    },
    artifacts: {
      listArtifacts: mocks.listArtifacts,
      deleteArtifact: mocks.deleteArtifact,
    },
    plugins: {
      listPlugins: vi.fn().mockResolvedValue([]),
    },
  },
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspaceId: () => "ws-1",
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

describe("ArtifactsPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.listWorkspaces.mockResolvedValue([
      { id: "ws-1", name: "Main Workspace", createdAt: new Date().toISOString() },
    ]);
    mocks.listArtifacts.mockResolvedValue([
      {
        id: "art-1",
        workspaceId: "ws-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "completed",
        input: {},
        output: {},
        error: null,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
    ]);
    mocks.deleteArtifact.mockResolvedValue(undefined);
  });

  it("opens ConfirmDialog on delete button click and deletes on confirmation", async () => {
    renderWithProviders(<ArtifactsPage />);

    expect(await screen.findByText("comparison_table")).toBeInTheDocument();

    const deleteBtn = screen.getByRole("button", { name: "Delete artifact" });
    fireEvent.click(deleteBtn);

    // Confirm dialog should appear
    expect(await screen.findByRole("alertdialog")).toBeInTheDocument();
    expect(screen.getByText("Delete this artifact?")).toBeInTheDocument();

    // Click confirm "Delete" button
    const confirmBtn = screen.getByRole("button", { name: "Delete" });
    fireEvent.click(confirmBtn);

    await waitFor(() => {
      expect(mocks.deleteArtifact).toHaveBeenCalledWith("art-1");
    });
  });

  it("cancels deletion when clicking Cancel in ConfirmDialog", async () => {
    renderWithProviders(<ArtifactsPage />);

    expect(await screen.findByText("comparison_table")).toBeInTheDocument();

    const deleteBtn = screen.getByRole("button", { name: "Delete artifact" });
    fireEvent.click(deleteBtn);

    const dialog = await screen.findByRole("alertdialog");
    expect(dialog).toBeInTheDocument();

    const cancelBtn = screen.getByRole("button", { name: "Cancel" });
    fireEvent.click(cancelBtn);

    await waitFor(() => {
      expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument();
    });
    expect(mocks.deleteArtifact).not.toHaveBeenCalled();
  });
});
