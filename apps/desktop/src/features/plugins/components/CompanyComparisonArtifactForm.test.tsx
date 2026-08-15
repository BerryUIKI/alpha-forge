import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import "@testing-library/jest-dom";

import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate, type MessageKey } from "@/lib/i18n/locale";
import { CompanyComparisonArtifactForm } from "./CompanyComparisonArtifactForm";

const mocks = vi.hoisted(() => ({
  listPlugins: vi.fn(),
  createPluginArtifact: vi.fn(),
  startViewingArtifact: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    plugins: {
      listPlugins: mocks.listPlugins,
      createPluginArtifact: mocks.createPluginArtifact,
    },
    artifacts: { startViewingArtifact: mocks.startViewingArtifact },
  },
}));

const artifactId = "2a707687-3fc5-4b02-81ba-043830213244";
const artifact = { id: artifactId };
const plugin = {
  manifest: { id: "company-comparison" },
  enabled: true,
};

function renderForm(onArtifactCreated = vi.fn()) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <LocaleContext.Provider
          value={{
            locale: "en",
            setLocale: async () => undefined,
            t: (key: MessageKey) => translate("en", key),
          }}
        >
          <CompanyComparisonArtifactForm
            workspaceId="workspace-1"
            onArtifactCreated={onArtifactCreated}
          />
        </LocaleContext.Provider>
      </MemoryRouter>
    </QueryClientProvider>,
  );
  return onArtifactCreated;
}

function fillValidComparison() {
  fireEvent.change(screen.getByLabelText("First ticker"), { target: { value: "aapl" } });
  fireEvent.change(screen.getByLabelText("First value"), { target: { value: "100" } });
  fireEvent.change(screen.getByLabelText("Second ticker"), { target: { value: "MSFT" } });
  fireEvent.change(screen.getByLabelText("Second value"), { target: { value: "120" } });
}

describe("CompanyComparisonArtifactForm", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.listPlugins.mockResolvedValue([plugin]);
    mocks.createPluginArtifact.mockResolvedValue(artifact);
    mocks.startViewingArtifact.mockResolvedValue(artifact);
  });

  it("blocks creation while the bundled plugin is disabled", async () => {
    mocks.listPlugins.mockResolvedValue([{ ...plugin, enabled: false }]);
    renderForm();

    expect(await screen.findByText("Company comparison is disabled")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Manage internal plugins" })).toHaveAttribute(
      "href",
      "/settings#internal-plugins",
    );
    expect(
      screen.queryByRole("button", { name: "Create and open Artifact" }),
    ).not.toBeInTheDocument();
    expect(mocks.createPluginArtifact).not.toHaveBeenCalled();
  });

  it("rejects duplicate tickers before invoking the desktop API", async () => {
    renderForm();
    await screen.findByRole("button", { name: "Create and open Artifact" });
    fireEvent.change(screen.getByLabelText("First ticker"), { target: { value: "AAPL" } });
    fireEvent.change(screen.getByLabelText("First value"), { target: { value: "100" } });
    fireEvent.change(screen.getByLabelText("Second ticker"), { target: { value: "aapl" } });
    fireEvent.change(screen.getByLabelText("Second value"), { target: { value: "120" } });
    fireEvent.click(screen.getByRole("button", { name: "Create and open Artifact" }));

    expect(await screen.findByRole("alert")).toHaveTextContent("Enter two different valid tickers");
    expect(mocks.createPluginArtifact).not.toHaveBeenCalled();
  });

  it("creates, selects, and opens a validated completed Artifact", async () => {
    const onArtifactCreated = renderForm();
    await screen.findByRole("button", { name: "Create and open Artifact" });
    fillValidComparison();
    fireEvent.click(screen.getByRole("button", { name: "Create and open Artifact" }));

    await waitFor(() => expect(onArtifactCreated).toHaveBeenCalledWith(artifactId));
    expect(mocks.createPluginArtifact).toHaveBeenCalledWith("workspace-1", "company-comparison", {
      companies: [
        { ticker: "AAPL", name: "AAPL", metrics: { revenue: 100 } },
        { ticker: "MSFT", name: "MSFT", metrics: { revenue: 120 } },
      ],
      comparisonDimensions: ["revenue"],
    });
    expect(mocks.startViewingArtifact).toHaveBeenCalledWith(artifactId);
  });

  it("retries opening the created Artifact without creating a duplicate", async () => {
    mocks.startViewingArtifact
      .mockRejectedValueOnce(new Error("window unavailable"))
      .mockResolvedValueOnce(artifact);
    renderForm();
    await screen.findByRole("button", { name: "Create and open Artifact" });
    fillValidComparison();
    fireEvent.click(screen.getByRole("button", { name: "Create and open Artifact" }));

    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent("The Artifact was created");
    fireEvent.click(screen.getByRole("button", { name: "Retry opening" }));
    await waitFor(() => expect(mocks.startViewingArtifact).toHaveBeenCalledTimes(2));
    expect(mocks.createPluginArtifact).toHaveBeenCalledTimes(1);
  });
});
