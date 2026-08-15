import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

import { createPluginArtifact, listPlugins, setPluginEnabled } from "./plugins";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked(invoke);
const pluginStatus = {
  manifest: {
    id: "company-comparison",
    name: "Company Comparison",
    version: "1.0.0",
    entry: "src/index.ts",
    inputSchema: "schema.json",
    permissions: [],
    window: { width: 900, height: 700, resizable: true },
  },
  enabled: true,
};

describe("plugin registry API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("lists registered plugins", async () => {
    mockInvoke.mockResolvedValueOnce([pluginStatus]);

    await expect(listPlugins()).resolves.toEqual([pluginStatus]);
    expect(mockInvoke).toHaveBeenCalledWith("list_plugins");
  });

  it("rejects malformed plugin status responses", async () => {
    mockInvoke.mockResolvedValueOnce([{ ...pluginStatus, enabled: "yes" }]);

    await expect(listPlugins()).rejects.toThrow();
  });

  it("changes a plugin enabled state", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);

    await expect(setPluginEnabled("company-comparison", false)).resolves.toBeUndefined();
    expect(mockInvoke).toHaveBeenCalledWith("set_plugin_enabled", {
      pluginId: "company-comparison",
      enabled: false,
    });
  });

  it("creates an artifact through a named plugin", async () => {
    mockInvoke.mockResolvedValueOnce({ id: "artifact-1" });
    const input = {
      companies: [{ ticker: "AAA" }, { ticker: "BBB" }],
      comparisonDimensions: ["revenue"],
    };

    await createPluginArtifact("workspace-1", "company-comparison", input);
    expect(mockInvoke).toHaveBeenCalledWith("create_plugin_artifact", {
      workspaceId: "workspace-1",
      pluginId: "company-comparison",
      input,
    });
  });

  it("rejects invalid plugin input before invoking Tauri", async () => {
    expect(() =>
      createPluginArtifact("workspace-1", "company-comparison", {
        companies: [{ ticker: "AAA" }],
        comparisonDimensions: [],
      }),
    ).toThrow();
    expect(mockInvoke).not.toHaveBeenCalled();
  });
});
