import { renderHook, act, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useAppReady } from "./useAppReady";

const mocks = vi.hoisted(() => ({
  listen: vi.fn(),
  unlisten: vi.fn(),
  listeners: new Map<string, (event: { payload: unknown }) => void>(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: mocks.listen,
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
      }
    );
  });

  afterEach(() => {
    vi.clearAllMocks();
    // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  });

  it("defaults to true in non-Tauri environment", () => {
    const { result } = renderHook(() => useAppReady());

    expect(result.current.isReady).toBe(true);
    expect(result.current.initError).toBe(null);
    expect(mocks.listen).not.toHaveBeenCalled();
  });

  it("starts not ready in Tauri environment", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const { result } = renderHook(() => useAppReady());

    expect(result.current.isReady).toBe(false);
    expect(result.current.initError).toBe(null);

    // Wait for async setupListeners to register both listeners
    await waitFor(() => {
      expect(mocks.listen).toHaveBeenCalledWith("app:ready", expect.any(Function));
      expect(mocks.listen).toHaveBeenCalledWith("app:init-failed", expect.any(Function));
    });
  });

  it("becomes ready on app:ready event", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const { result } = renderHook(() => useAppReady());

    expect(result.current.isReady).toBe(false);

    await waitFor(() => {
      expect(mocks.listeners.has("app:ready")).toBe(true);
    });

    act(() => {
      mocks.listeners.get("app:ready")?.({ payload: undefined });
    });

    expect(result.current.isReady).toBe(true);
    expect(result.current.initError).toBe(null);
  });

  it("sets initError on app:init-failed event", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const { result } = renderHook(() => useAppReady());

    expect(result.current.initError).toBe(null);

    await waitFor(() => {
      expect(mocks.listeners.has("app:init-failed")).toBe(true);
    });

    act(() => {
      mocks.listeners.get("app:init-failed")?.({ payload: "DB_INIT_FAILED" });
    });

    expect(result.current.isReady).toBe(false);
    expect(result.current.initError).toBe("DB_INIT_FAILED");
  });

  it("cleans up listeners on unmount", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};

    const { unmount } = renderHook(() => useAppReady());

    // Wait for both listeners to be registered before unmounting
    await waitFor(() => {
      expect(mocks.listeners.size).toBe(2);
    });

    unmount();

    expect(mocks.unlisten).toHaveBeenCalledTimes(2);
  });
});
