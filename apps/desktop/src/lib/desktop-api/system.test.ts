import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

import { checkForUpdate, exportLocalBackup } from "./system";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked(invoke);

describe("system API", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("requests a local backup export", async () => {
    mockInvoke.mockResolvedValueOnce("/tmp/investment-os-backup.db");

    await expect(exportLocalBackup()).resolves.toBe("/tmp/investment-os-backup.db");
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
});
