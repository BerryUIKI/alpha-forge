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
const artifact = {
  id: "2a707687-3fc5-4b02-81ba-043830213244",
  workspaceId: "workspace-1",
  taskId: null,
  artifactType: "comparison_table",
  status: "completed",
  input: {},
  output: {},
  error: null,
  createdAt: "2026-08-15T00:00:00Z",
  updatedAt: "2026-08-15T00:00:00Z",
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
    mockInvoke.mockResolvedValueOnce(artifact);
    const input = {
      companies: [
        { ticker: "aaa", name: "Alpha", metrics: { revenue: 10 } },
        { ticker: "BBB", name: "Beta", metrics: { revenue: 20 } },
      ],
      comparisonDimensions: ["revenue"],
    };

    await expect(
      createPluginArtifact("workspace-1", "company-comparison", input),
    ).resolves.toEqual(artifact);
    expect(mockInvoke).toHaveBeenCalledWith("create_plugin_artifact", {
      workspaceId: "workspace-1",
      pluginId: "company-comparison",
      input: {
        ...input,
        companies: [{ ...input.companies[0], ticker: "AAA" }, input.companies[1]],
      },
    });
  });

  it("rejects invalid plugin input before invoking Tauri", async () => {
    await expect(
      createPluginArtifact("workspace-1", "company-comparison", {
        companies: [{ ticker: "AAA" }],
        comparisonDimensions: [],
      }),
    ).rejects.toThrow();
    expect(mockInvoke).not.toHaveBeenCalled();
  });

  it("rejects malformed artifact responses", async () => {
    mockInvoke.mockResolvedValueOnce({ ...artifact, status: "done" });

    await expect(
      createPluginArtifact("workspace-1", "company-comparison", {
        companies: [
          { ticker: "AAA", name: "Alpha", metrics: { revenue: 10 } },
          { ticker: "BBB", name: "Beta", metrics: { revenue: 20 } },
        ],
        comparisonDimensions: ["revenue"],
      }),
    ).rejects.toThrow();
  });
});
