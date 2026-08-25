// Tests for Settings desktop API.

import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("Settings API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("getAppInfo parses valid response via Zod", async () => {
    const mockInfo = {
      name: "AlphaForge",
      version: "0.1.0",
      identifier: "com.alphaforge.app",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockInfo);

    const { getAppInfo } = await import("@/lib/desktop-api/settings");
    const result = await getAppInfo();

    expect(invoke).toHaveBeenCalledWith("get_app_info");
    expect(result).toEqual(mockInfo);
  });

  it("getAppInfo rejects malformed response", async () => {
    const malformed = {
      name: "AlphaForge",
      // missing version and identifier
    };

    vi.mocked(invoke).mockResolvedValueOnce(malformed);

    const { getAppInfo } = await import("@/lib/desktop-api/settings");
    await expect(getAppInfo()).rejects.toThrow();
  });

  it("listSettings parses valid list via Zod", async () => {
    const mockItems = [
      { key: "app.theme", value: "dark" },
      { key: "app.locale", value: "zh-CN" },
    ];

    vi.mocked(invoke).mockResolvedValueOnce(mockItems);

    const { listSettings } = await import("@/lib/desktop-api/settings");
    const result = await listSettings();

    expect(invoke).toHaveBeenCalledWith("list_settings");
    expect(result).toEqual(mockItems);
  });

  it("listSettings rejects malformed list items", async () => {
    const malformed = [{ invalid: "data" }];

    vi.mocked(invoke).mockResolvedValueOnce(malformed);

    const { listSettings } = await import("@/lib/desktop-api/settings");
    await expect(listSettings()).rejects.toThrow();
  });

  it("getSetting returns string or null", async () => {
    vi.mocked(invoke).mockResolvedValueOnce("dark");

    const { getSetting } = await import("@/lib/desktop-api/settings");
    const result = await getSetting("app.theme");

    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "app.theme" });
    expect(result).toBe("dark");
  });
});
