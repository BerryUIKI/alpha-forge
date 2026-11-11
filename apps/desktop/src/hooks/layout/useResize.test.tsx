import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { useResize } from "./useResize";

function keyEvent(key: string, shiftKey = false) {
  return {
    key,
    shiftKey,
    preventDefault: vi.fn(),
  } as unknown as React.KeyboardEvent;
}

describe("useResize keyboard resizing", () => {
  it("grows and shrinks a right-edge pane with arrow keys", () => {
    const { result } = renderHook(() =>
      useResize({ initialWidth: 220, minWidth: 180, maxWidth: 320, direction: "right" }),
    );

    act(() => result.current.handleKeyDown(keyEvent("ArrowRight")));
    expect(result.current.width).toBe(236);

    act(() => result.current.handleKeyDown(keyEvent("ArrowLeft")));
    expect(result.current.width).toBe(220);
  });

  it("inverts arrow direction for a left-edge pane", () => {
    const { result } = renderHook(() =>
      useResize({ initialWidth: 320, minWidth: 260, maxWidth: 400, direction: "left" }),
    );

    act(() => result.current.handleKeyDown(keyEvent("ArrowLeft")));
    expect(result.current.width).toBe(336);

    act(() => result.current.handleKeyDown(keyEvent("ArrowRight")));
    expect(result.current.width).toBe(320);
  });

  it("uses a larger step with Shift and clamps to bounds", () => {
    const { result } = renderHook(() =>
      useResize({ initialWidth: 300, minWidth: 180, maxWidth: 320, direction: "right" }),
    );

    act(() => result.current.handleKeyDown(keyEvent("ArrowRight", true)));
    expect(result.current.width).toBe(320);

    act(() => result.current.handleKeyDown(keyEvent("ArrowRight", true)));
    expect(result.current.width).toBe(320);

    act(() => result.current.handleKeyDown(keyEvent("ArrowLeft", true)));
    expect(result.current.width).toBe(272);
  });

  it("supports Home and End for min and max widths", () => {
    const { result } = renderHook(() =>
      useResize({ initialWidth: 220, minWidth: 180, maxWidth: 320, direction: "right" }),
    );

    act(() => result.current.handleKeyDown(keyEvent("End")));
    expect(result.current.width).toBe(320);

    act(() => result.current.handleKeyDown(keyEvent("Home")));
    expect(result.current.width).toBe(180);
  });

  it("ignores unrelated keys", () => {
    const event = keyEvent("Enter");
    const { result } = renderHook(() =>
      useResize({ initialWidth: 220, minWidth: 180, maxWidth: 320 }),
    );

    act(() => result.current.handleKeyDown(event));

    expect(result.current.width).toBe(220);
    expect(event.preventDefault).not.toHaveBeenCalled();
  });

  it("resizes with a mouse drag and stops on mouseup", () => {
    const { result } = renderHook(() =>
      useResize({ initialWidth: 220, minWidth: 180, maxWidth: 320, direction: "right" }),
    );

    act(() =>
      result.current.startResize({
        preventDefault: vi.fn(),
        clientX: 100,
      } as unknown as React.MouseEvent),
    );
    expect(result.current.isResizing).toBe(true);

    act(() => {
      document.dispatchEvent(new MouseEvent("mousemove", { clientX: 160 }));
    });
    expect(result.current.width).toBe(280);

    act(() => {
      document.dispatchEvent(new MouseEvent("mouseup"));
    });
    expect(result.current.isResizing).toBe(false);
  });
});
