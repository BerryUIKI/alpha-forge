import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router-dom";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import "@testing-library/jest-dom";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate, type MessageKey } from "@/lib/i18n/locale";
import type { Artifact } from "@/lib/desktop-api/artifacts";
import { ArtifactWindowPage } from "./ArtifactWindowPage";

const mocks = vi.hoisted(() => ({
  useArtifact: vi.fn(),
  listen: vi.fn(),
  unlisten: vi.fn(),
  close: vi.fn(),
  listeners: new Map<string, (event: { payload: unknown }) => void>(),
}));
vi.mock("@/features/artifacts/hooks/useArtifacts", () => ({
  useArtifact: mocks.useArtifact,
}));
vi.mock("@tauri-apps/api/event", () => ({
  listen: mocks.listen,
}));
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ close: mocks.close }),
}));

const artifactId = "2a707687-3fc5-4b02-81ba-043830213244";
const artifactWindowRoute = {
  path: "/artifact/:artifactId/:artifactType",
  element: <ArtifactWindowPage />,
};
const comparisonOutput = (companyName: string) => ({
  companies: [
    {
      ticker: "AFG",
      name: companyName,
      metrics: { revenue: 100 },
    },
  ],
  comparisonDimensions: ["revenue"],
});
function createArtifact(overrides: Partial<Artifact> = {}): Artifact {
  return {
    id: artifactId,
    workspaceId: "workspace-1",
    taskId: null,
    artifactType: "comparison_table",
    status: "completed",
    input: comparisonOutput("Input Company"),
    output: comparisonOutput("Initial Company"),
    error: null,
    createdAt: "2026-08-14T00:00:00.000Z",
    updatedAt: "2026-08-14T00:00:00.000Z",
    ...overrides,
  };
}
function renderRoute(path = `/artifact/${artifactId}/comparison_table`) {
  const router = createMemoryRouter([artifactWindowRoute], {
    initialEntries: [path],
  });
  return render(
    <LocaleContext.Provider
      value={{
        locale: "en",
        setLocale: async () => undefined,
        t: (key: MessageKey) => translate("en", key),
      }}
    >
      <RouterProvider router={router} />
    </LocaleContext.Provider>,
  );
}
describe("ArtifactWindowPage", () => {
  beforeEach(() => {
    document.documentElement.classList.remove("dark", "light");
    mocks.listeners.clear();
    mocks.useArtifact.mockReset();
    mocks.listen.mockReset();
    mocks.unlisten.mockReset();
    mocks.close.mockReset();
    mocks.close.mockResolvedValue(undefined);
    mocks.unlisten.mockImplementation(() => undefined);
    mocks.listen.mockImplementation(
      async (event: string, callback: (event: { payload: unknown }) => void) => {
        mocks.listeners.set(event, callback);
        return mocks.unlisten;
      },
    );
    mocks.useArtifact.mockReturnValue({
      data: createArtifact(),
      isLoading: false,
      error: null,
      refetch: vi.fn(),
    });
  });

  afterEach(() => {
    document.documentElement.classList.remove("dark", "light");
    vi.clearAllMocks();
  });

  it("renders a registered renderer on the isolated top-level route", async () => {
    renderRoute();

    expect(await screen.findByText("Initial Company")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "comparison_table" })).toBeInTheDocument();
    expect(mocks.useArtifact).toHaveBeenCalledWith(artifactId);
  });
  it("rejects invalid UUID and unsafe artifact type route values before IPC", () => {
    const invalidUuidView = renderRoute(`/artifact/not-a-uuid/comparison_table`);

    expect(
      screen.getByRole("heading", { name: "Invalid artifact window route" }),
    ).toBeInTheDocument();
    expect(mocks.useArtifact).toHaveBeenCalledWith("");

    invalidUuidView.unmount();
    renderRoute(`/artifact/${artifactId}/%3Cscript%3E`);

    expect(
      screen.getByRole("heading", { name: "Invalid artifact window route" }),
    ).toBeInTheDocument();
    expect(mocks.useArtifact).toHaveBeenLastCalledWith("");
  });
  it("rejects persisted artifact ID and type mismatches", () => {
    mocks.useArtifact.mockReturnValue({
      data: createArtifact({ id: "d3d1c3a5-2b2e-4c1c-9f77-2a6e0b4c9f22" }),
      isLoading: false,
      error: null,
      refetch: vi.fn(),
    });

    renderRoute();

    expect(screen.getByRole("heading", { name: "Artifact route mismatch" })).toBeInTheDocument();
    expect(screen.queryByText("Initial Company")).not.toBeInTheDocument();
  });
  it("updates renderer data and theme from artifact events", async () => {
    document.documentElement.classList.add("dark");
    const view = renderRoute();
    await waitFor(() => expect(mocks.listeners.size).toBe(2));

    await act(async () => {
      mocks.listeners.get("artifact:update")?.({
        payload: comparisonOutput("Updated Company"),
      });
    });

    expect(screen.getByText("Updated Company")).toBeInTheDocument();
    await act(async () => {
      mocks.listeners.get("artifact:theme")?.({ payload: { theme: "light" } });
    });
    expect(document.documentElement).toHaveClass("light");
    expect(document.documentElement).not.toHaveClass("dark");
    await act(async () => {
      mocks.listeners.get("artifact:theme")?.({ payload: { theme: "dark" } });
    });
    expect(document.documentElement).toHaveClass("dark");
    expect(document.documentElement).not.toHaveClass("light");
    await act(async () => {
      mocks.listeners.get("artifact:theme")?.({ payload: { theme: "system" } });
    });
    expect(document.documentElement).toHaveClass("dark");
    expect(document.documentElement).not.toHaveClass("light");
    view.unmount();
    expect(mocks.unlisten).toHaveBeenCalledTimes(2);
  });
  it("unlistens listeners that resolve after the window unmounts", async () => {
    const resolveUnlisteners: Array<(unlisten: typeof mocks.unlisten) => void> = [];
    mocks.listen.mockImplementation(
      async (event: string, callback: (event: { payload: unknown }) => void) => {
        mocks.listeners.set(event, callback);
        return new Promise<typeof mocks.unlisten>((resolve) => {
          resolveUnlisteners.push(resolve);
        });
      },
    );

    const view = renderRoute();
    view.unmount();

    await act(async () => {
      for (const resolve of resolveUnlisteners) {
        resolve(mocks.unlisten);
      }
    });

    expect(mocks.unlisten).toHaveBeenCalledTimes(2);
  });

  it("closes through the current Tauri window", async () => {
    renderRoute();

    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Close artifact window" }));
    });

    expect(mocks.close).toHaveBeenCalledOnce();
  });

  it("shows a close failure in every shell state", async () => {
    mocks.close.mockRejectedValueOnce(new Error("permission denied"));
    renderRoute();
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Close artifact window" }));
    });

    expect(await screen.findByRole("alert")).toHaveTextContent("Unable to close artifact window");
  });

  it("shows unsupported renderer and supports retry on load failure", () => {
    const refetch = vi.fn();
    mocks.useArtifact.mockReturnValue({
      data: createArtifact({ artifactType: "unsupported_type" }),
      isLoading: false,
      error: null,
      refetch,
    });

    const unsupportedView = renderRoute(`/artifact/${artifactId}/unsupported_type`);

    expect(screen.getByRole("heading", { name: "No renderer available" })).toBeInTheDocument();
    unsupportedView.unmount();

    mocks.useArtifact.mockReturnValue({
      data: undefined,
      isLoading: false,
      error: new Error("offline"),
      refetch,
    });
    renderRoute();
    fireEvent.click(screen.getByRole("button", { name: "Try Again" }));
    expect(refetch).toHaveBeenCalledOnce();
  });
});
