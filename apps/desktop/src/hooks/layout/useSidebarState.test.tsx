import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { useSidebarState } from "./useSidebarState";

const config = {
  storageKey: "test-sidebar",
  defaultState: "expanded" as const,
  defaultWidth: 220,
  minWidth: 180,
  maxWidth: 320,
};

describe("useSidebarState", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it("falls back to defaults for malformed persisted values", () => {
    localStorage.setItem("test-sidebar:state", "garbage");
    localStorage.setItem("test-sidebar:width", "not-a-number");

    const { result } = renderHook(() => useSidebarState(config));

    expect(result.current.state).toBe("expanded");
    expect(result.current.width).toBe(220);
  });

  it("clamps a persisted width to the configured bounds", () => {
    localStorage.setItem("test-sidebar:width", "9999");

    const { result } = renderHook(() => useSidebarState(config));

    expect(result.current.width).toBe(320);
  });

  it("persists toggles and widths", () => {
    const { result } = renderHook(() => useSidebarState(config));

    act(() => result.current.toggleState());
    expect(localStorage.getItem("test-sidebar:state")).toBe("collapsed");

    act(() => result.current.setWidth(275));
    expect(localStorage.getItem("test-sidebar:width")).toBe("275");

    act(() => result.current.setWidth(10_000));
    expect(result.current.width).toBe(320);
    expect(localStorage.getItem("test-sidebar:width")).toBe("320");
  });
});
