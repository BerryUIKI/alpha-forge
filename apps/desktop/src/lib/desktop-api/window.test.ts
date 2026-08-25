import { beforeEach, describe, expect, it, vi } from "vitest";
import { close, minimize, toggleMaximize } from "./window";

const controls = vi.hoisted(() => ({
  minimize: vi.fn().mockResolvedValue(undefined),
  toggleMaximize: vi.fn().mockResolvedValue(undefined),
  close: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => controls,
}));

beforeEach(() => {
  vi.clearAllMocks();
});

describe("desktopApi.window", () => {
  it("delegates the three custom title-bar controls to the current Tauri window", async () => {
    await minimize();
    await toggleMaximize();
    await close();

    expect(controls.minimize).toHaveBeenCalledOnce();
    expect(controls.toggleMaximize).toHaveBeenCalledOnce();
    expect(controls.close).toHaveBeenCalledOnce();
  });
});
