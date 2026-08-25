import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

import { checkForUpdate, exportLocalBackup, getSystemInfo } from "./system";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked(invoke);

describe("system API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("requests a local backup export", async () => {
    mockInvoke.mockResolvedValueOnce("/tmp/alpha-forge-backup.db");

    await expect(exportLocalBackup()).resolves.toBe("/tmp/alpha-forge-backup.db");
    expect(mockInvoke).toHaveBeenCalledWith("export_local_backup");
  });

  it("checks the release feed", async () => {
    const release = {
      currentVersion: "0.1.0",
      latestVersion: "0.2.0",
      releaseUrl: "https://github.com/BerryUIKI/alpha-forge/releases/tag/v0.2.0",
      updateAvailable: true,
    };
    mockInvoke.mockResolvedValueOnce(release);

    await expect(checkForUpdate()).resolves.toEqual(release);
    expect(mockInvoke).toHaveBeenCalledWith("check_for_update");
  });

  it("parses the camelCase system information contract", async () => {
    const systemInfo = {
      appName: "AlphaForge",
      appVersion: "0.1.0",
      platform: "windows",
      architecture: "x86_64",
    };
    mockInvoke.mockResolvedValueOnce(systemInfo);

    await expect(getSystemInfo()).resolves.toEqual(systemInfo);
    expect(mockInvoke).toHaveBeenCalledWith("get_system_info");
  });

  it("rejects malformed or legacy system information responses", async () => {
    mockInvoke.mockResolvedValueOnce({
      appName: "AlphaForge",
      appVersion: "0.1.0",
      platform: "windows",
    });
    await expect(getSystemInfo()).rejects.toThrow();

    mockInvoke.mockResolvedValueOnce({ os: "windows", arch: "x86_64", version: "0.1.0" });
    await expect(getSystemInfo()).rejects.toThrow();
  });
});
