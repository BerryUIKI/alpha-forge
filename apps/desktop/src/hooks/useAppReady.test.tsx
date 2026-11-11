import { renderHook, act, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useAppReady } from "./useAppReady";
import { desktopApi } from "@/lib/desktop-api";

const mocks = vi.hoisted(() => ({
  listen: vi.fn(),
  unlisten: vi.fn(),
  listeners: new Map<string, (event: { payload: unknown }) => void>(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: mocks.listen,
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    system: {
      getAppStatus: vi.fn(),
    },
  },
}));

describe("useAppReady", () => {
  beforeEach(() => {
    mocks.listeners.clear();
    mocks.listen.mockReset();
    mocks.unlisten.mockReset();
    mocks.unlisten.mockImplementation(() => undefined);
    mocks.listen.mockImplementation(
      async (event: string, callback: (event: { payload: unknown }) => void) => {
        mocks.listeners.set(event, callback);
        return mocks.unlisten;
      },
    );
    vi.mocked(desktopApi.system.getAppStatus).mockResolvedValue({
      status: "initializing",
      errorCode: null,
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
    vi.useRealTimers();
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  });

  function enterTauri() {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  }

  it("defaults to true in non-Tauri environment without probing", () => {
    const { result } = renderHook(() => useAppReady());

    expect(result.current.isReady).toBe(true);
    expect(result.current.initError).toBe(null);
    expect(mocks.listen).not.toHaveBeenCalled();
    expect(desktopApi.system.getAppStatus).not.toHaveBeenCalled();
  });

  it("starts not ready and registers listeners before probing readiness", async () => {
    enterTauri();
    const { result } = renderHook(() => useAppReady());

    expect(result.current.isReady).toBe(false);

    await waitFor(() => {
      expect(mocks.listen).toHaveBeenCalledWith("app:ready", expect.any(Function));
      expect(mocks.listen).toHaveBeenCalledWith("app:init-failed", expect.any(Function));
    });
    expect(desktopApi.system.getAppStatus).toHaveBeenCalledTimes(1);
  });

  it("becomes ready on app:ready event", async () => {
    enterTauri();
    const { result } = renderHook(() => useAppReady());

    await waitFor(() => expect(mocks.listeners.has("app:ready")).toBe(true));

    act(() => {
      mocks.listeners.get("app:ready")?.({ payload: undefined });
    });

    expect(result.current.isReady).toBe(true);
    expect(result.current.initError).toBe(null);
  });

  it("recovers readiness when a panel mounts after the ready event fired", async () => {
    enterTauri();
    vi.mocked(desktopApi.system.getAppStatus).mockResolvedValue({
      status: "ready",
      errorCode: null,
    });

    const { result } = renderHook(() => useAppReady());

    expect(result.current.isReady).toBe(false);
    await waitFor(() => expect(result.current.isReady).toBe(true));
    // No event was delivered; the snapshot probe supplied the state.
    expect(mocks.listeners.get("app:ready")).toBeTypeOf("function");
  });

  it("sets initError on app:init-failed event", async () => {
    enterTauri();
    const { result } = renderHook(() => useAppReady());

    await waitFor(() => expect(mocks.listeners.has("app:init-failed")).toBe(true));

    act(() => {
      mocks.listeners.get("app:init-failed")?.({ payload: "DB_INIT_FAILED" });
    });

    expect(result.current.isReady).toBe(false);
    expect(result.current.initError).toBe("DB_INIT_FAILED");
  });

  it("sets initError when the readiness probe reports a failure", async () => {
    enterTauri();
    vi.mocked(desktopApi.system.getAppStatus).mockResolvedValue({
      status: "failed",
      errorCode: "APP_STATE_FAILED",
    });

    const { result } = renderHook(() => useAppReady());

    await waitFor(() => expect(result.current.initError).toBe("APP_STATE_FAILED"));
  });

  it("reports listener registration failure", async () => {
    enterTauri();
    mocks.listen.mockRejectedValue(new Error("listen unavailable"));
    vi.mocked(desktopApi.system.getAppStatus).mockRejectedValue(new Error("probe unavailable"));

    const { result } = renderHook(() => useAppReady());

    await waitFor(() => expect(result.current.listenerError).toBe(true));
    expect(result.current.isReady).toBe(false);
  });

  it("flags prolonged initialization as stalled", async () => {
    enterTauri();
    vi.useFakeTimers();
    const { result } = renderHook(() => useAppReady());

    await act(async () => {
      await Promise.resolve();
    });
    expect(result.current.isStalled).toBe(false);

    act(() => {
      vi.advanceTimersByTime(20_000);
    });

    expect(result.current.isStalled).toBe(true);
  });

  it("cleans up listeners on unmount", async () => {
    enterTauri();

    const { unmount } = renderHook(() => useAppReady());

    await waitFor(() => {
      expect(mocks.listeners.size).toBe(2);
    });

    unmount();

    expect(mocks.unlisten).toHaveBeenCalledTimes(2);
  });
});
