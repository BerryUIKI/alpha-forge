import "@testing-library/jest-dom";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate } from "@/lib/i18n/locale";
import type { PluginStatus } from "@/lib/desktop-api/plugins";
import { InternalPluginsPanel } from "./InternalPluginsPanel";

const pluginsMock = vi.hoisted(() => ({
  listPlugins: vi.fn(),
  setPluginEnabled: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: { plugins: pluginsMock },
}));

const networkPlugin: PluginStatus = {
  manifest: {
    id: "company-comparison",
    name: "Company Comparison",
    version: "1.0.0",
    entry: "src/index.ts",
    inputSchema: "schema.json",
    permissions: ["network"],
    window: { width: 900, height: 700, resizable: true },
  },
  enabled: true,
};
const staticPlugin: PluginStatus = {
  manifest: {
    ...networkPlugin.manifest,
    id: "valuation-model",
    name: "Valuation Model",
    permissions: [],
  },
  enabled: true,
};

function renderPanel() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return render(
    <LocaleContext.Provider
      value={{ locale: "en", setLocale: async () => undefined, t: (key) => translate("en", key) }}
    >
      <QueryClientProvider client={queryClient}>
        <InternalPluginsPanel />
      </QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("InternalPluginsPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    pluginsMock.listPlugins.mockResolvedValue([]);
    pluginsMock.setPluginEnabled.mockResolvedValue(undefined);
  });

  it("shows loading and empty states", async () => {
    let resolvePlugins: (plugins: PluginStatus[]) => void = () => undefined;
    pluginsMock.listPlugins.mockReturnValue(
      new Promise<PluginStatus[]>((resolve) => {
        resolvePlugins = resolve;
      }),
    );

    renderPanel();
    expect(screen.getByText("Loading internal plugins…")).toBeInTheDocument();
    resolvePlugins([]);
    expect(await screen.findByText("No internal plugins")).toBeInTheDocument();
  });

  it("retries a failed list request", async () => {
    pluginsMock.listPlugins
      .mockRejectedValueOnce(new Error("database unavailable"))
      .mockResolvedValueOnce([networkPlugin]);

    renderPanel();
    expect(await screen.findByRole("alert")).toHaveTextContent("Failed to load internal plugins");
    fireEvent.click(screen.getByRole("button", { name: "Try Again" }));

    expect(await screen.findByText("Company Comparison")).toBeInTheDocument();
    expect(pluginsMock.listPlugins).toHaveBeenCalledTimes(2);
  });

  it("shows declared permissions and persists an enabled-state toggle", async () => {
    let plugins = [networkPlugin, staticPlugin];
    pluginsMock.listPlugins.mockImplementation(async () => plugins);
    pluginsMock.setPluginEnabled.mockImplementation(async (pluginId: string, enabled: boolean) => {
      plugins = plugins.map((plugin) =>
        plugin.manifest.id === pluginId ? { ...plugin, enabled } : plugin,
      );
    });

    renderPanel();
    expect(await screen.findByText("Network")).toBeInTheDocument();
    expect(screen.getByText("None")).toBeInTheDocument();
    fireEvent.click(
      screen.getByRole("switch", { name: "Company Comparison: Disable internal plugin" }),
    );

    await waitFor(() =>
      expect(pluginsMock.setPluginEnabled.mock.calls[0]?.slice(0, 2)).toEqual([
        "company-comparison",
        false,
      ]),
    );
    expect(
      await screen.findByRole("switch", { name: "Company Comparison: Enable internal plugin" }),
    ).toHaveAttribute("aria-checked", "false");
    await waitFor(() => expect(pluginsMock.listPlugins).toHaveBeenCalledTimes(2));
  });

  it("keeps the current state visible when a toggle fails", async () => {
    pluginsMock.listPlugins.mockResolvedValue([networkPlugin]);
    pluginsMock.setPluginEnabled.mockRejectedValueOnce(new Error("write failed"));

    renderPanel();
    const toggle = await screen.findByRole("switch", {
      name: "Company Comparison: Disable internal plugin",
    });
    fireEvent.click(toggle);

    expect(await screen.findByRole("alert")).toHaveTextContent("Failed to update");
    expect(toggle).toHaveAttribute("aria-checked", "true");
  });
});
